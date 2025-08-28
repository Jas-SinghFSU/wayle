use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use tracing::{instrument, warn};
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::{ConnectionType, NetworkError, Wifi, Wired};
use crate::services::{
    common::Property,
    network::{
        core::{
            access_point::AccessPoint,
            config::{
                dhcp4_config::Dhcp4Config, dhcp6_config::Dhcp6Config, ip4_config::Ip4Config,
                ip6_config::Ip6Config,
            },
            connection::ActiveConnection,
            device::{Device, wifi::DeviceWifi, wired::DeviceWired},
            settings::Settings,
            settings_connection::ConnectionSettings,
        },
        discovery::NetworkServiceDiscovery,
        monitoring::NetworkMonitoring,
    },
};

/// Manages network connectivity through NetworkManager D-Bus interface.
///
/// Provides unified access to both WiFi and wired network interfaces,
/// tracking their state and managing connections. The service monitors
/// the primary connection type and exposes reactive properties for
/// network status changes.
pub struct NetworkService {
    zbus_connection: Connection,
    cancellation_token: CancellationToken,
    /// NetworkManager Settings interface for managing connection profiles.
    pub settings: Arc<Settings>,
    /// Current WiFi device state, if available.
    pub wifi: Option<Arc<Wifi>>,
    /// Current wired device state, if available.
    pub wired: Option<Arc<Wired>>,
    /// Type of the primary network connection.
    pub primary: Property<ConnectionType>,
}

impl NetworkService {
    /// Starts the network service and initializes all components.
    ///
    /// Performs device discovery, creates WiFi and wired service instances
    /// for available devices, and sets up property monitoring. Handles
    /// the actual initialization logic for the service.
    ///
    /// # Errors
    /// Returns `NetworkError::ServiceInitializationFailed` if:
    /// - D-Bus connection fails
    /// - Device discovery encounters errors
    /// - Device proxy creation fails
    #[instrument]
    pub async fn new() -> Result<Self, NetworkError> {
        let connection = Connection::system().await.map_err(|err| {
            NetworkError::ServiceInitializationFailed(format!("D-Bus connection failed: {err}"))
        })?;

        let cancellation_token = CancellationToken::new();

        let settings = Settings::get_live(&connection, cancellation_token.child_token())
            .await
            .map_err(|err| {
                NetworkError::ServiceInitializationFailed(format!(
                    "Failed to initialize Settings: {err}"
                ))
            })?;

        let wifi_device_path = NetworkServiceDiscovery::wifi_device_path(&connection).await?;
        let wired_device_path = NetworkServiceDiscovery::wired_device_path(&connection).await?;

        let wifi = if let Some(path) = wifi_device_path {
            match Wifi::get_live(&connection, path.clone(), cancellation_token.child_token()).await
            {
                Ok(wifi) => Some(wifi),
                Err(e) => {
                    warn!("Failed to create WiFi service from path {}: {}", path, e);
                    None
                }
            }
        } else {
            None
        };

        let wired = if let Some(path) = wired_device_path {
            match Wired::get_live(&connection, path.clone(), cancellation_token.child_token()).await
            {
                Ok(wired) => Some(wired),
                Err(e) => {
                    warn!("Failed to create Wired service from path {}: {}", path, e);
                    None
                }
            }
        } else {
            None
        };

        let primary = Property::new(ConnectionType::Unknown);

        NetworkMonitoring::start(
            &connection,
            wifi.clone(),
            wired.clone(),
            primary.clone(),
            cancellation_token.child_token(),
        )
        .await?;

        let service = Self {
            zbus_connection: connection.clone(),
            cancellation_token,
            settings,
            wifi,
            wired,
            primary,
        };

        Ok(service)
    }

