use std::sync::{Arc, Weak};

use futures::StreamExt;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};
use wayle_traits::ModelMonitoring;

use super::{
    TrackMetadata,
    art::{ArtResolver, ResolveResult},
};
use crate::{error::Error, proxy::MediaPlayer2PlayerProxy};

impl ModelMonitoring for TrackMetadata {
    type Error = Error;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let Some(ref proxy) = self.proxy else {
            return Err(Error::Initialization(String::from("missing proxy")));
        };

        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(Error::Initialization(String::from(
                "missing cancellation token",
            )));
        };

        let weak_self = Arc::downgrade(&self);

        tokio::spawn(monitor_dbus(
            weak_self.clone(),
            proxy.clone(),
            cancellation_token.clone(),
        ));

        if let Some(ref resolver) = self.art_resolver {
            tokio::spawn(resolve_art_changes(
                weak_self,
                resolver.clone(),
                cancellation_token.clone(),
            ));
        }

        Ok(())
    }
}

async fn monitor_dbus(
    weak_metadata: Weak<TrackMetadata>,
    proxy: MediaPlayer2PlayerProxy<'static>,
    cancellation_token: CancellationToken,
) {
    let mut metadata_changed = proxy.receive_metadata_changed().await;

    loop {
        let Some(metadata) = weak_metadata.upgrade() else {
            return;
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => {
                debug!("metadata D-Bus monitor cancelled");
                return;
            }
            Some(change) = metadata_changed.next() => {
                if let Ok(new_metadata) = change.get().await {
                    TrackMetadata::update_from_dbus(&metadata, new_metadata);
                }
            }
            else => break
        }
    }

    debug!("metadata D-Bus monitor ended");
}

async fn resolve_art_changes(
    weak_metadata: Weak<TrackMetadata>,
    resolver: ArtResolver,
    cancellation_token: CancellationToken,
) {
    let Some(metadata) = weak_metadata.upgrade() else {
        return;
    };
    let mut art_url_stream = Box::pin(metadata.art_url.watch());
    drop(metadata);

    let mut pending_download: Option<JoinHandle<()>> = None;

    loop {
        tokio::select! {
            _ = cancellation_token.cancelled() => {
                abort_pending(&mut pending_download);
                debug!("art resolver cancelled");
                return;
            }
            Some(art_url) = art_url_stream.next() => {
                abort_pending(&mut pending_download);
                pending_download = handle_art_url_change(
                    art_url,
                    &resolver,
                    &weak_metadata,
                );
            }
            else => break,
        }
    }

    debug!("art resolver ended");
}

fn handle_art_url_change(
    art_url: Option<String>,
    resolver: &ArtResolver,
    weak_metadata: &Weak<TrackMetadata>,
) -> Option<JoinHandle<()>> {
    let Some(url) = art_url else {
        set_cover_art(weak_metadata, None);
        return None;
    };

    match resolver.resolve(&url) {
        ResolveResult::Ready(local_path) => {
            set_cover_art(weak_metadata, Some(local_path));
            None
        }
        ResolveResult::NeedsDownload {
            url: download_url,
            dest,
        } => {
            let weak = weak_metadata.clone();
            Some(tokio::spawn(async move {
                let local_path = match ArtResolver::download(&download_url, &dest).await {
                    Ok(path) => path,
                    Err(err) => {
                        warn!(error = %err, "album art download failed");
                        return;
                    }
                };

                let Some(metadata) = weak.upgrade() else {
                    return;
                };
                let stale = metadata.art_url.get().as_deref() != Some(download_url.as_str());
                if !stale {
                    metadata.cover_art.set(Some(local_path));
                }
            }))
        }
        ResolveResult::Unresolvable => {
            set_cover_art(weak_metadata, None);
            None
        }
    }
}

fn abort_pending(handle: &mut Option<JoinHandle<()>>) {
    if let Some(handle) = handle.take() {
        handle.abort();
    }
}

fn set_cover_art(weak_metadata: &Weak<TrackMetadata>, value: Option<String>) {
    if let Some(metadata) = weak_metadata.upgrade() {
        metadata.cover_art.set(value);
    }
}
