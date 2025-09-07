use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use tracing::{instrument, warn};
use zbus::{Connection, zvariant::OwnedObjectPath};

use super::{
    core::access_point::types::{AccessPointParams, LiveAccessPointParams},
    error::NetworkError,
    types::connectivity::ConnectionType,
    wifi::Wifi,
    wired::Wired,
};
use crate::services::{
    common::property::Property,
    network::{
        core::{
            access_point::AccessPoint,
            config::{
                dhcp4_config::{Dhcp4Config, Dhcp4ConfigParams},
                dhcp6_config::{Dhcp6Config, Dhcp6ConfigParams},
                ip4_config::{Ip4Config, Ip4ConfigParams},
                ip6_config::{Ip6Config, Ip6ConfigParams},
            },
            connection::{ActiveConnection, ActiveConnectionParams, LiveActiveConnectionParams},
            device::{
                Device, DeviceParams, LiveDeviceParams,
                wifi::{DeviceWifi, DeviceWifiParams, LiveDeviceWifiParams},
                wired::{DeviceWired, DeviceWiredParams, LiveDeviceWiredParams},
            },
            settings::{LiveSettingsParams, Settings},
            settings_connection::{
                ConnectionSettings, ConnectionSettingsParams, LiveConnectionSettingsParams,
            },
        },
        discovery::NetworkServiceDiscovery,
        wifi::LiveWifiParams,
        wired::LiveWiredParams,
    },
    traits::{Reactive, ServiceMonitoring, Static},
};

/// Manages network connectivity through NetworkManager D-Bus interface.
///
/// Provides unified access to both WiFi and wired network interfaces,
/// tracking their state and managing connections. The service monitors
/// the primary connection type and exposes reactive properties for
/// network status changes.
pub struct NetworkService {
    pub(crate) zbus_connection: Connection,
    pub(crate) cancellation_token: CancellationToken,
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

        let settings = Settings::get_live(LiveSettingsParams {
            zbus_connection: &connection,
            cancellation_token: &cancellation_token,
        })
        .await
        .map_err(|err| {
            NetworkError::ServiceInitializationFailed(format!(
                "Failed to initialize Settings: {err}"
            ))
        })?;

        let wifi_device_path = NetworkServiceDiscovery::wifi_device_path(&connection).await?;
        let wired_device_path = NetworkServiceDiscovery::wired_device_path(&connection).await?;

        let wifi = if let Some(path) = wifi_device_path {
            match Wifi::get_live(LiveWifiParams {
                connection: &connection,
                device_path: path.clone(),
                cancellation_token: &cancellation_token,
            })
            .await
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
            match Wired::get_live(LiveWiredParams {
                connection: &connection,
                device_path: path.clone(),
                cancellation_token: &cancellation_token,
            })
            .await
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

        let service = Self {
            zbus_connection: connection.clone(),
            cancellation_token,
            settings,
            wifi,
            wired,
            primary,
        };

        service.start_monitoring().await?;

        Ok(service)
    }

    /// Objects that implement the Connection.Active interface represent an attempt to
    /// connect to a network using the details provided by a Connection object.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the connection doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    #[instrument(skip(self, path), fields(path = ?path), err)]
    pub async fn connection(
        &self,
        path: OwnedObjectPath,
    ) -> Result<ActiveConnection, NetworkError> {
        ActiveConnection::get(ActiveConnectionParams {
            connection: &self.zbus_connection,
            path,
        })
        .await
    }

