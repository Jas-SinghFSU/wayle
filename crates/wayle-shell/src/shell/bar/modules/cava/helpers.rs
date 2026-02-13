use std::sync::Arc;

use wayle_cava::{CavaService, InputMethod};
use wayle_config::{
    ConfigService,
    schemas::{
        modules::CavaInput,
        styling::{ColorValue, CssToken},
    },
};

const REM_BASE: f32 = 16.0;

pub(crate) struct Rgba {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

pub(crate) struct DrawConfig {
    pub bar_width: f64,
    pub bar_gap: f64,
    pub color: Rgba,
    pub internal_padding: f64,
}

pub(super) fn rem_to_px(rem: f32, scale: f32) -> f64 {
    f64::from(rem * scale * REM_BASE)
}

pub(super) fn map_input(input: CavaInput) -> InputMethod {
    match input {
        CavaInput::PipeWire => InputMethod::PipeWire,
        CavaInput::Pulse => InputMethod::Pulse,
        CavaInput::Alsa => InputMethod::Alsa,
        CavaInput::Jack => InputMethod::Jack,
        CavaInput::Fifo => InputMethod::Fifo,
        CavaInput::PortAudio => InputMethod::PortAudio,
        CavaInput::Sndio => InputMethod::Sndio,
        CavaInput::Oss => InputMethod::Oss,
        CavaInput::Shmem => InputMethod::Shmem,
        CavaInput::Winscap => InputMethod::Winscap,
    }
}

pub(super) async fn build_cava_service(
    config: &Arc<ConfigService>,
) -> Result<Arc<CavaService>, wayle_cava::Error> {
    let cfg = &config.config().modules.cava;

    let service = CavaService::builder()
        .bars(cfg.bars.get().value())
        .framerate(cfg.framerate.get().value())
        .autosens(true)
        .stereo(cfg.stereo.get())
        .noise_reduction(cfg.noise_reduction.get().value())
        .monstercat(cfg.monstercat.get())
        .waves(cfg.waves.get())
        .low_cutoff(cfg.low_cutoff.get().value())
        .high_cutoff(cfg.high_cutoff.get().value())
        .input(map_input(cfg.input.get()))
        .source(cfg.source.get().clone())
        .build()
        .await?;

    Ok(Arc::new(service))
}

pub(super) fn calculate_widget_length(
    bars: u16,
    bar_width: u32,
    bar_gap: u32,
    padding: f64,
) -> i32 {
    let bar_count = f64::from(bars);
    let gap_count = (bar_count - 1.0).max(0.0);
    let bar_space = bar_count * f64::from(bar_width);
    let gap_space = gap_count * f64::from(bar_gap);
    let pad_space = padding * 2.0;

    let total = bar_space + gap_space + pad_space;
    total.round().max(1.0) as i32
}

pub(super) fn resolve_rgba(color: &ColorValue, config: &ConfigService) -> Rgba {
    let hex = match color {
        ColorValue::Token(token) => {
            let palette = config.config().styling.palette();
            match token {
                CssToken::BgBase => palette.bg.clone(),
                CssToken::BgSurface | CssToken::BgSurfaceElevated => palette.surface.clone(),
                CssToken::BgElevated
                | CssToken::BgOverlay
                | CssToken::BgHover
                | CssToken::BgActive
                | CssToken::BgSelected => palette.elevated.clone(),

                CssToken::FgDefault | CssToken::FgOnAccent => palette.fg.clone(),
                CssToken::FgMuted | CssToken::FgSubtle => palette.fg_muted.clone(),

                CssToken::Accent | CssToken::AccentSubtle | CssToken::AccentHover => {
                    palette.primary.clone()
                }

                CssToken::Red
                | CssToken::StatusError
                | CssToken::StatusErrorSubtle
                | CssToken::StatusErrorHover
                | CssToken::BorderError => palette.red.clone(),

                CssToken::Yellow | CssToken::StatusWarning | CssToken::StatusWarningSubtle => {
                    palette.yellow.clone()
                }

                CssToken::Green | CssToken::StatusSuccess | CssToken::StatusSuccessSubtle => {
                    palette.green.clone()
                }

                CssToken::Blue | CssToken::StatusInfo | CssToken::StatusInfoSubtle => {
                    palette.blue.clone()
                }

                CssToken::BorderSubtle
                | CssToken::BorderDefault
                | CssToken::BorderStrong
                | CssToken::BorderAccent => palette.primary.clone(),
            }
        }
        ColorValue::Custom(hex) => hex.to_string(),
        ColorValue::Transparent => {
            return Rgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0,
            };
        }
        ColorValue::Auto => {
            let palette = config.config().styling.palette();
            palette.primary.clone()
        }
    };

    parse_hex_rgba(&hex)
}

fn parse_hex_rgba(hex: &str) -> Rgba {
    let hex = hex.trim_start_matches('#');
    let (r, g, b, a) = match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
            (r, g, b, 255u8)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
            (r, g, b, a)
        }
        _ => (255, 255, 255, 255),
    };

    Rgba {
        red: f64::from(r) / 255.0,
        green: f64::from(g) / 255.0,
        blue: f64::from(b) / 255.0,
        alpha: f64::from(a) / 255.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widget_length_single_bar() {
        assert_eq!(calculate_widget_length(1, 3, 1, 0.0), 3);
    }

    #[test]
    fn widget_length_multiple_bars() {
        assert_eq!(calculate_widget_length(20, 3, 1, 0.0), 79);
    }

    #[test]
    fn widget_length_zero_gap() {
        assert_eq!(calculate_widget_length(10, 5, 0, 0.0), 50);
    }

    #[test]
    fn widget_length_with_padding() {
        assert_eq!(calculate_widget_length(20, 3, 1, 8.0), 95);
    }

    #[test]
    fn parse_hex_6_digit() {
        let color = parse_hex_rgba("#ff0000");
        assert!((color.red - 1.0).abs() < f64::EPSILON);
        assert!(color.green.abs() < f64::EPSILON);
        assert!(color.blue.abs() < f64::EPSILON);
        assert!((color.alpha - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn parse_hex_8_digit_with_alpha() {
        let color = parse_hex_rgba("#00ff0080");
        assert!(color.red.abs() < f64::EPSILON);
        assert!((color.green - 1.0).abs() < f64::EPSILON);
        assert!(color.blue.abs() < f64::EPSILON);
        let expected_alpha = 128.0 / 255.0;
        assert!((color.alpha - expected_alpha).abs() < 0.01);
    }
}
