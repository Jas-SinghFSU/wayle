use std::collections::HashMap;

use wayle_common::glob;
use wayle_config::schemas::modules::{BUILTIN_MAPPINGS, MediaIconType};
use wayle_media::types::PlaybackState;

pub const PLAY_ICON: &str = "󰐊";
pub const PAUSE_ICON: &str = "󰏤";
pub const STOP_ICON: &str = "󰓛";

pub struct FormatContext<'a> {
    pub format: &'a str,
    pub title: &'a str,
    pub artist: &'a str,
    pub album: &'a str,
    pub state: PlaybackState,
}

pub fn format_label(ctx: &FormatContext<'_>) -> String {
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

pub struct IconContext<'a> {
    pub icon_type: MediaIconType,
    pub icon_name: &'a str,
    pub spinning_disc_icon: &'a str,
    pub player_icons: &'a HashMap<String, String>,
    pub bus_name: &'a str,
    pub desktop_entry: Option<&'a str>,
}

pub fn resolve_icon(ctx: &IconContext<'_>) -> String {
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
