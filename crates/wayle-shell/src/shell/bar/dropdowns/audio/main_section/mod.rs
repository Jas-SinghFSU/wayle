mod app_volumes;
pub(crate) mod default_devices;
mod messages;

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_widgets::prelude::*;

pub(super) use self::messages::*;
use self::{
    app_volumes::AppVolumes,
    default_devices::{DefaultDevices, DefaultDevicesInit, DefaultDevicesOutput},
};
use crate::i18n::t;

pub(crate) struct MainSection {
    has_any_device: bool,
    default_devices: Controller<DefaultDevices>,
    app_volumes: Controller<AppVolumes>,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for MainSection {
    type Init = MainSectionInit;
    type Input = MainSectionInput;
    type Output = MainSectionOutput;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Box {
                add_css_class: "audio-fixed",
                set_orientation: gtk::Orientation::Vertical,
                #[watch]
                set_visible: model.has_any_device,

                #[local_ref]
                default_devices_widget -> gtk::Box {},

                #[template]
                HorizontalSeparator {
                    add_css_class: "audio-separator",
                },

                gtk::Label {
                    add_css_class: "section-label",
                    set_label: &t!("dropdown-audio-app-volume"),
                    set_halign: gtk::Align::Start,
                },
            },

            gtk::Box {
                set_vexpand: true,
                #[watch]
                set_visible: model.has_any_device,

                #[local_ref]
                app_volumes_widget -> gtk::Box {},
            },

            gtk::Box {
                #[watch]
                set_visible: !model.has_any_device,
                set_vexpand: true,
                set_valign: gtk::Align::Center,

                #[template]
                EmptyState {
                    #[template_child]
                    icon {
                        set_icon_name: Some("ld-volume-x-symbolic"),
                    },
                    #[template_child]
                    title {
                        set_label: &t!("dropdown-audio-no-devices-title"),
                    },
                    #[template_child]
                    description {
                        set_label: &t!("dropdown-audio-no-devices-description"),
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
        let has_output = !init.audio.output_devices.get().is_empty();
        let has_input = init
            .audio
            .input_devices
            .get()
            .iter()
            .any(|device| !device.is_monitor.get());

        let default_devices = DefaultDevices::builder()
            .launch(DefaultDevicesInit {
                audio: init.audio.clone(),
            })
            .forward(sender.input_sender(), MainSectionInput::DefaultDevices);

        let app_volumes = AppVolumes::builder()
            .launch(app_volumes::AppVolumesInit {
                audio: init.audio,
                config: init.config,
            })
            .detach();

        let model = Self {
            has_any_device: has_output || has_input,
            default_devices,
            app_volumes,
        };

        let default_devices_widget = model.default_devices.widget();
        let app_volumes_widget = model.app_volumes.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            MainSectionInput::DefaultDevices(output) => match output {
                DefaultDevicesOutput::ShowOutputDevices => {
                    let _ = sender.output(MainSectionOutput::ShowOutputDevices);
                }
                DefaultDevicesOutput::ShowInputDevices => {
                    let _ = sender.output(MainSectionOutput::ShowInputDevices);
                }
                DefaultDevicesOutput::HasDeviceChanged(has) => {
                    self.has_any_device = has;
                }
            },
        }
    }
}
