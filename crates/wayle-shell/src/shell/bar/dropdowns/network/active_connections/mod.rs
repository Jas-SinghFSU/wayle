mod messages;
mod methods;
mod watchers;

use std::sync::Arc;

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use tracing::warn;
use wayle_common::WatcherToken;
use wayle_network::{NetworkService, core::access_point::Ssid, types::states::NetworkStatus};
use wayle_widgets::prelude::*;

use self::messages::{ActiveConnectionsCmd, ConnectionProgress, WifiState, WiredState};
pub(crate) use self::messages::{ActiveConnectionsInit, ActiveConnectionsInput};
use crate::{i18n::t, shell::bar::dropdowns::network::helpers};

pub(crate) struct ActiveConnections {
    network: Arc<NetworkService>,
    wifi: WifiState,
    wired: WiredState,
    connection: ConnectionProgress,
    has_connections: bool,
    wifi_watcher: WatcherToken,
    wired_watcher: WatcherToken,
}

#[relm4::component(pub(crate))]
impl Component for ActiveConnections {
    type Init = ActiveConnectionsInit;
    type Input = ActiveConnectionsInput;
    type Output = ();
    type CommandOutput = ActiveConnectionsCmd;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            #[watch]
            set_visible: model.has_connections
                || model.is_wifi_connecting()
                || model.connection.error.is_some(),

            gtk::Label {
                add_css_class: "section-label",
                set_halign: gtk::Align::Start,
                #[watch]
                set_label: &if model.wired.connected && model.wifi.connected {
                    t!("dropdown-network-active-connections")
                } else {
                    t!("dropdown-network-active-connection")
                },
            },

