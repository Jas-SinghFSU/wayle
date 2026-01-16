use std::collections::HashMap;

use zbus::{Result, proxy, zvariant::ObjectPath};

#[proxy(
    interface = "org.mpris.MediaPlayer2",
    default_service = "org.mpris.MediaPlayer2",
    default_path = "/org/mpris/MediaPlayer2"
)]
pub(crate) trait MediaPlayer2 {
    fn quit(&self) -> Result<()>;

    fn raise(&self) -> Result<()>;

    #[zbus(property)]
    fn can_quit(&self) -> Result<bool>;

    #[zbus(property)]
    fn can_raise(&self) -> Result<bool>;

    #[zbus(property)]
    fn identity(&self) -> Result<String>;

    #[zbus(property)]
    fn desktop_entry(&self) -> Result<String>;

    #[zbus(property)]
    fn supported_mime_types(&self) -> Result<Vec<String>>;

    #[zbus(property)]
    fn supported_uri_schemes(&self) -> Result<Vec<String>>;

    #[zbus(property)]
    fn has_track_list(&self) -> Result<bool>;

    #[zbus(property)]
    fn fullscreen(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_fullscreen(&self, fullscreen: bool) -> Result<()>;

    #[zbus(property)]
    fn can_set_fullscreen(&self) -> Result<bool>;
}

#[proxy(
    interface = "org.mpris.MediaPlayer2.Player",
    default_service = "org.mpris.MediaPlayer2",
    default_path = "/org/mpris/MediaPlayer2"
)]
pub(crate) trait MediaPlayer2Player {
    fn play(&self) -> Result<()>;

    fn pause(&self) -> Result<()>;

    fn play_pause(&self) -> Result<()>;

    fn stop(&self) -> Result<()>;

    fn next(&self) -> Result<()>;

    fn previous(&self) -> Result<()>;

    fn seek(&self, offset: i64) -> Result<()>;

    fn set_position(&self, track_id: &ObjectPath<'_>, position: i64) -> Result<()>;

    fn open_uri(&self, uri: &str) -> Result<()>;

    #[zbus(signal)]
    fn seeked(&self, position: i64) -> Result<()>;

    #[zbus(property)]
    fn playback_status(&self) -> Result<String>;

    #[zbus(property)]
    fn loop_status(&self) -> Result<String>;

    #[zbus(property)]
    fn set_loop_status(&self, status: &str) -> Result<()>;

    #[zbus(property)]
    fn rate(&self) -> Result<f64>;

    #[zbus(property)]
    fn set_rate(&self, rate: f64) -> Result<()>;

    #[zbus(property)]
    fn shuffle(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_shuffle(&self, shuffle: bool) -> Result<()>;

    #[zbus(property)]
    fn metadata(&self) -> Result<HashMap<String, zbus::zvariant::OwnedValue>>;

    #[zbus(property)]
    fn volume(&self) -> Result<f64>;

    #[zbus(property)]
    fn set_volume(&self, volume: f64) -> Result<()>;

    #[zbus(property)]
    fn position(&self) -> Result<i64>;

    #[zbus(property)]
    fn minimum_rate(&self) -> Result<f64>;

    #[zbus(property)]
    fn maximum_rate(&self) -> Result<f64>;

    #[zbus(property)]
    fn can_go_next(&self) -> Result<bool>;

    #[zbus(property)]
    fn can_go_previous(&self) -> Result<bool>;

    #[zbus(property)]
    fn can_play(&self) -> Result<bool>;

    #[zbus(property)]
    fn can_pause(&self) -> Result<bool>;

    #[zbus(property)]
    fn can_seek(&self) -> Result<bool>;

    #[zbus(property)]
    fn can_control(&self) -> Result<bool>;
}
