mod helpers;
mod messages;
mod watchers;

use std::sync::Arc;

use relm4::prelude::*;
use wayle_audio::{AudioService, core::device::input::InputDevice};
use wayle_common::{ConfigProperty, WatcherToken, process};
use wayle_config::{
    ConfigService,
    schemas::{modules::MicrophoneConfig, styling::CssToken},
};
use wayle_widgets::prelude::{
    BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput, BarButtonOutput,
};

use self::helpers::{IconContext, select_icon};
pub(crate) use self::messages::{MicrophoneCmd, MicrophoneInit, MicrophoneMsg};

pub(crate) struct MicrophoneModule {
    bar_button: Controller<BarButton>,
    config: Arc<ConfigService>,
    active_device_watcher_token: WatcherToken,
    audio: Arc<AudioService>,
}

#[relm4::component(pub(crate))]
impl Component for MicrophoneModule {
    type Init = MicrophoneInit;
    type Input = MicrophoneMsg;
    type Output = ();
    type CommandOutput = MicrophoneCmd;

    view! {
        gtk::Box {
            #[local_ref]
            bar_button -> gtk::MenuButton {},
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let config = init.config.config();
        let mic_config = &config.modules.microphone;

        let initial_icon = mic_config.icon_muted.get();

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: initial_icon,
                label: String::new(),
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: mic_config.icon_color.clone(),
                    label_color: mic_config.label_color.clone(),
                    icon_background: mic_config.icon_bg_color.clone(),
                    button_background: mic_config.button_bg_color.clone(),
                    border_color: mic_config.border_color.clone(),
                    auto_icon_color: CssToken::Red,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: mic_config.label_max_length.clone(),
                    show_icon: mic_config.icon_show.clone(),
                    show_label: mic_config.label_show.clone(),
                    show_border: mic_config.border_show.clone(),
                    visible: ConfigProperty::new(true),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => MicrophoneMsg::LeftClick,
                BarButtonOutput::RightClick => MicrophoneMsg::RightClick,
                BarButtonOutput::MiddleClick => MicrophoneMsg::MiddleClick,
                BarButtonOutput::ScrollUp => MicrophoneMsg::ScrollUp,
                BarButtonOutput::ScrollDown => MicrophoneMsg::ScrollDown,
            });

        watchers::spawn_watchers(&sender, mic_config, &init.audio);

        let model = Self {
            bar_button,
            config: init.config,
            active_device_watcher_token: WatcherToken::new(),
            audio: init.audio,
        };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let config = &self.config.config().modules.microphone;

        let cmd = match msg {
            MicrophoneMsg::LeftClick => config.left_click.get(),
            MicrophoneMsg::RightClick => config.right_click.get(),
            MicrophoneMsg::MiddleClick => config.middle_click.get(),
            MicrophoneMsg::ScrollUp => config.scroll_up.get(),
            MicrophoneMsg::ScrollDown => config.scroll_down.get(),
        };

        process::run_if_set(&cmd);
    }

    fn update_cmd(
        &mut self,
        msg: MicrophoneCmd,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        let mic_config = &self.config.config().modules.microphone;

        match msg {
            MicrophoneCmd::DeviceChanged(device) => {
                if let Some(device) = device {
                    self.update_display(mic_config, &device);

                    let token = self.active_device_watcher_token.reset();
                    watchers::spawn_device_watchers(&sender, &device, token);
                }
            }
            MicrophoneCmd::VolumeOrMuteChanged | MicrophoneCmd::IconConfigChanged => {
                if let Some(device) = self.audio.default_input.get() {
                    self.update_display(mic_config, &device);
                }
            }
        }
    }
}

impl MicrophoneModule {
    fn update_display(&self, config: &MicrophoneConfig, device: &InputDevice) {
        let muted = device.muted.get();
        let percentage = device.volume.get().average_percentage().round() as u16;

        let label = format!("{percentage}%");
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
