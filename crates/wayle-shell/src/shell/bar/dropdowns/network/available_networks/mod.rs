mod messages;
mod methods;
mod network_item;
mod watchers;

use std::{collections::HashSet, sync::Arc, time::Duration};

use gtk::prelude::*;
use relm4::{factory::FactoryVecDeque, gtk, prelude::*};
use tracing::warn;
use wayle_common::WatcherToken;
use wayle_network::NetworkService;
use wayle_widgets::prelude::*;

pub(crate) use self::messages::{
    AvailableNetworksInit, AvailableNetworksInput, AvailableNetworksOutput,
};
use self::{
    messages::{AvailableNetworksCmd, SelectedNetwork},
    network_item::NetworkItem,
};
use crate::{
    i18n::t,
    shell::bar::dropdowns::network::{
        helpers::{self, AccessPointSnapshot},
        password_form::{PasswordForm, PasswordFormInput, PasswordFormOutput},
    },
};

const SCAN_TIMEOUT: Duration = Duration::from_secs(30);

pub(crate) struct AvailableNetworks {
    network: Arc<NetworkService>,
    wifi_available: bool,
    network_list: FactoryVecDeque<NetworkItem>,
    ap_cache: Vec<AccessPointSnapshot>,
    known_ssids: HashSet<String>,
    state: ListState,
    selection: Option<SelectedNetwork>,
    password_form: Controller<PasswordForm>,
    ap_watcher: WatcherToken,
    connection_watcher: WatcherToken,
    scan_watcher: WatcherToken,
}

#[derive(PartialEq)]
pub(super) enum ListState {
    Normal,
    PasswordEntry,
    Connecting,
    Scanning,
}

#[relm4::component(pub(crate))]
impl Component for AvailableNetworks {
    type Init = AvailableNetworksInit;
    type Input = AvailableNetworksInput;
    type Output = AvailableNetworksOutput;
    type CommandOutput = AvailableNetworksCmd;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Label {
                add_css_class: "section-label",
                set_halign: gtk::Align::Start,
                set_label: &t!("dropdown-network-available"),
                #[watch]
                set_visible: model.wifi_available,
            },

            #[local_ref]
            password_form_widget -> gtk::Box {
                #[watch]
                set_visible: model.wifi_available
                    && model.state == ListState::PasswordEntry,
            },

