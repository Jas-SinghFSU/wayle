use std::time::Duration;

use gtk::glib::DateTime;
use relm4::prelude::*;
use tokio_stream::wrappers::IntervalStream;
use tracing::error;
use wayle_common::{ConfigProperty, services, watch};
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
    ButtonClicked,
}

#[derive(Debug)]
pub(crate) enum ClockCmd {
    UpdateTime(String),
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
        let formatted_time = Self::get_current_time_string();

        let config_service = services::get::<ConfigService>();
        let config = config_service.config();
        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: String::from("tb-calendar-time-symbolic"),
                label: formatted_time.clone(),
                tooltip: None,
                scroll_sensitivity: 1.0,
                config: BarButtonConfig {
                    variant: config.bar.button_variant.clone(),
                    colors: BarButtonColors::default(),
                    behavior: BarButtonBehavior {
                        vertical: init.is_vertical,
                        ..Default::default()
                    },
                    theme_provider: config.styling.theme_provider.clone(),
                    border_location: config.bar.button_border_location.clone(),
                    border_width: config.bar.button_border_width.clone(),
                },
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => ClockMsg::ButtonClicked,
                _ => ClockMsg::ButtonClicked,
            });

        let interval = tokio::time::interval(Duration::from_secs(1));
        let interval_stream = IntervalStream::new(interval);

        watch!(sender, [interval_stream], |out| {
            let formatted_time = Self::get_current_time_string();
            let _ = out.send(ClockCmd::UpdateTime(formatted_time));
        });

        let model = Self { bar_button };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            ClockMsg::ButtonClicked => {
                // TODO: handle click
            }
        }
    }

    fn update_cmd(&mut self, msg: ClockCmd, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            ClockCmd::UpdateTime(time) => {
                self.bar_button.emit(BarButtonInput::SetLabel(time));
            }
        }
    }
}

impl ClockModule {
    fn get_current_time_string() -> String {
        DateTime::now_local()
            .and_then(|date_time| date_time.format("%a %b %d %I:%M:%S %p"))
            .map(|time_string| time_string.to_string())
            .inspect_err(|e| error!(error = %e, "cannot get current time"))
            .unwrap_or_else(|_| String::from("--"))
    }
}
