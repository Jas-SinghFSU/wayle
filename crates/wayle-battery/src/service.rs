use std::sync::Arc;

use derive_more::Debug;
use tokio_util::sync::CancellationToken;

use crate::{builder::BatteryServiceBuilder, core::device::Device, error::Error};

/// Battery service for monitoring power devices via UPower.
///
/// Provides a high-level interface to UPower's battery monitoring,
/// allowing access to battery state, capacity, charge status, and reactive
/// monitoring of changes through the D-Bus interface.
#[derive(Debug)]
pub struct BatteryService {
    #[debug(skip)]
    pub(crate) cancellation_token: CancellationToken,

    /// The UPower battery device proxy for power metrics and charging state.
    pub device: Arc<Device>,
}

impl BatteryService {
    /// Creates a new battery service for the default DisplayDevice.
    ///
    /// The DisplayDevice is UPower's composite device that represents the overall
    /// battery status, automatically handling multiple batteries if present.
    /// This is the recommended way to monitor battery status for most applications.
    ///
    /// # Errors
    ///
    /// Returns `Error::ServiceInitializationFailed` if service initialization fails.
    pub async fn new() -> Result<Self, Error> {
        Self::builder().build().await
    }

    /// Creates a builder for configuring a BatteryService.
    ///
    /// Use this when you need to monitor a specific battery device
    /// rather than the default aggregated DisplayDevice.
    pub fn builder() -> BatteryServiceBuilder {
        BatteryServiceBuilder::new()
    }
}

impl Drop for BatteryService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
