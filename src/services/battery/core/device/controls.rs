use tracing::instrument;
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::services::battery::{error::Error, proxy::device::DeviceProxy};

pub(super) struct DeviceController;

impl DeviceController {
    #[instrument(skip(connection), err)]
    pub(super) async fn refresh(
        connection: &Connection,
        device_path: &OwnedObjectPath,
    ) -> Result<(), Error> {
        let proxy = DeviceProxy::builder(connection)
            .path(device_path)?
            .build()
            .await?;

        proxy.refresh().await?;
        Ok(())
    }

    #[instrument(skip(connection), err)]
    pub(super) async fn get_history(
        connection: &Connection,
        device_path: &OwnedObjectPath,
        history_type: &str,
        timespan: u32,
        resolution: u32,
    ) -> Result<Vec<(u32, f64, u32)>, Error> {
        let proxy = DeviceProxy::builder(connection)
            .path(device_path)?
            .build()
            .await?;

        Ok(proxy
            .get_history(history_type, timespan, resolution)
            .await?)
    }

    #[instrument(skip(connection), err)]
    pub(super) async fn get_statistics(
        connection: &Connection,
        device_path: &OwnedObjectPath,
        stat_type: &str,
    ) -> Result<Vec<(f64, f64)>, Error> {
        let proxy = DeviceProxy::builder(connection)
            .path(device_path)?
            .build()
            .await?;

        Ok(proxy.get_statistics(stat_type).await?)
    }

    #[instrument(skip(connection), err)]
    pub(super) async fn enable_charge_threshold(
        connection: &Connection,
        device_path: &OwnedObjectPath,
        enabled: bool,
    ) -> Result<(), Error> {
        let proxy = DeviceProxy::builder(connection)
            .path(device_path)?
            .build()
            .await?;

        proxy.enable_charge_threshold(enabled).await?;
        Ok(())
    }
}