            #[template]
            Card {
                add_css_class: "network-connections-group",
                set_orientation: gtk::Orientation::Vertical,

                gtk::Box {
                    add_css_class: "network-connection-card",
                    #[watch]
                    set_visible: model.wired.connected,

                    gtk::Box {
                        add_css_class: "network-connection-icon",
                        add_css_class: "ethernet",
                        set_hexpand: false,

                        gtk::Image {
                            set_icon_name: Some("cm-wired-symbolic"),
                            set_hexpand: true,
                        },
                    },

                    gtk::Box {
                        add_css_class: "network-connection-info",
                        set_orientation: gtk::Orientation::Vertical,
                        set_hexpand: true,

                        gtk::Label {
                            add_css_class: "network-connection-name",
                            set_halign: gtk::Align::Start,
                            set_label: &t!("dropdown-network-ethernet"),
                        },

                        gtk::Label {
                            add_css_class: "network-connection-detail",
                            set_halign: gtk::Align::Start,
                            #[watch]
                            set_label: &model.wired_detail(),
                            #[watch]
                            set_visible: model.wired.speed > 0,
                        },
                    },

                    #[template]
                    SubtleSuccessBadge {
                        add_css_class: "network-connection-status",
                        set_label: &t!("dropdown-network-connected"),
                        set_vexpand: false,
                        set_valign: gtk::Align::Center,
                    },
                },

                #[name = "wifi_card"]
                gtk::Box {
                    add_css_class: "network-connection-card",
                    #[watch]
                    set_visible: model.wifi.connected
                        || model.is_wifi_connecting()
                        || model.connection.error.is_some(),

                    gtk::Box {
                        add_css_class: "network-connection-icon",
                        #[watch]
                        set_css_classes: &model.wifi_icon_classes(),
                        set_hexpand: false,

                        gtk::Image {
                            #[watch]
                            set_icon_name: Some(model.effective_wifi_icon()),
                            set_hexpand: true,
                        },
                    },

                    gtk::Box {
                        add_css_class: "network-connection-info",
                        set_orientation: gtk::Orientation::Vertical,
                        set_hexpand: true,

                        gtk::Label {
                            add_css_class: "network-connection-name",
                            set_halign: gtk::Align::Start,
                            set_ellipsize: gtk::pango::EllipsizeMode::End,
                            #[watch]
                            set_label: &model.display_wifi_name(),
                        },

                        gtk::Label {
                            add_css_class: "network-connection-detail",
                            set_halign: gtk::Align::Start,
                            #[watch]
                            set_label: &model.wifi_detail(),
                            #[watch]
                            set_visible: model.wifi_detail_visible(),
                        },
                    },

                    #[template]
                    SubtleBadge {
                        #[watch]
                        set_css_classes: &model.status_classes(),
                        #[watch]
                        set_label: &model.status_label(),
                        set_vexpand: false,
                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_visible: !(model.wifi.hovered
                            && model.wifi.connected),
                    },

                    gtk::Box {
                        add_css_class: "network-connection-actions",
                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_visible: model.wifi.hovered
                            && model.wifi.connected,

                        #[template]
                        GhostButton {
                            add_css_class: "network-action-disconnect",
                            #[template_child]
                            label { set_label: &t!("dropdown-network-disconnect") },
                            connect_clicked => ActiveConnectionsInput::DisconnectWifi,
                        },

                        #[template]
                        GhostButton {
                            add_css_class: "network-action-forget",
                            #[template_child]
                            label { set_label: &t!("dropdown-network-forget") },
                            connect_clicked => ActiveConnectionsInput::ForgetWifi,
                        },
                    },

                    #[template]
                    GhostIconButton {
                        add_css_class: "network-disconnect",
                        set_icon_name: "ld-x-symbolic",
                        #[watch]
                        set_visible: model.connection.error.is_some(),
                        connect_clicked => ActiveConnectionsInput::DismissError,
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
        let wifi = init
            .network
            .wifi
            .get()
            .map(|wifi| WifiState::from_network(&wifi))
            .unwrap_or_default();
        let wired = init
            .network
            .wired
            .get()
            .map(|wired| WiredState::from_network(&wired))
            .unwrap_or_default();
        let has_connections = wifi.connected || wifi.connecting || wired.connected;

        let mut model = Self {
            network: init.network.clone(),
            wifi,
            wired,
            connection: ConnectionProgress::default(),
            has_connections,
            wifi_watcher: WatcherToken::new(),
            wired_watcher: WatcherToken::new(),
        };

        watchers::spawn_device_watchers(&sender, &init.network);

        model.reset_wifi_watchers(&sender);
        model.reset_wired_watchers(&sender);

        let widgets = view_output!();

        let hover = gtk::EventControllerMotion::new();

        let hover_sender = sender.input_sender().clone();
        hover.connect_enter(move |_, _, _| {
            hover_sender.emit(ActiveConnectionsInput::WifiCardHovered(true));
        });

        let leave_sender = sender.input_sender().clone();
        hover.connect_leave(move |_| {
            leave_sender.emit(ActiveConnectionsInput::WifiCardHovered(false));
        });

        widgets.wifi_card.add_controller(hover);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            ActiveConnectionsInput::DisconnectWifi => {
                let network = self.network.clone();
                sender.command(|_out, _shutdown| async move {
                    if let Some(wifi) = network.wifi.get()
                        && let Err(err) = wifi.disconnect().await
                    {
                        warn!(error = %err, "wifi disconnect failed");
                    }
                });
            }
            ActiveConnectionsInput::ForgetWifi => {
                let network = self.network.clone();
                let ssid = self.wifi.ssid.clone();
                sender.command(|_out, _shutdown| async move {
                    let Some(ssid) = ssid.map(|s| Ssid::new(s.into_bytes())) else {
                        return;
                    };

                    for connection in network.settings.connections_for_ssid(&ssid).await {
                        if let Err(err) = connection.delete().await {
                            warn!(error = %err, "failed to delete saved wifi profile");
                        }
                    }

                    if let Some(wifi) = network.wifi.get()
                        && let Err(err) = wifi.disconnect().await
                    {
                        warn!(error = %err, "wifi disconnect after forget failed");
                    }
                });
            }
            ActiveConnectionsInput::DismissError => {
                self.connection.error = None;

                self.update_has_connections();
            }
            ActiveConnectionsInput::WifiCardHovered(hovered) => {
                self.wifi.hovered = hovered;
            }
            ActiveConnectionsInput::SetConnecting(ssid) => {
                self.connection.ssid = Some(ssid);
                self.connection.step = None;
            }
            ActiveConnectionsInput::SetConnectingStep(step) => {
                self.connection.step = Some(step);
            }
            ActiveConnectionsInput::ClearConnecting => {
                self.connection.ssid = None;
                self.connection.step = None;
            }
            ActiveConnectionsInput::SetConnectionError(error) => {
                self.connection = ConnectionProgress {
                    error: Some(error),
                    ..Default::default()
                };
            }
            ActiveConnectionsInput::ClearConnectionError => {
                self.connection.error = None;
            }
        }
    }

    fn update_cmd(
        &mut self,
        msg: ActiveConnectionsCmd,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            ActiveConnectionsCmd::WifiStateChanged {
                connectivity,
                ssid,
                strength,
                frequency,
                ip4_address,
            } => {
                self.wifi.connected = connectivity == NetworkStatus::Connected;
                self.wifi.connecting = connectivity == NetworkStatus::Connecting;
                self.wifi.ssid = ssid;
                self.wifi.strength = strength;
                self.wifi.frequency = frequency;
                self.wifi.ip = ip4_address;

                self.wifi.icon = helpers::signal_strength_icon(self.wifi.strength.unwrap_or(0));

                if self.wifi.connected {
                    self.connection = ConnectionProgress::default();
                }
                self.update_has_connections();
            }
            ActiveConnectionsCmd::WiredStateChanged {
                connectivity,
                speed,
                ip4_address,
            } => {
                self.wired.connected = connectivity == NetworkStatus::Connected;
                self.wired.speed = speed;
                self.wired.ip = ip4_address;

                self.update_has_connections();
            }
            ActiveConnectionsCmd::WifiDeviceChanged => {
                if self.network.wifi.get().is_none() {
                    self.wifi = WifiState::default();

                    self.connection = ConnectionProgress::default();
                }

                self.reset_wifi_watchers(&sender);
                self.update_has_connections();
            }
            ActiveConnectionsCmd::WiredDeviceChanged => {
                if self.network.wired.get().is_none() {
                    self.wired = WiredState::default();
                }

                self.reset_wired_watchers(&sender);
                self.update_has_connections();
            }
        }
    }
}
