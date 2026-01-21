use std::time::Duration;

use gtk::glib::DateTime;
use relm4::prelude::*;
use tokio_stream::wrappers::IntervalStream;
use tracing::error;
use wayle_common::{ConfigProperty, process, services, watch};
use wayle_config::ConfigService;
use wayle_widgets::prelude::{
    BarButton, BarButtonBehavior, BarButtonColors, BarButtonConfig, BarButtonInit, BarButtonInput,
    BarButtonOutput,
};

pub(crate) struct ClockInit {
    pub(crate) is_vertical: ConfigProperty<bool>,
}

pub(crate) struct ClockModule {
    bar_button: Controller<BarButton>,
}

#[derive(Debug)]
pub(crate) enum ClockMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum ClockCmd {
    UpdateTime(String),
    UpdateIcon(String),
    UpdateTooltip(Option<String>),
}

#[relm4::component(pub(crate))]
impl Component for ClockModule {
    type Init = ClockInit;
    type Input = ClockMsg;
    type Output = ();
    type CommandOutput = ClockCmd;

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
        let clock = &config.modules.clock;

        let format = clock.format.clone();
        let formatted_time = Self::format_time(&format.get());

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: clock.icon_name.get().clone(),
                label: formatted_time,
                tooltip: clock.tooltip.get().clone(),
                scroll_sensitivity: 1.0,
                config: BarButtonConfig {
                    variant: config.bar.button_variant.clone(),
                    colors: BarButtonColors {
                        icon_color: clock.icon_color.clone(),
                        label_color: clock.label_color.clone(),
                        icon_background: clock.icon_bg_color.clone(),
                        button_background: clock.button_bg_color.clone(),
                        border_color: clock.border_color.clone(),
                    },
                    behavior: BarButtonBehavior {
                        truncation_enabled: clock.label_truncate.clone(),
                        truncation_size: clock.label_max_length.clone(),
                        show_icon: clock.icon_show.clone(),
                        show_label: clock.label_show.clone(),
                        show_border: clock.border_show.clone(),
                        visible: ConfigProperty::new(true),
                        vertical: init.is_vertical,
                    },
                    theme_provider: config.styling.theme_provider.clone(),
                    border_location: config.bar.button_border_location.clone(),
                    border_width: config.bar.button_border_width.clone(),
                },
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => ClockMsg::LeftClick,
                BarButtonOutput::RightClick => ClockMsg::RightClick,
                BarButtonOutput::MiddleClick => ClockMsg::MiddleClick,
                BarButtonOutput::ScrollUp => ClockMsg::ScrollUp,
                BarButtonOutput::ScrollDown => ClockMsg::ScrollDown,
            });

        let interval = tokio::time::interval(Duration::from_secs(1));
        let interval_stream = IntervalStream::new(interval);

        watch!(sender, [interval_stream], |out| {
            let formatted_time = ClockModule::format_time(&format.get());
            let _ = out.send(ClockCmd::UpdateTime(formatted_time));
        });

        let icon_name = clock.icon_name.clone();
        watch!(sender, [icon_name.watch()], |out| {
            let _ = out.send(ClockCmd::UpdateIcon(icon_name.get().clone()));
        });

        let tooltip = clock.tooltip.clone();
        watch!(sender, [tooltip.watch()], |out| {
            let _ = out.send(ClockCmd::UpdateTooltip(tooltip.get().clone()));
        });

        let model = Self { bar_button };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let clock = &config_service.config().modules.clock;

        let cmd = match msg {
            ClockMsg::LeftClick => clock.left_click.get().clone(),
            ClockMsg::RightClick => clock.right_click.get().clone(),
            ClockMsg::MiddleClick => clock.middle_click.get().clone(),
            ClockMsg::ScrollUp => clock.scroll_up.get().clone(),
            ClockMsg::ScrollDown => clock.scroll_down.get().clone(),
        };

        if !cmd.is_empty()
            && let Err(e) = process::spawn_shell(&cmd)
        {
            error!(error = %e, cmd = %cmd, "failed to spawn command");
        }
    }

    fn update_cmd(&mut self, msg: ClockCmd, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            ClockCmd::UpdateTime(time) => {
                self.bar_button.emit(BarButtonInput::SetLabel(time));
            }
            ClockCmd::UpdateIcon(icon) => {
                self.bar_button.emit(BarButtonInput::SetIcon(icon));
            }
            ClockCmd::UpdateTooltip(tooltip) => {
                self.bar_button.emit(BarButtonInput::SetTooltip(tooltip));
            }
        }
    }
}

impl ClockModule {
    fn format_time(format: &str) -> String {
        DateTime::now_local()
            .and_then(|dt| dt.format(format))
            .map(|s| s.to_string())
            .inspect_err(|e| error!(error = %e, "cannot format time"))
            .unwrap_or_else(|_| String::from("--"))
    }
}
