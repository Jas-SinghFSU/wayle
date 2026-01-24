mod helpers;
mod messages;

use relm4::prelude::*;
use tracing::error;
use wayle_battery::BatteryService;
use wayle_common::{ConfigProperty, process::spawn_shell_quiet, services, watch};
use wayle_config::{
    ConfigService,
    schemas::{modules::BatteryConfig, styling::CssToken},
};
use wayle_widgets::prelude::{
    BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput, BarButtonOutput,
};

use self::helpers::{IconContext, format_label, select_icon};
pub(crate) use self::messages::{BatteryCmd, BatteryInit, BatteryMsg};

pub(crate) struct BatteryModule {
    bar_button: Controller<BarButton>,
}

#[relm4::component(pub(crate))]
impl Component for BatteryModule {
    type Init = BatteryInit;
    type Input = BatteryMsg;
    type Output = ();
    type CommandOutput = BatteryCmd;

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
        let battery_config = &config.modules.battery;

        let initial_icon = battery_config
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
                    icon_color: battery_config.icon_color.clone(),
                    label_color: battery_config.label_color.clone(),
                    icon_background: battery_config.icon_bg_color.clone(),
                    button_background: battery_config.button_bg_color.clone(),
                    border_color: battery_config.border_color.clone(),
                    auto_icon_color: CssToken::Yellow,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: battery_config.label_max_length.clone(),
                    show_icon: battery_config.icon_show.clone(),
                    show_label: battery_config.label_show.clone(),
                    show_border: battery_config.border_show.clone(),
                    visible: ConfigProperty::new(true),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => BatteryMsg::LeftClick,
                BarButtonOutput::RightClick => BatteryMsg::RightClick,
                BarButtonOutput::MiddleClick => BatteryMsg::MiddleClick,
                BarButtonOutput::ScrollUp => BatteryMsg::ScrollUp,
                BarButtonOutput::ScrollDown => BatteryMsg::ScrollDown,
            });

        Self::spawn_watchers(&sender, battery_config);

        let model = Self { bar_button };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let battery_config = &config_service.config().modules.battery;

        let cmd = match msg {
            BatteryMsg::LeftClick => battery_config.left_click.get().clone(),
            BatteryMsg::RightClick => battery_config.right_click.get().clone(),
            BatteryMsg::MiddleClick => battery_config.middle_click.get().clone(),
            BatteryMsg::ScrollUp => battery_config.scroll_up.get().clone(),
            BatteryMsg::ScrollDown => battery_config.scroll_down.get().clone(),
        };

        if !cmd.is_empty()
            && let Err(e) = spawn_shell_quiet(&cmd)
        {
            error!(error = %e, cmd = %cmd, "failed to spawn command");
        }
    }

    fn update_cmd(&mut self, msg: BatteryCmd, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            BatteryCmd::UpdateLabel(label) => {
                self.bar_button.emit(BarButtonInput::SetLabel(label));
            }
            BatteryCmd::UpdateIcon(icon) => {
                self.bar_button.emit(BarButtonInput::SetIcon(icon));
            }
        }
    }
}

impl BatteryModule {
    fn spawn_watchers(sender: &ComponentSender<Self>, config: &BatteryConfig) {
        let level_icons = config.level_icons.clone();
        let charging_icon = config.charging_icon.clone();
        let alert_icon = config.alert_icon.clone();

        let battery_service = services::get::<BatteryService>();
        let device = battery_service.device.clone();

        let percentage_stream = device.percentage.watch();
        let state_stream = device.state.watch();
        let is_present_stream = device.is_present.watch();
        let level_icons_stream = level_icons.watch();
        let charging_icon_stream = charging_icon.watch();
        let alert_icon_stream = alert_icon.watch();

        watch!(
            sender,
            [
                percentage_stream,
                state_stream,
                is_present_stream,
                level_icons_stream,
                charging_icon_stream,
                alert_icon_stream
            ],
            |out| {
                let percentage = device.percentage.get();
                let state = device.state.get();
                let is_present = device.is_present.get();

                let label = format_label(percentage, is_present);
                let _ = out.send(BatteryCmd::UpdateLabel(label));

                let level_icons_val = level_icons.get();
                let charging_icon_val = charging_icon.get();
                let alert_icon_val = alert_icon.get();
                let icon = select_icon(&IconContext {
                    percentage,
                    state,
                    is_present,
                    level_icons: &level_icons_val,
                    charging_icon: &charging_icon_val,
                    alert_icon: &alert_icon_val,
                });
                let _ = out.send(BatteryCmd::UpdateIcon(icon));
            }
        );
    }
}
