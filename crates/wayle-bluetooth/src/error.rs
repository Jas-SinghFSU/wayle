use std::fmt;

#[derive(Debug)]
pub(crate) struct ResponderDropped;

impl fmt::Display for ResponderDropped {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pairing responder receiver was dropped")
    }
}

impl std::error::Error for ResponderDropped {}

/// Bluetooth service errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// D-Bus communication error.
    #[error("dbus error: {0}")]
    Dbus(#[from] zbus::Error),

    /// Service initialization failed.
    #[error("cannot initialize bluetooth service")]
    ServiceInitialization(#[source] Box<dyn std::error::Error + Send + Sync>),

    /// Agent registration failed.
    #[error("cannot register bluetooth agent")]
    AgentRegistration(#[source] Box<dyn std::error::Error + Send + Sync>),

    /// Adapter operation failed.
    #[error("cannot {operation} on adapter")]
    AdapterOperation {
        /// The operation that failed.
        operation: &'static str,
        /// The underlying D-Bus error.
        #[source]
        source: zbus::Error,
    },

    /// No primary adapter available for the requested operation.
    #[error("cannot {operation}: no primary adapter available")]
    NoPrimaryAdapter {
        /// The operation that requires an adapter.
        operation: &'static str,
    },

    /// Object discovery failed.
    #[error("cannot discover bluetooth objects")]
    Discovery(#[source] zbus::fdo::Error),

    /// Monitoring requires a cancellation token but none was provided.
    #[error("cannot start monitoring: no cancellation token configured")]
    NoCancellationToken,

    /// Pairing request type mismatch.
    #[error("cannot provide {request_type}: no {request_type} request is pending")]
    NoPendingRequest {
        /// The type of pairing request expected.
        request_type: &'static str,
    },

    /// Pairing responder unavailable.
    #[error("cannot provide {request_type}: no responder available")]
    NoResponder {
        /// The type of responder expected.
        request_type: &'static str,
    },

    /// Pairing response channel send failed.
    #[error("cannot send {request_type} response")]
    ResponderSend {
        /// The type of response being sent.
        request_type: &'static str,
        /// The underlying send error.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}
