use relm4::ComponentController;
use wayle_audio::core::device::input::InputDevice;
use wayle_config::schemas::modules::MicrophoneConfig;
use wayle_widgets::prelude::BarButtonInput;

use super::{
    MicrophoneModule,
    helpers::{IconContext, format_label, select_icon},
};

impl MicrophoneModule {
    pub(super) fn update_display(&self, config: &MicrophoneConfig, device: &InputDevice) {
        let muted = device.muted.get();
        let percentage = device.volume.get().average_percentage().round() as u16;

        let label = format_label(percentage);
        self.bar_button.emit(BarButtonInput::SetLabel(label));

        let icon_active = config.icon_active.get();
        let icon_muted = config.icon_muted.get();
        let icon = select_icon(&IconContext {
            muted,
            icon_active: &icon_active,
            icon_muted: &icon_muted,
        });
        self.bar_button.emit(BarButtonInput::SetIcon(icon));
    }
}
