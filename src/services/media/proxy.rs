use std::collections::HashMap;

use zbus::{Result, proxy, zvariant::ObjectPath};

/// MPRIS MediaPlayer2 interface proxy
///
/// Provides access to the base MPRIS interface for media player control
#[proxy(
    interface = "org.mpris.MediaPlayer2",
    default_service = "org.mpris.MediaPlayer2",
    default_path = "/org/mpris/MediaPlayer2"
)]
pub trait MediaPlayer2 {
    /// Quit the media player application
    fn quit(&self) -> Result<()>;

    /// Raise the media player window to the foreground
    fn raise(&self) -> Result<()>;

    /// Whether the player can be quit
    #[zbus(property)]
    fn can_quit(&self) -> Result<bool>;

    /// Whether the player window can be raised
    #[zbus(property)]
    fn can_raise(&self) -> Result<bool>;

    /// Human-readable name of the player
    #[zbus(property)]
    fn identity(&self) -> Result<String>;

    /// Desktop entry name for the player
    #[zbus(property)]
    fn desktop_entry(&self) -> Result<String>;

    /// MIME types supported by the player
    #[zbus(property)]
    fn supported_mime_types(&self) -> Result<Vec<String>>;

    /// URI schemes supported by the player
    #[zbus(property)]
    fn supported_uri_schemes(&self) -> Result<Vec<String>>;

    /// Whether the player has a track list
    #[zbus(property)]
    fn has_track_list(&self) -> Result<bool>;

    /// Whether the player is in fullscreen mode
    #[zbus(property)]
    fn fullscreen(&self) -> Result<bool>;

    /// Set the player's fullscreen mode
    #[zbus(property)]
    fn set_fullscreen(&self, fullscreen: bool) -> Result<()>;

    /// Whether the player can change fullscreen mode
    #[zbus(property)]
    fn can_set_fullscreen(&self) -> Result<bool>;
}

/// MPRIS MediaPlayer2.Player interface proxy
///
/// Provides access to the playback control interface for media players
#[proxy(
    interface = "org.mpris.MediaPlayer2.Player",
    default_service = "org.mpris.MediaPlayer2",
    default_path = "/org/mpris/MediaPlayer2"
)]
pub trait MediaPlayer2Player {
    /// Start playback
    fn play(&self) -> Result<()>;

    /// Pause playback
    fn pause(&self) -> Result<()>;

    /// Toggle play/pause state
    fn play_pause(&self) -> Result<()>;

    /// Stop playback
    fn stop(&self) -> Result<()>;

    /// Skip to next track
    fn next(&self) -> Result<()>;

    /// Skip to previous track
    fn previous(&self) -> Result<()>;

    /// Seek by a relative offset in microseconds
    fn seek(&self, offset: i64) -> Result<()>;

    /// Set absolute playback position in microseconds
    fn set_position(&self, track_id: &ObjectPath<'_>, position: i64) -> Result<()>;

    /// Open and play a URI
    fn open_uri(&self, uri: &str) -> Result<()>;

    /// Signal emitted when playback position changes
    #[zbus(signal)]
    fn seeked(&self, position: i64) -> Result<()>;

    /// Current playback status (Playing, Paused, Stopped)
    #[zbus(property)]
    fn playback_status(&self) -> Result<String>;

    /// Current loop status (None, Track, Playlist)
    #[zbus(property)]
    fn loop_status(&self) -> Result<String>;

    /// Set the loop status
    #[zbus(property)]
    fn set_loop_status(&self, status: &str) -> Result<()>;

    /// Current playback rate (1.0 is normal speed)
    #[zbus(property)]
    fn rate(&self) -> Result<f64>;

    /// Set the playback rate
    #[zbus(property)]
    fn set_rate(&self, rate: f64) -> Result<()>;

    /// Whether shuffle mode is enabled
    #[zbus(property)]
    fn shuffle(&self) -> Result<bool>;

    /// Set shuffle mode
    #[zbus(property)]
    fn set_shuffle(&self, shuffle: bool) -> Result<()>;

    /// Current track metadata
    #[zbus(property)]
    fn metadata(&self) -> Result<HashMap<String, zbus::zvariant::OwnedValue>>;

    /// Current volume level (0.0 to 1.0)
    #[zbus(property)]
    fn volume(&self) -> Result<f64>;

    /// Set volume level
    #[zbus(property)]
    fn set_volume(&self, volume: f64) -> Result<()>;

    /// Current playback position in microseconds
    #[zbus(property)]
    fn position(&self) -> Result<i64>;

    /// Minimum supported playback rate
    #[zbus(property)]
    fn minimum_rate(&self) -> Result<f64>;

    /// Maximum supported playback rate
    #[zbus(property)]
    fn maximum_rate(&self) -> Result<f64>;

    /// Whether the player can skip to next track
    #[zbus(property)]
    fn can_go_next(&self) -> Result<bool>;

    /// Whether the player can skip to previous track
    #[zbus(property)]
    fn can_go_previous(&self) -> Result<bool>;

    /// Whether the player can start playback
    #[zbus(property)]
    fn can_play(&self) -> Result<bool>;

    /// Whether the player can pause playback
    #[zbus(property)]
    fn can_pause(&self) -> Result<bool>;

    /// Whether the player supports seeking
    #[zbus(property)]
    fn can_seek(&self) -> Result<bool>;

    /// Whether the player can be controlled
    #[zbus(property)]
    fn can_control(&self) -> Result<bool>;
}
