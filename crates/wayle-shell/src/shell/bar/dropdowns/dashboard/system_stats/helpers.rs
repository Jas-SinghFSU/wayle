use wayle_widgets::primitives::progress_ring::ColorVariant;

const WARNING_THRESHOLD: f32 = 60.0;
const ERROR_THRESHOLD: f32 = 85.0;

pub(super) fn threshold_color(percent: f32) -> ColorVariant {
    if percent >= ERROR_THRESHOLD {
        ColorVariant::Error
    } else if percent >= WARNING_THRESHOLD {
        ColorVariant::Warning
    } else {
        ColorVariant::Success
    }
}

const TEMP_WARNING_CELSIUS: f32 = 65.0;
const TEMP_ERROR_CELSIUS: f32 = 85.0;

pub(super) fn temp_color(celsius: f32) -> ColorVariant {
    if celsius >= TEMP_ERROR_CELSIUS {
        ColorVariant::Error
    } else if celsius >= TEMP_WARNING_CELSIUS {
        ColorVariant::Warning
    } else {
        ColorVariant::Success
    }
}
