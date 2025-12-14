use tokio::sync::mpsc;

/// Applies TOML values to the config layer of ConfigProperty fields.
///
/// Used when loading or hot-reloading config.toml. The config layer sits
/// between defaults and runtime overrides in precedence.
pub trait ApplyConfigLayer {
    /// Apply TOML values to the config layer.
    ///
    /// Missing fields are skipped. Deserialization failures are logged
    /// and skipped, allowing partial updates to succeed.
    fn apply_config_layer(&self, value: &toml::Value);
}

/// Applies TOML values to the runtime layer of ConfigProperty fields.
///
/// Used when loading runtime.toml (GUI overrides). The runtime layer
/// has highest precedence, overriding both config and default values.
pub trait ApplyRuntimeLayer {
    /// Apply TOML values to the runtime layer.
    ///
    /// Missing fields are skipped. Deserialization failures are logged
    /// and skipped, allowing partial updates to succeed.
    fn apply_runtime_layer(&self, value: &toml::Value);
}

/// Extracts runtime layer values for persistence to runtime.toml.
///
/// Walks the config tree and collects only values that have been set
/// in the runtime layer (GUI overrides). Returns None if no runtime
/// value exists, allowing sparse serialization.
pub trait ExtractRuntimeValues {
    /// Extract runtime values as TOML.
    ///
    /// Returns Some(Value) if this field or any nested field has a runtime
    /// override, None otherwise. For structs, returns a Table containing
    /// only fields with runtime values.
    fn extract_runtime_values(&self) -> Option<toml::Value>;
}

/// Trait for subscribing to changes in config structures.
///
/// Enables automatic persistence by watching all fields for changes.
pub trait SubscribeChanges {
    /// Subscribe to changes by sending notifications to the provided channel.
    ///
    /// Spawns background tasks that watch for changes and send () to the channel.
    fn subscribe_changes(&self, tx: mpsc::UnboundedSender<()>);
}
