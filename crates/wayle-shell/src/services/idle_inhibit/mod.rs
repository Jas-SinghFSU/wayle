//! Idle inhibit service with D-Bus interface.

mod dbus;
mod state;

use dbus::{IdleInhibitDaemon, SERVICE_NAME, SERVICE_PATH};
pub use state::IdleInhibitState;
use tracing::info;
use zbus::Connection;

/// Idle inhibit service providing D-Bus control interface.
///
/// Owns the shared state that modules watch for changes. The D-Bus interface
/// at `com.wayle.IdleInhibit1` allows external control of idle inhibition.
pub struct IdleInhibitService {
    state: IdleInhibitState,
    _connection: Connection,
}

impl IdleInhibitService {
    /// Creates the service and registers D-Bus interface.
    ///
    /// # Arguments
    ///
    /// * `startup_duration_mins` - Initial duration in minutes (0 = indefinite)
    pub async fn new(startup_duration_mins: u32) -> Result<Self, Error> {
        let state = IdleInhibitState::new(startup_duration_mins);

        let connection = Connection::session()
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;

        let daemon = IdleInhibitDaemon::new(state.clone());

        connection
            .object_server()
            .at(SERVICE_PATH, daemon)
            .await
            .map_err(|e| Error::Registration(e.to_string()))?;

        connection
            .request_name(SERVICE_NAME)
            .await
            .map_err(|e| Error::NameRequest(e.to_string()))?;

        info!("IdleInhibit service registered at {SERVICE_NAME}");

        Ok(Self {
            state,
            _connection: connection,
        })
    }

    /// Returns a clone of the shared state for modules to watch.
    pub fn state(&self) -> IdleInhibitState {
        self.state.clone()
    }
}

/// Errors from idle inhibit service initialization.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot connect to session bus")]
    Connection(String),

    #[error("cannot register D-Bus object")]
    Registration(String),

    #[error("cannot request D-Bus name")]
    NameRequest(String),
}
