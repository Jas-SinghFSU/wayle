use std::sync::Mutex;

use derive_more::Debug;
use tokio::sync::Mutex as AsyncMutex;
use tokio_util::sync::CancellationToken;
use tracing::instrument;
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;

use crate::{
    Error,
    builder::CavaServiceBuilder,
    types::{BarCount, Framerate, InputMethod},
};

pub(crate) const DEFAULT_AUTOSENS: bool = true;
pub(crate) const DEFAULT_STEREO: bool = false;
pub(crate) const DEFAULT_NOISE_REDUCTION: f64 = 0.77;
pub(crate) const DEFAULT_MONSTERCAT: f64 = 0.0;
pub(crate) const DEFAULT_WAVES: u32 = 0;
pub(crate) const DEFAULT_INPUT: InputMethod = InputMethod::PipeWire;
pub(crate) const DEFAULT_SOURCE: &str = "auto";
pub(crate) const DEFAULT_LOW_CUTOFF: u32 = 50;
pub(crate) const DEFAULT_HIGH_CUTOFF: u32 = 10000;
pub(crate) const DEFAULT_SAMPLERATE: u32 = 44100;

/// Audio visualization service wrapping libcava.
///
/// See the [crate-level documentation](crate) for usage examples and field descriptions.
#[derive(Debug)]
pub struct CavaService {
    #[debug(skip)]
    pub(crate) cancellation_token: Mutex<CancellationToken>,
    #[debug(skip)]
    pub(crate) restart_lock: AsyncMutex<()>,

    /// Bar amplitudes, updated at `framerate`. Stereo splits L/R halves.
    pub values: Property<Vec<f64>>,

    /// Frequency bar count, clamped to 1-256 (libcava limitation).
    pub bars: Property<BarCount>,

    /// Auto-adjust sensitivity to keep values in 0-1 range.
    pub autosens: Property<bool>,

    /// Split bars between left and right audio channels.
    pub stereo: Property<bool>,

    /// Smoothing filter: 0.0 = fast/noisy, 1.0 = slow/smooth (default 0.77).
    pub noise_reduction: Property<f64>,

    /// Monstercat-style smoothing across adjacent bars (0.0 = off).
    pub monstercat: Property<f64>,

    /// Wave-style smoothing (0 = off).
    pub waves: Property<u32>,

    /// Visualization update rate, clamped to 1-360 fps.
    pub framerate: Property<Framerate>,

    /// Audio capture backend (default PipeWire).
    pub input: Property<InputMethod>,

    /// Audio source identifier ("auto" for auto-detect).
    pub source: Property<String>,

    /// Low frequency cutoff in Hz (default 50).
    pub low_cutoff: Property<u32>,

    /// High frequency cutoff in Hz (default 10000).
    pub high_cutoff: Property<u32>,

    /// Audio sample rate in Hz (default 44100).
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
    /// Accepts [`BarCount`] (clamped to 1-256) or raw `usize`/`u16`.
    /// Restarts the visualization service with the new bar count.
    ///
    /// # Errors
    /// Returns error if service restart fails.
    pub async fn set_bars(&self, bars: impl Into<BarCount>) -> Result<(), Error> {
        self.bars.set(bars.into());
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

    /// Sets monstercat-style smoothing level across adjacent bars.
    ///
    /// 0.0 disables monstercat smoothing. Higher values produce smoother falloff
    /// between neighboring bars.
    ///
    /// # Errors
    /// Returns error if `monstercat` is negative or if service restart fails.
    pub async fn set_monstercat(&self, monstercat: f64) -> Result<(), Error> {
        if monstercat < 0.0 {
            return Err(Error::InvalidParameter("monstercat must be >= 0.0".into()));
        }
        self.monstercat.set(monstercat);
        self.restart().await
    }

    /// Sets wave-style smoothing count.
    ///
    /// 0 disables wave smoothing. Mutually exclusive with monstercat smoothing
    /// (monstercat takes priority if both are non-zero).
    ///
    /// # Errors
    /// Returns error if service restart fails.
    pub async fn set_waves(&self, waves: u32) -> Result<(), Error> {
        self.waves.set(waves);
        self.restart().await
    }

    /// Sets the visualization update framerate.
    ///
    /// Accepts [`Framerate`] (clamped to 1-360) or raw `u32`.
    /// Restarts the visualization service with the new framerate.
    ///
    /// # Errors
    /// Returns error if service restart fails.
    pub async fn set_framerate(&self, framerate: impl Into<Framerate>) -> Result<(), Error> {
        self.framerate.set(framerate.into());
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
                .map_err(|_| Error::InitFailed("cannot lock cancellation token".to_string()))?;

            token.cancel();
            *token = CancellationToken::new();
        }

        self.start_monitoring().await
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

    const VALID_BARS: BarCount = BarCount::DEFAULT;
    const VALID_FRAMERATE: Framerate = Framerate::DEFAULT;
    const VALID_LOW_CUTOFF: u32 = 50;
    const VALID_HIGH_CUTOFF: u32 = 10000;
    const VALID_SAMPLERATE: u32 = 44100;
    const VALID_NOISE_REDUCTION: f64 = 0.77;

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

    #[tokio::test]
    async fn builder_build_with_negative_monstercat_returns_error() {
        let negative = -0.1;

        let result = valid_builder().monstercat(negative).build().await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidParameter(_)));
    }
}
