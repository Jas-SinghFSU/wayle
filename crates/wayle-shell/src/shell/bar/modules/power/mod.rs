mod messages;
mod watchers;

use relm4::prelude::*;
use wayle_common::{ConfigProperty, process, services};
use wayle_config::{ConfigService, schemas::styling::CssToken};
use wayle_widgets::prelude::{
    BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput, BarButtonOutput,
    ColorValue,
};

pub(crate) use self::messages::{PowerCmd, PowerInit, PowerMsg};

pub(crate) struct PowerModule {
    bar_button: Controller<BarButton>,
}

#[relm4::component(pub(crate))]
impl Component for PowerModule {
    type Init = PowerInit;
    type Input = PowerMsg;
    type Output = ();
    type CommandOutput = PowerCmd;

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
        let power = &config.modules.power;

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: power.icon_name.get(),
                label: String::new(),
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: power.icon_color.clone(),
                    label_color: ConfigProperty::new(ColorValue::Token(CssToken::FgDefault)),
                    icon_background: power.icon_bg_color.clone(),
                    button_background: ConfigProperty::new(ColorValue::Token(
                        CssToken::BgSurfaceElevated,
                    )),
                    border_color: power.border_color.clone(),
                    auto_icon_color: CssToken::Red,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: ConfigProperty::new(0),
                    show_icon: ConfigProperty::new(true),
                    show_label: ConfigProperty::new(false),
                    show_border: power.border_show.clone(),
                    visible: ConfigProperty::new(true),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => PowerMsg::LeftClick,
                BarButtonOutput::RightClick => PowerMsg::RightClick,
                BarButtonOutput::MiddleClick => PowerMsg::MiddleClick,
                BarButtonOutput::ScrollUp => PowerMsg::ScrollUp,
                BarButtonOutput::ScrollDown => PowerMsg::ScrollDown,
            });

        watchers::spawn_watchers(&sender, power);

        let model = Self { bar_button };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let power = &config_service.config().modules.power;

        let cmd = match msg {
            PowerMsg::LeftClick => String::new(),
            PowerMsg::RightClick => power.right_click.get(),
            PowerMsg::MiddleClick => power.middle_click.get(),
            PowerMsg::ScrollUp => power.scroll_up.get(),
            PowerMsg::ScrollDown => power.scroll_down.get(),
        };

        process::run_if_set(&cmd);
    }

    fn update_cmd(&mut self, msg: PowerCmd, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            PowerCmd::IconConfigChanged => {
                let config_service = services::get::<ConfigService>();
                let power = &config_service.config().modules.power;
                self.bar_button
                    .emit(BarButtonInput::SetIcon(power.icon_name.get()));
            }
        }
    }
}
