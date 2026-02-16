use std::sync::Arc;

use relm4::prelude::*;
use tracing::warn;
use wayle_audio::{
    core::device::{input::InputDevice, output::OutputDevice},
    volume::types::Volume,
};

use super::{
    AudioDropdown, AudioPage,
    device_picker::{DeviceInfo, DevicePickerInput, DevicePickerOutput},
    helpers,
    messages::AudioDropdownCmd,
    volume_section::{VolumeSection, VolumeSectionInput, VolumeSectionOutput},
    watchers,
};

pub(super) trait DeviceKind {
    const PICKER_PAGE: AudioPage;

    fn section(dropdown: &AudioDropdown) -> &Controller<VolumeSection>;

    fn commit_volume(
        dropdown: &AudioDropdown,
        percentage: f64,
        sender: &ComponentSender<AudioDropdown>,
    );
    fn toggle_mute(dropdown: &AudioDropdown, sender: &ComponentSender<AudioDropdown>);
    fn select_device(
        dropdown: &mut AudioDropdown,
        index: usize,
        sender: &ComponentSender<AudioDropdown>,
    );
    fn send_devices(dropdown: &AudioDropdown);
    fn resume_watcher(dropdown: &mut AudioDropdown, sender: &ComponentSender<AudioDropdown>);
}

impl DeviceKind for OutputDevice {
    const PICKER_PAGE: AudioPage = AudioPage::OutputDevices;

    fn section(dropdown: &AudioDropdown) -> &Controller<VolumeSection> {
        &dropdown.output_section
    }

    fn commit_volume(
        dropdown: &AudioDropdown,
        percentage: f64,
        sender: &ComponentSender<AudioDropdown>,
    ) {
        let Some(device) = dropdown.default_output.clone() else {
            return;
        };
        let channels = device.volume.get().channels();
        let volume = Volume::from_percentage(percentage, channels);
        sender.command(move |_out, _shutdown| async move {
            if let Err(err) = device.set_volume(volume).await {
                warn!(error = %err, "failed to set output volume");
            }
        });
    }

    fn toggle_mute(dropdown: &AudioDropdown, sender: &ComponentSender<AudioDropdown>) {
        let Some(ref device) = dropdown.default_output else {
            return;
        };
        let new_muted = !device.muted.get();
        let device = device.clone();
        sender.oneshot_command(async move {
            if let Err(err) = device.set_mute(new_muted).await {
                warn!(error = %err, "failed to toggle output mute");
            }
            AudioDropdownCmd::OutputVolumeOrMuteChanged
        });
    }

    fn select_device(
        dropdown: &mut AudioDropdown,
        index: usize,
        sender: &ComponentSender<AudioDropdown>,
    ) {
        if let Some(device) = dropdown.output_devices.get(index) {
            let device = device.clone();
            sender.oneshot_command(async move {
                if let Err(err) = device.set_as_default().await {
                    warn!(error = %err, "failed to set default output");
                }
                AudioDropdownCmd::DefaultOutputChanged(None)
            });
        }
    }

    fn send_devices(dropdown: &AudioDropdown) {
        let devices = dropdown
            .output_devices
            .iter()
            .map(|device| DeviceInfo {
                description: device.description.get(),
                subtitle: helpers::active_port_description(
                    &device.active_port.get(),
                    &device.ports.get(),
                ),
                icon: helpers::output_device_icon(
                    &device.name.get(),
                    &device.description.get(),
                    &device.properties.get(),
                ),
                is_active: dropdown
                    .default_output
                    .as_ref()
                    .is_some_and(|default_device| default_device.key == device.key),
            })
            .collect();
        dropdown
            .output_picker
            .emit(DevicePickerInput::SetDevices(devices));
    }

    fn resume_watcher(dropdown: &mut AudioDropdown, sender: &ComponentSender<AudioDropdown>) {
        if let Some(ref device) = dropdown.default_output {
            let token = dropdown.output_watcher.reset();
            watchers::spawn_output_device(sender, device, token);
        }
    }
}

impl DeviceKind for InputDevice {
    const PICKER_PAGE: AudioPage = AudioPage::InputDevices;

    fn section(dropdown: &AudioDropdown) -> &Controller<VolumeSection> {
        &dropdown.input_section
    }

    fn commit_volume(
        dropdown: &AudioDropdown,
        percentage: f64,
        sender: &ComponentSender<AudioDropdown>,
    ) {
        let Some(device) = dropdown.default_input.clone() else {
            return;
        };
        let channels = device.volume.get().channels();
        let volume = Volume::from_percentage(percentage, channels);
        sender.command(move |_out, _shutdown| async move {
            if let Err(err) = device.set_volume(volume).await {
                warn!(error = %err, "failed to set input volume");
            }
        });
    }

    fn toggle_mute(dropdown: &AudioDropdown, sender: &ComponentSender<AudioDropdown>) {
        let Some(ref device) = dropdown.default_input else {
            return;
        };
        let new_muted = !device.muted.get();
        let device = device.clone();
        sender.oneshot_command(async move {
            if let Err(err) = device.set_mute(new_muted).await {
                warn!(error = %err, "failed to toggle input mute");
            }
            AudioDropdownCmd::InputVolumeOrMuteChanged
        });
    }

