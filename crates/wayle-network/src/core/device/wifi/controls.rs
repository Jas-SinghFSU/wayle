use std::collections::HashMap;

use tracing::instrument;
use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::{error::Error, proxy::devices::wireless::DeviceWirelessProxy};

pub(super) struct DeviceWifiControls;

impl DeviceWifiControls {
    #[instrument(skip(connection), fields(device = %path), err)]
    pub(super) async fn get_all_access_points(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<Vec<OwnedObjectPath>, Error> {
        let proxy = DeviceWirelessProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        proxy
            .get_all_access_points()
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "get all access points",
                source: e.into(),
            })
    }

    #[instrument(
        skip(connection, options),
        fields(device = %path),
        err
    )]
    pub(super) async fn request_scan(
        connection: &Connection,
        path: &OwnedObjectPath,
        options: HashMap<String, OwnedValue>,
    ) -> Result<(), Error> {
        let proxy = DeviceWirelessProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        proxy
            .request_scan(options)
            .await
            .map_err(|e| Error::OperationFailed {
                operation: "request wifi scan",
                source: e.into(),
            })?;

        Ok(())
    }
}
