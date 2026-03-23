mod controls;
mod monitoring;
mod types;

use std::{collections::HashMap, sync::Arc};

use controls::ConnectionSettingsControls;
use derive_more::Debug;
use futures::{Stream, StreamExt};
use tokio_util::sync::CancellationToken;
pub(crate) use types::{ConnectionSettingsParams, LiveConnectionSettingsParams};
use wayle_core::{Property, unwrap_dbus};
use wayle_traits::{ModelMonitoring, Reactive};
use zbus::{
    Connection,
    zvariant::{self, OwnedObjectPath, OwnedValue},
};

use super::access_point::types::Ssid;
use crate::{
    error::Error,
    proxy::settings::connection::SettingsConnectionProxy,
    types::{connectivity::ConnectionType, flags::NMConnectionSettingsFlags},
};

/// Connection Settings Profile.
///
/// Represents a single network connection configuration.
#[derive(Debug, Clone)]
pub struct ConnectionSettings {
    #[debug(skip)]
    pub(crate) connection: Connection,
    #[debug(skip)]
    pub(crate) cancellation_token: Option<CancellationToken>,
    /// D-Bus object path for this settings connection.
    pub object_path: OwnedObjectPath,

    /// Human-readable connection name (e.g. "Home WiFi", "Wired connection 1").
    pub id: Property<String>,

    /// Stable unique identifier for this connection profile.
    pub uuid: Property<String>,

    /// Network connection type.
    pub connection_type: Property<ConnectionType>,

    /// WiFi SSID, if this is a wireless connection.
    pub wifi_ssid: Property<Option<Ssid>>,

    /// Whether the in-memory state differs from the on-disk state.
    pub unsaved: Property<bool>,

    /// Additional flags of the connection profile.
    pub flags: Property<NMConnectionSettingsFlags>,

    /// File that stores the connection in case the connection is file-backed.
    pub filename: Property<String>,
}

impl Reactive for ConnectionSettings {
    type Context<'a> = ConnectionSettingsParams<'a>;
    type LiveContext<'a> = LiveConnectionSettingsParams<'a>;
    type Error = Error;

    async fn get(params: Self::Context<'_>) -> Result<Self, Self::Error> {
        Self::from_path(params.connection, params.path, None).await
    }

    async fn get_live(params: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let properties = Self::fetch_properties(params.connection, &params.path).await?;
        let settings = Arc::new(Self::from_props(
            params.path.clone(),
            properties,
            params.connection,
            Some(params.cancellation_token.child_token()),
        ));

        settings.clone().start_monitoring().await?;

        Ok(settings)
    }
}

impl PartialEq for ConnectionSettings {
    fn eq(&self, other: &Self) -> bool {
        self.object_path == other.object_path
    }
}

impl ConnectionSettings {
    /// Update the connection with new settings and properties (replacing all
    /// previous settings and properties) and save the connection to disk.
    /// Secrets may be part of the update request, and will be either stored
    /// in persistent storage or sent to a Secret Agent for storage, depending
    /// on the flags associated with each secret.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::OperationFailed` if the update operation fails.
    pub async fn update(
        &self,
        properties: HashMap<String, HashMap<String, OwnedValue>>,
    ) -> Result<(), Error> {
        ConnectionSettingsControls::update(&self.connection, &self.object_path, properties).await
    }

    /// Update the connection without immediately saving to disk.
    ///
    /// Update the connection with new settings and properties (replacing all
    /// previous settings and properties) but do not immediately save the
    /// connection to disk. Secrets may be part of the update request and may
    /// be sent to a Secret Agent for storage, depending on the flags associated
    /// with each secret. Use the 'Save' method to save these changes to disk.
    /// Note that unsaved changes will be lost if the connection is reloaded
    /// from disk (either automatically on file change or due to an explicit
    /// ReloadConnections call).
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::OperationFailed` if the update operation fails.
    pub async fn update_unsaved(
        &self,
        properties: HashMap<String, HashMap<String, OwnedValue>>,
    ) -> Result<(), Error> {
        ConnectionSettingsControls::update_unsaved(&self.connection, &self.object_path, properties)
            .await
    }

    /// Delete the connection.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::OperationFailed` if the delete operation fails.
    pub async fn delete(&self) -> Result<(), Error> {
        ConnectionSettingsControls::delete(&self.connection, &self.object_path).await
    }

