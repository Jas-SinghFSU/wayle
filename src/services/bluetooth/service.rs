use std::{fmt::Display, sync::Arc};

use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::services::{
    bluetooth::{
        BluetoothError,
        agent::BluetoothAgent,
        proxy::AgentManager1Proxy,
        types::{AgentCapability, AgentEvent},
    },
    common::Property,
};

use super::core::{Adapter, Device};

/// Manages Bluetooth connectivity through the BlueZ D-Bus interface.
///
/// Provides a high-level API for Bluetooth operations including device discovery,
/// pairing, and connection management. Automatically tracks adapter state and
/// maintains reactive properties for UI consumption.
pub struct BluetoothService {
    zbus_connection: Connection,
    cancellation_token: CancellationToken,

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
    pub connected: Property<bool>,
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

        let (agent_tx, _) = mpsc::channel::<AgentEvent>(32);
        let agent = BluetoothAgent {
            service_tx: agent_tx,
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

        // Start initial discovery

        // Initialize property containers

        // Initialize monitoring

        // Instantialize service

        // Return service
        todo!()
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
}

impl Drop for BluetoothService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
