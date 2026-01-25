use std::collections::HashMap;

use gtk::{gio::prelude::AppInfoExt, glib::prelude::Cast as _, prelude::IconExt};
use relm4::gtk;
use wayle_common::glob;
use wayle_config::schemas::modules::{BUILTIN_MAPPINGS, MediaConfig, MediaIconType};
use wayle_media::{core::player::Player, types::PlaybackState};

pub(crate) const PLAY_ICON: &str = "󰐊";
pub(crate) const PAUSE_ICON: &str = "󰏤";
pub(crate) const STOP_ICON: &str = "󰓛";

pub(crate) struct FormatContext<'a> {
    pub(crate) format: &'a str,
    pub(crate) title: &'a str,
    pub(crate) artist: &'a str,
    pub(crate) album: &'a str,
    pub(crate) state: PlaybackState,
}

pub(crate) fn format_label(ctx: &FormatContext<'_>) -> String {
    let status_text = match ctx.state {
        PlaybackState::Playing => "Playing",
        PlaybackState::Paused => "Paused",
        PlaybackState::Stopped => "Stopped",
    };

    let status_icon = match ctx.state {
        PlaybackState::Playing => PLAY_ICON,
        PlaybackState::Paused => PAUSE_ICON,
        PlaybackState::Stopped => STOP_ICON,
    };

    ctx.format
        .replace("{title}", ctx.title)
        .replace("{artist}", ctx.artist)
        .replace("{album}", ctx.album)
        .replace("{status}", status_text)
        .replace("{status_icon}", status_icon)
}

pub(crate) struct IconContext<'a> {
    pub(crate) icon_type: MediaIconType,
    pub(crate) icon_name: &'a str,
    pub(crate) spinning_disc_icon: &'a str,
    pub(crate) player_icons: &'a HashMap<String, String>,
    pub(crate) bus_name: &'a str,
    pub(crate) desktop_entry: Option<&'a str>,
}

pub(crate) fn resolve_icon(ctx: &IconContext<'_>) -> String {
    match ctx.icon_type {
        MediaIconType::Default => ctx.icon_name.to_string(),
        MediaIconType::Application => ctx
            .desktop_entry
            .map(|entry| format!("{entry}-symbolic"))
            .unwrap_or_else(|| ctx.icon_name.to_string()),
        MediaIconType::SpinningDisc => ctx.spinning_disc_icon.to_string(),
        MediaIconType::ApplicationMapped => {
            if let Some(icon) = glob::find_match(
                ctx.player_icons
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str())),
                ctx.bus_name,
            ) {
                return icon.to_string();
            }

            if let Some(icon) = glob::find_match(BUILTIN_MAPPINGS.iter().copied(), ctx.bus_name) {
                return icon.to_string();
            }

            ctx.desktop_entry
                .map(|entry| format!("{entry}-symbolic"))
                .unwrap_or_else(|| ctx.icon_name.to_string())
        }
    }
}

pub(super) fn build_label(config: &MediaConfig, player: &Player) -> String {
    let format = config.format.get();
    let title = player.metadata.title.get();
    let artist = player.metadata.artist.get();
    let album = player.metadata.album.get();
    format_label(&FormatContext {
        format: &format,
        title: &title,
        artist: &artist,
        album: &album,
        state: player.playback_state.get(),
    })
}

pub(super) fn build_icon(config: &MediaConfig, player: &Player) -> String {
    let icon_name = config.icon_name.get();
    let icon_type = config.icon_type.get();
    let desktop_entry = player.desktop_entry.get();

    if icon_type == MediaIconType::Application {
        if let Some(icon) = desktop_entry_icon(desktop_entry.as_deref()) {
            return icon;
        }
        return icon_name;
    }

    let spinning_disc_icon = config.spinning_disc_icon.get();
    let player_icons = config.player_icons.get();
    let resolved = resolve_icon(&IconContext {
        icon_type,
        icon_name: &icon_name,
        spinning_disc_icon: &spinning_disc_icon,
        player_icons: &player_icons,
        bus_name: player.id.bus_name(),
        desktop_entry: desktop_entry.as_deref(),
    });

    if icon_exists(&resolved) {
        return resolved;
    }

    if icon_type == MediaIconType::ApplicationMapped
        && let Some(icon) = desktop_entry_icon(desktop_entry.as_deref()) {
            return icon;
        }

    icon_name
}

pub(super) fn desktop_entry_icon(desktop_entry: Option<&str>) -> Option<String> {
    let entry = desktop_entry?;
    let app_info = lookup_desktop_entry(entry)?;
    let icon = app_info.icon()?;
    Some(icon.to_string()?.into())
}

fn lookup_desktop_entry(entry: &str) -> Option<gtk::gio::DesktopAppInfo> {
    let candidates = [
        format!("{entry}.desktop"),
        format!("{entry}-launcher.desktop"),
    ];
    for desktop_id in &candidates {
        if let Some(app) = gtk::gio::DesktopAppInfo::new(desktop_id) {
            return Some(app);
        }
    }

    find_by_startup_wm_class(entry)
}

fn find_by_startup_wm_class(wm_class: &str) -> Option<gtk::gio::DesktopAppInfo> {
    let wm_class_lower = wm_class.to_lowercase();
    for app_info in gtk::gio::AppInfo::all() {
        let Ok(desktop_app) = app_info.downcast::<gtk::gio::DesktopAppInfo>() else {
            continue;
        };
        if let Some(startup_class) = desktop_app.startup_wm_class()
            && startup_class.to_lowercase() == wm_class_lower {
                return Some(desktop_app);
            }
    }
    None
}

