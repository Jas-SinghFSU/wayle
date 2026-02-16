mod device_item;
mod messages;

use gtk::prelude::*;
use relm4::{factory::FactoryVecDeque, gtk, prelude::*};
use wayle_widgets::prelude::GhostIconButton;

use self::device_item::{DeviceOptionItem, DeviceOptionOutput};
pub(super) use self::messages::*;

pub(crate) struct DevicePicker {
    title: String,
    devices: FactoryVecDeque<DeviceOptionItem>,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for DevicePicker {
    type Init = DevicePickerInit;
    type Input = DevicePickerInput;
    type Output = DevicePickerOutput;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Box {
                add_css_class: "picker-header",

                #[template]
                GhostIconButton {
                    add_css_class: "picker-back",
                    set_icon_name: "ld-arrow-left-symbolic",
                    connect_clicked => DevicePickerInput::BackClicked,
                },

                gtk::Label {
                    add_css_class: "picker-title",
                    #[watch]
                    set_label: &model.title,
                },
            },

            gtk::ScrolledWindow {
                add_css_class: "picker-body",
                set_vexpand: true,
                set_hscrollbar_policy: gtk::PolicyType::Never,

                #[local_ref]
                device_list -> gtk::ListBox {
                    add_css_class: "audio-device-list",
                    set_selection_mode: gtk::SelectionMode::None,
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let devices = FactoryVecDeque::builder()
            .launch(gtk::ListBox::new())
            .forward(sender.input_sender(), |output| match output {
                DeviceOptionOutput::Selected(index) => DevicePickerInput::DeviceSelected(index),
            });

        let model = Self {
            title: init.title,
            devices,
        };

        let device_list = model.devices.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            DevicePickerInput::SetDevices(devices) => {
                let mut guard = self.devices.guard();
                guard.clear();
                for device in devices {
                    guard.push_back(device);
                }
            }
            DevicePickerInput::DeviceSelected(index) => {
                let _ = sender.output(DevicePickerOutput::DeviceSelected(index));
            }
            DevicePickerInput::BackClicked => {
                let _ = sender.output(DevicePickerOutput::NavigateBack);
            }
        }
    }
}
