use std::fmt::{self, Display, Formatter};

use crate::ffi;

const BAR_COUNT_MIN: u16 = 1;
const BAR_COUNT_MAX: u16 = 256;
const FRAMERATE_MIN: u32 = 1;
const FRAMERATE_MAX: u32 = 360;

/// Frequency bar count clamped to 1-256 (libcava limitation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BarCount(u16);

impl BarCount {
    /// Default bar count (20).
    pub const DEFAULT: Self = Self(20);

    /// Creates a bar count, clamping to 1-256.
    #[must_use]
    pub fn new(value: u16) -> Self {
        Self(value.clamp(BAR_COUNT_MIN, BAR_COUNT_MAX))
    }

    /// Returns the inner u16 value.
    #[must_use]
    pub fn value(self) -> u16 {
        self.0
    }

    /// Returns a bar count compatible with stereo output.
    ///
    /// libcava requires an even bar count when stereo is enabled (bars are
    /// split equally between left and right channels). Odd values are
    /// rounded up to the next even number.
    #[must_use]
    pub(crate) fn adjusted_for_stereo(self, stereo: bool) -> Self {
        if stereo && !self.0.is_multiple_of(2) {
            Self(self.0.saturating_add(1).min(BAR_COUNT_MAX))
        } else {
            self
        }
    }
}

impl Default for BarCount {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Display for BarCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u16> for BarCount {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<i32> for BarCount {
    fn from(value: i32) -> Self {
        Self::new(value.clamp(BAR_COUNT_MIN as i32, BAR_COUNT_MAX as i32) as u16)
    }
}

impl From<usize> for BarCount {
    fn from(value: usize) -> Self {
        let clamped = value.min(BAR_COUNT_MAX as usize) as u16;
        Self::new(clamped)
    }
}

/// Visualization framerate clamped to 1-360 fps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Framerate(u32);

impl Framerate {
    /// Default framerate (60 fps).
    pub const DEFAULT: Self = Self(60);

    /// Creates a framerate, clamping to 1-360.
    #[must_use]
    pub fn new(value: u32) -> Self {
        Self(value.clamp(FRAMERATE_MIN, FRAMERATE_MAX))
    }

    /// Returns the inner u32 value.
    #[must_use]
    pub fn value(self) -> u32 {
        self.0
    }
}

impl Default for Framerate {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Display for Framerate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for Framerate {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<i32> for Framerate {
    fn from(value: i32) -> Self {
        Self::new(value.clamp(FRAMERATE_MIN as i32, FRAMERATE_MAX as i32) as u32)
    }
}

/// Audio input method for capturing system audio.
///
/// Specifies which audio backend CAVA should use to capture audio data for visualization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMethod {
    /// Read audio from a named pipe (FIFO).
    Fifo,

    /// PortAudio cross-platform audio I/O library.
    PortAudio,

    /// PipeWire multimedia server (default).
    PipeWire,

    /// Advanced Linux Sound Architecture (ALSA).
    Alsa,

    /// PulseAudio sound server.
    Pulse,

    /// sndio audio subsystem.
    Sndio,

    /// Open Sound System.
    Oss,

    /// JACK Audio Connection Kit.
    Jack,

    /// Read audio from shared memory.
    Shmem,

    /// Windows audio capture (WASAPI).
    Winscap,
}

impl From<InputMethod> for ffi::InputMethod {
    fn from(method: InputMethod) -> Self {
        match method {
            InputMethod::Fifo => ffi::InputMethod::Fifo,
            InputMethod::PortAudio => ffi::InputMethod::PortAudio,
            InputMethod::PipeWire => ffi::InputMethod::PipeWire,
            InputMethod::Alsa => ffi::InputMethod::Alsa,
            InputMethod::Pulse => ffi::InputMethod::Pulse,
            InputMethod::Sndio => ffi::InputMethod::Sndio,
            InputMethod::Oss => ffi::InputMethod::Oss,
            InputMethod::Jack => ffi::InputMethod::Jack,
            InputMethod::Shmem => ffi::InputMethod::Shmem,
            InputMethod::Winscap => ffi::InputMethod::Winscap,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bar_count_clamps_zero_to_minimum() {
        assert_eq!(BarCount::new(0).value(), BAR_COUNT_MIN);
    }

    #[test]
    fn bar_count_clamps_above_max() {
        assert_eq!(BarCount::new(BAR_COUNT_MAX + 1).value(), BAR_COUNT_MAX);
    }

    #[test]
    fn bar_count_preserves_valid_value() {
        assert_eq!(BarCount::new(128).value(), 128);
    }

    #[test]
    fn bar_count_from_usize_clamps_above_max() {
        let large: usize = 1000;
        assert_eq!(BarCount::from(large).value(), BAR_COUNT_MAX);
    }

    #[test]
    fn bar_count_from_usize_zero_clamps_to_minimum() {
        assert_eq!(BarCount::from(0_usize).value(), BAR_COUNT_MIN);
    }

    #[test]
    fn bar_count_adjusted_for_stereo_rounds_odd_up() {
        assert_eq!(BarCount::new(21).adjusted_for_stereo(true).value(), 22);
    }

    #[test]
    fn bar_count_adjusted_for_stereo_preserves_even() {
        assert_eq!(BarCount::new(20).adjusted_for_stereo(true).value(), 20);
    }

    #[test]
    fn bar_count_adjusted_for_stereo_unchanged_when_mono() {
        assert_eq!(BarCount::new(21).adjusted_for_stereo(false).value(), 21);
    }

    #[test]
    fn bar_count_adjusted_for_stereo_at_max_boundary() {
        assert_eq!(BarCount::new(255).adjusted_for_stereo(true).value(), 256);
    }

    #[test]
    fn bar_count_adjusted_for_stereo_at_min_boundary() {
        assert_eq!(BarCount::new(1).adjusted_for_stereo(true).value(), 2);
    }

    #[test]
    fn framerate_clamps_zero_to_minimum() {
        assert_eq!(Framerate::new(0).value(), FRAMERATE_MIN);
    }

    #[test]
    fn framerate_clamps_above_max() {
        assert_eq!(Framerate::new(FRAMERATE_MAX + 1).value(), FRAMERATE_MAX);
    }

    #[test]
    fn framerate_preserves_valid_value() {
        assert_eq!(Framerate::new(144).value(), 144);
    }
}
