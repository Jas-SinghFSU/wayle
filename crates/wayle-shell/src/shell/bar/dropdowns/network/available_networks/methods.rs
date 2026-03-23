use relm4::prelude::*;
use tracing::warn;
use wayle_network::core::access_point::{SecurityType, Ssid};

use crate::{
    i18n::t,
    shell::bar::dropdowns::network::{
        available_networks::{
            AvailableNetworks, ListState, SCAN_TIMEOUT,
            messages::{
                AvailableNetworksCmd, AvailableNetworksInput, AvailableNetworksOutput,
                SelectedNetwork,
            },
            network_item::{NetworkItemInit, NetworkItemOutput},
            watchers,
        },
        helpers,
        password_form::{PasswordFormInput, PasswordFormOutput},
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

    pub(super) fn handle_wifi_availability(
        &mut self,
        available: bool,
        sender: &ComponentSender<Self>,
    ) {
        self.wifi_available = available;

        let token = self.ap_watcher.reset();

        if let Some(wifi) = self.network.wifi.get() {
            watchers::spawn(sender, &wifi, token);
        }

        if !available {
            let _ = self.connection_watcher.reset();
            let _ = self.scan_watcher.reset();

            if self.state == ListState::Scanning {
                let _ = sender.output(AvailableNetworksOutput::ScanComplete);
            }

            if self.state == ListState::Connecting {
                let _ = sender.output(AvailableNetworksOutput::ClearConnecting);
            }

            self.state = ListState::Normal;
            self.clear_selection();
        }

        self.rebuild_network_list(None);
    }

    pub(super) fn handle_wifi_enabled(&mut self, enabled: bool, sender: &ComponentSender<Self>) {
        if enabled {
            self.rebuild_network_list(None);
            return;
        }

        let _ = self.connection_watcher.reset();
        let _ = self.scan_watcher.reset();

        self.ap_cache.clear();

        self.network_list.guard().clear();

        if self.state == ListState::Scanning {
            let _ = sender.output(AvailableNetworksOutput::ScanComplete);
        }

        if self.state == ListState::Connecting {
            let _ = sender.output(AvailableNetworksOutput::ClearConnecting);
        }

        self.state = ListState::Normal;
        self.clear_selection();
    }

    pub(super) fn start_scan(&mut self, sender: &ComponentSender<Self>) {
        self.state = ListState::Scanning;

        let _ = sender.output(AvailableNetworksOutput::ScanStarted);

        let network = self.network.clone();
        let token = self.scan_watcher.reset();

        sender.command(move |out, shutdown| async move {
            if let Some(wifi) = network.wifi.get()
                && let Err(err) = wifi.device.request_scan().await
            {
                warn!(error = %err, "wifi scan failed");
                let _ = out.send(AvailableNetworksCmd::ScanComplete);
                return;
            }

            tokio::select! {
                () = shutdown.wait() => {}
                () = token.cancelled() => {}
                () = tokio::time::sleep(SCAN_TIMEOUT) => {
                    let _ = out.send(AvailableNetworksCmd::ScanComplete);
                }
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

    pub(super) fn select_network(&mut self, index: usize, sender: &ComponentSender<Self>) {
        let Some(ap) = self.ap_cache.get(index) else {
            return;
        };

        let security_label = translate_security_type(ap.security);
        let signal_icon = helpers::signal_strength_icon(ap.strength);

        self.selection = Some(SelectedNetwork {
            ap_path: ap.object_path.clone(),
            ssid: ap.ssid.clone(),
            security_label: security_label.clone(),
            signal_icon,
        });

        if helpers::requires_password(ap.security) && !ap.known {
            self.state = ListState::PasswordEntry;

            self.password_form.emit(PasswordFormInput::Show {
                ssid: ap.ssid.clone(),
                security_label,
                signal_icon,
                error_message: None,
            });
        } else {
            self.connect_to_selected(None, sender);
        }
    }

    pub(super) fn handle_password_form(
        &mut self,
        form_output: PasswordFormOutput,
        sender: &ComponentSender<Self>,
    ) {
        match form_output {
            PasswordFormOutput::Connect { password } => {
                self.connect_to_selected(Some(password), sender);
            }
            PasswordFormOutput::Cancel => {
                self.state = ListState::Normal;
                self.clear_selection();
            }
        }
    }

    pub(super) fn forget_network(&self, ssid: String, sender: &ComponentSender<Self>) {
        let network = self.network.clone();

        sender.oneshot_command(async move {
            let ssid = Ssid::new(ssid.into_bytes());
            network.settings.delete_connections_for_ssid(&ssid).await;

            AvailableNetworksCmd::AccessPointsChanged
        });
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

        NetworkItemOutput::ForgetRequested(ssid) => AvailableNetworksInput::ForgetNetwork(ssid),
    }
}
