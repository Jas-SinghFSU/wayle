//! D-Bus client proxy for system tray.
#![allow(missing_docs)]

use zbus::{Result, proxy};

/// D-Bus client proxy for querying system tray items.
#[proxy(
    interface = "com.wayle.SystemTray1",
    default_service = "com.wayle.SystemTray1",
    default_path = "/com/wayle/SystemTray",
    gen_blocking = false
)]
pub trait SystemTrayWayle {
    /// Lists all current system tray items.
    ///
    /// Returns array of (id, title, icon_name, status).
    async fn list(&self) -> Result<Vec<(String, String, String, String)>>;

    /// Activates a tray item by ID (simulates left-click).
    async fn activate(&self, id: String) -> Result<()>;

    /// Number of current tray items.
    #[zbus(property)]
    fn count(&self) -> Result<u32>;

    /// Whether this service is operating as the StatusNotifierWatcher.
    #[zbus(property)]
    fn is_watcher(&self) -> Result<bool>;
}
