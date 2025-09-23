use zbus::{Result, proxy};

/// Proxy for the org.kde.StatusNotifierWatcher interface.
///
/// Service: `org.kde.StatusNotifierWatcher`
/// Interface: `org.kde.StatusNotifierWatcher`
/// Object path: `/StatusNotifierWatcher`
#[proxy(
    interface = "org.kde.StatusNotifierWatcher",
    default_service = "org.kde.StatusNotifierWatcher",
    default_path = "/StatusNotifierWatcher"
)]
pub(crate) trait StatusNotifierWatcher {
    /// Register a StatusNotifierItem into the StatusNotifierWatcher, in the form of its full
    /// name on the session bus, for instance org.freedesktop.StatusNotifierItem-4077-1.
    ///
    /// A StatusNotifierItem instance must be registered to the watcher in order to be noticed
    /// from both the watcher and the StatusNotifierHost instances.
    ///
    /// If the string is a bus name, the StatusNotifierItem interface will be searched on the
    /// object /StatusNotifierItem, otherwise it will be assumed to be an object path.
    fn register_status_notifier_item(&self, service: &str) -> Result<()>;

    /// Register a StatusNotifierHost into the StatusNotifierWatcher, in the form of its full
    /// name on the session bus, for instance org.freedesktop.StatusNotifierHost-4005.
    ///
    /// Every NotficationHost instance that intends to display StatusNotifierItem representations
    /// should register to StatusNotifierWatcher with this method.
    ///
    /// This method may also be used by items to check whether a StatusNotifierHost is present.
    fn register_status_notifier_host(&self, service: &str) -> Result<()>;

    /// List containing all the registered instances of StatusNotifierItem.
    ///
    /// The items are represented by their full names on the session bus, for instance
    /// org.freedesktop.StatusNotifierItem-4077-1.
    #[zbus(property)]
    fn registered_status_notifier_items(&self) -> Result<Vec<String>>;

    /// True if at least one StatusNotifierHost has been registered.
    ///
    /// Items may use this property to check whether they should register or not.
    #[zbus(property)]
    fn is_status_notifier_host_registered(&self) -> Result<bool>;

    /// The version of the protocol the StatusNotifierWatcher instance implements.
    #[zbus(property)]
    fn protocol_version(&self) -> Result<i32>;

    /// A new StatusNotifierItem has been registered, the argument of the signal is the session
    /// bus name of the instance.
    ///
    /// StatusNotifierHost instances should react to this signal by refreshing their
    /// representation of the item list.
    #[zbus(signal)]
    fn status_notifier_item_registered(&self, service: String) -> Result<()>;

    /// A StatusNotifierItem instance has disappeared from the bus, the argument of the signal is
    /// the session bus name of the instance.
    ///
    /// StatusNotifierHost instances should react to this signal by refreshing their
    /// representation of the item list.
    #[zbus(signal)]
    fn status_notifier_item_unregistered(&self, service: String) -> Result<()>;

    /// A new StatusNotifierHost has been registered.
    ///
    /// StatusNotifierItem instances that previously did not register if no hosts were available
    /// may now reconsider to register.
    #[zbus(signal)]
    fn status_notifier_host_registered(&self) -> Result<()>;

    /// There are no more StatusNotifierHost instances running.
    ///
    /// StatusNotifierItem instances may choose to skip registration if there are no hosts
    /// available.
    #[zbus(signal)]
    fn status_notifier_host_unregistered(&self) -> Result<()>;
}
