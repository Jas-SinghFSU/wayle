use std::{sync::Arc, time::Duration};

use derive_more::Debug;
use tokio::{
    sync::{Mutex, broadcast, mpsc},
    time::sleep,
};
use tokio_util::sync::CancellationToken;
use tracing::{error, instrument};
use wayle_common::Property;
use wayle_traits::{Reactive, ServiceMonitoring};
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::{
    agent::{BluetoothAgent, event_processor, providers},
    core::{
        adapter::{Adapter, AdapterParams, LiveAdapterParams},
        device::{Device, DeviceParams, LiveDeviceParams},
    },
    types::{
        ServiceNotification,
        agent::{PairingRequest, PairingResponder},
    },
};
use crate::{
    Error,
    discovery::BluetoothDiscovery,
    proxy::agent_manager::AgentManager1Proxy,
    types::agent::{AgentCapability, AgentEvent},
};

/// Bluetooth connectivity via BlueZ D-Bus.
///
/// See [crate-level documentation](crate) for reactive property patterns
/// and live vs snapshot instance behavior.
#[derive(Debug)]
pub struct BluetoothService {
    #[debug(skip)]
    pub(crate) zbus_connection: Connection,
    #[debug(skip)]
    pub(crate) cancellation_token: CancellationToken,
    #[debug(skip)]
    pub(crate) notifier_tx: broadcast::Sender<ServiceNotification>,
    #[debug(skip)]
    pub(crate) pairing_responder: Arc<Mutex<Option<PairingResponder>>>,

    /// All Bluetooth adapters on the system (live).
    pub adapters: Property<Vec<Arc<Adapter>>>,
    /// Active adapter for discovery and operations (live).
    pub primary_adapter: Property<Option<Arc<Adapter>>>,
    /// All discovered devices across adapters (live).
    pub devices: Property<Vec<Arc<Device>>>,
    /// Whether any adapter is present.
    pub available: Property<bool>,
    /// Whether any adapter is powered.
    pub enabled: Property<bool>,
    /// Addresses of connected devices.
    pub connected: Property<Vec<String>>,

    /// Pending pairing request awaiting response.
    pub pairing_request: Property<Option<PairingRequest>>,
}

impl BluetoothService {
    /// Creates a new Bluetooth service instance.
    ///
    /// Establishes D-Bus connection, discovers available adapters,
    /// and initializes monitoring for device and adapter changes.
    ///
    /// # Errors
    /// Returns error if D-Bus connection fails or service initialization fails.
    pub async fn new() -> Result<Self, Error> {
        let connection = Connection::system()
            .await
            .map_err(|err| Error::ServiceInitialization(Box::new(err)))?;

        let cancellation_token = CancellationToken::new();

        let (notifier_tx, _) = broadcast::channel::<ServiceNotification>(100);
        let (agent_tx, agent_rx) = mpsc::unbounded_channel::<AgentEvent>();

        let agent = BluetoothAgent {
            service_tx: agent_tx.clone(),
        };
        let agent_path = OwnedObjectPath::try_from("/com/wayle/BluetoothAgent")
            .map_err(|err| Error::AgentRegistration(Box::new(err)))?;

        connection.object_server().at(&agent_path, agent).await?;

        let agent_manager = AgentManager1Proxy::new(&connection).await?;
        agent_manager
            .register_agent(&agent_path, &AgentCapability::DisplayYesNo.to_string())
            .await?;

        let BluetoothDiscovery {
            adapters,
            primary_adapter,
            devices,
            available,
            enabled,
            connected,
        } = BluetoothDiscovery::new(&connection, cancellation_token.child_token(), &notifier_tx)
            .await?;

        let service = Self {
            notifier_tx,
            pairing_responder: Arc::new(Mutex::new(None)),
            zbus_connection: connection.clone(),
            cancellation_token: cancellation_token.clone(),
            adapters: Property::new(adapters),
            primary_adapter: Property::new(primary_adapter),
            devices: Property::new(devices),
            available: Property::new(available),
            enabled: Property::new(enabled),
            connected: Property::new(connected),
            pairing_request: Property::new(None),
        };

        event_processor::start(
            agent_rx,
            &service.pairing_responder,
            &service.pairing_request,
            cancellation_token.child_token(),
        )
        .await
        .unwrap_or_else(|error| {
            error!(error = %error, "cannot start agent event processor");
            error!("Bluetooth pairing may be degraded");
        });

        service.start_monitoring().await?;

        Ok(service)
    }