            #[template]
            Card {
                add_css_class: "network-list",
                set_overflow: gtk::Overflow::Hidden,
                set_vexpand: true,
                #[watch]
                set_visible: model.wifi_available && !model.ap_cache.is_empty(),

                gtk::ScrolledWindow {
                    add_css_class: "network-list-scroll",
                    set_vexpand: true,
                    set_hscrollbar_policy: gtk::PolicyType::Never,

                    #[local_ref]
                    network_list_widget -> gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                    },
                },
            },

            gtk::Box {
                #[watch]
                set_visible: model.wifi_available
                    && model.ap_cache.is_empty(),
                set_vexpand: true,
                set_valign: gtk::Align::Center,

                #[template]
                EmptyState {
                    #[template_child]
                    icon {
                        add_css_class: "sm",
                        set_icon_name: Some("cm-wireless-disabled-symbolic"),
                    },
                    #[template_child]
                    title {
                        set_label: &t!("dropdown-network-no-networks-title"),
                    },
                    #[template_child]
                    description {
                        set_label: &t!("dropdown-network-no-networks-description"),
                    },
                },
            },

            gtk::Box {
                #[watch]
                set_visible: !model.wifi_available,
                set_vexpand: true,
                set_valign: gtk::Align::Center,

                #[template]
                EmptyState {
                    #[template_child]
                    icon {
                        add_css_class: "sm",
                        set_icon_name: Some("tb-wifi-off-symbolic"),
                    },
                    #[template_child]
                    title {
                        set_label: &t!("dropdown-network-no-adapter-title"),
                    },
                    #[template_child]
                    description {
                        set_label: &t!("dropdown-network-no-adapter-description"),
                    },
                },
            },

        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let password_form = PasswordForm::builder()
            .launch(())
            .forward(sender.input_sender(), |form_output| {
                AvailableNetworksInput::PasswordForm(form_output)
            });

        let network_list = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), methods::forward_network_item_output);

        watchers::spawn_settings_watcher(&sender, &init.network.settings);

        let wifi_available = init.network.wifi.get().is_some();

        let mut model = Self {
            network: init.network.clone(),
            wifi_available,
            network_list,
            ap_cache: vec![],
            known_ssids: HashSet::new(),
            state: ListState::Normal,
            selection: None,
            password_form,
            ap_watcher: WatcherToken::new(),
            connection_watcher: WatcherToken::new(),
            scan_watcher: WatcherToken::new(),
        };

        if let Some(wifi) = init.network.wifi.get() {
            let token = model.ap_watcher.reset();
            watchers::spawn(&sender, &wifi, token);
        }

        model.rebuild_network_list(None);

        let password_form_widget = model.password_form.widget();
        let network_list_widget = model.network_list.widget();

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            AvailableNetworksInput::WifiAvailabilityChanged(available) => {
                self.wifi_available = available;

                let token = self.ap_watcher.reset();

                if let Some(wifi) = self.network.wifi.get() {
                    watchers::spawn(&sender, &wifi, token);
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
            AvailableNetworksInput::WifiEnabledChanged(enabled) => {
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
            AvailableNetworksInput::ScanRequested => {
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
            AvailableNetworksInput::NetworkSelected(index) => {
                let Some(ap) = self.ap_cache.get(index) else {
                    return;
                };

                let security_label = methods::translate_security_type(ap.security);
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
                    self.connect_to_selected(None, &sender);
                }
            }
            AvailableNetworksInput::PasswordForm(form_output) => match form_output {
                PasswordFormOutput::Connect { password } => {
                    self.connect_to_selected(Some(password), &sender);
                }
                PasswordFormOutput::Cancel => {
                    self.state = ListState::Normal;
                    self.clear_selection();
                }
            },
        }
    }

    fn update_cmd(
        &mut self,
        msg: AvailableNetworksCmd,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            AvailableNetworksCmd::AccessPointsChanged => {
                let connected_ssid = self.network.wifi.get().and_then(|wifi| wifi.ssid.get());

                self.rebuild_network_list(connected_ssid.as_deref());

                if self.state == ListState::Scanning && !self.ap_cache.is_empty() {
                    self.state = ListState::Normal;

                    let _ = sender.output(AvailableNetworksOutput::ScanComplete);
                }
            }
            AvailableNetworksCmd::KnownSsidsUpdated(known_ssids) => {
                self.known_ssids = known_ssids;

                let connected_ssid = self.network.wifi.get().and_then(|wifi| wifi.ssid.get());
                self.rebuild_network_list(connected_ssid.as_deref());
            }
            AvailableNetworksCmd::ConnectionProgress(step) => {
                let _ = sender.output(AvailableNetworksOutput::ConnectionProgress(step));
            }
            AvailableNetworksCmd::ConnectImmediateError(err) => {
                warn!(error = %err, "wifi connection failed immediately");

                self.handle_connection_failure(err, &sender);
            }
            AvailableNetworksCmd::ConnectionActivated => {
                self.state = ListState::Normal;
                self.clear_selection();

                let _ = sender.output(AvailableNetworksOutput::Connected);
            }
            AvailableNetworksCmd::ConnectionAuthFailed => {
                self.state = ListState::PasswordEntry;

                let _ = sender.output(AvailableNetworksOutput::ClearConnecting);

                if let Some(selection) = &self.selection {
                    self.password_form.emit(PasswordFormInput::Show {
                        ssid: selection.ssid.clone(),
                        security_label: selection.security_label.clone(),
                        signal_icon: selection.signal_icon,
                        error_message: Some(t!("dropdown-network-error-wrong-password")),
                    });
                }
            }
            AvailableNetworksCmd::ConnectionTimedOut => {
                self.handle_connection_failure(t!("dropdown-network-error-timeout"), &sender);
            }

            AvailableNetworksCmd::ConnectionFailed(reason) => {
                self.handle_connection_failure(reason, &sender);
            }

            AvailableNetworksCmd::ScanComplete => {
                if self.state == ListState::Scanning {
                    self.state = ListState::Normal;
                }

                let _ = sender.output(AvailableNetworksOutput::ScanComplete);
            }
        }
    }
}
