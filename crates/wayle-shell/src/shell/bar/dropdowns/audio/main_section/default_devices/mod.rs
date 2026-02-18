pub(crate) mod volume_section;

mod messages;

use gtk::prelude::*;
use relm4::{gtk, prelude::*};

pub(super) use self::messages::*;
use self::volume_section::{
    VolumeSection, VolumeSectionInit, VolumeSectionKind, VolumeSectionOutput,
};
use crate::i18n::t;

pub(crate) struct DefaultDevices {
    has_output: bool,
    has_input: bool,
    output_section: Controller<VolumeSection>,
    input_section: Controller<VolumeSection>,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for DefaultDevices {
    type Init = DefaultDevicesInit;
    type Input = DefaultDevicesInput;
    type Output = DefaultDevicesOutput;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            #[local_ref]
            output_section_widget -> gtk::Box {},

            #[local_ref]
            input_section_widget -> gtk::Box {},
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let output_section = VolumeSection::builder()
            .launch(VolumeSectionInit {
                audio: init.audio.clone(),
                kind: VolumeSectionKind::Output,
                title: t!("dropdown-audio-output"),
            })
            .forward(sender.input_sender(), DefaultDevicesInput::OutputSection);

        let input_section = VolumeSection::builder()
            .launch(VolumeSectionInit {
                audio: init.audio.clone(),
                kind: VolumeSectionKind::Input,
                title: t!("dropdown-audio-input"),
            })
            .forward(sender.input_sender(), DefaultDevicesInput::InputSection);

        let has_output = !init.audio.output_devices.get().is_empty();
        let has_input = init
            .audio
            .input_devices
            .get()
            .iter()
            .any(|device| !device.is_monitor.get());

        let model = Self {
            has_output,
            has_input,
            output_section,
            input_section,
        };

        let output_section_widget = model.output_section.widget();
        let input_section_widget = model.input_section.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            DefaultDevicesInput::OutputSection(output) => match output {
                VolumeSectionOutput::ShowDevices => {
                    let _ = sender.output(DefaultDevicesOutput::ShowOutputDevices);
                }
                VolumeSectionOutput::HasDeviceChanged(has) => {
                    self.has_output = has;
                    let _ = sender.output(DefaultDevicesOutput::HasDeviceChanged(
                        self.has_output || self.has_input,
                    ));
                }
            },
            DefaultDevicesInput::InputSection(output) => match output {
                VolumeSectionOutput::ShowDevices => {
                    let _ = sender.output(DefaultDevicesOutput::ShowInputDevices);
                }
                VolumeSectionOutput::HasDeviceChanged(has) => {
                    self.has_input = has;
                    let _ = sender.output(DefaultDevicesOutput::HasDeviceChanged(
                        self.has_output || self.has_input,
                    ));
                }
            },
        }
    }
}
