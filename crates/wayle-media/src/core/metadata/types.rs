use std::{collections::HashMap, time::Duration};

use tokio_util::sync::CancellationToken;
use zbus::zvariant::OwnedValue;

use crate::proxy::MediaPlayer2PlayerProxy;

/// Parameters for creating a TrackMetadata instance.
///
/// **Note**: This type is exposed for trait implementation requirements
/// but should not be constructed directly by external consumers.
#[doc(hidden)]
pub struct TrackMetadataParams<'a> {
    pub(crate) proxy: &'a MediaPlayer2PlayerProxy<'a>,
}

/// Parameters for creating a live TrackMetadata instance.
///
/// **Note**: This type is exposed for trait implementation requirements
/// but should not be constructed directly by external consumers.
#[doc(hidden)]
pub struct LiveTrackMetadataParams<'a> {
    pub(crate) proxy: MediaPlayer2PlayerProxy<'static>,
    pub(crate) cancellation_token: &'a CancellationToken,
}

#[derive(Debug, Clone)]
pub(crate) struct TrackProperties {
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

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, time::Duration};

    use zbus::zvariant::{Array, Signature, Value};

    use super::TrackProperties;

    #[test]
    fn track_properties_from_mpris_with_empty_map_returns_defaults() {
        let metadata = HashMap::new();

        let props = TrackProperties::from_mpris(metadata);

        assert_eq!(props.title, "");
        assert_eq!(props.artist, "");
        assert_eq!(props.album, "");
        assert_eq!(props.album_artist, "");
        assert_eq!(props.art_url, None);
        assert_eq!(props.length, None);
        assert_eq!(props.track_id, None);
    }

    #[test]
    fn track_properties_from_mpris_extracts_title() {
        let mut metadata = HashMap::new();
        metadata.insert(
            String::from("xesam:title"),
            Value::new("Test Song").try_to_owned().unwrap(),
        );

        let props = TrackProperties::from_mpris(metadata);

        assert_eq!(props.title, "Test Song");
    }

    #[test]
    fn track_properties_from_mpris_extracts_artist_as_string_array() {
        let mut metadata = HashMap::new();
        let artists = vec![Value::new("Artist One"), Value::new("Artist Two")];
        let array = Array::try_from(artists).unwrap();
        metadata.insert(
            String::from("xesam:artist"),
            Value::Array(array).try_to_owned().unwrap(),
        );

        let props = TrackProperties::from_mpris(metadata);

        assert_eq!(props.artist, "Artist One, Artist Two");
    }

    #[test]
    fn track_properties_from_mpris_extracts_album() {
        let mut metadata = HashMap::new();
        metadata.insert(
            String::from("xesam:album"),
            Value::new("Test Album").try_to_owned().unwrap(),
        );

        let props = TrackProperties::from_mpris(metadata);

        assert_eq!(props.album, "Test Album");
    }

    #[test]
    fn track_properties_from_mpris_extracts_album_artist() {
        let mut metadata = HashMap::new();
        let artists = vec![Value::new("Album Artist")];
        let array = Array::try_from(artists).unwrap();
        metadata.insert(
            String::from("xesam:albumArtist"),
            Value::Array(array).try_to_owned().unwrap(),
        );

        let props = TrackProperties::from_mpris(metadata);

        assert_eq!(props.album_artist, "Album Artist");
    }

    #[test]
    fn track_properties_from_mpris_extracts_art_url() {
        let mut metadata = HashMap::new();
        metadata.insert(
            String::from("mpris:artUrl"),
            Value::new("file:///path/to/art.png")
                .try_to_owned()
                .unwrap(),
        );

        let props = TrackProperties::from_mpris(metadata);

        assert_eq!(props.art_url, Some(String::from("file:///path/to/art.png")));
    }

    #[test]
    fn track_properties_from_mpris_extracts_length_as_duration() {
        let mut metadata = HashMap::new();
        metadata.insert(
            String::from("mpris:length"),
            Value::I64(5000000).try_to_owned().unwrap(),
        );

        let props = TrackProperties::from_mpris(metadata);

        assert_eq!(props.length, Some(Duration::from_micros(5000000)));
    }

    #[test]
    fn track_properties_from_mpris_extracts_track_id() {
        let mut metadata = HashMap::new();
        metadata.insert(
            String::from("mpris:trackid"),
            Value::new("/org/mpris/track/123").try_to_owned().unwrap(),
        );

        let props = TrackProperties::from_mpris(metadata);

        assert_eq!(props.track_id, Some(String::from("/org/mpris/track/123")));
    }

    #[test]
    fn as_string_converts_str_value() {
        let value = Value::new("test string").try_to_owned().unwrap();

        let result = TrackProperties::as_string(&value);

        assert_eq!(result, Some(String::from("test string")));
    }

    #[test]
    fn as_string_with_invalid_type_returns_none() {
        let value = Value::I32(42).try_to_owned().unwrap();

        let result = TrackProperties::as_string(&value);

        assert_eq!(result, None);
    }

    #[test]
    fn as_string_array_joins_multiple_strings_with_comma() {
        let strings = vec![
            Value::new("First"),
            Value::new("Second"),
            Value::new("Third"),
        ];
        let array = Array::try_from(strings).unwrap();
        let value = Value::Array(array).try_to_owned().unwrap();

        let result = TrackProperties::as_string_array(&value);

        assert_eq!(result, Some(String::from("First, Second, Third")));
    }

    #[test]
    fn as_string_array_handles_single_element() {
        let strings = vec![Value::new("Single")];
        let array = Array::try_from(strings).unwrap();
        let value = Value::Array(array).try_to_owned().unwrap();

        let result = TrackProperties::as_string_array(&value);

        assert_eq!(result, Some(String::from("Single")));
    }

    #[test]
    fn as_string_array_with_empty_array_falls_back_to_as_string() {
        let sig = Signature::try_from("as").unwrap();
        let array = Array::new(&sig);
        let value = Value::Array(array).try_to_owned().unwrap();

        let result = TrackProperties::as_string_array(&value);

        assert_eq!(result, None);
    }

    #[test]
    fn as_string_array_filters_out_invalid_types() {
        let mixed = vec![
            Value::new("Valid"),
            Value::I32(123),
            Value::new("Also Valid"),
        ];
        let array = Array::try_from(mixed).unwrap();
        let value = Value::Array(array).try_to_owned().unwrap();

        let result = TrackProperties::as_string_array(&value);

        assert_eq!(result, Some(String::from("Valid, Also Valid")));
    }

    #[test]
    fn duration_converts_positive_i64_to_duration() {
        let value = Value::I64(1000000).try_to_owned().unwrap();

        let result = TrackProperties::duration(&value);

        assert_eq!(result, Some(Duration::from_micros(1000000)));
    }

    #[test]
    fn duration_converts_positive_u64_to_duration() {
        let value = Value::U64(2000000).try_to_owned().unwrap();

        let result = TrackProperties::duration(&value);

        assert_eq!(result, Some(Duration::from_micros(2000000)));
    }

    #[test]
    fn duration_with_zero_returns_none() {
        let value = Value::I64(0).try_to_owned().unwrap();

        let result = TrackProperties::duration(&value);

        assert_eq!(result, None);
    }

    #[test]
    fn duration_with_negative_returns_none() {
        let value = Value::I64(-1000).try_to_owned().unwrap();

        let result = TrackProperties::duration(&value);

        assert_eq!(result, None);
    }

    #[test]
    fn duration_with_invalid_type_returns_none() {
        let value = Value::new("not a number").try_to_owned().unwrap();

        let result = TrackProperties::duration(&value);

        assert_eq!(result, None);
    }
}
