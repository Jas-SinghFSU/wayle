use tracing::instrument;
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::{
    error::Error,
    proxy::{adapter::Adapter1Proxy, device::Device1Proxy},
    types::UUID,
};

pub(super) struct DeviceControls;

impl DeviceControls {
    #[instrument(skip(connection), fields(device = %device_path), err)]
    pub(super) async fn connect(
        connection: &Connection,
        device_path: &OwnedObjectPath,
    ) -> Result<(), Error> {
        let proxy = Device1Proxy::new(connection, device_path).await?;
        Ok(proxy.connect().await?)
    }

    #[instrument(skip(connection), fields(device = %device_path), err)]
    pub(super) async fn disconnect(
        connection: &Connection,
        device_path: &OwnedObjectPath,
    ) -> Result<(), Error> {
        let proxy = Device1Proxy::new(connection, device_path).await?;
        Ok(proxy.disconnect().await?)
    }

    #[instrument(skip(connection), fields(device = %device_path, profile = %profile_uuid), err)]
    pub(super) async fn connect_profile(
        connection: &Connection,
        device_path: &OwnedObjectPath,
        profile_uuid: UUID,
    ) -> Result<(), Error> {
        let proxy = Device1Proxy::new(connection, device_path).await?;
        Ok(proxy.connect_profile(&profile_uuid).await?)
    }

    #[instrument(skip(connection), fields(device = %device_path, profile = %profile_uuid), err)]
    pub(super) async fn disconnect_profile(
        connection: &Connection,
        device_path: &OwnedObjectPath,
        profile_uuid: UUID,
    ) -> Result<(), Error> {
        let proxy = Device1Proxy::new(connection, device_path).await?;
        Ok(proxy.disconnect_profile(&profile_uuid).await?)
    }

    #[instrument(skip(connection), fields(device = %device_path), err)]
    pub(super) async fn pair(
        connection: &Connection,
        device_path: &OwnedObjectPath,
    ) -> Result<(), Error> {
        let proxy = Device1Proxy::new(connection, device_path).await?;
        Ok(proxy.pair().await?)
    }

    #[instrument(skip(connection), fields(device = %device_path), err)]
    pub(super) async fn cancel_pairing(
        connection: &Connection,
        device_path: &OwnedObjectPath,
    ) -> Result<(), Error> {
        let proxy = Device1Proxy::new(connection, device_path).await?;
        Ok(proxy.cancel_pairing().await?)
    }

    #[instrument(skip(connection), fields(adapter = %adapter_path, device = %device_path), err)]
    pub(super) async fn forget(
        connection: &Connection,
        adapter_path: &OwnedObjectPath,
        device_path: &OwnedObjectPath,
    ) -> Result<(), Error> {
        let proxy = Adapter1Proxy::new(connection, adapter_path).await?;
        Ok(proxy.remove_device(device_path).await?)
    }

    #[instrument(skip(connection), fields(device = %device_path), err)]
    pub(super) async fn get_service_records(
        connection: &Connection,
        device_path: &OwnedObjectPath,
    ) -> Result<Vec<Vec<u8>>, Error> {
        let proxy = Device1Proxy::new(connection, device_path).await?;
        Ok(proxy.get_service_records().await?)
    }

    #[instrument(
        skip(connection),
        fields(device = %device_path, trusted = trusted),
        err
    )]
    pub(super) async fn set_trusted(
        connection: &Connection,
        device_path: &OwnedObjectPath,
        trusted: bool,
    ) -> Result<(), Error> {
        let proxy = Device1Proxy::new(connection, device_path).await?;
        Ok(proxy.set_trusted(trusted).await?)
    }

    #[instrument(
        skip(connection),
        fields(device = %device_path, blocked = blocked),
        err
    )]
    pub(super) async fn set_blocked(
        connection: &Connection,
        device_path: &OwnedObjectPath,
        blocked: bool,
    ) -> Result<(), Error> {
        let proxy = Device1Proxy::new(connection, device_path).await?;
        Ok(proxy.set_blocked(blocked).await?)
    }

    #[instrument(
        skip(connection),
        fields(device = %device_path, wake_allowed = wake_allowed),
        err
    )]
    pub(super) async fn set_wake_allowed(
        connection: &Connection,
        device_path: &OwnedObjectPath,
        wake_allowed: bool,
    ) -> Result<(), Error> {
        let proxy = Device1Proxy::new(connection, device_path).await?;
        Ok(proxy.set_wake_allowed(wake_allowed).await?)
    }

    #[instrument(
        skip(connection),
        fields(device = %device_path, alias = %alias),
        err
    )]
    pub(super) async fn set_alias(
        connection: &Connection,
        device_path: &OwnedObjectPath,
        alias: &str,
    ) -> Result<(), Error> {
        let proxy = Device1Proxy::new(connection, device_path).await?;
        Ok(proxy.set_alias(alias).await?)
    }

    #[instrument(
        skip(connection),
        fields(device = %device_path, bearer = %bearer),
        err
    )]
    pub(super) async fn set_preferred_bearer(
        connection: &Connection,
        device_path: &OwnedObjectPath,
        bearer: &str,
    ) -> Result<(), Error> {
        let proxy = Device1Proxy::new(connection, device_path).await?;
        Ok(proxy.set_preferred_bearer(bearer).await?)
    }
}
