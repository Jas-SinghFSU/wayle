pub(crate) mod monitoring;
/// Track metadata types
pub mod types;

use std::{collections::HashMap, sync::Arc, time::Duration};

use derive_more::Debug;
use futures::stream::Stream;
use tokio_util::sync::CancellationToken;
pub(crate) use types::{LiveTrackMetadataParams, TrackMetadataParams, TrackProperties};
use wayle_common::{Property, watch_all};
use wayle_traits::{ModelMonitoring, Reactive};
use zbus::zvariant::OwnedValue;

use crate::{error::Error, proxy::MediaPlayer2PlayerProxy};

pub(crate) const UNKNOWN_METADATA: &str = "Unknown";

/// Track metadata with reactive properties.
///
/// Accessed via [`Player::metadata`](super::player::Player::metadata). Live players have live-updating metadata;
/// snapshot players have frozen metadata.
#[derive(Debug, Clone)]
pub struct TrackMetadata {
    #[debug(skip)]
    pub(crate) proxy: Option<MediaPlayer2PlayerProxy<'static>>,
    #[debug(skip)]
    pub(crate) cancellation_token: Option<CancellationToken>,
    /// Track title.
    pub title: Property<String>,
    /// Artist name(s), comma-separated if multiple.
    pub artist: Property<String>,
    /// Album name.
    pub album: Property<String>,
    /// Album artist(s), comma-separated if multiple.
    pub album_artist: Property<String>,
    /// Track duration.
    pub length: Property<Option<Duration>>,
    /// Album art URL (file:// or http://).
    pub art_url: Property<Option<String>>,
    /// MPRIS track ID (D-Bus object path).
    pub track_id: Property<Option<String>>,
}

impl Reactive for TrackMetadata {
    type Context<'a> = TrackMetadataParams<'a>;
    type LiveContext<'a> = LiveTrackMetadataParams<'a>;
    type Error = Error;

    async fn get(params: Self::Context<'_>) -> Result<Self, Self::Error> {
        let metadata = Self::unknown();
        let metadata_arc = Arc::new(metadata);

        if let Ok(metadata_map) = params.proxy.metadata().await {
            Self::update_from_dbus(&metadata_arc, metadata_map);
        }

        Ok((*metadata_arc).clone())
    }

    async fn get_live(params: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let mut metadata = Self::unknown();
        metadata.proxy = Some(params.proxy.clone());
        metadata.cancellation_token = Some(params.cancellation_token.child_token());
        let metadata = Arc::new(metadata);

        if let Ok(metadata_map) = params.proxy.metadata().await {
            Self::update_from_dbus(&metadata, metadata_map);
        }

        metadata.clone().start_monitoring().await?;

        Ok(metadata)
    }
}

impl TrackMetadata {
    pub(crate) fn unknown() -> Self {
        Self {
            proxy: None,
            cancellation_token: None,
            title: Property::new(String::from(UNKNOWN_METADATA)),
            artist: Property::new(String::from(UNKNOWN_METADATA)),
            album: Property::new(String::from(UNKNOWN_METADATA)),
            album_artist: Property::new(String::from(UNKNOWN_METADATA)),
            length: Property::new(None),
            art_url: Property::new(None),
            track_id: Property::new(None),
        }
    }

    pub(crate) fn update_from_dbus(
        metadata: &Arc<Self>,
        dbus_metadata: HashMap<String, OwnedValue>,
    ) {
        let props = TrackProperties::from_mpris(dbus_metadata);

        metadata.title.set(props.title);
        metadata.artist.set(props.artist);
        metadata.album.set(props.album);
        metadata.album_artist.set(props.album_artist);
        metadata.length.set(props.length);
        metadata.art_url.set(props.art_url);
        metadata.track_id.set(props.track_id);
    }

    /// Watch for any metadata changes.
    ///
    /// Emits whenever any metadata field changes.
    pub fn watch(&self) -> impl Stream<Item = TrackMetadata> + Send + 'static {
        watch_all!(
            self,
            title,
            artist,
            album,
            album_artist,
            length,
            art_url,
            track_id
        )
    }
}