    /// Creates a point-in-time Device instance for the specified device path.
    ///
    /// # Errors
    /// Returns error if the device path is invalid or D-Bus communication fails.
    pub async fn device(&self, device_path: OwnedObjectPath) -> Result<Device, Error> {
        Device::get(DeviceParams {
            connection: &self.zbus_connection,
            notifier_tx: &self.notifier_tx,
            path: device_path,
        })
        .await
    }

    /// Creates a monitored Device instance that tracks property changes.
    ///
    /// # Errors
    /// Returns error if the device path is invalid or D-Bus communication fails.
    pub async fn device_monitored(
        &self,
        device_path: OwnedObjectPath,
    ) -> Result<Arc<Device>, Error> {
        Device::get_live(LiveDeviceParams {
            connection: &self.zbus_connection,
            notifier_tx: &self.notifier_tx,
            path: device_path,
            cancellation_token: &self.cancellation_token,
        })
        .await
    }

    /// Creates a point-in-time Adapter instance for the specified adapter path.
    ///
    /// # Errors
    /// Returns error if the adapter path is invalid or D-Bus communication fails.
    pub async fn adapter(&self, adapter_path: OwnedObjectPath) -> Result<Adapter, Error> {
        Adapter::get(AdapterParams {
            connection: &self.zbus_connection,
            path: adapter_path,
        })
        .await
    }

    /// Creates a monitored Adapter instance that tracks property changes.
    ///
    /// # Errors
    /// Returns error if the adapter path is invalid or D-Bus communication fails.
    pub async fn adapter_monitored(
        &self,
        adapter_path: OwnedObjectPath,
    ) -> Result<Arc<Adapter>, Error> {
        Adapter::get_live(LiveAdapterParams {
            connection: &self.zbus_connection,
            path: adapter_path,
            cancellation_token: &self.cancellation_token,
        })
        .await
    }

    /// Starts device discovery on the primary adapter.
    ///
    /// Begins scanning for nearby Bluetooth devices. Discovery will continue
    /// until explicitly stopped with `stop_discovery()`.
    ///
    /// # Errors
    ///
    /// Returns error if no primary adapter is available or discovery operation fails.
    #[instrument(skip(self), err)]
    pub async fn start_discovery(&self) -> Result<(), Error> {
        let Some(active_adapter) = self.primary_adapter.get() else {
            return Err(Error::NoPrimaryAdapter {
                operation: "start discovery",
            });
        };

        active_adapter.start_discovery().await
    }

    /// Starts device discovery on the primary adapter for a limited time.
    ///
    /// Begins scanning for nearby Bluetooth devices. Discovery will continue
    /// for the provided duration.
    ///
    /// # Errors
    ///
    /// Returns error if no primary adapter is available or discovery operation fails.
    #[instrument(skip(self), fields(duration_secs = duration.as_secs()), err)]
    pub async fn start_timed_discovery(&self, duration: Duration) -> Result<(), Error> {
        let Some(active_adapter) = self.primary_adapter.get() else {
            return Err(Error::NoPrimaryAdapter {
                operation: "start timed discovery",
            });
        };

        active_adapter.start_discovery().await?;

        tokio::spawn(async move {
            let _ = sleep(duration).await;
            if let Err(error) = active_adapter.stop_discovery().await {
                error!(error = %error, "cannot stop timed discovery");
            };
        });

        Ok(())
    }

