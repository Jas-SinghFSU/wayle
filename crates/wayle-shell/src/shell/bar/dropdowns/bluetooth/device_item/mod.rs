pub(crate) mod messages;
mod methods;

use gtk::{pango, prelude::*};
use relm4::{gtk, prelude::*};
use wayle_widgets::prelude::*;
use zbus::zvariant::OwnedObjectPath;

use self::messages::{DeviceItemInit, DeviceItemInput, DeviceItemOutput, PendingAction};
use crate::{
    i18n::{t, td},
    shell::bar::dropdowns::bluetooth::helpers::{DeviceCategory, battery_level_icon},
};

const DETAIL_SEPARATOR: &str = "\u{2022}";
const HOVER_TRANSITION_MS: u32 = 150;

pub(crate) struct DeviceItem {
    name: String,
    device_type: String,
    battery_text: Option<String>,
    battery_icon: Option<&'static str>,
    icon: &'static str,

    connected: bool,
    paired: bool,
    hovered: bool,
    pub(crate) pending: Option<PendingAction>,

    category: DeviceCategory,
    pub(crate) device_path: OwnedObjectPath,
}

#[relm4::factory(pub(crate))]
impl FactoryComponent for DeviceItem {
    type Init = DeviceItemInit;
    type Input = DeviceItemInput;
    type Output = DeviceItemOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box {
            add_css_class: "bluetooth-device",
            set_cursor_from_name: Some("pointer"),
            #[watch]
            set_css_classes: &self.root_css_classes(),

            #[name = "icon_container"]
            gtk::Box {
                #[watch]
                set_css_classes: &match self.category {
                    DeviceCategory::Connected => vec![
                        "bluetooth-device-icon",
                        "connected",
                    ],
                    DeviceCategory::Paired => vec![
                        "bluetooth-device-icon",
                        "paired",
                    ],
                    DeviceCategory::Available => vec![
                        "bluetooth-device-icon",
                    ],
                },
                set_hexpand: false,

                #[name = "device_icon"]
                gtk::Image {
                    add_css_class: "bluetooth-icon",
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    #[watch]
                    set_icon_name: Some(self.icon),
                },
            },

            #[name = "info_column"]
            gtk::Box {
                add_css_class: "bluetooth-device-info",
                set_orientation:
                    gtk::Orientation::Vertical,
                set_hexpand: true,
                set_valign: gtk::Align::Center,

                #[name = "device_name"]
                gtk::Label {
                    add_css_class:
                        "bluetooth-device-name",
                    set_halign: gtk::Align::Start,
                    set_ellipsize:
                        pango::EllipsizeMode::End,
                    #[watch]
                    set_label: &self.name,
                },

                #[name = "detail_row"]
                gtk::Box {
                    add_css_class:
                        "bluetooth-device-detail-row",
                    set_halign: gtk::Align::Start,

                    #[name = "device_type_label"]
                    gtk::Label {
                        add_css_class:
                            "bluetooth-device-detail",
                        #[watch]
                        set_label: &self.device_type,
                    },

                    #[name = "battery_separator"]
                    gtk::Label {
                        add_css_class:
                            "bluetooth-detail-separator",
                        set_label: DETAIL_SEPARATOR,
                        #[watch]
                        set_visible:
                            self.battery_icon.is_some(),
                    },

                    #[name = "battery_icon"]
                    gtk::Image {
                        add_css_class:
                            "bluetooth-battery-icon",
                        #[watch]
                        set_visible:
                            self.battery_icon.is_some(),
                        #[watch]
                        set_icon_name:
                            self.battery_icon,
                    },

                    #[name = "battery_label"]
                    gtk::Label {
                        add_css_class:
                            "bluetooth-device-detail",
                        #[watch]
                        set_visible:
                            self.battery_text.is_some(),
                        #[watch]
                        set_label:
                            self.battery_text
                                .as_deref()
                                .unwrap_or_default(),
                    },
                },
            },

            gtk::Stack {
                add_css_class: "bluetooth-hover-stack",
                set_transition_type:
                    gtk::StackTransitionType::Crossfade,
                set_transition_duration: HOVER_TRANSITION_MS,
                set_valign: gtk::Align::Center,
                set_hexpand: false,
                #[watch]
                set_visible: self.is_my_device()
                    || self.pending.is_some(),
                #[watch]
                set_visible_child_name:
                    if self.hovered
                        && self.pending.is_none()
                    {
                        "actions"
                    } else {
                        "status"
                    },

                add_named[Some("status")] = &gtk::Box {
                    set_halign: gtk::Align::End,
                    set_valign: gtk::Align::Center,

                    gtk::Label {
                        set_vexpand: false,
                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_css_classes:
                            &self.status_css_classes(),
                        #[watch]
                        set_label: &self.status_label(),
                        #[watch]
                        set_visible: self.status_visible(),
                    },
                },

                add_named[Some("actions")] = &gtk::Box {
                    add_css_class:
                        "bluetooth-device-actions",
                    set_valign: gtk::Align::Center,

                    #[template]
                    GhostButton {
                        add_css_class:
                            "bluetooth-action-toggle",
                        #[watch]
                        set_sensitive:
                            self.pending.is_none(),
                        #[template_child]
                        label {
                            #[watch]
                            set_label: &if self.connected {
                                t!(
                                    "dropdown-bluetooth-disconnect"
                                )
                            } else {
                                t!(
                                    "dropdown-bluetooth-connect"
                                )
                            },
                        },
                        connect_clicked =>
                            DeviceItemInput::Clicked,
                    },

                    #[template]
                    GhostButton {
                        add_css_class:
                            "bluetooth-forget",
                        #[watch]
                        set_sensitive:
                            self.pending.is_none(),
                        #[template_child]
                        label {
                            set_label: &t!(
                                "dropdown-bluetooth-forget"
                            ),
                        },
                        connect_clicked =>
                            DeviceItemInput::ForgetClicked,
                    },
                },
            },
        }
    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        let snapshot = init.snapshot;
        let device_type = td!(snapshot.device_type_key);
        let battery_text = snapshot
            .battery
            .map(|percent| t!("dropdown-bluetooth-battery", percent = percent));
        let battery_icon = snapshot.battery.map(battery_level_icon);