    /// Get the settings maps describing this network configuration.
    ///
    /// This will never include any secrets required for connection to the
    /// network, as those are often protected. Secrets must be requested
    /// separately using the GetSecrets() call.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::OperationFailed` if retrieving settings fails.
    pub async fn get_settings(
        &self,
    ) -> Result<HashMap<String, HashMap<String, OwnedValue>>, Error> {
        ConnectionSettingsControls::get_settings(&self.connection, &self.object_path).await
    }

    /// Get the secrets belonging to this network configuration.
    ///
    /// Only secrets from persistent storage or a Secret Agent running in the
    /// requestor's session will be returned. The user will never be prompted
    /// for secrets as a result of this request.
    ///
    /// # Arguments
    ///
    /// * `setting_name` - Name of the setting to return secrets for. If empty,
    ///   all secrets will be returned.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::OperationFailed` if retrieving secrets fails.
    pub async fn get_secrets(
        &self,
        setting_name: &str,
    ) -> Result<HashMap<String, HashMap<String, OwnedValue>>, Error> {
        ConnectionSettingsControls::get_secrets(&self.connection, &self.object_path, setting_name)
            .await
    }

    /// Clear the secrets belonging to this network connection profile.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::OperationFailed` if clearing secrets fails.
    pub async fn clear_secrets(&self) -> Result<(), Error> {
        ConnectionSettingsControls::clear_secrets(&self.connection, &self.object_path).await
    }

    /// Saves a "dirty" connection to persistent storage.
    ///
    /// Saves a connection (that had previously been updated with UpdateUnsaved)
    /// to persistent storage.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::OperationFailed` if saving fails.
    pub async fn save(&self) -> Result<(), Error> {
        ConnectionSettingsControls::save(&self.connection, &self.object_path).await
    }

    /// Update the connection with new settings and properties.
    ///
    /// Update2 is an alternative to Update, UpdateUnsaved and Save extensible
    /// with extra flags and args arguments.
    ///
    /// # Arguments
    ///
    /// * `settings` - New connection settings, properties, and (optionally) secrets.
    ///   Provide an empty HashMap to use the current settings.
    /// * `flags` - Optional flags. Unknown flags cause the call to fail.
    ///   - 0x1 (to-disk): The connection is persisted to disk.
    ///   - 0x2 (in-memory): The change is only made in memory.
    ///   - 0x4 (in-memory-detached): Like "in-memory", but behaves slightly different when migrating.
    ///   - 0x8 (in-memory-only): Like "in-memory", but behaves slightly different when migrating.
    ///   - 0x10 (volatile): Connection is volatile.
    ///   - 0x20 (block-autoconnect): Blocks auto-connect on the updated profile.
    ///   - 0x40 (no-reapply): Prevents "connection.zone" and "connection.metered" from taking effect on active devices.
    /// * `args` - Optional arguments dictionary. Accepts "plugin" and "version-id" keys.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::OperationFailed` if the update operation fails.
    pub async fn update2(
        &self,
        settings: HashMap<String, HashMap<String, OwnedValue>>,
        flags: u32,
        args: HashMap<String, OwnedValue>,
    ) -> Result<HashMap<String, OwnedValue>, Error> {
        ConnectionSettingsControls::update2(
            &self.connection,
            &self.object_path,
            settings,
            flags,
            args,
        )
        .await
    }

    /// Whether this is a wireless connection with the given SSID.
    pub(crate) fn matches_ssid(&self, ssid: &Ssid) -> bool {
        self.wifi_ssid
            .get()
            .as_ref()
            .is_some_and(|stored| stored == ssid)
    }

    async fn from_path(
        connection: &Connection,
        path: OwnedObjectPath,
        cancellation_token: Option<CancellationToken>,
    ) -> Result<Self, Error> {
        let properties = Self::fetch_properties(connection, &path).await?;
        Ok(Self::from_props(
            path,
            properties,
            connection,
            cancellation_token,
        ))
    }

