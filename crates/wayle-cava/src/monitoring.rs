use std::time::Duration;

use tracing::{debug, error};
use wayle_traits::ServiceMonitoring;

use crate::{
    Error,
    ffi::{AudioInput, AudioOutput, Config, Plan},
    service::CavaService,
};

impl ServiceMonitoring for CavaService {
    type Error = Error;

    async fn start_monitoring(&self) -> Result<(), Self::Error> {
        let bars = self.bars.get();
        let autosens = self.autosens.get();
        let stereo = self.stereo.get();
        let noise_reduction = self.noise_reduction.get();
        let framerate = self.framerate.get();
        let input = self.input.get();
        let source = self.source.get();
        let low_cutoff = self.low_cutoff.get();
        let high_cutoff = self.high_cutoff.get();
        let samplerate = self.samplerate.get();

        let channels = if stereo { 2 } else { 1 };
        let buffer_size = calculate_cava_buffer_size(samplerate, channels);

        let mut audio_input = AudioInput::new(buffer_size, channels, samplerate, &source)?;
        let mut config = Config::new(
            bars,
            autosens,
            stereo,
            noise_reduction,
            framerate,
            input.into(),
            channels,
            samplerate,
            low_cutoff,
            high_cutoff,
            &source,
        )?;
        let mut audio_output = AudioOutput::new(bars);

        let plan = Plan::new(
            bars,
            samplerate,
            channels,
            autosens,
            noise_reduction,
            low_cutoff,
            high_cutoff,
        )?;

        audio_output.init(&mut audio_input, &mut config, &plan)?;

        audio_input.start_input(config)?;

        let values = self.values.clone();
        let cancellation = {
            let token = self
                .cancellation_token
                .lock()
                .map_err(|_| Error::InitFailed("Failed to lock cancellation token".to_string()))?;
            token.child_token()
        };

        tokio::spawn(async move {
            let interval_duration = Duration::from_millis(1000 / framerate as u64);
            let mut interval = tokio::time::interval(interval_duration);

            loop {
                tokio::select! {
                    _ = cancellation.cancelled() => {
                        debug!("Cava visualization loop cancelled");
                        return;
                    }
                    _ = interval.tick() => {
                        if let Err(e) = audio_input.lock() {
                            error!("Failed to lock audio input mutex: {}", e);
                            continue;
                        }

                        plan.execute(&audio_input, &audio_output);

                        if audio_input.samples_counter() > 0 {
                            audio_input.reset_samples_counter();
                        }

                        if let Err(e) = audio_input.unlock() {
                            error!("Failed to unlock audio input mutex: {}", e);
                        }

                        let new_values = audio_output.values().to_vec();
                        values.set(new_values);
                    }
                }
            }
        });

        Ok(())
    }
}