    /// Objects that implement the Connection.Active interface represent an attempt to
    /// connect to a network using the details provided by a Connection object.
    /// This variant monitors the connection for changes.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the connection doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    #[instrument(skip(self, path), fields(path = ?path), err)]
    pub async fn connection_monitored(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<ActiveConnection>, NetworkError> {
        ActiveConnection::get_live(LiveActiveConnectionParams {
            connection: &self.zbus_connection,
            path,
            cancellation_token: &self.cancellation_token,
        })
        .await
    }

    /// Wi-Fi Access Point.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the access point doesn't exist.
    /// Returns `NetworkError::ObjectCreationFailed` if access point creation fails.
    #[instrument(skip(self, path), fields(path = ?path), err)]
    pub async fn access_point(&self, path: OwnedObjectPath) -> Result<AccessPoint, NetworkError> {
        AccessPoint::get(AccessPointParams {
            connection: &self.zbus_connection,
            path,
        })
        .await
    }

    /// Wi-Fi Access Point.
    /// This variant monitors the access point for changes.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the access point doesn't exist.
    /// Returns `NetworkError::ObjectCreationFailed` if access point creation fails.
    #[instrument(skip(self, path), fields(path = ?path), err)]
    pub async fn access_point_monitored(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<AccessPoint>, NetworkError> {
        AccessPoint::get_live(LiveAccessPointParams {
            connection: &self.zbus_connection,
            path,
            cancellation_token: &self.cancellation_token,
        })
        .await
    }

    /// Represents a single network connection configuration.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the connection profile doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    #[instrument(skip(self, path), fields(path = ?path), err)]
    pub async fn connection_settings(
        &self,
        path: OwnedObjectPath,
    ) -> Result<ConnectionSettings, NetworkError> {
        ConnectionSettings::get(ConnectionSettingsParams {
            connection: &self.zbus_connection,
            path,
        })
        .await
    }

    /// Represents a single network connection configuration.
    /// This variant monitors the connection settings for changes.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the connection profile doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    #[instrument(skip(self, path), fields(path = ?path), err)]
    pub async fn connection_settings_monitored(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<ConnectionSettings>, NetworkError> {
        ConnectionSettings::get_live(LiveConnectionSettingsParams {
            connection: &self.zbus_connection,
            path,
            cancellation_token: &self.cancellation_token,
        })
        .await
    }

    /// Represents a network device.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the device doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    #[instrument(skip(self, path), fields(path = ?path), err)]
    pub async fn device(&self, path: OwnedObjectPath) -> Result<Device, NetworkError> {
        Device::get(DeviceParams {
            connection: &self.zbus_connection,
            object_path: path,
        })
        .await
    }

    /// Represents a network device.
    /// This variant monitors the device for changes.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the device doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    #[instrument(skip(self, path), fields(path = ?path), err)]
    pub async fn device_monitored(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<Device>, NetworkError> {
        Device::get_live(LiveDeviceParams {
            connection: &self.zbus_connection,
            object_path: path,
            cancellation_token: &self.cancellation_token,
        })
        .await
    }

    /// Represents a Wi-Fi device.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the device doesn't exist.
    /// Returns `NetworkError::WrongObjectType` if the device is not a WiFi device.
    #[instrument(skip(self, path), fields(path = ?path), err)]
    pub async fn device_wifi(&self, path: OwnedObjectPath) -> Result<DeviceWifi, NetworkError> {
        DeviceWifi::get(DeviceWifiParams {
            connection: &self.zbus_connection,
            device_path: path,
        })
        .await
    }

    /// Represents a Wi-Fi device.
    /// This variant monitors the device for changes.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the device doesn't exist.
    /// Returns `NetworkError::WrongObjectType` if the device is not a WiFi device.
    #[instrument(skip(self, path), fields(path = ?path), err)]
    pub async fn device_wifi_monitored(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<DeviceWifi>, NetworkError> {
        DeviceWifi::get_live(LiveDeviceWifiParams {
            connection: &self.zbus_connection,
            device_path: path,
            cancellation_token: &self.cancellation_token,
        })
        .await
    }

    /// Represents a wired Ethernet device.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the device doesn't exist.
    /// Returns `NetworkError::WrongObjectType` if the device is not an ethernet device.
    #[instrument(skip(self, path), fields(path = ?path), err)]
    pub async fn device_wired(&self, path: OwnedObjectPath) -> Result<DeviceWired, NetworkError> {
        DeviceWired::get(DeviceWiredParams {
            connection: &self.zbus_connection,
            device_path: path,
        })
        .await
    }

    /// Represents a wired Ethernet device.
    /// This variant monitors the device for changes.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the device doesn't exist.
    /// Returns `NetworkError::WrongObjectType` if the device is not an ethernet device.
    #[instrument(skip(self, path), fields(path = ?path), err)]
    pub async fn device_wired_monitored(
        &self,
        path: OwnedObjectPath,
    ) -> Result<Arc<DeviceWired>, NetworkError> {
        DeviceWired::get_live(LiveDeviceWiredParams {
            connection: &self.zbus_connection,
            device_path: path,
            cancellation_token: &self.cancellation_token,
        })
        .await
    }

    /// IPv4 Configuration Set.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the configuration doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    #[instrument(skip(self, path), fields(path = ?path), err)]
    pub async fn ip4_config(&self, path: OwnedObjectPath) -> Result<Ip4Config, NetworkError> {
        Ip4Config::get(Ip4ConfigParams {
            connection: &self.zbus_connection,
            path,
        })
        .await
    }

    /// IPv6 Configuration Set.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the configuration doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    #[instrument(skip(self, path), fields(path = ?path), err)]
    pub async fn ip6_config(&self, path: OwnedObjectPath) -> Result<Ip6Config, NetworkError> {
        Ip6Config::get(Ip6ConfigParams {
            connection: &self.zbus_connection,
            path,
        })
        .await
    }

    /// DHCP4 Configuration.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the configuration doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    #[instrument(skip(self, path), fields(path = ?path), err)]
    pub async fn dhcp4_config(&self, path: OwnedObjectPath) -> Result<Dhcp4Config, NetworkError> {
        Dhcp4Config::get(Dhcp4ConfigParams {
            connection: &self.zbus_connection,
            path,
        })
        .await
    }

    /// DHCP6 Configuration.
    ///
    /// # Errors
    /// Returns `NetworkError::ObjectNotFound` if the configuration doesn't exist.
    /// Returns `NetworkError::DbusError` if DBus operations fail.
    #[instrument(skip(self, path), fields(path = ?path), err)]
    pub async fn dhcp6_config(&self, path: OwnedObjectPath) -> Result<Dhcp6Config, NetworkError> {
        Dhcp6Config::get(Dhcp6ConfigParams {
            connection: &self.zbus_connection,
            path,
        })
        .await
    }
}

impl Drop for NetworkService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
