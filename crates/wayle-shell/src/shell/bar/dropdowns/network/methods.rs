use relm4::prelude::*;
use tracing::warn;

use super::{NetworkDropdown, watchers};

impl NetworkDropdown {
    pub(super) fn reset_wifi_watchers(&mut self, sender: &ComponentSender<Self>) {
        let token = self.wifi_watcher.reset();
        watchers::spawn_wifi_watchers(sender, &self.network, token);
    }

    pub(super) fn toggle_wifi(&mut self, active: bool, sender: &ComponentSender<Self>) {
        self.wifi_enabled = active;

        let network = self.network.clone();

        sender.command(move |_out, _shutdown| async move {
            if let Some(wifi) = network.wifi.get()
                && let Err(err) = wifi.set_enabled(active).await
            {
                warn!(error = %err, "wifi toggle failed");
            }
        });
    }
}
