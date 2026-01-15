/// Battery service errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// D-Bus communication error.
    #[error("dbus operation failed")]
    Dbus(
        #[from]
        #[source]
        zbus::Error,
    ),

    /// Cannot parse D-Bus object path.
    #[error("cannot parse D-Bus object path")]
    InvalidObjectPath(
        #[from]
        #[source]
        zbus::zvariant::Error,
    ),

    /// Monitoring requires a cancellation token.
    #[error("cannot start monitoring without a cancellation token")]
    MissingCancellationToken,
}
