mod device_item;
mod factory;
pub(crate) mod helpers;
pub(crate) mod messages;
mod methods;
mod pairing_card;
mod watchers;

use std::{sync::Arc, time::Duration};

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_bluetooth::BluetoothService;
use wayle_common::WatcherToken;
use wayle_widgets::prelude::*;

pub(super) use self::factory::Factory;
use self::{
    device_item::DeviceItem,
    messages::{BluetoothDropdownCmd, BluetoothDropdownInit, BluetoothDropdownMsg},
    pairing_card::{PairingCard, messages::PairingCardInit},
};
use crate::{i18n::t, shell::bar::dropdowns::scaled_dimension};

const BASE_WIDTH: f32 = 382.0;
const BASE_HEIGHT: f32 = 512.0;
const SCAN_DURATION: Duration = Duration::from_secs(30);
const ACTION_TIMEOUT: Duration = Duration::from_secs(30);

pub(crate) struct BluetoothDropdown {
    bluetooth: Arc<BluetoothService>,
    scaled_width: i32,
    scaled_height: i32,
    enabled: bool,
    available: bool,
    scanning: bool,
    my_devices: FactoryVecDeque<DeviceItem>,
    available_devices: FactoryVecDeque<DeviceItem>,
    pairing_card: Controller<PairingCard>,
    device_watcher: WatcherToken,
    scan_token: WatcherToken,
}

#[relm4::component(pub(crate))]
impl Component for BluetoothDropdown {
    type Init = BluetoothDropdownInit;
    type Input = BluetoothDropdownMsg;
    type Output = ();
    type CommandOutput = BluetoothDropdownCmd;

