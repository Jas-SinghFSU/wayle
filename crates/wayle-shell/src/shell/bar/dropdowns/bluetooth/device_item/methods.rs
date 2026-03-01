use super::{DeviceItem, messages::PendingAction};
use crate::{
    i18n::{t, td},
    shell::bar::dropdowns::bluetooth::helpers::{
        DeviceCategory, DeviceSnapshot, battery_level_icon,
    },
};

impl DeviceItem {
    pub(crate) fn differs_from(&self, snapshot: &DeviceSnapshot) -> bool {
        self.name != snapshot.name
            || self.icon != snapshot.icon
            || self.connected != snapshot.connected
            || self.paired != snapshot.paired
            || self.category != snapshot.category
            || self.battery_icon != snapshot.battery.map(battery_level_icon)
            || self.battery_text
                != snapshot
                    .battery
                    .map(|percent| t!("dropdown-bluetooth-battery", percent = percent))
    }

    pub(crate) fn update_from_snapshot(&mut self, snapshot: DeviceSnapshot) {
        if let Some(action) = &self.pending {
            let completed = match action {
                PendingAction::Connecting => snapshot.connected,
                PendingAction::Disconnecting => !snapshot.connected,
                PendingAction::Forgetting => false,
            };

            if completed {
                self.pending = None;
            }
        }

        self.name = snapshot.name;
        self.device_type = td!(snapshot.device_type_key);
        self.battery_text = snapshot
            .battery
            .map(|percent| t!("dropdown-bluetooth-battery", percent = percent));
        self.battery_icon = snapshot.battery.map(battery_level_icon);
        self.icon = snapshot.icon;
        self.connected = snapshot.connected;
        self.paired = snapshot.paired;
        self.category = snapshot.category;
        self.device_path = snapshot.device.object_path.clone();
    }

    pub(crate) fn clear_pending(&mut self) {
        self.pending = None;
    }

    pub(crate) fn is_my_device(&self) -> bool {
        matches!(
            self.category,
            DeviceCategory::Connected | DeviceCategory::Paired
        )
    }

    pub(super) fn root_css_classes(&self) -> Vec<&'static str> {
        let mut classes = vec!["bluetooth-device"];

        if self.category == DeviceCategory::Available {
            classes.push("available");
        }

        if self.pending.is_some() {
            classes.push("pending");
        }

        classes
    }
}
