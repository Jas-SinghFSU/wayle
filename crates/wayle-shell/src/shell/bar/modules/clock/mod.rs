mod helpers;
mod messages;
mod watchers;

use relm4::prelude::*;
use tracing::error;
use wayle_common::{ConfigProperty, process::spawn_shell_quiet, services};
use wayle_config::{ConfigService, schemas::styling::CssToken};
use wayle_widgets::prelude::{
    BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput, BarButtonOutput,
};

pub(crate) use self::messages::{ClockCmd, ClockInit, ClockMsg};

pub(crate) struct ClockModule {
    bar_button: Controller<BarButton>,
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
        let formatted_time = helpers::format_time(&clock.format.get());

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: clock.icon_name.get().clone(),
                label: formatted_time,
                tooltip: clock.tooltip.get().clone(),
                colors: BarButtonColors {
                    icon_color: clock.icon_color.clone(),
                    label_color: clock.label_color.clone(),
                    icon_background: clock.icon_bg_color.clone(),
                    button_background: clock.button_bg_color.clone(),
                    border_color: clock.border_color.clone(),
                    auto_icon_color: CssToken::Accent,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: clock.label_max_length.clone(),
                    show_icon: clock.icon_show.clone(),
                    show_label: clock.label_show.clone(),
                    show_border: clock.border_show.clone(),
                    visible: ConfigProperty::new(true),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => ClockMsg::LeftClick,
                BarButtonOutput::RightClick => ClockMsg::RightClick,
                BarButtonOutput::MiddleClick => ClockMsg::MiddleClick,
                BarButtonOutput::ScrollUp => ClockMsg::ScrollUp,
                BarButtonOutput::ScrollDown => ClockMsg::ScrollDown,
            });

        watchers::spawn_watchers(&sender, clock);

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
            && let Err(e) = spawn_shell_quiet(&cmd)
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
