mod helpers;
mod messages;
mod watchers;

use std::sync::Arc;

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_network::NetworkService;
use wayle_sysinfo::SysinfoService;
use wayle_widgets::WatcherToken;

use self::messages::NetworkSectionCmd;
pub(crate) use self::messages::{NetworkSectionInit, NetworkSectionInput};
use crate::i18n::t;

pub(crate) struct NetworkSection {
    network: Option<Arc<NetworkService>>,
    sysinfo: Arc<SysinfoService>,
    active: bool,
    watcher: WatcherToken,
    connected: bool,
    upload: String,
    upload_is_megabytes: bool,
    download: String,
    download_is_megabytes: bool,
}

impl NetworkSection {
    fn speed_unit_label(is_megabytes: bool) -> String {
        if is_megabytes {
            t!("dropdown-dashboard-network-speed-mbs")
        } else {
            t!("dropdown-dashboard-network-speed-kbs")
        }
    }
}

#[relm4::component(pub(crate))]
impl Component for NetworkSection {
    type Init = NetworkSectionInit;
    type Input = NetworkSectionInput;
    type Output = ();
    type CommandOutput = NetworkSectionCmd;

    view! {
        #[root]
        gtk::Box {
            set_css_classes: &["card", "dashboard-card"],
            set_orientation: gtk::Orientation::Vertical,

            #[name = "header"]
            gtk::Box {
                add_css_class: "card-header",

                #[name = "card_title"]
                gtk::Box {
                    add_css_class: "card-title",

                    gtk::Image {
                        set_icon_name: Some("ld-wifi-symbolic"),
                    },

                    gtk::Label {
                        set_label: &t!("dropdown-dashboard-network"),
                    },
                },

            },

            #[name = "speeds_container"]
            gtk::Box {
                add_css_class: "network-speeds",

                #[name = "upload_stat"]
                gtk::Box {
                    #[watch]
                    set_css_classes: if model.connected {
                        &["speed-stat", "up"]
                    } else {
                        &["speed-stat", "muted"]
                    },
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,

                    gtk::Image {
                        add_css_class: "speed-arrow",
                        set_icon_name: Some("ld-arrow-up-symbolic"),
                    },

                    #[name = "upload_value"]
                    gtk::Label {
                        add_css_class: "speed-value",
                        #[watch]
                        set_class_active: ("muted", !model.connected),
                        #[watch]
                        set_label: if model.connected {
                            &model.upload
                        } else {
                            "--"
                        },
                    },

                    #[name = "upload_unit"]
                    gtk::Label {
                        add_css_class: "speed-unit",
                        #[watch]
                        set_label: &Self::speed_unit_label(model.upload_is_megabytes),
                    },
                },

                #[name = "download_stat"]
                gtk::Box {
                    #[watch]
                    set_css_classes: if model.connected {
                        &["speed-stat", "down"]
                    } else {
                        &["speed-stat", "muted"]
                    },
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,

                    gtk::Image {
                        add_css_class: "speed-arrow",
                        set_icon_name: Some("ld-arrow-down-symbolic"),
                    },

                    #[name = "download_value"]
                    gtk::Label {
                        add_css_class: "speed-value",
                        #[watch]
                        set_class_active: ("muted", !model.connected),
                        #[watch]
                        set_label: if model.connected {
                            &model.download
                        } else {
                            "--"
                        },
                    },

                    #[name = "download_unit"]
                    gtk::Label {
                        add_css_class: "speed-unit",
                        #[watch]
                        set_label: &Self::speed_unit_label(model.download_is_megabytes),
                    },
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            network: init.network,
            sysinfo: init.sysinfo,

            active: false,
            watcher: WatcherToken::new(),

            connected: false,
            upload: String::from("0.0"),
            upload_is_megabytes: false,
            download: String::from("0.0"),
            download_is_megabytes: false,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            NetworkSectionInput::SetActive(active) => {
                if self.active == active {
                    return;
                }

                self.active = active;
                if active {
                    let token = self.watcher.reset();
                    watchers::spawn(&sender, &self.network, &self.sysinfo, token);
                } else {
                    self.watcher = WatcherToken::new();
                }
            }
        }
    }

    fn update_cmd(
        &mut self,
        msg: NetworkSectionCmd,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            NetworkSectionCmd::ConnectionChanged { connected } => {
                self.connected = connected;
            }

            NetworkSectionCmd::SpeedChanged {
                upload,
                upload_is_megabytes,
                download,
                download_is_megabytes,
            } => {
                self.upload = upload;
                self.upload_is_megabytes = upload_is_megabytes;
                self.download = download;
                self.download_is_megabytes = download_is_megabytes;
            }
        }
    }
}
