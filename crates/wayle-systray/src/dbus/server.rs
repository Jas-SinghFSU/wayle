use std::sync::Arc;

use tracing::instrument;
use zbus::{fdo, interface};

use crate::{service::SystemTrayService, types::Coordinates};

#[derive(Debug)]
pub(crate) struct SystemTrayDaemon {
    pub service: Arc<SystemTrayService>,
}

#[interface(name = "com.wayle.SystemTray1")]
impl SystemTrayDaemon {
    #[instrument(skip(self))]
    pub async fn list(&self) -> Vec<(String, String, String, String)> {
        self.service
            .items
            .get()
            .iter()
            .map(|item| {
                (
                    item.id.get(),
                    item.title.get(),
                    item.icon_name.get().unwrap_or_default(),
                    item.status.get().to_string(),
                )
            })
            .collect()
    }

    #[instrument(skip(self), fields(id = %id))]
    pub async fn activate(&self, id: String) -> fdo::Result<()> {
        let items = self.service.items.get();
        let item = items
            .iter()
            .find(|item| item.id.get() == id)
            .ok_or_else(|| fdo::Error::InvalidArgs(format!("Tray item not found: {id}")))?;

        item.activate(Coordinates::new(0, 0))
            .await
            .map_err(|e| fdo::Error::Failed(e.to_string()))
    }

    #[zbus(property)]
    pub async fn count(&self) -> u32 {
        self.service.items.get().len() as u32
    }

    #[zbus(property)]
    pub async fn is_watcher(&self) -> bool {
        self.service.is_watcher
    }
}