fn icon_exists(icon_name: &str) -> bool {
    let Some(display) = gtk::gdk::Display::default() else {
        return false;
    };
    let theme = gtk::IconTheme::for_display(&display);
    theme.has_icon(icon_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_label_basic_placeholders() {
        let result = format_label(&FormatContext {
            format: "{title} - {artist}",
            title: "Song Name",
            artist: "Artist Name",
            album: "Album Name",
            state: PlaybackState::Playing,
        });

        assert_eq!(result, "Song Name - Artist Name");
    }

    #[test]
    fn format_label_all_placeholders() {
        let result = format_label(&FormatContext {
            format: "{status_icon} {title} by {artist} from {album} ({status})",
            title: "Track",
            artist: "Band",
            album: "Record",
            state: PlaybackState::Playing,
        });

        assert_eq!(
            result,
            format!("{PLAY_ICON} Track by Band from Record (Playing)")
        );
    }

    #[test]
    fn format_label_paused_state() {
        let result = format_label(&FormatContext {
            format: "{status_icon} {status}",
            title: "",
            artist: "",
            album: "",
            state: PlaybackState::Paused,
        });

        assert_eq!(result, format!("{PAUSE_ICON} Paused"));
    }

    #[test]
    fn format_label_stopped_state() {
        let result = format_label(&FormatContext {
            format: "{status}",
            title: "",
            artist: "",
            album: "",
            state: PlaybackState::Stopped,
        });

        assert_eq!(result, "Stopped");
    }

    #[test]
    fn resolve_icon_default_mode() {
        let result = resolve_icon(&IconContext {
            icon_type: MediaIconType::Default,
            icon_name: "my-icon-symbolic",
            spinning_disc_icon: "disc-symbolic",
            player_icons: &HashMap::new(),
            bus_name: "org.mpris.MediaPlayer2.spotify",
            desktop_entry: Some("spotify"),
        });

        assert_eq!(result, "my-icon-symbolic");
    }

    #[test]
    fn resolve_icon_application_mode_with_entry() {
        let result = resolve_icon(&IconContext {
            icon_type: MediaIconType::Application,
            icon_name: "fallback-symbolic",
            spinning_disc_icon: "disc-symbolic",
            player_icons: &HashMap::new(),
            bus_name: "org.mpris.MediaPlayer2.spotify",
            desktop_entry: Some("spotify"),
        });

        assert_eq!(result, "spotify-symbolic");
    }

    #[test]
    fn resolve_icon_application_mode_without_entry() {
        let result = resolve_icon(&IconContext {
            icon_type: MediaIconType::Application,
            icon_name: "fallback-symbolic",
            spinning_disc_icon: "disc-symbolic",
            player_icons: &HashMap::new(),
            bus_name: "org.mpris.MediaPlayer2.unknown",
            desktop_entry: None,
        });

        assert_eq!(result, "fallback-symbolic");
    }

    #[test]
    fn resolve_icon_spinning_disc_mode() {
        let result = resolve_icon(&IconContext {
            icon_type: MediaIconType::SpinningDisc,
            icon_name: "fallback-symbolic",
            spinning_disc_icon: "ld-disc-3-symbolic",
            player_icons: &HashMap::new(),
            bus_name: "org.mpris.MediaPlayer2.spotify",
            desktop_entry: Some("spotify"),
        });

        assert_eq!(result, "ld-disc-3-symbolic");
    }

    #[test]
    fn resolve_icon_mapped_mode_user_config_priority() {
        let mut player_icons = HashMap::new();
        player_icons.insert(
            "*spotify*".to_string(),
            "custom-spotify-symbolic".to_string(),
        );

        let result = resolve_icon(&IconContext {
            icon_type: MediaIconType::ApplicationMapped,
            icon_name: "fallback-symbolic",
            spinning_disc_icon: "disc-symbolic",
            player_icons: &player_icons,
            bus_name: "org.mpris.MediaPlayer2.spotify.instance123",
            desktop_entry: Some("spotify"),
        });

        assert_eq!(result, "custom-spotify-symbolic");
    }

    #[test]
    fn resolve_icon_mapped_mode_builtin_fallback() {
        let result = resolve_icon(&IconContext {
            icon_type: MediaIconType::ApplicationMapped,
            icon_name: "fallback-symbolic",
            spinning_disc_icon: "disc-symbolic",
            player_icons: &HashMap::new(),
            bus_name: "org.mpris.MediaPlayer2.spotify.instance123",
            desktop_entry: Some("spotify"),
        });

        assert_eq!(result, "si-spotify-symbolic");
    }

    #[test]
    fn resolve_icon_mapped_mode_desktop_entry_fallback() {
        let result = resolve_icon(&IconContext {
            icon_type: MediaIconType::ApplicationMapped,
            icon_name: "fallback-symbolic",
            spinning_disc_icon: "disc-symbolic",
            player_icons: &HashMap::new(),
            bus_name: "org.mpris.MediaPlayer2.unknown_player",
            desktop_entry: Some("unknown_player"),
        });

        assert_eq!(result, "unknown_player-symbolic");
    }

    #[test]
    fn resolve_icon_mapped_mode_final_fallback() {
        let result = resolve_icon(&IconContext {
            icon_type: MediaIconType::ApplicationMapped,
            icon_name: "fallback-symbolic",
            spinning_disc_icon: "disc-symbolic",
            player_icons: &HashMap::new(),
            bus_name: "org.mpris.MediaPlayer2.mystery",
            desktop_entry: None,
        });

        assert_eq!(result, "fallback-symbolic");
    }
}
