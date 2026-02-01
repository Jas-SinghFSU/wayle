mod helpers;
mod messages;
mod watchers;

use relm4::prelude::*;
use wayle_common::{ConfigProperty, process, services};
use wayle_config::{ConfigService, schemas::styling::CssToken};
use wayle_widgets::{
    prelude::{
        BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput,
        BarButtonOutput,
    },
    utils::force_window_resize,
};

pub(crate) use self::messages::{WeatherCmd, WeatherInit, WeatherMsg};

pub(crate) struct WeatherModule {
    bar_button: Controller<BarButton>,
}

#[relm4::component(pub(crate))]
impl Component for WeatherModule {
    type Init = WeatherInit;
    type Input = WeatherMsg;
    type Output = ();
    type CommandOutput = WeatherCmd;

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
        let weather = &config.modules.weather;

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: weather.icon_name.get().clone(),
                label: String::from("--"),
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: weather.icon_color.clone(),
                    label_color: weather.label_color.clone(),
                    icon_background: weather.icon_bg_color.clone(),
                    button_background: weather.button_bg_color.clone(),
                    border_color: weather.border_color.clone(),
                    auto_icon_color: CssToken::Accent,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: weather.label_max_length.clone(),
                    show_icon: weather.icon_show.clone(),
                    show_label: weather.label_show.clone(),
                    show_border: weather.border_show.clone(),
                    visible: ConfigProperty::new(true),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => WeatherMsg::LeftClick,
                BarButtonOutput::RightClick => WeatherMsg::RightClick,
                BarButtonOutput::MiddleClick => WeatherMsg::MiddleClick,
                BarButtonOutput::ScrollUp => WeatherMsg::ScrollUp,
                BarButtonOutput::ScrollDown => WeatherMsg::ScrollDown,
            });

        watchers::spawn_watchers(&sender, weather);

        let model = Self { bar_button };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let weather = &config_service.config().modules.weather;

        let cmd = match msg {
            WeatherMsg::LeftClick => weather.left_click.get(),
            WeatherMsg::RightClick => weather.right_click.get(),
            WeatherMsg::MiddleClick => weather.middle_click.get(),
            WeatherMsg::ScrollUp => weather.scroll_up.get(),
            WeatherMsg::ScrollDown => weather.scroll_down.get(),
        };

        process::run_if_set(&cmd);
    }

    fn update_cmd(&mut self, msg: WeatherCmd, _sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
            WeatherCmd::UpdateLabel(label) => {
                self.bar_button.emit(BarButtonInput::SetLabel(label));
                force_window_resize(root);
            }
            WeatherCmd::UpdateIcon(icon) => {
                self.bar_button.emit(BarButtonInput::SetIcon(icon));
            }
        }
    }
}
