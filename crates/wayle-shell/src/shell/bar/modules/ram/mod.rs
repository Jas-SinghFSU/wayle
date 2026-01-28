mod helpers;
mod messages;
mod watchers;

use relm4::prelude::*;
use wayle_common::{ConfigProperty, process, services};
use wayle_config::{ConfigService, schemas::styling::CssToken};
use wayle_sysinfo::SysinfoService;
use wayle_widgets::prelude::{
    BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput, BarButtonOutput,
};

pub(crate) use self::messages::{RamCmd, RamInit, RamMsg};

pub(crate) struct RamModule {
    bar_button: Controller<BarButton>,
}

#[relm4::component(pub(crate))]
impl Component for RamModule {
    type Init = RamInit;
    type Input = RamMsg;
    type Output = ();
    type CommandOutput = RamCmd;

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
        let ram_config = &config.modules.ram;

        let sysinfo = services::get::<SysinfoService>();
        let initial_label = helpers::format_label(&ram_config.format.get(), &sysinfo.memory.get());

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: ram_config.icon_name.get().clone(),
                label: initial_label,
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: ram_config.icon_color.clone(),
                    label_color: ram_config.label_color.clone(),
                    icon_background: ram_config.icon_bg_color.clone(),
                    button_background: ram_config.button_bg_color.clone(),
                    border_color: ram_config.border_color.clone(),
                    auto_icon_color: CssToken::Green,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: ram_config.label_max_length.clone(),
                    show_icon: ram_config.icon_show.clone(),
                    show_label: ram_config.label_show.clone(),
                    show_border: ram_config.border_show.clone(),
                    visible: ConfigProperty::new(true),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => RamMsg::LeftClick,
                BarButtonOutput::RightClick => RamMsg::RightClick,
                BarButtonOutput::MiddleClick => RamMsg::MiddleClick,
                BarButtonOutput::ScrollUp => RamMsg::ScrollUp,
                BarButtonOutput::ScrollDown => RamMsg::ScrollDown,
            });

        watchers::spawn_watchers(&sender, ram_config);

        let model = Self { bar_button };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let ram_config = &config_service.config().modules.ram;

        let cmd = match msg {
            RamMsg::LeftClick => ram_config.left_click.get(),
            RamMsg::RightClick => ram_config.right_click.get(),
            RamMsg::MiddleClick => ram_config.middle_click.get(),
            RamMsg::ScrollUp => ram_config.scroll_up.get(),
            RamMsg::ScrollDown => ram_config.scroll_down.get(),
        };

        process::run_if_set(&cmd);
    }

    fn update_cmd(&mut self, msg: RamCmd, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            RamCmd::UpdateLabel(label) => {
                self.bar_button.emit(BarButtonInput::SetLabel(label));
            }
            RamCmd::UpdateIcon(icon) => {
                self.bar_button.emit(BarButtonInput::SetIcon(icon));
            }
        }
    }
}
