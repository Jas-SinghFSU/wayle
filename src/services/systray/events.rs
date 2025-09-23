/// Internal events for the system tray service.
#[derive(Debug, Clone)]
pub(crate) enum TrayEvent {
    /// A new StatusNotifierItem has been registered.
    ItemRegistered(String),
    /// A StatusNotifierItem has been unregistered.
    ItemUnregistered(String),
    /// A service has disconnected from D-Bus.
    ServiceDisconnected(String),
}