    /// Stops device discovery on all adapters.
    ///
    /// Halts the scanning process started by `start_discovery()`.
    ///
    /// # Errors
    ///
    /// Returns error if no primary adapter is available or stop operation fails.
    #[instrument(skip(self), err)]
    pub async fn stop_discovery(&self) -> Result<(), Error> {
        let Some(active_adapter) = self.primary_adapter.get() else {
            return Err(Error::NoPrimaryAdapter {
                operation: "stop discovery",
            });
        };

        active_adapter.stop_discovery().await
    }

    /// Enables Bluetooth by powering on the primary adapter.
    ///
    /// If no primary adapter is set, powers on the first available adapter.
    ///
    /// # Errors
    ///
    /// Returns error if no primary adapter is available or power operation fails.
    #[instrument(skip(self), err)]
    pub async fn enable(&self) -> Result<(), Error> {
        let Some(active_adapter) = self.primary_adapter.get() else {
            return Err(Error::NoPrimaryAdapter {
                operation: "enable bluetooth",
            });
        };

        active_adapter.set_powered(true).await
    }

    /// Disables Bluetooth by powering off all adapters.
    ///
    /// All active connections will be terminated.
    ///
    /// # Errors
    ///
    /// Returns error if no primary adapter is available or power operation fails.
    #[instrument(skip(self), err)]
    pub async fn disable(&self) -> Result<(), Error> {
        let Some(active_adapter) = self.primary_adapter.get() else {
            return Err(Error::NoPrimaryAdapter {
                operation: "disable bluetooth",
            });
        };

        active_adapter.set_powered(false).await
    }

    /// Provides a PIN code for legacy device pairing.
    ///
    /// Responds to `PairingRequest::RequestPinCode`.
    /// PIN must be 1-16 alphanumeric characters.
    ///
    /// # Errors
    ///
    /// Returns error if no PIN request is pending or responder channel is closed.
    #[instrument(skip(self, pin), err)]
    pub async fn provide_pin(&self, pin: String) -> Result<(), Error> {
        providers::pin(self, pin).await
    }

    /// Provides a numeric passkey for device pairing.
    ///
    /// Responds to `PairingRequest::RequestPasskey`.
    /// Passkey must be between 0-999999.
    ///
    /// # Errors
    ///
    /// Returns error if no passkey request is pending or responder channel is closed.
    #[instrument(skip(self, passkey), err)]
    pub async fn provide_passkey(&self, passkey: u32) -> Result<(), Error> {
        providers::passkey(self, passkey).await
    }

    /// Provides confirmation for passkey matching.
    ///
    /// Responds to `PairingRequest::RequestConfirmation`.
    /// Confirms whether the displayed passkey matches the remote device.
    ///
    /// # Errors
    ///
    /// Returns error if no confirmation request is pending or responder channel is closed.
    #[instrument(skip(self), fields(confirmed = confirmation), err)]
    pub async fn provide_confirmation(&self, confirmation: bool) -> Result<(), Error> {
        providers::confirmation(self, confirmation).await
    }

    /// Provides authorization for device pairing or connection.
    ///
    /// Responds to `PairingRequest::RequestAuthorization`.
    /// Grants or denies permission for the device to pair/connect.
    ///
    /// # Errors
    ///
    /// Returns error if no authorization request is pending or responder channel is closed.
    pub async fn provide_authorization(&self, authorization: bool) -> Result<(), Error> {
        providers::authorization(self, authorization).await
    }

    /// Provides authorization for specific Bluetooth service access.
    ///
    /// Responds to `PairingRequest::RequestServiceAuthorization`.
    /// Grants or denies permission for the device to use a specific service/profile.
    ///
    /// # Errors
    ///
    /// Returns error if no service authorization request is pending or responder channel is closed.
    pub async fn provide_service_authorization(&self, authorization: bool) -> Result<(), Error> {
        providers::service_authorization(self, authorization).await
    }
}

impl Drop for BluetoothService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
