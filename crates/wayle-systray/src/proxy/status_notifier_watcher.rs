use zbus::{Result, proxy};

#[proxy(
    interface = "org.kde.StatusNotifierWatcher",
    default_service = "org.kde.StatusNotifierWatcher",
    default_path = "/StatusNotifierWatcher"
)]
pub(crate) trait StatusNotifierWatcher {
    fn register_status_notifier_item(&self, service: &str) -> Result<()>;

    fn register_status_notifier_host(&self, service: &str) -> Result<()>;

    #[zbus(property)]
    fn registered_status_notifier_items(&self) -> Result<Vec<String>>;

    #[zbus(property)]
    fn is_status_notifier_host_registered(&self) -> Result<bool>;

    #[zbus(property)]
    fn protocol_version(&self) -> Result<i32>;

    #[zbus(signal)]
    fn status_notifier_item_registered(&self, service: String) -> Result<()>;

    #[zbus(signal)]
    fn status_notifier_item_unregistered(&self, service: String) -> Result<()>;

    #[zbus(signal)]
    fn status_notifier_host_registered(&self) -> Result<()>;

    #[zbus(signal)]
    fn status_notifier_host_unregistered(&self) -> Result<()>;
}
