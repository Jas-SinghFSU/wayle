use std::collections::HashMap;

use tracing::instrument;
use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue, Value},
};

use crate::services::network::{
    core::access_point::types::Ssid,
    error::Error,
    proxy::{access_point::AccessPointProxy, devices::DeviceProxy, manager::NetworkManagerProxy},
};

type ConnectionSettings = HashMap<String, HashMap<String, OwnedValue>>;

// List of manufacturer default SSIDs that should be locked to BSSID
// to prevent connecting to neighbors' routers with the same default name.
// Inspired by nm-applet's approach to handling duplicate SSIDs.
const MANUFACTURER_DEFAULT_SSIDS: &[&str] = &[
    "linksys",
    "linksys-a",
    "linksys-g",
    "default",
    "belkin54g",
    "NETGEAR",
    "o2DSL",
    "WLAN",
    "ALICE-WLAN",
];

pub(super) struct WifiControls;

impl WifiControls {
    #[instrument(skip(connection), fields(enabled = enabled), err)]
    pub(super) async fn set_enabled(connection: &Connection, enabled: bool) -> Result<(), Error> {
        let proxy = NetworkManagerProxy::new(connection).await?;

        proxy
            .set_wireless_enabled(enabled)
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "set_wireless_enabled",
                reason: e.to_string(),
            })?;

        Ok(())
    }

    #[instrument(skip(connection), fields(device = %device_path), err)]
    pub(super) async fn disconnect(
        connection: &Connection,
        device_path: &str,
    ) -> Result<(), Error> {
        let proxy = NetworkManagerProxy::new(connection).await?;

        let device_proxy = DeviceProxy::new(connection, device_path)
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "device_proxy",
                reason: e.to_string(),
            })?;

        let active_connection_path =
            device_proxy
                .active_connection()
                .await
                .map_err(|e| Error::OperationFailed {
                    operation: "active_connection",
                    reason: e.to_string(),
                })?;

        if active_connection_path.as_str() == "/" || active_connection_path.as_str().is_empty() {
            return Ok(());
        }

        proxy
            .deactivate_connection(&active_connection_path)
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "deactivate_connection",
                reason: e.to_string(),
            })?;

        Ok(())
    }

    #[instrument(
        skip(connection, password),
        fields(device = %device_path, ap = %ap_path),
        err
    )]
    pub(super) async fn connect(
        connection: &Connection,
        device_path: &str,
        ap_path: OwnedObjectPath,
        password: Option<String>,
    ) -> Result<(), Error> {
        let proxy = NetworkManagerProxy::new(connection).await?;

        let ap_proxy = AccessPointProxy::new(connection, ap_path.clone())
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "create_ap_proxy",
                reason: e.to_string(),
            })?;

        let ssid_bytes = ap_proxy.ssid().await.map_err(|e| Error::OperationFailed {
            operation: "get_ssid",
            reason: e.to_string(),
        })?;

        let ssid_string = Ssid::new(ssid_bytes.clone()).as_str();

        let bssid = if Self::is_manufacturer_default(&ssid_string) {
            ap_proxy.hw_address().await.ok()
        } else {
            None
        };

        let connection_settings =
            Self::build_connection_settings(ssid_string, ssid_bytes, bssid, password)?;

        let device_path =
            OwnedObjectPath::try_from(device_path).map_err(|e| Error::DbusError(e.into()))?;

        proxy
            .add_and_activate_connection(connection_settings, &device_path, &ap_path)
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "add_and_activate_connection",
                reason: e.to_string(),
            })?;

        Ok(())
    }

    fn is_manufacturer_default(ssid: &str) -> bool {
        MANUFACTURER_DEFAULT_SSIDS.contains(&ssid)
    }

    fn build_connection_settings(
        ssid_string: String,
        ssid_bytes: Vec<u8>,
        bssid: Option<String>,
        password: Option<String>,
    ) -> Result<ConnectionSettings, Error> {
        let to_owned = |value: Value| {
            value.try_to_owned().map_err(|e| Error::OperationFailed {
                operation: "to_owned",
                reason: e.to_string(),
            })
        };

        let mut settings = HashMap::new();

        let mut connection = HashMap::new();
        connection.insert(
            String::from("type"),
            to_owned(Value::from("802-11-wireless"))?,
        );
        connection.insert(String::from("id"), to_owned(Value::from(ssid_string))?);
        settings.insert(String::from("connection"), connection);

        let mut wireless = HashMap::new();
        wireless.insert(String::from("ssid"), to_owned(Value::from(ssid_bytes))?);

        if let Some(bssid_str) = bssid {
            let mac_bytes: Result<Vec<u8>, _> = bssid_str
                .split(':')
                .map(|part| u8::from_str_radix(part, 16))
                .collect();

            if let Ok(bytes) = mac_bytes {
                wireless.insert(String::from("bssid"), to_owned(Value::from(bytes))?);
            }
        }

        settings.insert(String::from("802-11-wireless"), wireless);

        if let Some(pwd) = password {
            let mut security = HashMap::new();
            security.insert(String::from("key-mgmt"), to_owned(Value::from("wpa-psk"))?);
            security.insert(String::from("psk"), to_owned(Value::from(pwd))?);
            settings.insert(String::from("802-11-wireless-security"), security);
        }

        Ok(settings)
    }
}
