use relm4::prelude::*;

use super::{NetworkDropdown, watchers};

impl NetworkDropdown {
    pub(super) fn reset_wifi_watchers(&mut self, sender: &ComponentSender<Self>) {
        let token = self.wifi_watcher.reset();
        watchers::spawn_wifi_watchers(sender, &self.network, token);
    }
}