    view! {
        #[root]
        gtk::Popover {
            set_css_classes: &[
                "dropdown",
                "bluetooth-dropdown",
            ],
            set_has_arrow: false,
            #[watch]
            set_width_request: model.scaled_width,
            #[watch]
            set_height_request: model.scaled_height,

            #[template]
            Dropdown {
                set_overflow: gtk::Overflow::Hidden,

                #[template]
                DropdownHeader {
                    #[template_child]
                    icon {
                        set_visible: true,
                        #[watch]
                        set_icon_name: Some(
                            if model.enabled {
                                "ld-bluetooth-symbolic"
                            } else {
                                "ld-bluetooth-off-symbolic"
                            }
                        ),
                    },
                    #[template_child]
                    label {
                        set_label: &t!(
                            "dropdown-bluetooth-title"
                        ),
                    },
                    #[template_child]
                    actions {
                        #[template]
                        GhostIconButton {
                            add_css_class:
                                "bluetooth-scan-btn",
                            set_icon_name:
                                "tb-refresh-symbolic",
                            #[watch]
                            set_visible:
                                model.available
                                    && model.enabled,
                            #[watch]
                            set_sensitive:
                                !model.scanning,
                            #[watch]
                            set_css_classes: &if
                                model.scanning
                            {
                                vec![
                                    "ghost-icon",
                                    "bluetooth-scan-btn",
                                    "scanning",
                                ]
                            } else {
                                vec![
                                    "ghost-icon",
                                    "bluetooth-scan-btn",
                                ]
                            },
                            connect_clicked =>
                                BluetoothDropdownMsg::ScanRequested,
                        },

                        #[template]
                        Switch {
                            #[watch]
                            #[block_signal(bt_toggle)]
                            set_active: model.enabled,
                            #[watch]
                            set_visible: model.available,
                            connect_state_set[sender] =>
                                move |switch, active|
                            {
                                sender.input(
                                    BluetoothDropdownMsg
                                        ::BluetoothToggled(
                                            active,
                                        ),
                                );
                                switch.set_state(active);
                                gtk::glib::Propagation::Stop
                            } @bt_toggle,
                        },
                    },
                },

                #[template]
                DropdownContent {
                    add_css_class: "bluetooth-content",

                    gtk::ScrolledWindow {
                        add_css_class: "bluetooth-scroll",
                        set_vexpand: true,
                        set_hscrollbar_policy: gtk::PolicyType::Never,

                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,

                            #[local_ref]
                            pairing_card_widget -> gtk::Box {},

                            #[name = "my_devices_label"]
                            gtk::Label {
                                add_css_class: "section-label",
                                set_halign: gtk::Align::Start,
                                set_label: &t!(
                                    "dropdown-bluetooth-my-devices"
                                ),
                                #[watch]
                                set_visible: model.enabled
                                    && !model
                                        .my_devices
                                        .is_empty(),
                            },

                            #[name = "my_devices_card"]
                            #[template]
                            Card {
                                add_css_class:
                                    "bluetooth-device-list",
                                #[watch]
                                set_visible: model.enabled
                                    && !model
                                        .my_devices
                                        .is_empty(),
                                #[local_ref]
                                my_devices_widget -> gtk::Box {
                                    set_orientation:
                                        gtk::Orientation::Vertical,
                                },
                            },

                            #[name = "available_devices_label"]
                            gtk::Label {
                                add_css_class: "section-label",
                                set_halign: gtk::Align::Start,
                                set_label: &t!(
                                    "dropdown-bluetooth-available-devices"
                                ),
                                #[watch]
                                set_visible: model.enabled
                                    && (!model
                                        .available_devices
                                        .is_empty()
                                        || model.scanning),
                            },

                            #[name = "available_devices_card"]
                            #[template]
                            Card {
                                add_css_class:
                                    "bluetooth-device-list",
                                #[watch]
                                set_visible: model.enabled
                                    && !model
                                        .available_devices
                                        .is_empty(),
                                #[local_ref]
                                available_devices_widget -> gtk::Box {
                                    set_orientation:
                                        gtk::Orientation::Vertical,
                                },
                            },

                            #[name = "scanning_hint"]
                            gtk::Label {
                                add_css_class:
                                    "bluetooth-no-new-devices",
                                #[watch]
                                set_visible: model.enabled
                                    && model.scanning
                                    && model
                                        .my_devices
                                        .is_empty()
                                    && model
                                        .available_devices
                                        .is_empty(),
                                set_label: &t!(
                                    "dropdown-bluetooth-no-new"
                                ),
                            },

                            #[name = "empty_no_devices"]
                            #[template]
                            EmptyState {
                                #[watch]
                                set_visible: model.enabled
                                    && !model.scanning
                                    && model
                                        .my_devices
                                        .is_empty()
                                    && model
                                        .available_devices
                                        .is_empty(),
                                #[template_child]
                                icon {
                                    set_icon_name: Some(
                                        "ld-bluetooth-searching-symbolic"
                                    ),
                                },
                                #[template_child]
                                title {
                                    set_label: &t!(
                                        "dropdown-bluetooth-no-devices-title"
                                    ),
                                },
                                #[template_child]
                                description {
                                    set_label: &t!(
                                        "dropdown-bluetooth-no-devices-description"
                                    ),
                                },
                            },

                            #[name = "empty_bt_off"]
                            #[template]
                            EmptyState {
                                #[watch]
                                set_visible: !model.enabled
                                    && model.available,
                                #[template_child]
                                icon {
                                    set_icon_name: Some(
                                        "ld-bluetooth-off-symbolic"
                                    ),
                                },
                                #[template_child]
                                title {
                                    set_label: &t!(
                                        "dropdown-bluetooth-off-title"
                                    ),
                                },
                                #[template_child]
                                description {
                                    set_label: &t!(
                                        "dropdown-bluetooth-off-description"
                                    ),
                                },
                            },

                            #[name = "empty_no_adapter"]
                            #[template]
                            EmptyState {
                                #[watch]
                                set_visible: !model.available,
                                #[template_child]
                                icon {
                                    set_icon_name: Some(
                                        "ld-bluetooth-off-symbolic"
                                    ),
                                },
                                #[template_child]
                                title {
                                    set_label: &t!(
                                        "dropdown-bluetooth-no-adapter-title"
                                    ),
                                },
                                #[template_child]
                                description {
                                    set_label: &t!(
                                        "dropdown-bluetooth-no-adapter-description"
                                    ),
                                },
                            },
                        },
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
        let my_devices = Self::build_device_list(&sender);
        let available_devices = Self::build_device_list(&sender);

        let pairing_card = PairingCard::builder()
            .launch(PairingCardInit)
            .forward(sender.input_sender(), BluetoothDropdownMsg::PairingCard);

        let scale = init.config.config().styling.scale.get().value();

        watchers::spawn(&sender, &init.config, &init.bluetooth);

        let mut model = Self {
            bluetooth: init.bluetooth,
            scaled_width: scaled_dimension(BASE_WIDTH, scale),
            scaled_height: scaled_dimension(BASE_HEIGHT, scale),
            enabled: false,
            available: false,
            scanning: false,
            my_devices,
            available_devices,
            pairing_card,
            device_watcher: WatcherToken::new(),
            scan_token: WatcherToken::new(),
        };

        model.rebuild_device_lists();
        model.reset_device_watchers(&sender);

        let pairing_card_widget = model.pairing_card.widget();
        let my_devices_widget = model.my_devices.widget();
        let available_devices_widget = model.available_devices.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            BluetoothDropdownMsg::BluetoothToggled(active) => {
                self.handle_bluetooth_toggled(active, &sender);
            }

            BluetoothDropdownMsg::ScanRequested => {
                self.handle_scan_requested(&sender);
            }

            BluetoothDropdownMsg::DeviceAction(action) => {
                self.handle_device_action(action, &sender);
            }

            BluetoothDropdownMsg::PairingCard(output) => {
                self.handle_pairing_output(output, &sender);
            }
        }
    }

    fn update_cmd(
        &mut self,
        msg: BluetoothDropdownCmd,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            BluetoothDropdownCmd::ScaleChanged(scale) => {
                self.scaled_width = scaled_dimension(BASE_WIDTH, scale);
                self.scaled_height = scaled_dimension(BASE_HEIGHT, scale);
            }

            BluetoothDropdownCmd::ScanComplete => {
                self.scanning = false;
            }

            BluetoothDropdownCmd::EnabledChanged(enabled) => {
                self.enabled = enabled;
            }

            BluetoothDropdownCmd::AvailableChanged(available) => {
                self.available = available;
            }

            BluetoothDropdownCmd::DevicesChanged => {
                self.rebuild_device_lists();
                self.reset_device_watchers(&sender);
            }

            BluetoothDropdownCmd::DevicePropertyChanged => {
                self.rebuild_device_lists();
            }

            BluetoothDropdownCmd::DeviceActionFailed(path) => {
                self.clear_device_pending(&path);
            }

            BluetoothDropdownCmd::PairingRequested(request) => {
                self.handle_pairing_request(request);
            }
        }
    }
}
