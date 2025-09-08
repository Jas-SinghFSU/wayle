use std::{fmt, ops::Deref};

/// Configuration for the MPRIS service
#[derive(Default)]
pub struct Config {
    /// Patterns to ignore when discovering players
    pub ignored_players: Vec<String>,
}

/// Unique identifier for a media player
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerId(String);

impl PlayerId {
    /// Create a PlayerId from a D-Bus bus name
    pub fn from_bus_name(bus_name: &str) -> Self {
        Self(bus_name.to_string())
    }

    /// Get the D-Bus bus name
    pub fn bus_name(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Current playback state of a media player
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackState {
    /// Player is currently playing
    Playing,

    /// Player is paused
    Paused,

    /// Player is stopped
    Stopped,
}

impl From<&str> for PlaybackState {
    fn from(status: &str) -> Self {
        match status {
            "Playing" => Self::Playing,
            "Paused" => Self::Paused,
            _ => Self::Stopped,
        }
    }
}

/// Loop mode for track or playlist repetition
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoopMode {
    /// No looping
    None,

    /// Loop current track
    Track,

    /// Loop entire playlist
    Playlist,

    /// Loop mode not supported by player
    Unsupported,
}

impl From<&str> for LoopMode {
    fn from(status: &str) -> Self {
        match status {
            "None" => Self::None,
            "Track" => Self::Track,
            "Playlist" => Self::Playlist,
            _ => Self::Unsupported,
        }
    }
}

/// Shuffle mode for randomizing playback order
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShuffleMode {
    /// Shuffle enabled
    On,

    /// Shuffle disabled
    Off,

    /// Shuffle mode not supported by player
    Unsupported,
}

impl From<bool> for ShuffleMode {
    fn from(shuffle: bool) -> Self {
        if shuffle { Self::On } else { Self::Off }
    }
}

/// Volume of the player
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Volume(f64);

impl Volume {
    /// Create a new instance of a volume with safeguarded values
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// Get the volume as a percentage
    pub fn as_percentage(&self) -> f64 {
        let clamped_volume = self.0.clamp(0.0, 1.0);
        clamped_volume * 100.0
    }
}

impl Deref for Volume {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<f64> for Volume {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}