    /// Objects that implement the Connection.Active interface represent an attempt to
    /// connect to a network using the details provided by a Connection object.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the connection doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    pub async fn connection(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<ActiveConnection>, NetworkError> {
        ActiveConnection::get(&self.zbus_connection, path).await
    }

    /// Objects that implement the Connection.Active interface represent an attempt to
    /// connect to a network using the details provided by a Connection object.
    /// This variant monitors the connection for changes.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the connection doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    pub async fn connection_monitored(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<ActiveConnection>, NetworkError> {
        ActiveConnection::get_live(
            &self.zbus_connection,
            path,
            self.cancellation_token.child_token(),
        )
        .await
    }

    /// Wi-Fi Access Point.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the access point doesn't exist.
    /// Returns `NetworkError::ObjectCreationFailed` if access point creation fails.
    pub async fn access_point(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<AccessPoint>, NetworkError> {
        AccessPoint::get(&self.zbus_connection, path).await
    }

    /// Wi-Fi Access Point.
    /// This variant monitors the access point for changes.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the access point doesn't exist.
    /// Returns `NetworkError::ObjectCreationFailed` if access point creation fails.
    pub async fn access_point_monitored(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<AccessPoint>, NetworkError> {
        AccessPoint::get_live(
            &self.zbus_connection,
            path,
            self.cancellation_token.child_token(),
        )
        .await
    }

    /// Represents a single network connection configuration.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the connection profile doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    pub async fn connection_settings(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<ConnectionSettings>, NetworkError> {
        ConnectionSettings::get(&self.zbus_connection, path).await
    }

    /// Represents a single network connection configuration.
    /// This variant monitors the connection settings for changes.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the connection profile doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    pub async fn connection_settings_monitored(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<ConnectionSettings>, NetworkError> {
        ConnectionSettings::get_live(
            &self.zbus_connection,
            path,
            self.cancellation_token.child_token(),
        )
        .await
    }

    /// Represents a network device.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the device doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    pub async fn device(&self, path: OwnedObjectPath) -> Result<Arc<Device>, NetworkError> {
        Device::get(&self.zbus_connection, path).await
    }

    /// Represents a network device.
    /// This variant monitors the device for changes.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the device doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    pub async fn device_monitored(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<Device>, NetworkError> {
        Device::get_live(
            &self.zbus_connection,
            path,
            self.cancellation_token.child_token(),
        )
        .await
    }

    /// Represents a Wi-Fi device.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the device doesn't exist.
    /// Returns `NetworkError::WrongObjectType` if the device is not a WiFi device.
    pub async fn device_wifi(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<DeviceWifi>, NetworkError> {
        DeviceWifi::get(&self.zbus_connection, path).await
    }

    /// Represents a Wi-Fi device.
    /// This variant monitors the device for changes.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the device doesn't exist.
    /// Returns `NetworkError::WrongObjectType` if the device is not a WiFi device.
    pub async fn device_wifi_monitored(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<DeviceWifi>, NetworkError> {
        DeviceWifi::get_live(
            &self.zbus_connection,
            path,
            self.cancellation_token.child_token(),
        )
        .await
    }

    /// Represents a wired Ethernet device.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the device doesn't exist.
    /// Returns `NetworkError::WrongObjectType` if the device is not an ethernet device.
    pub async fn device_wired(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<DeviceWired>, NetworkError> {
        DeviceWired::get(&self.zbus_connection, path).await
    }

    /// Represents a wired Ethernet device.
    /// This variant monitors the device for changes.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the device doesn't exist.
    /// Returns `NetworkError::WrongObjectType` if the device is not an ethernet device.
    pub async fn device_wired_monitored(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<DeviceWired>, NetworkError> {
        DeviceWired::get_live(
            &self.zbus_connection,
            path,
            self.cancellation_token.child_token(),
        )
        .await
    }

    /// IPv4 Configuration Set.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the configuration doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    pub async fn ip4_config(&self, path: OwnedObjectPath) -> Result<Arc<Ip4Config>, NetworkError> {
        Ip4Config::get(&self.zbus_connection, path).await
    }

    /// IPv6 Configuration Set.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the configuration doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    pub async fn ip6_config(&self, path: OwnedObjectPath) -> Result<Arc<Ip6Config>, NetworkError> {
        Ip6Config::get(&self.zbus_connection, path).await
    }

    /// DHCP4 Configuration.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the configuration doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    pub async fn dhcp4_config(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<Dhcp4Config>, NetworkError> {
        Dhcp4Config::get(&self.zbus_connection, path).await
    }

    /// DHCP6 Configuration.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the configuration doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    pub async fn dhcp6_config(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<Dhcp6Config>, NetworkError> {
        Dhcp6Config::get(&self.zbus_connection, path).await
    }
}

impl Drop for NetworkService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
