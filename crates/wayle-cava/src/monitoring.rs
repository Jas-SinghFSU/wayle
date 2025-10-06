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
