mod helpers;
mod messages;

use std::sync::Arc;

use relm4::prelude::*;
use tokio_util::sync::CancellationToken;
use tracing::error;
use wayle_audio::{AudioService, core::device::input::InputDevice};
use wayle_common::{
    ConfigProperty, WatcherToken, process::spawn_shell_quiet, services, watch, watch_cancellable,
};
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
    active_device_watcher_token: WatcherToken,
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
        let config_service = services::get::<ConfigService>();
        let config = config_service.config();
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

        Self::spawn_watchers(&sender, mic_config);

        let model = Self {
            bar_button,
            active_device_watcher_token: WatcherToken::new(),
        };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let mic_config = &config_service.config().modules.microphone;

        let cmd = match msg {
            MicrophoneMsg::LeftClick => mic_config.left_click.get().clone(),
            MicrophoneMsg::RightClick => mic_config.right_click.get().clone(),
            MicrophoneMsg::MiddleClick => mic_config.middle_click.get().clone(),
            MicrophoneMsg::ScrollUp => mic_config.scroll_up.get().clone(),
            MicrophoneMsg::ScrollDown => mic_config.scroll_down.get().clone(),
        };

        if !cmd.is_empty()
            && let Err(e) = spawn_shell_quiet(&cmd)
        {
            error!(error = %e, cmd = %cmd, "failed to spawn command");
        }
    }

    fn update_cmd(
        &mut self,
        msg: MicrophoneCmd,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        let config_service = services::get::<ConfigService>();
        let mic_config = &config_service.config().modules.microphone;

        match msg {
            MicrophoneCmd::DeviceChanged(device) => {
                if let Some(device) = device {
                    self.update_display(mic_config, &device);

                    let token = self.active_device_watcher_token.reset();
                    Self::spawn_device_watchers(&sender, &device, token);
                }
            }
            MicrophoneCmd::VolumeOrMuteChanged | MicrophoneCmd::IconConfigChanged => {
                let audio_service = services::get::<AudioService>();
                if let Some(device) = audio_service.default_input.get() {
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

    fn spawn_watchers(sender: &ComponentSender<Self>, config: &MicrophoneConfig) {
        let audio_service = services::get::<AudioService>();

        let default_input = audio_service.default_input.clone();
        watch!(sender, [default_input.watch()], |out| {
            let audio_service = services::get::<AudioService>();
            let _ = out.send(MicrophoneCmd::DeviceChanged(
                audio_service.default_input.get(),
            ));
        });

        let icon_active = config.icon_active.clone();
        let icon_muted = config.icon_muted.clone();
        watch!(sender, [icon_active.watch(), icon_muted.watch()], |out| {
            let _ = out.send(MicrophoneCmd::IconConfigChanged);
        });
    }

    fn spawn_device_watchers(
        sender: &ComponentSender<Self>,
        device: &Arc<InputDevice>,
        token: CancellationToken,
    ) {
        let volume = device.volume.clone();
        let muted = device.muted.clone();
        watch_cancellable!(sender, token, [volume.watch(), muted.watch()], |out| {
            let _ = out.send(MicrophoneCmd::VolumeOrMuteChanged);
        });
    }
}
