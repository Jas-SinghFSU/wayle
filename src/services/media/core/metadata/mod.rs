pub(crate) mod monitoring;

use std::{collections::HashMap, sync::Arc, time::Duration};

use futures::stream::Stream;
use monitoring::TrackMetadataMonitor;
use tokio_util::sync::CancellationToken;
use zbus::zvariant::OwnedValue;

use crate::{
    services::{common::Property, media::proxy::MediaPlayer2PlayerProxy},
    watch_all,
};

/// Default value for unknown metadata fields.
pub const UNKNOWN_METADATA: &str = "Unknown";

/// Metadata for a media track with reactive properties
#[derive(Debug, Clone)]
pub struct TrackMetadata {
    /// Track title
    pub title: Property<String>,

    /// Track artist(s)
    pub artist: Property<String>,

    /// Album name
    pub album: Property<String>,

    /// Album artist(s)
    pub album_artist: Property<String>,

    /// Track length
    pub length: Property<Option<Duration>>,

    /// Artwork URL
    pub art_url: Property<Option<String>>,

    /// Track ID (unique identifier)
    pub track_id: Property<Option<String>>,
}

impl TrackMetadata {
    /// Get metadata snapshot without monitoring.
    ///
    /// Fetches current metadata without setting up monitoring for changes.
    pub(crate) async fn get(proxy: &MediaPlayer2PlayerProxy<'_>) -> Arc<Self> {
        let metadata = Self::unknown();
        let metadata = Arc::new(metadata);

        if let Ok(metadata_map) = proxy.metadata().await {
            Self::update_from_dbus(&metadata, metadata_map);
        }

        metadata
    }

    /// Get metadata with live monitoring.
    ///
    /// Fetches initial metadata and starts monitoring for changes.
    pub(crate) async fn get_live(
        proxy: MediaPlayer2PlayerProxy<'static>,
        cancellation_token: CancellationToken,
    ) -> Arc<Self> {
        let metadata = Self::unknown();
        let metadata = Arc::new(metadata);

        if let Ok(metadata_map) = proxy.metadata().await {
            Self::update_from_dbus(&metadata, metadata_map);
        }

        TrackMetadataMonitor::start(Arc::clone(&metadata), proxy, cancellation_token);

        metadata
    }

    /// Create empty metadata with "Unknown" defaults
    pub fn unknown() -> Self {
        Self {
            title: Property::new(UNKNOWN_METADATA.to_string()),
            artist: Property::new(UNKNOWN_METADATA.to_string()),
            album: Property::new(UNKNOWN_METADATA.to_string()),
            album_artist: Property::new(UNKNOWN_METADATA.to_string()),
            length: Property::new(None),
            art_url: Property::new(None),
            track_id: Property::new(None),
        }
    }

    pub(crate) fn update_from_dbus(
        metadata: &Arc<Self>,
        dbus_metadata: HashMap<String, OwnedValue>,
    ) {
        let new_data = TrackMetadata::from(dbus_metadata);

        metadata.title.set(new_data.title.get());
        metadata.artist.set(new_data.artist.get());
        metadata.album.set(new_data.album.get());
        metadata.album_artist.set(new_data.album_artist.get());
        metadata.length.set(new_data.length.get());
        metadata.art_url.set(new_data.art_url.get());
        metadata.track_id.set(new_data.track_id.get());
    }

    /// Watch for any metadata changes.
    ///
    /// Emits whenever any metadata field changes.
    pub fn watch(&self) -> impl Stream<Item = TrackMetadata> + Send {
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

    fn extract_string(value: &OwnedValue) -> Option<String> {
        if let Ok(s) = String::try_from(value.clone()) {
            return Some(s);
        }
        if let Ok(s) = value.downcast_ref::<String>() {
            return Some(s.clone());
        }
        if let Ok(s) = value.downcast_ref::<&str>() {
            return Some(s.to_string());
        }
        None
    }

    fn extract_string_array(value: &OwnedValue) -> Option<String> {
        if let Ok(array) = <&zbus::zvariant::Array>::try_from(value) {
            let strings: Vec<String> = array
                .iter()
                .filter_map(|item| {
                    item.downcast_ref::<String>()
                        .or_else(|_| item.downcast_ref::<&str>().map(|s| s.to_string()))
                        .ok()
                })
                .collect();

            if !strings.is_empty() {
                return Some(strings.join(", "));
            }
        }

        Self::extract_string(value)
    }

    fn extract_duration(value: &OwnedValue) -> Option<Duration> {
        if let Ok(length) = i64::try_from(value.clone())
            && length > 0
        {
            return Some(Duration::from_micros(length as u64));
        }

        if let Ok(length) = u64::try_from(value.clone())
            && length > 0
        {
            return Some(Duration::from_micros(length));
        }

        None
    }
}

impl From<HashMap<String, OwnedValue>> for TrackMetadata {
    fn from(metadata: HashMap<String, OwnedValue>) -> Self {
        Self {
            title: Property::new(
                metadata
                    .get("xesam:title")
                    .and_then(Self::extract_string)
                    .unwrap_or_default(),
            ),

            artist: Property::new(
                metadata
                    .get("xesam:artist")
                    .and_then(Self::extract_string_array)
                    .unwrap_or_default(),
            ),

            album: Property::new(
                metadata
                    .get("xesam:album")
                    .and_then(Self::extract_string)
                    .unwrap_or_default(),
            ),

            album_artist: Property::new(
                metadata
                    .get("xesam:albumArtist")
                    .and_then(Self::extract_string_array)
                    .unwrap_or_default(),
            ),

            art_url: Property::new(metadata.get("mpris:artUrl").and_then(Self::extract_string)),

            length: Property::new(
                metadata
                    .get("mpris:length")
                    .and_then(Self::extract_duration),
            ),

            track_id: Property::new(metadata.get("mpris:trackid").and_then(Self::extract_string)),
        }
    }
}
