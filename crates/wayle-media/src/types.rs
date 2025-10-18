use std::{fmt, ops::Deref};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playback_state_from_str_with_playing_returns_playing() {
        let state = PlaybackState::from("Playing");
        assert_eq!(state, PlaybackState::Playing);
    }

    #[test]
    fn playback_state_from_str_with_paused_returns_paused() {
        let state = PlaybackState::from("Paused");
        assert_eq!(state, PlaybackState::Paused);
    }

    #[test]
    fn playback_state_from_str_with_stopped_returns_stopped() {
        let state = PlaybackState::from("Stopped");
        assert_eq!(state, PlaybackState::Stopped);
    }

    #[test]
    fn playback_state_from_str_with_unknown_value_returns_stopped() {
        let state = PlaybackState::from("Unknown");
        assert_eq!(state, PlaybackState::Stopped);
    }

    #[test]
    fn loop_mode_from_str_with_none_returns_none() {
        let mode = LoopMode::from("None");
        assert_eq!(mode, LoopMode::None);
    }

    #[test]
    fn loop_mode_from_str_with_track_returns_track() {
        let mode = LoopMode::from("Track");
        assert_eq!(mode, LoopMode::Track);
    }

    #[test]
    fn loop_mode_from_str_with_playlist_returns_playlist() {
        let mode = LoopMode::from("Playlist");
        assert_eq!(mode, LoopMode::Playlist);
    }

    #[test]
    fn loop_mode_from_str_with_unknown_value_returns_unsupported() {
        let mode = LoopMode::from("Unknown");
        assert_eq!(mode, LoopMode::Unsupported);
    }

    #[test]
    fn shuffle_mode_from_bool_with_true_returns_on() {
        let mode = ShuffleMode::from(true);
        assert_eq!(mode, ShuffleMode::On);
    }

    #[test]
    fn shuffle_mode_from_bool_with_false_returns_off() {
        let mode = ShuffleMode::from(false);
        assert_eq!(mode, ShuffleMode::Off);
    }

    #[test]
    fn volume_new_clamps_negative_to_zero() {
        let volume = Volume::new(-0.5);
        assert_eq!(*volume, 0.0);
    }

    #[test]
    fn volume_new_clamps_above_one_to_one() {
        let volume = Volume::new(1.5);
        assert_eq!(*volume, 1.0);
    }

    #[test]
    fn volume_new_preserves_valid_value() {
        let volume = Volume::new(0.5);
        assert_eq!(*volume, 0.5);
    }

    #[test]
    fn volume_new_with_zero_returns_zero() {
        let volume = Volume::new(0.0);
        assert_eq!(*volume, 0.0);
    }

    #[test]
    fn volume_new_with_one_returns_one() {
        let volume = Volume::new(1.0);
        assert_eq!(*volume, 1.0);
    }

    #[test]
    fn volume_as_percentage_converts_zero_to_zero() {
        let volume = Volume::new(0.0);
        assert_eq!(volume.as_percentage(), 0.0);
    }

    #[test]
    fn volume_as_percentage_converts_one_to_hundred() {
        let volume = Volume::new(1.0);
        assert_eq!(volume.as_percentage(), 100.0);
    }

    #[test]
    fn volume_as_percentage_converts_half_to_fifty() {
        let volume = Volume::new(0.5);
        assert_eq!(volume.as_percentage(), 50.0);
    }
}
