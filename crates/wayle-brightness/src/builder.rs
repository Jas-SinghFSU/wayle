use std::sync::{Arc, Mutex};

use tokio::sync::{broadcast, mpsc};
use tokio_util::sync::CancellationToken;
use tracing::info;
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;

use crate::{backend, error::Error, service::BrightnessService};

const EVENT_CHANNEL_CAPACITY: usize = 100;

/// Configuration for [`BrightnessService`](crate::BrightnessService) construction.
#[derive(Default)]
pub struct BrightnessServiceBuilder;

impl BrightnessServiceBuilder {
    pub(crate) fn new() -> Self {
        Self
    }

    /// Returns `Ok(None)` if no backlight devices exist (desktops, servers, VMs).
    ///
    /// # Errors
    ///
    /// Returns error if backend initialization fails.
    pub async fn build(self) -> Result<Option<Arc<BrightnessService>>, Error> {
        let initial_devices = backend::sysfs::enumerate();

        if initial_devices.is_empty() {
            info!("no backlight devices found, brightness service disabled");
            return Ok(None);
        }

        let device_count = initial_devices.len();

        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (event_tx, _) = broadcast::channel(EVENT_CHANNEL_CAPACITY);
        let cancellation_token = CancellationToken::new();

        let devices = Property::new(Vec::new());
        let primary = Property::new(None);

        let service = Arc::new(BrightnessService {
            command_tx,
            event_tx,
            cancellation_token,
            backend_handle: Mutex::new(None),
            devices,
            primary,
        });

        service.start_monitoring().await?;

        let backend_handle = backend::start(
            initial_devices,
            command_rx,
            service.event_tx.clone(),
            service.cancellation_token.child_token(),
        );

        service.set_backend_handle(backend_handle);

        info!(device_count, "brightness service started");

        Ok(Some(service))
    }
}
