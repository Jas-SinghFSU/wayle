use relm4::ComponentSender;

use super::ActiveConnections;
use crate::{i18n::t, shell::bar::dropdowns::network::helpers};

impl ActiveConnections {
    pub(super) fn has_wifi_error(&self) -> bool {
        self.connection.error.is_some() && !self.wifi.connected
    }

    pub(super) fn is_wifi_connecting(&self) -> bool {
        self.connection.ssid.is_some() || self.wifi.connecting
    }

    pub(super) fn update_has_connections(&mut self) {
        self.has_connections = self.wifi.connected || self.wifi.connecting || self.wired.connected;
    }

    pub(super) fn reset_wifi_watchers(&mut self, sender: &ComponentSender<Self>) {
        let token = self.wifi_watcher.reset();

        super::watchers::spawn_wifi_watchers(sender, &self.network, token);
    }

    pub(super) fn reset_wired_watchers(&mut self, sender: &ComponentSender<Self>) {
        let token = self.wired_watcher.reset();

        super::watchers::spawn_wired_watchers(sender, &self.network, token);
    }

    pub(super) fn display_wifi_name(&self) -> String {
        if let Some(ssid) = &self.wifi.ssid {
            return ssid.clone();
        }

        if let Some(connecting) = &self.connection.ssid {
            return connecting.clone();
        }

        t!("dropdown-network-wifi")
    }

    pub(super) fn status_label(&self) -> String {
        if let Some(error) = &self.connection.error {
            return error.clone();
        }

        if self.wifi.connected {
            return t!("dropdown-network-connected");
        }

        if self.is_wifi_connecting() {
            return t!("dropdown-network-connecting");
        }

        String::new()
    }

    pub(super) fn wired_detail(&self) -> String {
        let speed = helpers::format_wired_speed(self.wired.speed);
        match &self.wired.ip {
            Some(ip) => format!("{ip} - {speed}"),
            None => speed,
        }
    }

    pub(super) fn wifi_detail_visible(&self) -> bool {
        self.connection.step.is_some() || self.wifi.frequency.is_some() || self.wifi.ip.is_some()
    }

    pub(super) fn wifi_detail(&self) -> String {
        if let Some(step) = &self.connection.step {
            return step.clone();
        }

        let band = self.wifi.frequency.and_then(helpers::frequency_to_band);
        match (&self.wifi.ip, band) {
            (Some(ip), Some(band)) => format!("{ip} - {band}"),
            (Some(ip), None) => ip.clone(),
            (None, Some(band)) => band.to_string(),
            (None, None) => String::new(),
        }
    }

    pub(super) fn wifi_icon_classes(&self) -> Vec<&'static str> {
        let mut classes = vec!["network-connection-icon"];

        if self.has_wifi_error() {
            classes.push("error");
        } else {
            classes.push("wifi");
        }

        classes
    }

    pub(super) fn effective_wifi_icon(&self) -> &'static str {
        if self.has_wifi_error() {
            return "cm-wireless-disabled-symbolic";
        }

        self.wifi.icon
    }

    pub(super) fn status_classes(&self) -> Vec<&'static str> {
        let mut classes = vec!["badge-subtle", "network-connection-status"];

        if self.connection.error.is_some() {
            classes.push("error");
        } else if self.wifi.connected {
            classes.push("success");
            classes.push("connected");
        } else if self.is_wifi_connecting() {
            classes.push("warning");
            classes.push("connecting");
        }

        classes
    }
}
