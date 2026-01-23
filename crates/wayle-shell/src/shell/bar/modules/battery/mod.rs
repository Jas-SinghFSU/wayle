use relm4::prelude::*;
use tracing::error;
use wayle_battery::{BatteryService, types::DeviceState};
use wayle_common::{ConfigProperty, process, services, watch};
use wayle_config::{ConfigService, schemas::modules::BatteryConfig};
use wayle_widgets::prelude::{
    BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput, BarButtonOutput,
    BarSettings,
};

pub(crate) struct BatteryInit {
    pub(crate) settings: BarSettings,
}

pub(crate) struct BatteryModule {
    bar_button: Controller<BarButton>,
}

#[derive(Debug)]
pub(crate) enum BatteryMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum BatteryCmd {
    UpdateLabel(String),
    UpdateIcon(String),
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
            && let Err(e) = process::spawn_shell(&cmd)
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

                let label = if is_present {
                    format!("{:.0}%", percentage)
                } else {
                    String::from("N/A")
                };
                let _ = out.send(BatteryCmd::UpdateLabel(label));

                let icon = Self::select_icon(
                    percentage,
                    state,
                    is_present,
                    &level_icons.get(),
                    &charging_icon.get(),
                    &alert_icon.get(),
                );
                let _ = out.send(BatteryCmd::UpdateIcon(icon));
            }
        );
    }

    fn select_icon(
        percentage: f64,
        state: DeviceState,
        is_present: bool,
        level_icons: &[String],
        charging_icon: &str,
        alert_icon: &str,
    ) -> String {
        if !is_present || matches!(state, DeviceState::Unknown) {
            return alert_icon.to_string();
        }

        if matches!(state, DeviceState::Charging | DeviceState::PendingCharge) {
            return charging_icon.to_string();
        }

        if level_icons.is_empty() {
            return String::new();
        }

        let index = ((percentage / 100.0) * level_icons.len() as f64)
            .floor()
            .min((level_icons.len() - 1) as f64) as usize;

        level_icons
            .get(index)
            .cloned()
            .unwrap_or_else(|| level_icons.last().cloned().unwrap_or_default())
    }
}