        Self {
            name: snapshot.name,
            device_type,
            battery_text,
            battery_icon,
            icon: snapshot.icon,
            connected: snapshot.connected,
            paired: snapshot.paired,
            hovered: false,
            pending: None,
            category: snapshot.category,
            device_path: snapshot.device.object_path.clone(),
        }
    }

    fn update(&mut self, msg: DeviceItemInput, sender: FactorySender<Self>) {
        match msg {
            DeviceItemInput::Clicked => {
                self.handle_click(&sender);
            }

            DeviceItemInput::Hovered(hovered) => {
                self.hovered = hovered;
            }

            DeviceItemInput::ForgetClicked => {
                self.handle_forget(&sender);
            }
        }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();

        let click = gtk::GestureClick::new();
        let click_sender = sender.input_sender().clone();
        click.connect_released(move |gesture, _, _, _| {
            gesture.set_state(gtk::EventSequenceState::Claimed);
            click_sender.emit(DeviceItemInput::Clicked);
        });
        root.add_controller(click);

        if self.is_my_device() {
            let hover = gtk::EventControllerMotion::new();
            let hover_sender = sender.input_sender().clone();
            hover.connect_enter(move |_, _, _| {
                hover_sender.emit(DeviceItemInput::Hovered(true));
            });
            let leave_sender = sender.input_sender().clone();
            hover.connect_leave(move |_| {
                leave_sender.emit(DeviceItemInput::Hovered(false));
            });
            root.add_controller(hover);
        }

        widgets
    }
}
