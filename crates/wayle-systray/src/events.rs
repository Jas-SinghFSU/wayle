#[derive(Debug, Clone)]
pub(crate) enum TrayEvent {
    ItemRegistered(String),
    ItemUnregistered(String),
    ServiceDisconnected(String),
}
