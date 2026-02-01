mod watchers;

use relm4::prelude::*;
use tracing::warn;
use wayle_common::{ConfigProperty, process, services};
use wayle_config::{ConfigService, schemas::styling::CssToken};
use wayle_hyprland::HyprlandService;
use wayle_widgets::{
    prelude::{
        BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput,
        BarButtonOutput,
    },
    utils::force_window_resize,
};

use super::{
    helpers,
    messages::{KeyboardInputCmd, KeyboardInputInit, KeyboardInputMsg},
};

pub(crate) struct HyprlandKeyboardInput {
    bar_button: Controller<BarButton>,
    current_layout: String,
}

#[relm4::component(pub(crate))]
impl Component for HyprlandKeyboardInput {
    type Init = KeyboardInputInit;
    type Input = KeyboardInputMsg;
    type Output = ();
    type CommandOutput = KeyboardInputCmd;

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
        let keyboard_input = &config.modules.keyboard_input;

        let initial_layout = initial_layout();
        let formatted_label = helpers::format_label(&keyboard_input.format.get(), &initial_layout);

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: keyboard_input.icon_name.get().clone(),
                label: formatted_label,
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: keyboard_input.icon_color.clone(),
                    label_color: keyboard_input.label_color.clone(),
                    icon_background: keyboard_input.icon_bg_color.clone(),
                    button_background: keyboard_input.button_bg_color.clone(),
                    border_color: keyboard_input.border_color.clone(),
                    auto_icon_color: CssToken::Yellow,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: keyboard_input.label_max_length.clone(),
                    show_icon: keyboard_input.icon_show.clone(),
                    show_label: keyboard_input.label_show.clone(),
                    show_border: keyboard_input.border_show.clone(),
                    visible: ConfigProperty::new(true),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => KeyboardInputMsg::LeftClick,
                BarButtonOutput::RightClick => KeyboardInputMsg::RightClick,
                BarButtonOutput::MiddleClick => KeyboardInputMsg::MiddleClick,
                BarButtonOutput::ScrollUp => KeyboardInputMsg::ScrollUp,
                BarButtonOutput::ScrollDown => KeyboardInputMsg::ScrollDown,
            });

        watchers::spawn_watchers(&sender, keyboard_input);

        let model = Self {
            bar_button,
            current_layout: initial_layout,
        };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let keyboard_input = &config_service.config().modules.keyboard_input;

        let cmd = match msg {
            KeyboardInputMsg::LeftClick => keyboard_input.left_click.get(),
            KeyboardInputMsg::RightClick => keyboard_input.right_click.get(),
            KeyboardInputMsg::MiddleClick => keyboard_input.middle_click.get(),
            KeyboardInputMsg::ScrollUp => keyboard_input.scroll_up.get(),
            KeyboardInputMsg::ScrollDown => keyboard_input.scroll_down.get(),
        };

        process::run_if_set(&cmd);
    }

    fn update_cmd(
        &mut self,
        msg: KeyboardInputCmd,
        _sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match msg {
            KeyboardInputCmd::LayoutChanged { layout, format } => {
                self.current_layout = layout;
                self.update_label(&format, root);
            }
            KeyboardInputCmd::FormatChanged => {
                let config_service = services::get::<ConfigService>();
                let format = config_service.config().modules.keyboard_input.format.get();
                self.update_label(&format, root);
            }
            KeyboardInputCmd::UpdateIcon(icon) => {
                self.bar_button.emit(BarButtonInput::SetIcon(icon));
            }
        }
    }
}

impl HyprlandKeyboardInput {
    fn update_label(&self, format: &str, root: &gtk::Box) {
        let label = helpers::format_label(format, &self.current_layout);
        self.bar_button.emit(BarButtonInput::SetLabel(label));
        force_window_resize(root);
    }
}

fn initial_layout() -> String {
    let Some(hyprland) = services::try_get::<HyprlandService>() else {
        warn!(
            service = "HyprlandService",
            "unavailable, using fallback layout"
        );
        return String::from("?");
    };

    let runtime = tokio::runtime::Handle::current();
    match runtime.block_on(hyprland.devices()) {
        Ok(devices) => helpers::main_keyboard_layout(&devices)
            .unwrap_or("?")
            .to_string(),
        Err(err) => {
            warn!(error = %err, "cannot get keyboard devices");
            String::from("?")
        }
    }
}
