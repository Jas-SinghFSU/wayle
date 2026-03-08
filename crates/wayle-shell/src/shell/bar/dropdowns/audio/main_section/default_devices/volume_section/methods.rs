use relm4::prelude::*;
use tracing::warn;
use wayle_audio::volume::types::Volume;

use crate::{
    i18n::t,
    shell::bar::dropdowns::audio::{
        helpers,
        main_section::default_devices::volume_section::{
            VolumeSection,
            messages::{ActiveDevice, VolumeSectionCmd, VolumeSectionKind},
        },
    },
};

impl VolumeSection {
    pub(super) fn mute_icon(&self) -> &'static str {
        match self.kind {
            VolumeSectionKind::Output => helpers::volume_icon(self.slider.value(), self.muted),
            VolumeSectionKind::Input => helpers::input_icon(self.muted),
        }
    }

    pub(super) fn label(&self) -> String {
        match self.kind {
            VolumeSectionKind::Output => t!("dropdown-audio-output"),
            VolumeSectionKind::Input => t!("dropdown-audio-input"),
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

    pub(super) fn commit_volume(&self, percentage: f64, sender: &ComponentSender<Self>) {
        if let Some(ref device) = self.device {
            let channels = device.channels();
            let volume = Volume::from_percentage(percentage, channels);
            let device = device.clone();
            sender.command(|_out, _shutdown| async move {
                if let Err(err) = device.set_volume(volume).await {
                    warn!(error = %err, "failed to set volume");
                }
            });
        }
    }

    pub(super) fn toggle_mute(&self, sender: &ComponentSender<Self>) {
        if let Some(ref device) = self.device {
            let new_muted = !device.muted();
            let device = device.clone();
            sender.oneshot_command(async move {
                if let Err(err) = device.set_mute(new_muted).await {
                    warn!(error = %err, "failed to toggle mute");
                }
                VolumeSectionCmd::VolumeOrMuteChanged
            });
        }
    }
}
