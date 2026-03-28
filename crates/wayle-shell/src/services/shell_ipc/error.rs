/// Errors from shell IPC service initialization.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot connect to session bus: {0}")]
    Connection(String),

    #[error("cannot register D-Bus object: {0}")]
    Registration(String),

    #[error("cannot request D-Bus name: {0}")]
    NameRequest(String),
}
