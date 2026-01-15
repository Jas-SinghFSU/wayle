/// System tray service errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// D-Bus communication error.
    #[error("dbus operation failed")]
    Dbus(#[from] zbus::Error),

    /// Service initialization failed.
    #[error("cannot initialize system tray service: {0}")]
    ServiceInitialization(String),

    /// Cannot register as StatusNotifierWatcher.
    #[error("cannot register as StatusNotifierWatcher: {0}")]
    WatcherRegistration(String),

    /// StatusNotifierItem not found.
    #[error("cannot find StatusNotifierItem: {service}")]
    ItemNotFound {
        /// D-Bus service name of the missing item.
        service: String,
    },

    /// Cannot connect to StatusNotifierItem.
    #[error("cannot connect to tray item {service}")]
    ItemConnection {
        /// D-Bus service name of the item.
        service: String,
        /// Underlying D-Bus error.
        #[source]
        source: zbus::Error,
    },

    /// Menu operation failed.
    #[error("cannot perform menu operation for item {service}")]
    MenuOperation {
        /// D-Bus service name of the item.
        service: String,
        /// Underlying D-Bus error.
        #[source]
        source: zbus::Error,
    },

    /// Icon data parsing failed.
    #[error("cannot parse icon data for {service}: {details}")]
    IconParsing {
        /// D-Bus service name of the item.
        service: String,
        /// Description of the parsing failure.
        details: String,
    },

    /// Property conversion failed.
    #[error("cannot convert property {property} for {service}: expected {expected}")]
    PropertyConversion {
        /// D-Bus service name.
        service: String,
        /// Property name that failed to convert.
        property: String,
        /// Expected type.
        expected: String,
    },

    /// System tray operation failed.
    #[error("cannot perform tray operation '{operation}'")]
    Operation {
        /// The operation that failed.
        operation: &'static str,
        /// Underlying D-Bus error.
        #[source]
        source: zbus::Error,
    },

    /// Tray item does not support the requested activation method.
    #[error("tray item does not support '{operation}', use its menu instead")]
    OperationNotSupported {
        /// The operation that is not supported.
        operation: &'static str,
    },

    /// Invalid service name format.
    #[error("invalid bus name format: {0}")]
    InvalidBusName(String),

    /// ZVariant conversion error.
    #[error("zvariant conversion failed")]
    ZVariant(#[from] zbus::zvariant::Error),
}
