use std::collections::HashMap;

use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::services::network::{NetworkError, proxy::devices::wireless::DeviceWirelessProxy};

pub(super) struct DeviceWifiControls;

impl DeviceWifiControls {
    pub(super) async fn get_all_access_points(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<Vec<OwnedObjectPath>, NetworkError> {
        let proxy = DeviceWirelessProxy::new(connection, path)
            .await
            .map_err(NetworkError::DbusError)?;

        proxy
            .get_all_access_points()
            .await
            .map_err(|e| NetworkError::OperationFailed {
                operation: "get_all_access_points",
                reason: e.to_string(),
            })
    }

    pub(super) async fn request_scan(
        connection: &Connection,
        path: &OwnedObjectPath,
        options: HashMap<String, OwnedValue>,
    ) -> Result<(), NetworkError> {
        let proxy = DeviceWirelessProxy::new(connection, path)
            .await
            .map_err(NetworkError::DbusError)?;

        proxy
            .request_scan(options)
            .await
            .map_err(|e| NetworkError::OperationFailed {
                operation: "request_scan",
                reason: e.to_string(),
            })?;

        Ok(())
    }
}