    fn select_device(
        dropdown: &mut AudioDropdown,
        index: usize,
        sender: &ComponentSender<AudioDropdown>,
    ) {
        let real_devices: Vec<_> = dropdown
            .input_devices
            .iter()
            .filter(|device| !device.is_monitor.get())
            .collect();
        if let Some(device) = real_devices.get(index) {
            let device = (*device).clone();
            sender.oneshot_command(async move {
                if let Err(err) = device.set_as_default().await {
                    warn!(error = %err, "failed to set default input");
                }
                AudioDropdownCmd::DefaultInputChanged(None)
            });
        }
    }

    fn send_devices(dropdown: &AudioDropdown) {
        let devices = dropdown
            .input_devices
            .iter()
            .filter(|device| !device.is_monitor.get())
            .map(|device| DeviceInfo {
                description: device.description.get(),
                subtitle: helpers::active_port_description(
                    &device.active_port.get(),
                    &device.ports.get(),
                ),
                icon: helpers::input_device_icon(
                    &device.name.get(),
                    &device.description.get(),
                    &device.properties.get(),
                ),
                is_active: dropdown
                    .default_input
                    .as_ref()
                    .is_some_and(|default_device| default_device.key == device.key),
            })
            .collect();
        dropdown
            .input_picker
            .emit(DevicePickerInput::SetDevices(devices));
    }

    fn resume_watcher(dropdown: &mut AudioDropdown, sender: &ComponentSender<AudioDropdown>) {
        if let Some(ref device) = dropdown.default_input {
            let token = dropdown.input_watcher.reset();
            watchers::spawn_input_device(sender, device, token);
        }
    }
}

impl AudioDropdown {
    pub(super) fn handle_volume_output<DeviceType: DeviceKind>(
        &mut self,
        output: VolumeSectionOutput,
        sender: &ComponentSender<Self>,
    ) {
        match output {
            VolumeSectionOutput::VolumeChanged(percentage) => {
                DeviceType::commit_volume(self, percentage, sender);
            }
            VolumeSectionOutput::ToggleMute => {
                DeviceType::toggle_mute(self, sender);
            }
            VolumeSectionOutput::ShowDevices => {
                self.active_page = DeviceType::PICKER_PAGE;
            }
        }
    }

    pub(super) fn handle_picker_output<DeviceType: DeviceKind>(
        &mut self,
        output: DevicePickerOutput,
        sender: &ComponentSender<Self>,
    ) {
        match output {
            DevicePickerOutput::NavigateBack => {
                self.active_page = AudioPage::Main;
            }
            DevicePickerOutput::DeviceSelected(index) => {
                self.active_page = AudioPage::Main;
                DeviceType::select_device(self, index, sender);
            }
        }
    }

    pub(super) fn handle_default_output_changed(
        &mut self,
        device: Option<Arc<OutputDevice>>,
        sender: &ComponentSender<Self>,
    ) {
        let device = device.or_else(|| self.audio.default_output.get());
        self.default_output = device;
        self.sync_section_device::<OutputDevice>();

        if self.default_output.is_some() {
            OutputDevice::resume_watcher(self, sender);
        }
        OutputDevice::send_devices(self);
    }

    pub(super) fn handle_default_input_changed(
        &mut self,
        device: Option<Arc<InputDevice>>,
        sender: &ComponentSender<Self>,
    ) {
        let device = device.or_else(|| self.audio.default_input.get());
        self.default_input = device;
        self.sync_section_device::<InputDevice>();

        if self.default_input.is_some() {
            InputDevice::resume_watcher(self, sender);
        }
        InputDevice::send_devices(self);
    }

    fn sync_section_device<DeviceType: DeviceKind>(&self) {
        let (name, icon, volume, muted) = if DeviceType::PICKER_PAGE == AudioPage::OutputDevices {
            let (device_volume, device_muted) = self
                .default_output
                .as_ref()
                .map(|device| (device.volume.get().average_percentage(), device.muted.get()))
                .unwrap_or((0.0, false));
            (
                self.default_output
                    .as_ref()
                    .map(|device| device.description.get())
                    .unwrap_or_default(),
                helpers::output_trigger_icon(&self.default_output),
                device_volume,
                device_muted,
            )
        } else {
            let (device_volume, device_muted) = self
                .default_input
                .as_ref()
                .map(|device| (device.volume.get().average_percentage(), device.muted.get()))
                .unwrap_or((0.0, false));
            (
                self.default_input
                    .as_ref()
                    .map(|device| device.description.get())
                    .unwrap_or_default(),
                helpers::input_trigger_icon(&self.default_input),
                device_volume,
                device_muted,
            )
        };

        DeviceType::section(self).emit(VolumeSectionInput::SetDevice {
            name,
            icon,
            volume,
            muted,
        });
    }
}
