use std::sync::Arc;

use wayle_cava::{CavaService, InputMethod};
use wayle_config::{ConfigService, schemas::modules::CavaInput};

const REM_BASE: f32 = 16.0;

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
}
