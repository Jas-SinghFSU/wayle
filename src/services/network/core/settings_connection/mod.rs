mod controls;
mod monitoring;
mod types;

use std::{collections::HashMap, sync::Arc};

use controls::ConnectionSettingsControls;
use tokio_util::sync::CancellationToken;
pub(crate) use types::{ConnectionSettingsParams, LiveConnectionSettingsParams};
use zbus::{
    Connection,
    zvariant::{self, OwnedObjectPath, OwnedValue},
};

use crate::{
    services::{
        common::Property,
        network::{
            NMConnectionSettingsFlags, NetworkError, core::access_point::SSID,
            proxy::settings::connection::SettingsConnectionProxy,
        },
        traits::{ModelMonitoring, Reactive},
    },
    unwrap_bool, unwrap_string, unwrap_u32,
};

/// Connection Settings Profile.
///
/// Represents a single network connection configuration.
#[derive(Debug, Clone)]
pub struct ConnectionSettings {
    pub(crate) connection: Connection,
    pub(crate) cancellation_token: Option<CancellationToken>,
    /// D-Bus object path for this settings connection
    pub object_path: OwnedObjectPath,

    /// If set, indicates that the in-memory state of the connection does not
    /// match the on-disk state. This flag will be set when UpdateUnsaved() is
    /// called or when any connection details change, and cleared when the
    /// connection is saved to disk via Save() or from internal operations.
    pub unsaved: Property<bool>,

    /// Additional flags of the connection profile.
    pub flags: Property<NMConnectionSettingsFlags>,

    /// File that stores the connection in case the connection is file-backed.
    pub filename: Property<String>,
}

impl Reactive for ConnectionSettings {
    type Context<'a> = ConnectionSettingsParams<'a>;
    type LiveContext<'a> = LiveConnectionSettingsParams<'a>;
    type Error = NetworkError;

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
    /// Get a snapshot of the current settings connection state.
    ///
    /// Update the connection with new settings and properties.
    ///
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
    ) -> Result<(), NetworkError> {
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
    ) -> Result<(), NetworkError> {
        ConnectionSettingsControls::update_unsaved(&self.connection, &self.object_path, properties)
            .await
    }

    /// Delete the connection.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::OperationFailed` if the delete operation fails.
    pub async fn delete(&self) -> Result<(), NetworkError> {
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
    ) -> Result<HashMap<String, HashMap<String, OwnedValue>>, NetworkError> {
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
    ) -> Result<HashMap<String, HashMap<String, OwnedValue>>, NetworkError> {
        ConnectionSettingsControls::get_secrets(&self.connection, &self.object_path, setting_name)
            .await
    }

    /// Clear the secrets belonging to this network connection profile.
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::OperationFailed` if clearing secrets fails.
    pub async fn clear_secrets(&self) -> Result<(), NetworkError> {
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
    pub async fn save(&self) -> Result<(), NetworkError> {
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
    ) -> Result<HashMap<String, OwnedValue>, NetworkError> {
        ConnectionSettingsControls::update2(
            &self.connection,
            &self.object_path,
            settings,
            flags,
            args,
        )
        .await
    }

    pub(crate) async fn matches_ssid(&self, ssid: &SSID) -> bool {
        let Ok(settings) = self.get_settings().await else {
            return false;
        };

        settings
            .get("802-11-wireless")
            .and_then(|wireless| wireless.get("ssid"))
            .and_then(|ssid| ssid.downcast_ref::<zvariant::Array>().ok())
            .and_then(|arr| TryInto::<Vec<u8>>::try_into(arr).ok())
            .is_some_and(|bytes| bytes == ssid.as_bytes())
    }

    async fn from_path(
        connection: &Connection,
        path: OwnedObjectPath,
        cancellation_token: Option<CancellationToken>,
    ) -> Result<Self, NetworkError> {
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
    ) -> Result<SettingsConnectionProperties, NetworkError> {
        let proxy = SettingsConnectionProxy::new(connection, path)
            .await
            .map_err(NetworkError::DbusError)?;

        let (unsaved, flags, filename) =
            tokio::join!(proxy.unsaved(), proxy.flags(), proxy.filename());

        Ok(SettingsConnectionProperties {
            unsaved: unwrap_bool!(unsaved, path),
            flags: unwrap_u32!(flags, path),
            filename: unwrap_string!(filename, path),
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
            unsaved: Property::new(props.unsaved),
            flags: Property::new(NMConnectionSettingsFlags::from_bits_truncate(props.flags)),
            filename: Property::new(props.filename),
        }
    }
}

struct SettingsConnectionProperties {
    unsaved: bool,
    flags: u32,
    filename: String,
}