fn calculate_cava_buffer_size(sample_rate: u32, channels: u32) -> usize {
    const BASE_FFT_SIZE: usize = 512;
    const BASS_BUFFER_MULTIPLIER: usize = 2;

    let fft_multiplier = match sample_rate {
        0..=8125 => 1,
        8126..=16250 => 2,
        16251..=32500 => 4,
        32501..=75000 => 8,
        75001..=150000 => 16,
        150001..=300000 => 32,
        _ => 64,
    };

    let fft_size = BASE_FFT_SIZE * fft_multiplier;
    let fft_bass_size = fft_size * BASS_BUFFER_MULTIPLIER;
    fft_bass_size * channels as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE_FFT_SIZE: usize = 512;
    const BASS_BUFFER_MULTIPLIER: usize = 2;

    const TIER_1_MAX: u32 = 8125;
    const TIER_2_MIN: u32 = 8126;
    const TIER_2_MAX: u32 = 16250;
    const TIER_3_MIN: u32 = 16251;
    const TIER_3_MAX: u32 = 32500;
    const TIER_4_MIN: u32 = 32501;
    const TIER_4_MAX: u32 = 75000;
    const TIER_5_MIN: u32 = 75001;
    const TIER_5_MAX: u32 = 150000;
    const TIER_6_MIN: u32 = 150001;
    const TIER_6_MAX: u32 = 300000;
    const TIER_7_MIN: u32 = 300001;

    const MULTIPLIER_TIER_1: usize = 1;
    const MULTIPLIER_TIER_2: usize = 2;
    const MULTIPLIER_TIER_3: usize = 4;
    const MULTIPLIER_TIER_4: usize = 8;
    const MULTIPLIER_TIER_5: usize = 16;
    const MULTIPLIER_TIER_6: usize = 32;
    const MULTIPLIER_TIER_7: usize = 64;

    const MONO_CHANNELS: u32 = 1;
    const STEREO_CHANNELS: u32 = 2;

    #[test]
    fn calculate_cava_buffer_size_with_low_sample_rate_returns_base_multiplier() {
        let sample_rate = TIER_1_MAX;
        let channels = MONO_CHANNELS;

        let result = calculate_cava_buffer_size(sample_rate, channels);

        let expected =
            BASE_FFT_SIZE * MULTIPLIER_TIER_1 * BASS_BUFFER_MULTIPLIER * channels as usize;
        assert_eq!(result, expected);
    }

    #[test]
    fn calculate_cava_buffer_size_with_mid_sample_rate_returns_doubled_multiplier() {
        let sample_rate = TIER_2_MAX;
        let channels = MONO_CHANNELS;

        let result = calculate_cava_buffer_size(sample_rate, channels);

        let expected =
            BASE_FFT_SIZE * MULTIPLIER_TIER_2 * BASS_BUFFER_MULTIPLIER * channels as usize;
        assert_eq!(result, expected);
    }

    #[test]
    fn calculate_cava_buffer_size_with_high_sample_rate_returns_max_multiplier() {
        let sample_rate = TIER_7_MIN * 2;
        let channels = MONO_CHANNELS;

        let result = calculate_cava_buffer_size(sample_rate, channels);

        let expected =
            BASE_FFT_SIZE * MULTIPLIER_TIER_7 * BASS_BUFFER_MULTIPLIER * channels as usize;
        assert_eq!(result, expected);
    }

    #[test]
    fn calculate_cava_buffer_size_multiplies_by_channel_count() {
        let sample_rate = TIER_4_MAX;
        let channels = STEREO_CHANNELS;

        let result = calculate_cava_buffer_size(sample_rate, channels);

        let expected =
            BASE_FFT_SIZE * MULTIPLIER_TIER_4 * BASS_BUFFER_MULTIPLIER * channels as usize;
        assert_eq!(result, expected);
    }

    #[test]
    fn calculate_cava_buffer_size_at_boundary_values() {
        let channels = MONO_CHANNELS;

        assert_eq!(
            calculate_cava_buffer_size(TIER_1_MAX, channels),
            BASE_FFT_SIZE * MULTIPLIER_TIER_1 * BASS_BUFFER_MULTIPLIER * channels as usize
        );
        assert_eq!(
            calculate_cava_buffer_size(TIER_2_MIN, channels),
            BASE_FFT_SIZE * MULTIPLIER_TIER_2 * BASS_BUFFER_MULTIPLIER * channels as usize
        );

        assert_eq!(
            calculate_cava_buffer_size(TIER_2_MAX, channels),
            BASE_FFT_SIZE * MULTIPLIER_TIER_2 * BASS_BUFFER_MULTIPLIER * channels as usize
        );
        assert_eq!(
            calculate_cava_buffer_size(TIER_3_MIN, channels),
            BASE_FFT_SIZE * MULTIPLIER_TIER_3 * BASS_BUFFER_MULTIPLIER * channels as usize
        );

        assert_eq!(
            calculate_cava_buffer_size(TIER_3_MAX, channels),
            BASE_FFT_SIZE * MULTIPLIER_TIER_3 * BASS_BUFFER_MULTIPLIER * channels as usize
        );
        assert_eq!(
            calculate_cava_buffer_size(TIER_4_MIN, channels),
            BASE_FFT_SIZE * MULTIPLIER_TIER_4 * BASS_BUFFER_MULTIPLIER * channels as usize
        );

        assert_eq!(
            calculate_cava_buffer_size(TIER_4_MAX, channels),
            BASE_FFT_SIZE * MULTIPLIER_TIER_4 * BASS_BUFFER_MULTIPLIER * channels as usize
        );
        assert_eq!(
            calculate_cava_buffer_size(TIER_5_MIN, channels),
            BASE_FFT_SIZE * MULTIPLIER_TIER_5 * BASS_BUFFER_MULTIPLIER * channels as usize
        );

        assert_eq!(
            calculate_cava_buffer_size(TIER_5_MAX, channels),
            BASE_FFT_SIZE * MULTIPLIER_TIER_5 * BASS_BUFFER_MULTIPLIER * channels as usize
        );
        assert_eq!(
            calculate_cava_buffer_size(TIER_6_MIN, channels),
            BASE_FFT_SIZE * MULTIPLIER_TIER_6 * BASS_BUFFER_MULTIPLIER * channels as usize
        );

        assert_eq!(
            calculate_cava_buffer_size(TIER_6_MAX, channels),
            BASE_FFT_SIZE * MULTIPLIER_TIER_6 * BASS_BUFFER_MULTIPLIER * channels as usize
        );
        assert_eq!(
            calculate_cava_buffer_size(TIER_7_MIN, channels),
            BASE_FFT_SIZE * MULTIPLIER_TIER_7 * BASS_BUFFER_MULTIPLIER * channels as usize
        );
    }

    #[test]
    fn calculate_cava_buffer_size_above_threshold_uses_fallback_multiplier() {
        let sample_rate = TIER_7_MIN * 10;
        let channels = MONO_CHANNELS;

        let result = calculate_cava_buffer_size(sample_rate, channels);

        let expected =
            BASE_FFT_SIZE * MULTIPLIER_TIER_7 * BASS_BUFFER_MULTIPLIER * channels as usize;
        assert_eq!(result, expected);
    }
}
