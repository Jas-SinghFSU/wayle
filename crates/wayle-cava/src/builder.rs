use std::sync::Mutex;

use tokio::sync::Mutex as AsyncMutex;
use tokio_util::sync::CancellationToken;
use tracing::{instrument, warn};
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;

use crate::{
    Error,
    service::{
        CavaService, DEFAULT_AUTOSENS, DEFAULT_HIGH_CUTOFF, DEFAULT_INPUT, DEFAULT_LOW_CUTOFF,
        DEFAULT_MONSTERCAT, DEFAULT_NOISE_REDUCTION, DEFAULT_SAMPLERATE, DEFAULT_SOURCE,
        DEFAULT_STEREO, DEFAULT_WAVES,
    },
    types::{BarCount, Framerate, InputMethod},
};

/// Builder for configuring and creating a [`CavaService`].
///
/// Provides a fluent interface for setting audio visualization parameters.
/// All parameters have sensible defaults and can be selectively overridden.
pub struct CavaServiceBuilder {
    bars: BarCount,
    autosens: bool,
    stereo: bool,
    noise_reduction: f64,
    monstercat: f64,
    waves: u32,
    framerate: Framerate,
    input: InputMethod,
    source: String,
    low_cutoff: u32,
    high_cutoff: u32,
    samplerate: u32,
}

impl Default for CavaServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CavaServiceBuilder {
    /// Creates a new builder with default audio visualization settings.
    pub fn new() -> Self {
        Self {
            bars: BarCount::DEFAULT,
            autosens: DEFAULT_AUTOSENS,
            stereo: DEFAULT_STEREO,
            noise_reduction: DEFAULT_NOISE_REDUCTION,
            monstercat: DEFAULT_MONSTERCAT,
            waves: DEFAULT_WAVES,
            framerate: Framerate::DEFAULT,
            input: DEFAULT_INPUT,
            source: DEFAULT_SOURCE.to_string(),
            low_cutoff: DEFAULT_LOW_CUTOFF,
            high_cutoff: DEFAULT_HIGH_CUTOFF,
            samplerate: DEFAULT_SAMPLERATE,
        }
    }

    /// Sets the number of frequency bars to generate.
    ///
    /// Accepts [`BarCount`] (clamped to 1-256) or raw `usize`/`u16`.
    pub fn bars(mut self, bars: impl Into<BarCount>) -> Self {
        self.bars = bars.into();
        self
    }

    /// Enables or disables automatic sensitivity adjustment.
    ///
    /// When enabled, CAVA automatically adjusts sensitivity to keep values in 0-1 range.
    pub fn autosens(mut self, autosens: bool) -> Self {
        self.autosens = autosens;
        self
    }

    /// Enables or disables stereo channel visualization.
    ///
    /// When enabled, splits bars between left and right audio channels.
    pub fn stereo(mut self, stereo: bool) -> Self {
        self.stereo = stereo;
        self
    }

    /// Sets the noise reduction filter strength.
    ///
    /// Range: 0.0 (fast, noisy) to 1.0 (slow, smooth).
    pub fn noise_reduction(mut self, noise_reduction: f64) -> Self {
        self.noise_reduction = noise_reduction;
        self
    }

    /// Sets monstercat-style smoothing across adjacent bars.
    ///
    /// 0.0 disables monstercat smoothing. Higher values produce smoother falloff.
    pub fn monstercat(mut self, monstercat: f64) -> Self {
        self.monstercat = monstercat;
        self
    }

    /// Sets wave-style smoothing count.
    ///
    /// 0 disables wave smoothing. Mutually exclusive with monstercat
    /// (monstercat takes priority if both non-zero).
    pub fn waves(mut self, waves: u32) -> Self {
        self.waves = waves;
        self
    }

    /// Sets the visualization update rate in frames per second.
    ///
    /// Accepts [`Framerate`] (clamped to 1-360) or raw `u32`.
    pub fn framerate(mut self, framerate: impl Into<Framerate>) -> Self {
        self.framerate = framerate.into();
        self
    }

    /// Sets the audio input method/backend.
    ///
    /// Determines which audio system to use for capturing audio (PipeWire, PulseAudio, ALSA, etc.).
    pub fn input(mut self, input: InputMethod) -> Self {
        self.input = input;
        self
    }

