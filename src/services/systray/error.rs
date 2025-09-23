/// System tray service errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// D-Bus communication error
    #[error("D-Bus operation failed: {0:#?}")]
    DbusError(#[from] zbus::Error),

    /// Service initialization failed
    #[error("Failed to initialize system tray service: {0:#?}")]
    ServiceInitializationFailed(String),

    /// Failed to register as StatusNotifierWatcher
    #[error("Failed to register as StatusNotifierWatcher: {0:#?}")]
    WatcherRegistrationFailed(String),

    /// StatusNotifierItem not found
    #[error("StatusNotifierItem not found: {service}")]
    ItemNotFound {
        /// D-Bus service name of the missing item
        service: String,
    },

    /// Failed to connect to StatusNotifierItem
    #[error("Failed to connect to item {service}: {reason}")]
    ItemConnectionFailed {
        /// D-Bus service name of the item
        service: String,
        /// Reason for connection failure
        reason: String,
    },

    /// Menu operation failed
    #[error("Menu operation failed for item {service}: {reason}")]
    MenuOperationFailed {
        /// D-Bus service name of the item
        service: String,
        /// Reason for menu operation failure
        reason: String,
    },

    /// Icon data parsing failed
    #[error("Failed to parse icon data for {service}: {reason}")]
    IconParsingFailed {
        /// D-Bus service name of the item
        service: String,
        /// Reason for parsing failure
        reason: String,
    },

    /// Property conversion failed
    #[error("Failed to convert property {property} for {service}: expected {expected}")]
    PropertyConversionFailed {
        /// D-Bus service name
        service: String,
        /// Property name that failed to convert
        property: String,
        /// Expected type
        expected: String,
    },

    /// System tray operation failed
    #[error("System tray operation failed: {operation} - {reason}")]
    OperationFailed {
        /// The operation that failed
        operation: &'static str,
        /// The reason the operation failed
        reason: String,
    },

    /// Invalid service name format
    #[error("Invalid bus name format: {0}")]
    InvalidBusName(String),

    /// ZVariant conversion error
    #[error("ZVariant error: {0}")]
    ZVariantError(#[from] zbus::zvariant::Error),
}