    async fn fetch_properties(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<SettingsConnectionProperties, Error> {
        let proxy = SettingsConnectionProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        let (unsaved, flags, filename, settings) = tokio::join!(
            proxy.unsaved(),
            proxy.flags(),
            proxy.filename(),
            proxy.get_settings()
        );

        let (id, uuid, connection_type, wifi_ssid) = match settings {
            Ok(ref settings_map) => extract_identity(settings_map),
            Err(err) => {
                tracing::debug!("cannot fetch GetSettings for {:?}: {}", path, err);
                (String::new(), String::new(), ConnectionType::None, None)
            }
        };

        Ok(SettingsConnectionProperties {
            unsaved: unwrap_dbus!(unsaved, path),
            flags: unwrap_dbus!(flags, path),
            filename: unwrap_dbus!(filename, path),
            id,
            uuid,
            connection_type,
            wifi_ssid,
        })
    }

    fn from_props(
        path: OwnedObjectPath,
        props: SettingsConnectionProperties,
        connection: &Connection,
        cancellation_token: Option<CancellationToken>,
    ) -> Self {
        Self {
            connection: connection.clone(),
            cancellation_token,
            object_path: path,
            id: Property::new(props.id),
            uuid: Property::new(props.uuid),
            connection_type: Property::new(props.connection_type),
            wifi_ssid: Property::new(props.wifi_ssid),
            unsaved: Property::new(props.unsaved),
            flags: Property::new(NMConnectionSettingsFlags::from_bits_truncate(props.flags)),
            filename: Property::new(props.filename),
        }
    }

    /// Emitted when any property of any settings object within this Connection has changed.
    ///
    /// # Errors
    /// Returns error if D-Bus proxy creation fails.
    pub async fn properties_changed_signal(
        &self,
    ) -> Result<impl Stream<Item = HashMap<String, OwnedValue>>, Error> {
        let proxy = SettingsConnectionProxy::new(&self.connection, &self.object_path).await?;
        let stream = proxy.receive_properties_changed().await?;

        Ok(stream
            .filter_map(|signal| async move { signal.args().ok().map(|args| args.properties) }))
    }

    /// Emitted when the connection is updated.
    ///
    /// # Errors
    /// Returns error if D-Bus proxy creation fails.
    pub async fn updated_signal(&self) -> Result<impl Stream<Item = ()>, Error> {
        let proxy = SettingsConnectionProxy::new(&self.connection, &self.object_path).await?;
        let stream = proxy.receive_updated().await?;

        Ok(stream.filter_map(|_signal| async move { Some(()) }))
    }

    /// Emitted when the connection is removed.
    ///
    /// # Errors
    /// Returns error if D-Bus proxy creation fails.
    pub async fn removed_signal(&self) -> Result<impl Stream<Item = ()>, Error> {
        let proxy = SettingsConnectionProxy::new(&self.connection, &self.object_path).await?;
        let stream = proxy.receive_removed().await?;

        Ok(stream.filter_map(|_signal| async move { Some(()) }))
    }
}

fn extract_identity(
    settings_map: &HashMap<String, HashMap<String, OwnedValue>>,
) -> (String, String, ConnectionType, Option<Ssid>) {
    let connection_group = settings_map.get("connection");

    let id = connection_group
        .and_then(|conn| conn.get("id"))
        .and_then(|val| String::try_from(val.clone()).ok())
        .unwrap_or_default();

    let uuid = connection_group
        .and_then(|conn| conn.get("uuid"))
        .and_then(|val| String::try_from(val.clone()).ok())
        .unwrap_or_default();

    let connection_type = connection_group
        .and_then(|conn| conn.get("type"))
        .and_then(|val| String::try_from(val.clone()).ok())
        .map(|type_str| ConnectionType::from_nm_type(&type_str))
        .unwrap_or(ConnectionType::None);

    let wifi_ssid = settings_map
        .get("802-11-wireless")
        .and_then(|wireless| wireless.get("ssid"))
        .and_then(|val| val.downcast_ref::<zvariant::Array>().ok())
        .and_then(|arr| <Vec<u8>>::try_from(arr).ok())
        .map(Ssid::new);

    (id, uuid, connection_type, wifi_ssid)
}

struct SettingsConnectionProperties {
    unsaved: bool,
    flags: u32,
    filename: String,
    id: String,
    uuid: String,
    connection_type: ConnectionType,
    wifi_ssid: Option<Ssid>,
}