    /// Sets the audio source identifier.
    ///
    /// Source string format depends on the input method. Use "auto" for automatic selection.
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }

    /// Sets the low frequency cutoff in Hz.
    ///
    /// Frequencies below this value are filtered out. Must be greater than 0.
    pub fn low_cutoff(mut self, low_cutoff: u32) -> Self {
        self.low_cutoff = low_cutoff;
        self
    }

    /// Sets the high frequency cutoff in Hz.
    ///
    /// Frequencies above this value are filtered out. Must be greater than low_cutoff
    /// and less than samplerate/2.
    pub fn high_cutoff(mut self, high_cutoff: u32) -> Self {
        self.high_cutoff = high_cutoff;
        self
    }

    /// Sets the audio sample rate in Hz.
    ///
    /// Should match the input audio sample rate. Must be greater than 0 and at least
    /// 2*high_cutoff (Nyquist theorem).
    pub fn samplerate(mut self, samplerate: u32) -> Self {
        self.samplerate = samplerate;
        self
    }

    /// Builds and initializes the CAVA service with the configured parameters.
    ///
    /// `bars` and `framerate` are validated by their newtypes (clamped on construction).
    /// Cross-field constraints (cutoffs, samplerate, noise_reduction) are validated here.
    ///
    /// # Errors
    /// Returns error if:
    /// - `low_cutoff`, `high_cutoff`, or `samplerate` is 0
    /// - `high_cutoff` is not greater than `low_cutoff`
    /// - `samplerate` is not greater than 2 * `high_cutoff` (Nyquist theorem)
    /// - `noise_reduction` is not in range 0.0-1.0
    /// - `monstercat` is negative
    /// - Audio initialization fails or input method is unavailable
    #[instrument(skip(self))]
    pub async fn build(self) -> Result<CavaService, Error> {
        if self.low_cutoff == 0 {
            return Err(Error::InvalidParameter(
                "low_cutoff must be greater than 0".into(),
            ));
        }

        if self.high_cutoff == 0 {
            return Err(Error::InvalidParameter(
                "high_cutoff must be greater than 0".into(),
            ));
        }

        if self.samplerate == 0 {
            return Err(Error::InvalidParameter(
                "samplerate must be greater than 0".into(),
            ));
        }

        if self.high_cutoff <= self.low_cutoff {
            return Err(Error::InvalidParameter(format!(
                "high_cutoff ({}) must be greater than low_cutoff ({})",
                self.high_cutoff, self.low_cutoff
            )));
        }

        if self.samplerate / 2 <= self.high_cutoff {
            return Err(Error::InvalidParameter(format!(
                "samplerate ({}) must be greater than 2 * high_cutoff ({})",
                self.samplerate, self.high_cutoff
            )));
        }

        if !(0.0..=1.0).contains(&self.noise_reduction) {
            return Err(Error::InvalidParameter(format!(
                "noise_reduction must be between 0.0 and 1.0, got {}",
                self.noise_reduction
            )));
        }

        if self.monstercat < 0.0 {
            return Err(Error::InvalidParameter("monstercat must be >= 0.0".into()));
        }

        let effective_bars = self.bars.adjusted_for_stereo(self.stereo);
        if effective_bars != self.bars {
            warn!(
                requested = %self.bars,
                adjusted = %effective_bars,
                "odd bar count rounded up for stereo output"
            );
        }
        let bar_count = effective_bars.value() as usize;

        let service = CavaService {
            cancellation_token: Mutex::new(CancellationToken::new()),
            restart_lock: AsyncMutex::new(()),
            values: Property::new(vec![0.0; bar_count]),
            bars: Property::new(self.bars),
            autosens: Property::new(self.autosens),
            stereo: Property::new(self.stereo),
            noise_reduction: Property::new(self.noise_reduction),
            monstercat: Property::new(self.monstercat),
            waves: Property::new(self.waves),
            framerate: Property::new(self.framerate),
            input: Property::new(self.input),
            source: Property::new(self.source),
            low_cutoff: Property::new(self.low_cutoff),
            high_cutoff: Property::new(self.high_cutoff),
            samplerate: Property::new(self.samplerate),
        };

        service.start_monitoring().await?;

        Ok(service)
    }
}
