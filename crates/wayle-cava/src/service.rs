use std::sync::Mutex;

use derive_more::Debug;
use tokio::sync::Mutex as AsyncMutex;
use tokio_util::sync::CancellationToken;
use tracing::instrument;
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;

use crate::{Error, types::InputMethod};

const DEFAULT_BARS: usize = 20;
const MAX_BARS: usize = 256;
const DEFAULT_AUTOSENS: bool = true;
const DEFAULT_STEREO: bool = false;
const DEFAULT_NOISE_REDUCTION: f64 = 0.77;
const DEFAULT_FRAMERATE: u32 = 60;
const DEFAULT_INPUT: InputMethod = InputMethod::PipeWire;
const DEFAULT_SOURCE: &str = "auto";
const DEFAULT_LOW_CUTOFF: u32 = 50;
const DEFAULT_HIGH_CUTOFF: u32 = 10000;
const DEFAULT_SAMPLERATE: u32 = 44100;

/// CAVA audio visualization service.
///
/// Provides real-time audio frequency visualization using the CAVA library.
/// The service captures system audio and outputs frequency bar data that can be
/// used for visual representations.
#[derive(Debug)]
pub struct CavaService {
    #[debug(skip)]
    pub(crate) cancellation_token: Mutex<CancellationToken>,
    #[debug(skip)]
    restart_lock: AsyncMutex<()>,

    /// Current visualization bar values (0.0 to 1.0, can overshoot).
    ///
    /// Updates at the configured framerate. Length equals the `bars` property.
    /// In stereo mode, first half is left channel, second half is right channel.
    pub values: Property<Vec<f64>>,

    /// Number of frequency bars to generate.
    ///
    /// Valid range: 1-256. Default: 20. Changing this property requires a service restart.
    pub bars: Property<usize>,

    /// Automatic sensitivity adjustment.
    ///
    /// When enabled, sensitivity is automatically adjusted to keep values in 0-1 range.
    pub autosens: Property<bool>,

    /// Stereo channel visualization.
    ///
    /// When enabled, splits bars between left and right channels.
    pub stereo: Property<bool>,

    /// Noise reduction filter strength.
    ///
    /// Range: 0.0 (fast, noisy) to 1.0 (slow, smooth). Default: 0.77.
    pub noise_reduction: Property<f64>,

    /// Visualization update rate in frames per second.
    pub framerate: Property<u32>,

    /// Audio input method/backend.
    ///
    /// Determines which audio system to use for capturing audio.
    pub input: Property<InputMethod>,

    /// Audio source identifier.
    ///
    /// Source string format depends on the input method. Use "auto" for automatic selection.
    pub source: Property<String>,

    /// Low frequency cutoff in Hz.
    ///
    /// Frequencies below this value are filtered out. Default: 50.
    pub low_cutoff: Property<u32>,

    /// High frequency cutoff in Hz.
    ///
    /// Frequencies above this value are filtered out. Default: 10000.
    pub high_cutoff: Property<u32>,

    /// Audio sample rate in Hz.
    ///
    /// Should match the input audio sample rate. Default: 44100.
    pub samplerate: Property<u32>,
}

impl CavaService {
    /// Creates a new CAVA service with default configuration.
    ///
    /// Initializes audio capture and starts the visualization loop.
    ///
    /// # Errors
    /// Returns error if audio initialization fails or if the selected input method
    /// is unavailable.
    #[instrument]
    pub async fn new() -> Result<Self, Error> {
        CavaServiceBuilder::new().build().await
    }

    /// Creates a builder for customizing service configuration.
    pub fn builder() -> CavaServiceBuilder {
        CavaServiceBuilder::new()
    }

