use std::collections::HashMap;

use super::format::{ChannelMap, SampleSpec};
use crate::volume::types::Volume;

/// Stream state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamState {
    /// Stream is creating
    Creating,
    /// Stream is ready
    Ready,
    /// Stream is running
    Running,
    /// Stream is failed
    Failed,
    /// Stream is terminated
    Terminated,
    /// Stream is unlinked
    Unlinked,
}

/// Stream type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamType {
    /// Playback stream (sink input)
    Playback,
    /// Recording stream (source output)
    Record,
}

/// Stream key for unique identification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StreamKey {
    /// Stream index
    pub index: u32,
    /// Stream type
    pub stream_type: StreamType,
}

impl StreamKey {
    /// Create a new stream key
    pub fn new(index: u32, stream_type: StreamType) -> Self {
        Self { index, stream_type }
    }
}

/// Media information for a stream
#[derive(Debug, Clone, PartialEq)]
pub struct MediaInfo {
    /// Track title
    pub title: Option<String>,
    /// Artist name
    pub artist: Option<String>,
    /// Album name
    pub album: Option<String>,
    /// Icon name
    pub icon_name: Option<String>,
}

/// Complete stream information from PulseAudio
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq)]
pub struct StreamInfo {
    /// Stream index
    pub index: u32,
    /// Stream name
    pub name: String,
    /// Application name
    pub application_name: Option<String>,
    /// Application binary path
    pub binary: Option<String>,
    /// Process ID
    pub pid: Option<u32>,
    /// Index of the owning module
    pub owner_module: Option<u32>,
    /// Index of the client this stream belongs to
    pub client: Option<u32>,
    /// Device index this stream is connected to (sink for playback, source for record)
    pub device_index: u32,
    /// Stream volume
    pub volume: Volume,
    /// Whether stream is muted
    pub muted: bool,
    /// Whether stream is corked (paused)
    pub corked: bool,
    /// Whether stream has volume control
    pub has_volume: bool,
    /// Whether volume is writable by clients
    pub volume_writable: bool,
    /// Stream state
    pub state: StreamState,
    /// Sample specification
    pub sample_spec: SampleSpec,
    /// Channel map
    pub channel_map: ChannelMap,
    /// Stream properties from PulseAudio
    pub properties: HashMap<String, String>,
    /// Media information
    pub media: MediaInfo,
    /// Buffer latency in microseconds
    pub buffer_latency: u64,
    /// Sink/source latency in microseconds
    pub device_latency: u64,
    /// Resample method
    pub resample_method: Option<String>,
    /// Driver name
    pub driver: String,
    /// Format information for the stream
    pub format: Option<String>,
}

impl StreamInfo {
    /// Get stream key for identification
    pub fn key(&self) -> StreamKey {
        StreamKey {
            index: self.index,
            stream_type: self.stream_type(),
        }
    }

    /// Determine stream type based on properties
    pub fn stream_type(&self) -> StreamType {
        if self.properties.get("media.role") == Some(&String::from("source-output")) {
            StreamType::Record
        } else {
            StreamType::Playback
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_stream_info(properties: HashMap<String, String>) -> StreamInfo {
        use crate::{
            types::format::{ChannelMap, SampleFormat, SampleSpec},
            volume::types::Volume,
        };

        StreamInfo {
            index: 0,
            name: String::from("test"),
            application_name: None,
            binary: None,
            pid: None,
            owner_module: None,
            client: None,
            device_index: 0,
            volume: Volume::mono(1.0),
            muted: false,
            corked: false,
            has_volume: true,
            volume_writable: true,
            state: StreamState::Running,
            sample_spec: SampleSpec {
                format: SampleFormat::S16LE,
                rate: 44100,
                channels: 2,
            },
            channel_map: ChannelMap {
                channels: 2,
                positions: vec![],
            },
            properties,
            media: MediaInfo {
                title: None,
                artist: None,
                album: None,
                icon_name: None,
            },
            buffer_latency: 0,
            device_latency: 0,
            resample_method: None,
            driver: String::from("test-driver"),
            format: None,
        }
    }

    #[test]
    fn stream_type_returns_record_when_media_role_is_source_output() {
        let mut props = HashMap::new();
        props.insert(String::from("media.role"), String::from("source-output"));
        let info = create_test_stream_info(props);

        assert_eq!(info.stream_type(), StreamType::Record);
    }

    #[test]
    fn stream_type_returns_playback_when_media_role_is_not_source_output() {
        let mut props = HashMap::new();
        props.insert(String::from("media.role"), String::from("sink-input"));
        let info = create_test_stream_info(props);

        assert_eq!(info.stream_type(), StreamType::Playback);
    }

    #[test]
    fn stream_type_returns_playback_when_media_role_missing() {
        let props = HashMap::new();
        let info = create_test_stream_info(props);

        assert_eq!(info.stream_type(), StreamType::Playback);
    }

    #[test]
    fn key_returns_correct_stream_key_for_record_stream() {
        let mut props = HashMap::new();
        props.insert(String::from("media.role"), String::from("source-output"));
        let mut info = create_test_stream_info(props);
        info.index = 42;

        let key = info.key();

        assert_eq!(key.index, 42);
        assert_eq!(key.stream_type, StreamType::Record);
    }

    #[test]
    fn key_returns_correct_stream_key_for_playback_stream() {
        let props = HashMap::new();
        let mut info = create_test_stream_info(props);
        info.index = 123;

        let key = info.key();

        assert_eq!(key.index, 123);
        assert_eq!(key.stream_type, StreamType::Playback);
    }
}
