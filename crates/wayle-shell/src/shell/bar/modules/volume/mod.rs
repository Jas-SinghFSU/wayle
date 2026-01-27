mod helpers;
mod messages;
mod watchers;

use relm4::prelude::*;
use wayle_audio::{AudioService, core::device::output::OutputDevice};
use wayle_common::{ConfigProperty, WatcherToken, process, services};
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

        watchers::spawn_watchers(&sender, volume_config);

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
            VolumeMsg::LeftClick => volume_config.left_click.get(),
            VolumeMsg::RightClick => volume_config.right_click.get(),
            VolumeMsg::MiddleClick => volume_config.middle_click.get(),
            VolumeMsg::ScrollUp => volume_config.scroll_up.get(),
            VolumeMsg::ScrollDown => volume_config.scroll_down.get(),
        };

        process::run_if_set(&cmd);
    }

    fn update_cmd(&mut self, msg: VolumeCmd, sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let volume_config = &config_service.config().modules.volume;

        match msg {
            VolumeCmd::DeviceChanged(device) => {
                if let Some(device) = device {
                    self.update_display(volume_config, &device);

                    let token = self.active_device_watcher_token.reset();
                    watchers::spawn_device_watchers(&sender, &device, token);
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
}
