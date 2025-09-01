use std::{collections::HashMap, time::Duration};

use tokio_util::sync::CancellationToken;
use zbus::zvariant::OwnedValue;

use crate::services::media::proxy::MediaPlayer2PlayerProxy;

pub(crate) struct TrackMetadataParams<'a> {
    pub proxy: &'a MediaPlayer2PlayerProxy<'a>,
}

pub(crate) struct LiveTrackMetadataParams {
    pub proxy: MediaPlayer2PlayerProxy<'static>,
    pub cancellation_token: CancellationToken,
}

#[derive(Debug, Clone)]
pub struct TrackProperties {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub length: Option<Duration>,
    pub art_url: Option<String>,
    pub track_id: Option<String>,
}

impl TrackProperties {
    pub fn from_mpris(metadata: HashMap<String, OwnedValue>) -> Self {
        Self {
            title: metadata
                .get("xesam:title")
                .and_then(Self::as_string)
                .unwrap_or_default(),
            artist: metadata
                .get("xesam:artist")
                .and_then(Self::as_string_array)
                .unwrap_or_default(),
            album: metadata
                .get("xesam:album")
                .and_then(Self::as_string)
                .unwrap_or_default(),
            album_artist: metadata
                .get("xesam:albumArtist")
                .and_then(Self::as_string_array)
                .unwrap_or_default(),
            art_url: metadata.get("mpris:artUrl").and_then(Self::as_string),
            length: metadata.get("mpris:length").and_then(Self::duration),
            track_id: metadata.get("mpris:trackid").and_then(Self::as_string),
        }
    }

    fn as_string(value: &OwnedValue) -> Option<String> {
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

    fn as_string_array(value: &OwnedValue) -> Option<String> {
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

        Self::as_string(value)
    }

    fn duration(value: &OwnedValue) -> Option<Duration> {
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
