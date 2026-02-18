use relm4::prelude::*;

use crate::shell::bar::dropdowns::audio::{
    helpers,
    main_section::default_devices::volume_section::{
        VolumeSection,
        messages::{ActiveDevice, VolumeSectionKind},
    },
};

impl VolumeSection {
    pub(super) fn mute_icon(&self) -> &'static str {
        match self.kind {
            VolumeSectionKind::Output => helpers::volume_icon(self.slider.value(), self.muted),
            VolumeSectionKind::Input => helpers::input_icon(self.muted),
        }
    }

    pub(super) fn sync_from_device(&mut self, device: &ActiveDevice) {
        self.device_name = device.description();
        self.device_icon = device.trigger_icon();
        self.slider.set_value(device.volume_percentage());
        self.muted = device.muted();
    }

    pub(super) fn resume_device_watcher(&mut self, sender: &ComponentSender<Self>) {
        let Some(ref device) = self.device else {
            return;
        };
        let token = self.device_watcher.reset();
        super::watchers::spawn_device(sender, device, token);
    }

    pub(super) fn current_default(&self) -> Option<ActiveDevice> {
        match self.kind {
            VolumeSectionKind::Output => self.audio.default_output.get().map(ActiveDevice::Output),
            VolumeSectionKind::Input => self
                .audio
                .default_input
                .get()
                .filter(|device| !device.is_monitor.get())
                .map(ActiveDevice::Input),
        }
    }

    pub(super) fn check_has_device(&self) -> bool {
        match self.kind {
            VolumeSectionKind::Output => !self.audio.output_devices.get().is_empty(),
            VolumeSectionKind::Input => self
                .audio
                .input_devices
                .get()
                .iter()
                .any(|device| !device.is_monitor.get()),
        }
    }
}
