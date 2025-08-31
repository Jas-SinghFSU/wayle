use std::sync::Arc;

use tokio::sync::{
    Mutex,
    mpsc::{self, UnboundedSender},
};
use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::{
    core::{Adapter, Device},
    monitoring::BluetoothMonitoring,
    types::{PairingRequest, PairingResponder, ServiceNotification},
};
use crate::services::{
    bluetooth::{
        BluetoothError,
        agent::BluetoothAgent,
        discovery::BluetoothDiscovery,
        proxy::AgentManager1Proxy,
        types::{AgentCapability, AgentEvent},
    },
    common::Property,
};

/// Manages Bluetooth connectivity through the BlueZ D-Bus interface.
///
/// Provides a high-level API for Bluetooth operations including device discovery,
/// pairing, and connection management. Automatically tracks adapter state and
/// maintains reactive properties for UI consumption.
pub struct BluetoothService {
    zbus_connection: Connection,
    cancellation_token: CancellationToken,
    agent_tx: UnboundedSender<AgentEvent>,
    notifier_tx: UnboundedSender<ServiceNotification>,
    pairing_responder: Arc<Mutex<Option<PairingResponder>>>,

    /// All available Bluetooth adapters on the system.
    pub adapters: Property<Vec<Arc<Adapter>>>,
    /// Currently active adapter for discovery and operations.
    pub primary_adapter: Property<Option<Arc<Adapter>>>,
    /// All discovered Bluetooth devices across all adapters.
    pub devices: Property<Vec<Arc<Device>>>,
    /// Whether any Bluetooth adapter is present on the system.
    pub available: Property<bool>,
    /// Whether Bluetooth is enabled (at least one adapter powered).
    pub enabled: Property<bool>,
    /// Whether any device is currently connected.
    pub connected: Property<Vec<String>>,

    pub pairing_request: Property<Option<PairingRequest>>,
}

impl BluetoothService {
    /// Creates a new Bluetooth service instance.
    ///
    /// Establishes D-Bus connection, discovers available adapters,
    /// and initializes monitoring for device and adapter changes.
    pub async fn new() -> Result<Self, BluetoothError> {
        let connection = Connection::system().await.map_err(|err| {
            BluetoothError::ServiceInitializationFailed(format!("D-Bus connection failed: {err}"))
        })?;

        let cancellation_token = CancellationToken::new();

        let (notifier_tx, _notifier_rx) = mpsc::unbounded_channel::<ServiceNotification>();
        let (agent_tx, _agent_rx) = mpsc::unbounded_channel::<AgentEvent>();

        let agent = BluetoothAgent {
            service_tx: agent_tx.clone(),
        };
        let agent_path = OwnedObjectPath::try_from("/org/wayle/BluetoothAgent").map_err(|err| {
            BluetoothError::AgentRegistrationFailed(format!(
                "Failed to construct agent path: {err}"
            ))
        })?;

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
            agent_tx: agent_tx.clone(),
            notifier_tx: notifier_tx.clone(),
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

        BluetoothMonitoring::start(
            &connection,
            cancellation_token,
            &service.adapters,
            &service.primary_adapter,
            &service.devices,
            &service.enabled,
            &service.available,
        )
        .await?;

        Ok(service)
    }

    /// Starts device discovery on the primary adapter.
    ///
    /// Begins scanning for nearby Bluetooth devices. Discovery will continue
    /// until explicitly stopped with `stop_discovery()`.
    pub async fn start_discovery() {
        todo!()
    }

    /// Stops device discovery on all adapters.
    ///
    /// Halts the scanning process started by `start_discovery()`.
    pub async fn stop_discovery() {
        todo!()
    }

    /// Enables Bluetooth by powering on the primary adapter.
    ///
    /// If no primary adapter is set, powers on the first available adapter.
    pub async fn enable() {
        todo!()
    }

    /// Disables Bluetooth by powering off all adapters.
    ///
    /// All active connections will be terminated.
    pub async fn disable() {
        todo!()
    }

    /// Provides a PIN code for legacy device pairing.
    ///
    /// Called in response to `PairingRequest::RequestPinCode`.
    /// PIN must be 1-16 alphanumeric characters.
    ///
    /// # Errors
    ///
    /// Returns error if no PIN request is pending or responder channel is closed.
    pub async fn provide_pin() {
        todo!()
    }

    /// Provides a numeric passkey for device pairing.
    ///
    /// Called in response to `PairingRequest::RequestPasskey`.
    /// Passkey must be between 0-999999.
    ///
    /// # Errors
    ///
    /// Returns error if no passkey request is pending or responder channel is closed.
    pub async fn provide_passkey() {
        todo!()
    }

    /// Provides confirmation for passkey matching.
    ///
    /// Called in response to `PairingRequest::RequestConfirmation`.
    /// Confirms whether displayed passkey matches remote device.
    ///
    /// # Errors
    ///
    /// Returns error if no confirmation request is pending or responder channel is closed.
    pub async fn provide_confirmation() {
        todo!()
    }

    /// Provides authorization for pairing or service connection.
    ///
    /// Called in response to `PairingRequest::RequestAuthorization` or
    /// `PairingRequest::AuthorizeService`.
    ///
    /// # Errors
    ///
    /// Returns error if no authorization request is pending or responder channel is closed.
    pub async fn provide_authorization() {
        todo!()
    }
}

impl Drop for BluetoothService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
