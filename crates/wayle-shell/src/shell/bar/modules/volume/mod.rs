mod helpers;
mod messages;

use std::sync::Arc;

use relm4::prelude::*;
use tokio_util::sync::CancellationToken;
use tracing::error;
use wayle_audio::{AudioService, core::device::output::OutputDevice};
use wayle_common::{
    ConfigProperty, WatcherToken, process::spawn_shell_quiet, services, watch, watch_cancellable,
};
use wayle_config::{
    ConfigService,
    schemas::{modules::VolumeConfig, styling::CssToken},
};
use wayle_widgets::prelude::{
    BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput, BarButtonOutput,
};

use self::helpers::{IconContext, format_label, select_icon};
pub(crate) use self::messages::{VolumeCmd, VolumeInit, VolumeMsg};

pub(crate) struct VolumeModule {
    bar_button: Controller<BarButton>,
    active_device_watcher_token: WatcherToken,
}

#[relm4::component(pub(crate))]
impl Component for VolumeModule {
    type Init = VolumeInit;
    type Input = VolumeMsg;
    type Output = ();
    type CommandOutput = VolumeCmd;

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
        let volume_config = &config.modules.volume;

        let initial_icon = volume_config
            .level_icons
            .get()
            .first()
            .cloned()
            .unwrap_or_default();

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: initial_icon,
                label: String::from("--%"),
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: volume_config.icon_color.clone(),
                    label_color: volume_config.label_color.clone(),
                    icon_background: volume_config.icon_bg_color.clone(),
                    button_background: volume_config.button_bg_color.clone(),
                    border_color: volume_config.border_color.clone(),
                    auto_icon_color: CssToken::Red,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: volume_config.label_max_length.clone(),
                    show_icon: volume_config.icon_show.clone(),
                    show_label: volume_config.label_show.clone(),
                    show_border: volume_config.border_show.clone(),
                    visible: ConfigProperty::new(true),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => VolumeMsg::LeftClick,
                BarButtonOutput::RightClick => VolumeMsg::RightClick,
                BarButtonOutput::MiddleClick => VolumeMsg::MiddleClick,
                BarButtonOutput::ScrollUp => VolumeMsg::ScrollUp,
                BarButtonOutput::ScrollDown => VolumeMsg::ScrollDown,
            });

        Self::spawn_watchers(&sender, volume_config);

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
        let volume_config = &config_service.config().modules.volume;

        let cmd = match msg {
            VolumeMsg::LeftClick => volume_config.left_click.get().clone(),
            VolumeMsg::RightClick => volume_config.right_click.get().clone(),
            VolumeMsg::MiddleClick => volume_config.middle_click.get().clone(),
            VolumeMsg::ScrollUp => volume_config.scroll_up.get().clone(),
            VolumeMsg::ScrollDown => volume_config.scroll_down.get().clone(),
        };

        if !cmd.is_empty()
            && let Err(e) = spawn_shell_quiet(&cmd)
        {
            error!(error = %e, cmd = %cmd, "failed to spawn command");
        }
    }

    fn update_cmd(&mut self, msg: VolumeCmd, sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let volume_config = &config_service.config().modules.volume;

        match msg {
            VolumeCmd::DeviceChanged(device) => {
                if let Some(device) = device {
                    self.update_display(volume_config, &device);

                    let token = self.active_device_watcher_token.reset();
                    Self::spawn_device_watchers(&sender, &device, token);
                }
            }
            VolumeCmd::VolumeOrMuteChanged | VolumeCmd::IconConfigChanged => {
                let audio_service = services::get::<AudioService>();
                if let Some(device) = audio_service.default_output.get() {
                    self.update_display(volume_config, &device);
                }
            }
        }
    }
}

impl VolumeModule {
    fn update_display(&self, config: &VolumeConfig, device: &OutputDevice) {
        let percentage = device.volume.get().average_percentage().round() as u16;
        let muted = device.muted.get();

        let label = format_label(percentage);
        self.bar_button.emit(BarButtonInput::SetLabel(label));

        let icons = config.level_icons.get();
        let muted_icon_val = config.icon_muted.get();
        let icon = select_icon(&IconContext {
            percentage,
            muted,
            level_icons: &icons,
            muted_icon: &muted_icon_val,
        });
        self.bar_button.emit(BarButtonInput::SetIcon(icon));
    }

    fn spawn_watchers(sender: &ComponentSender<Self>, config: &VolumeConfig) {
        let audio_service = services::get::<AudioService>();

        let default_output = audio_service.default_output.clone();
        watch!(sender, [default_output.watch()], |out| {
            let audio_service = services::get::<AudioService>();
            let _ = out.send(VolumeCmd::DeviceChanged(audio_service.default_output.get()));
        });

        let level_icons = config.level_icons.clone();
        let muted_icon = config.icon_muted.clone();
        watch!(sender, [level_icons.watch(), muted_icon.watch()], |out| {
            let _ = out.send(VolumeCmd::IconConfigChanged);
        });
    }

    fn spawn_device_watchers(
        sender: &ComponentSender<Self>,
        device: &Arc<OutputDevice>,
        token: CancellationToken,
    ) {
        let volume = device.volume.clone();
        let muted = device.muted.clone();
        watch_cancellable!(sender, token, [volume.watch(), muted.watch()], |out| {
            let _ = out.send(VolumeCmd::VolumeOrMuteChanged);
        });
    }
}
