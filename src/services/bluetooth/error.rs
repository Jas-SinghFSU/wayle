use zbus::zvariant::OwnedObjectPath;

/// Bluetooth service errors
#[derive(thiserror::Error, Debug)]
pub enum BluetoothError {
    /// D-Bus communication error
    #[error("D-Bus operation failed: {0}")]
    DbusError(#[from] zbus::Error),

    /// Service initialization failed
    #[error("Failed to initialize Bluetooth service: {0}")]
    ServiceInitializationFailed(String),

    /// Adapter not found at the specified D-Bus path
    #[error("Adapter not found at path: {0}")]
    AdapterNotFound(OwnedObjectPath),

    /// Device not found by address
    #[error("Device {0} not found")]
    DeviceNotFound(String),

    /// No Bluetooth adapters available
    #[error("No Bluetooth adapters available")]
    NoAdaptersAvailable,

    /// No powered adapters available
    #[error("No powered Bluetooth adapters available")]
    NoPoweredAdapters,

    /// Pairing operation failed
    #[error("Pairing failed: {0}")]
    PairingFailed(String),

    /// Connection operation failed
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// Discovery operation failed
    #[error("Discovery operation failed: {0}")]
    DiscoveryFailed(String),

    /// Agent registration failed
    #[error("Failed to register agent: {0}")]
    AgentRegistrationFailed(String),

    /// Wrong pairing request type for response
    #[error("Wrong pairing request type: expected {expected}, got {actual}")]
    WrongPairingRequestType {
        /// Expected request type
        expected: &'static str,
        /// Actual request type
        actual: &'static str,
    },

    /// No pending pairing request
    #[error("No pending pairing request")]
    NoPendingPairingRequest,

    /// Pairing responder channel closed
    #[error("Pairing responder channel closed")]
    PairingResponderClosed,

    /// Operation not supported by device
    #[error("Operation {operation} not supported by device")]
    OperationNotSupported {
        /// The unsupported operation
        operation: &'static str,
    },

    /// Bluetooth operation failed
    #[error("Bluetooth operation failed: {operation} - {reason}")]
    OperationFailed {
        /// The operation that failed
        operation: &'static str,
        /// The reason the operation failed
        reason: String,
    },
}