    /// Sets the number of frequency bars.
    ///
    /// Updates the configuration and restarts the visualization service.
    ///
    /// # Errors
    /// Returns error if bars is 0, exceeds 256, or if service restart fails.
    pub async fn set_bars(&self, bars: usize) -> Result<(), Error> {
        if bars == 0 {
            return Err(Error::InvalidParameter(
                "bars must be greater than 0".into(),
            ));
        }

        if bars > MAX_BARS {
            return Err(Error::InvalidParameter(format!(
                "bars must not exceed {MAX_BARS} (CAVA limitation), got {bars}"
            )));
        }

        self.bars.set(bars);
        self.restart().await
    }

    /// Enables or disables automatic sensitivity adjustment.
    ///
    /// Updates the configuration and restarts the visualization service.
    ///
    /// # Errors
    /// Returns error if service restart fails.
    pub async fn set_autosens(&self, autosens: bool) -> Result<(), Error> {
        self.autosens.set(autosens);
        self.restart().await
    }

    /// Enables or disables stereo channel visualization.
    ///
    /// Updates the configuration and restarts the visualization service.
    ///
    /// # Errors
    /// Returns error if service restart fails.
    pub async fn set_stereo(&self, stereo: bool) -> Result<(), Error> {
        self.stereo.set(stereo);
        self.restart().await
    }

    /// Sets the noise reduction filter strength.
    ///
    /// Updates the configuration and restarts the visualization service.
    ///
    /// # Errors
    /// Returns error if service restart fails.
    pub async fn set_noise_reduction(&self, noise_reduction: f64) -> Result<(), Error> {
        self.noise_reduction.set(noise_reduction);
        self.restart().await
    }

    /// Sets the visualization update framerate.
    ///
    /// Updates the configuration and restarts the visualization service.
    ///
    /// # Errors
    /// Returns error if framerate is 0 or if service restart fails.
    pub async fn set_framerate(&self, framerate: u32) -> Result<(), Error> {
        if framerate == 0 {
            return Err(Error::InvalidParameter(
                "framerate must be greater than 0".into(),
            ));
        }
        self.framerate.set(framerate);
        self.restart().await
    }

    /// Sets the audio input method.
    ///
    /// Updates the configuration and restarts the visualization service.
    ///
    /// # Errors
    /// Returns error if the new input method is unavailable or if service restart fails.
    pub async fn set_input(&self, input: InputMethod) -> Result<(), Error> {
        self.input.set(input);
        self.restart().await
    }

    /// Sets the audio source identifier.
    ///
    /// Updates the configuration and restarts the visualization service.
    ///
    /// # Errors
    /// Returns error if the source string contains null bytes or if service restart fails.
    pub async fn set_source(&self, source: impl Into<String>) -> Result<(), Error> {
        self.source.set(source.into());
        self.restart().await
    }

    /// Sets the low frequency cutoff.
    ///
    /// Updates the configuration and restarts the visualization service.
    ///
    /// # Errors
    /// Returns error if low_cutoff is 0 or if service restart fails.
    pub async fn set_low_cutoff(&self, low_cutoff: u32) -> Result<(), Error> {
        if low_cutoff == 0 {
            return Err(Error::InvalidParameter(
                "low_cutoff must be greater than 0".into(),
            ));
        }
        self.low_cutoff.set(low_cutoff);
        self.restart().await
    }

    /// Sets the high frequency cutoff.
    ///
    /// Updates the configuration and restarts the visualization service.
    ///
    /// # Errors
    /// Returns error if high_cutoff is 0 or if service restart fails.
    pub async fn set_high_cutoff(&self, high_cutoff: u32) -> Result<(), Error> {
        if high_cutoff == 0 {
            return Err(Error::InvalidParameter(
                "high_cutoff must be greater than 0".into(),
            ));
        }
        self.high_cutoff.set(high_cutoff);
        self.restart().await
    }

