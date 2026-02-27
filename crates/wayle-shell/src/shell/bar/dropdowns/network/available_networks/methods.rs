use relm4::prelude::*;
use wayle_network::core::access_point::SecurityType;

use crate::{
    i18n::t,
    shell::bar::dropdowns::network::{
        available_networks::{
            AvailableNetworks, ListState,
            messages::{AvailableNetworksCmd, AvailableNetworksInput, AvailableNetworksOutput},
            network_item::{NetworkItemInit, NetworkItemOutput},
            watchers,
        },
        helpers,
    },
};

impl AvailableNetworks {
    pub(super) fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub(super) fn handle_connection_failure(
        &mut self,
        message: String,
        sender: &ComponentSender<Self>,
    ) {
        self.state = ListState::Normal;
        self.clear_selection();
        let _ = sender.output(AvailableNetworksOutput::ConnectionFailed(message));
    }

    pub(super) fn connect_to_selected(
        &mut self,
        password: Option<String>,
        sender: &ComponentSender<Self>,
    ) {
        let Some(selection) = &self.selection else {
            return;
        };

        let Some(wifi) = self.network.wifi.get() else {
            return;
        };

        let ap_path = selection.ap_path.clone();
        let ssid = selection.ssid.clone();
        self.state = ListState::Connecting;
        let _ = sender.output(AvailableNetworksOutput::Connecting(ssid));

        let token = self.connection_watcher.reset();
        watchers::spawn_connection_watcher(sender, &wifi, token);

        sender.command(move |out, _shutdown| async move {
            if let Err(err) = wifi.connect(ap_path, password).await {
                let _ = out.send(AvailableNetworksCmd::ConnectImmediateError(err.to_string()));
            }
        });
    }

    pub(super) fn rebuild_network_list(&mut self, connected_ssid: Option<&str>) {
        let raw_aps = self.network.wifi.get().map(|wifi| wifi.access_points.get());
        let snapshots = match raw_aps {
            Some(aps) => {
                helpers::sorted_unique_access_points(&aps, connected_ssid, &self.known_ssids)
            }
            None => vec![],
        };

        if snapshots.is_empty() && !self.ap_cache.is_empty() && self.state == ListState::Scanning {
            return;
        }

        self.ap_cache = snapshots;

        let mut guard = self.network_list.guard();
        guard.clear();

        for snapshot in &self.ap_cache {
            guard.push_back(NetworkItemInit {
                snapshot: snapshot.clone(),
            });
        }
    }
}

pub(super) fn translate_security_type(security: SecurityType) -> String {
    match security {
        SecurityType::None => t!("dropdown-network-security-open"),
        SecurityType::Wep => t!("dropdown-network-security-wep"),
        SecurityType::Wpa => t!("dropdown-network-security-wpa"),
        SecurityType::Wpa2 => t!("dropdown-network-security-wpa2"),
        SecurityType::Wpa3 => t!("dropdown-network-security-wpa3"),
        SecurityType::Enterprise => t!("dropdown-network-security-enterprise"),
    }
}

pub(super) fn forward_network_item_output(
    item_output: NetworkItemOutput,
) -> AvailableNetworksInput {
    match item_output {
        NetworkItemOutput::Selected(index) => {
            AvailableNetworksInput::NetworkSelected(index.current_index())
        }
    }
}
