use tracing::warn;

use super::error::VolumeError;

/// Multi-channel volume with safety warnings
///
/// # Volume Safety Guidelines
/// - **Safe range**: 0.0 to 2.0 (0% to 200%)
/// - **Warning range**: 2.0 to 4.0 (may cause audio damage)
/// - **Invalid**: Above 4.0 (clamped) or below 0.0 (clamped)
///
/// # Volume Levels
/// - 0.0 = Muted
/// - 1.0 = Normal volume (100%)
/// - 2.0 = Safe maximum (200%)
/// - 4.0 = Absolute maximum (400% - **Audio damage possible**)
#[derive(Debug, Clone, PartialEq)]
pub struct Volume {
    volumes: Vec<f64>,
}

impl Volume {
    /// Create a new volume with the given channel volumes
    ///
    /// Volume levels are automatically clamped to valid range (0.0 to 4.0).
    /// - 0.0 = Muted
    /// - 1.0 = Normal volume (100%)
    /// - 4.0 = Maximum amplification (400%)
    pub fn new(volumes: Vec<f64>) -> Self {
        let volumes = volumes.into_iter().map(|v| {
            let clamped = v.clamp(0.0, 4.0);
            if v > 2.0 && v <= 4.0 {
                warn!("Volume {v} exceeds safe limit (2.0). Audio damage possible at high amplification.");
            } else if v > 4.0 {
                warn!("Volume {v} clamped to maximum (4.0). Use values ≤2.0 for safe operation.");
            } else if v < 0.0 {
                warn!("Negative volume {v} clamped to 0.0.");
            }
            clamped
        }).collect();
        Self { volumes }
    }

    /// Create volume with amplification (allows up to 4.0)
    ///
    /// # Safety
    /// Volumes above 2.0 may cause audio damage or distortion.
    /// Only use when amplification is explicitly required.
    ///
    /// # Errors
    /// Returns error if any volume is negative or exceeds 4.0.
    pub fn with_amplification(volumes: Vec<f64>) -> Result<Self, VolumeError> {
        for &volume in &volumes {
            if !(0.0..=4.0).contains(&volume) {
                return Err(VolumeError::InvalidVolume { channel: 0, volume });
            }
        }
        Ok(Self { volumes })
    }

    /// Create a mono volume
    ///
    /// Volume is automatically clamped to valid range (0.0 to 4.0).
    /// Use 1.0 for normal volume, values above 1.0 for amplification.
    pub fn mono(volume: f64) -> Self {
        Self::new(vec![volume])
    }

    /// Create a stereo volume
    ///
    /// Volume levels are automatically clamped to valid range (0.0 to 4.0).
    /// Use 1.0 for normal volume, values above 1.0 for amplification.
    pub fn stereo(left: f64, right: f64) -> Self {
        Self::new(vec![left, right])
    }

    /// Get volume for a specific channel
    pub fn channel(&self, channel: usize) -> Option<f64> {
        self.volumes.get(channel).copied()
    }

    /// Set volume for a specific channel
    ///
    /// Volume is automatically clamped to valid range (0.0 to 4.0).
    ///
    /// # Errors
    /// Returns error if channel index is out of bounds.
    pub fn set_channel(&mut self, channel: usize, volume: f64) -> Result<(), VolumeError> {
        if let Some(vol) = self.volumes.get_mut(channel) {
            let clamped = volume.clamp(0.0, 4.0);
            if volume > 2.0 && volume <= 4.0 {
                warn!(
                    "Volume {volume} exceeds safe limit (2.0). Audio damage possible at high amplification."
                );
            } else if volume > 4.0 {
                warn!(
                    "Volume {volume} clamped to maximum (4.0). Use values ≤2.0 for safe operation."
                );
            } else if volume < 0.0 {
                warn!("Negative volume {volume} clamped to 0.0.");
            }
            *vol = clamped;
            Ok(())
        } else {
            Err(VolumeError::InvalidChannel { channel })
        }
    }

    /// Get average volume across all channels
    pub fn average(&self) -> f64 {
        if self.volumes.is_empty() {
            0.0
        } else {
            self.volumes.iter().sum::<f64>() / self.volumes.len() as f64
        }
    }

    /// Get number of channels
    pub fn channels(&self) -> usize {
        self.volumes.len()
    }

    /// Get all channel volumes
    pub fn as_slice(&self) -> &[f64] {
        &self.volumes
    }

    /// Create a muted volume (0.0)
    pub fn muted(channels: usize) -> Self {
        Self::new(vec![0.0; channels])
    }

    /// Create a normal volume (1.0 = 100%)
    pub fn normal(channels: usize) -> Self {
        Self::new(vec![1.0; channels])
    }

    /// Create a volume from percentage (0-100% maps to 0.0-1.0)
    pub fn from_percentage(percentage: f64, channels: usize) -> Self {
        let volume = percentage / 100.0;
        Self::new(vec![volume; channels])
    }

    /// Get volume as percentage (1.0 = 100%)
    pub fn to_percentage(&self) -> Vec<f64> {
        self.volumes.iter().map(|&v| v * 100.0).collect()
    }

    /// Check if volume is muted (all channels at 0.0)
    pub fn is_muted(&self) -> bool {
        self.volumes.iter().all(|&v| v == 0.0)
    }

    /// Check if volume is at normal level (all channels at 1.0)
    pub fn is_normal(&self) -> bool {
        self.volumes.iter().all(|&v| v == 1.0)
    }
}