    /// Sets the audio sample rate.
    ///
    /// Updates the configuration and restarts the visualization service.
    ///
    /// # Errors
    /// Returns error if samplerate is 0 or if service restart fails.
    pub async fn set_samplerate(&self, samplerate: u32) -> Result<(), Error> {
        if samplerate == 0 {
            return Err(Error::InvalidParameter(
                "samplerate must be greater than 0".into(),
            ));
        }
        self.samplerate.set(samplerate);
        self.restart().await
    }

    async fn restart(&self) -> Result<(), Error> {
        let _ = self.restart_lock.lock().await;

        {
            let mut token = self
                .cancellation_token
                .lock()
                .map_err(|_| Error::InitFailed("Failed to lock cancellation token".to_string()))?;

            token.cancel();
            *token = CancellationToken::new();
        }

        self.start_monitoring().await
    }
}

/// Builder for configuring and creating a [`CavaService`].
///
/// Provides a fluent interface for setting audio visualization parameters.
/// All parameters have sensible defaults and can be selectively overridden.
pub struct CavaServiceBuilder {
    bars: usize,
    autosens: bool,
    stereo: bool,
    noise_reduction: f64,
    framerate: u32,
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
            bars: DEFAULT_BARS,
            autosens: DEFAULT_AUTOSENS,
            stereo: DEFAULT_STEREO,
            noise_reduction: DEFAULT_NOISE_REDUCTION,
            framerate: DEFAULT_FRAMERATE,
            input: DEFAULT_INPUT,
            source: DEFAULT_SOURCE.to_string(),
            low_cutoff: DEFAULT_LOW_CUTOFF,
            high_cutoff: DEFAULT_HIGH_CUTOFF,
            samplerate: DEFAULT_SAMPLERATE,
        }
    }

    /// Sets the number of frequency bars to generate.
    ///
    /// Valid range: 1-256.
    pub fn bars(mut self, bars: usize) -> Self {
        self.bars = bars;
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

    /// Sets the visualization update rate in frames per second.
    ///
    /// Must be greater than 0.
    pub fn framerate(mut self, framerate: u32) -> Self {
        self.framerate = framerate;
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
    /// Validates all parameters and starts the audio visualization loop.
    ///
    /// # Errors
    /// Returns error if:
    /// - `bars` is 0 or exceeds 256
    /// - `framerate`, `low_cutoff`, `high_cutoff`, or `samplerate` is 0
    /// - `high_cutoff` is not greater than `low_cutoff`
    /// - `samplerate` is not greater than 2 * `high_cutoff` (violates Nyquist theorem)
    /// - `noise_reduction` is not in range 0.0-1.0
    /// - Audio initialization fails
    /// - Selected input method is unavailable
    #[instrument(skip(self))]
    pub async fn build(self) -> Result<CavaService, Error> {
        if self.bars == 0 {
            return Err(Error::InvalidParameter(
                "bars must be greater than 0".into(),
            ));
        }

        if self.bars > MAX_BARS {
            return Err(Error::InvalidParameter(format!(
                "bars must not exceed {} (CAVA limitation), got {}",
                MAX_BARS, self.bars
            )));
        }

        if self.framerate == 0 {
            return Err(Error::InvalidParameter(
                "framerate must be greater than 0".into(),
            ));
        }

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

        let service = CavaService {
            cancellation_token: Mutex::new(CancellationToken::new()),
            restart_lock: AsyncMutex::new(()),
            values: Property::new(vec![0.0; self.bars]),
            bars: Property::new(self.bars),
            autosens: Property::new(self.autosens),
            stereo: Property::new(self.stereo),
            noise_reduction: Property::new(self.noise_reduction),
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

impl Drop for CavaService {
    fn drop(&mut self) {
        if let Ok(token) = self.cancellation_token.lock() {
            token.cancel();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_BARS: usize = 20;
    const VALID_FRAMERATE: u32 = 60;
    const VALID_LOW_CUTOFF: u32 = 50;
    const VALID_HIGH_CUTOFF: u32 = 10000;
    const VALID_SAMPLERATE: u32 = 44100;
    const VALID_NOISE_REDUCTION: f64 = 0.77;

    const ZERO_BARS: usize = 0;
    const EXCESSIVE_BARS: usize = MAX_BARS + 1;
    const ZERO_FRAMERATE: u32 = 0;
    const ZERO_LOW_CUTOFF: u32 = 0;
    const ZERO_HIGH_CUTOFF: u32 = 0;
    const ZERO_SAMPLERATE: u32 = 0;

    fn valid_builder() -> CavaServiceBuilder {
        CavaServiceBuilder::new()
            .bars(VALID_BARS)
            .framerate(VALID_FRAMERATE)
            .low_cutoff(VALID_LOW_CUTOFF)
            .high_cutoff(VALID_HIGH_CUTOFF)
            .samplerate(VALID_SAMPLERATE)
            .noise_reduction(VALID_NOISE_REDUCTION)
    }

    #[tokio::test]
    async fn builder_build_with_zero_bars_returns_error() {
        let builder = valid_builder().bars(ZERO_BARS);

        let result = builder.build().await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidParameter(_)));
    }

    #[tokio::test]
    async fn builder_build_with_bars_exceeding_max_returns_error() {
        let builder = valid_builder().bars(EXCESSIVE_BARS);

        let result = builder.build().await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::InvalidParameter(_)));
    }

    #[tokio::test]
    async fn builder_build_with_zero_framerate_returns_error() {
        let builder = valid_builder().framerate(ZERO_FRAMERATE);

        let result = builder.build().await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidParameter(_)));
    }

    #[tokio::test]
    async fn builder_build_with_zero_low_cutoff_returns_error() {
        let builder = valid_builder().low_cutoff(ZERO_LOW_CUTOFF);

        let result = builder.build().await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidParameter(_)));
    }

    #[tokio::test]
    async fn builder_build_with_zero_high_cutoff_returns_error() {
        let builder = valid_builder().high_cutoff(ZERO_HIGH_CUTOFF);

        let result = builder.build().await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidParameter(_)));
    }

    #[tokio::test]
    async fn builder_build_with_zero_samplerate_returns_error() {
        let builder = valid_builder().samplerate(ZERO_SAMPLERATE);

        let result = builder.build().await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidParameter(_)));
    }

    #[tokio::test]
    async fn builder_build_with_high_cutoff_less_than_or_equal_to_low_cutoff_returns_error() {
        let low = VALID_LOW_CUTOFF;
        let high_equal = low;
        let high_less = low - 1;

        let result_equal = valid_builder()
            .low_cutoff(low)
            .high_cutoff(high_equal)
            .build()
            .await;

        assert!(result_equal.is_err());
        assert!(matches!(
            result_equal.unwrap_err(),
            Error::InvalidParameter(_)
        ));

        let result_less = valid_builder()
            .low_cutoff(low)
            .high_cutoff(high_less)
            .build()
            .await;

        assert!(result_less.is_err());
        assert!(matches!(
            result_less.unwrap_err(),
            Error::InvalidParameter(_)
        ));
    }

    #[tokio::test]
    async fn builder_build_with_samplerate_violating_nyquist_returns_error() {
        let high_cutoff = VALID_HIGH_CUTOFF;
        let invalid_samplerate = high_cutoff * 2;

        let result = valid_builder()
            .high_cutoff(high_cutoff)
            .samplerate(invalid_samplerate)
            .build()
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidParameter(_)));
    }

    #[tokio::test]
    async fn builder_build_with_noise_reduction_below_zero_returns_error() {
        let below_zero = -0.1;

        let result = valid_builder().noise_reduction(below_zero).build().await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidParameter(_)));
    }

    #[tokio::test]
    async fn builder_build_with_noise_reduction_above_one_returns_error() {
        let above_one = 1.1;

        let result = valid_builder().noise_reduction(above_one).build().await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidParameter(_)));
    }
}
