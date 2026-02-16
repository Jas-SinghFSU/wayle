mod watchers;

use std::{rc::Rc, sync::Arc};

use gtk::prelude::*;
use relm4::prelude::*;
use tokio::runtime::Handle;
use tracing::warn;
use wayle_common::ConfigProperty;
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
    helpers::{self, IconContext},
    messages::{WindowTitleCmd, WindowTitleInit, WindowTitleMsg},
};
use crate::{
    i18n::t,
    shell::bar::dropdowns::{self, DropdownRegistry},
};

pub(crate) struct HyprlandWindowTitle {
    bar_button: Controller<BarButton>,
    config: Arc<ConfigService>,
    current_title: String,
    current_class: String,
    dropdowns: Rc<DropdownRegistry>,
}

#[relm4::component(pub(crate))]
impl Component for HyprlandWindowTitle {
    type Init = WindowTitleInit;
    type Input = WindowTitleMsg;
    type Output = ();
    type CommandOutput = WindowTitleCmd;

    view! {
        gtk::Box {
            add_css_class: "window-title",

            #[local_ref]
            bar_button -> gtk::MenuButton {},
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let config = init.config.config();
        let window_title = &config.modules.window_title;

        let (initial_title, initial_class) = initial_window(&init.hyprland);
        let formatted_label =
            helpers::format_label(&window_title.format.get(), &initial_title, &initial_class);
        let initial_icon = helpers::resolve_icon(&IconContext {
            title: &initial_title,
            class: &initial_class,
            user_mappings: &window_title.icon_mappings.get(),
            fallback: &window_title.icon_name.get(),
        });

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: initial_icon,
                label: formatted_label,
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: window_title.icon_color.clone(),
                    label_color: window_title.label_color.clone(),
                    icon_background: window_title.icon_bg_color.clone(),
                    button_background: window_title.button_bg_color.clone(),
                    border_color: window_title.border_color.clone(),
                    auto_icon_color: CssToken::Blue,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: window_title.label_max_length.clone(),
                    show_icon: window_title.icon_show.clone(),
                    show_label: window_title.label_show.clone(),
                    show_border: window_title.border_show.clone(),
                    visible: ConfigProperty::new(true),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => WindowTitleMsg::LeftClick,
                BarButtonOutput::RightClick => WindowTitleMsg::RightClick,
                BarButtonOutput::MiddleClick => WindowTitleMsg::MiddleClick,
                BarButtonOutput::ScrollUp => WindowTitleMsg::ScrollUp,
                BarButtonOutput::ScrollDown => WindowTitleMsg::ScrollDown,
            });

        watchers::spawn_watchers(&sender, window_title, &init.hyprland);

        let model = Self {
            bar_button,
            config: init.config,
            current_title: initial_title,
            current_class: initial_class,
            dropdowns: init.dropdowns,
        };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let window_title = &self.config.config().modules.window_title;

        let action = match msg {
            WindowTitleMsg::LeftClick => window_title.left_click.get(),
            WindowTitleMsg::RightClick => window_title.right_click.get(),
            WindowTitleMsg::MiddleClick => window_title.middle_click.get(),
            WindowTitleMsg::ScrollUp => window_title.scroll_up.get(),
            WindowTitleMsg::ScrollDown => window_title.scroll_down.get(),
        };

        dropdowns::dispatch_click(&action, &self.dropdowns, &self.bar_button);
    }

    fn update_cmd(
        &mut self,
        msg: WindowTitleCmd,
        _sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match msg {
            WindowTitleCmd::WindowChanged {
                title,
                class,
                format,
            } => {
                self.current_title = title;
                self.current_class = class;
                self.update_display(&format, root);
            }
            WindowTitleCmd::FormatChanged => {
                let format = self.config.config().modules.window_title.format.get();
                self.update_label(&format, root);
            }
            WindowTitleCmd::IconConfigChanged => {
                let window_title = &self.config.config().modules.window_title;
                let icon = helpers::resolve_icon(&IconContext {
                    title: &self.current_title,
                    class: &self.current_class,
                    user_mappings: &window_title.icon_mappings.get(),
                    fallback: &window_title.icon_name.get(),
                });
                self.bar_button.emit(BarButtonInput::SetIcon(icon));
            }
        }
    }
}

impl HyprlandWindowTitle {
    fn update_display(&self, format: &str, root: &gtk::Box) {
        let window_title = &self.config.config().modules.window_title;

        let label = helpers::format_label(format, &self.current_title, &self.current_class);
        let icon = helpers::resolve_icon(&IconContext {
            title: &self.current_title,
            class: &self.current_class,
            user_mappings: &window_title.icon_mappings.get(),
            fallback: &window_title.icon_name.get(),
        });

        self.bar_button.emit(BarButtonInput::SetLabel(label));
        self.bar_button.emit(BarButtonInput::SetIcon(icon));
        force_window_resize(root);
    }

    fn update_label(&self, format: &str, root: &gtk::Box) {
        let label = helpers::format_label(format, &self.current_title, &self.current_class);
        self.bar_button.emit(BarButtonInput::SetLabel(label));
        force_window_resize(root);
    }
}

fn initial_window(hyprland: &Option<Arc<HyprlandService>>) -> (String, String) {
    let fallback = || (t!("bar-window-title-empty"), t!("bar-window-title-empty"));

    let Some(hyprland) = hyprland else {
        warn!(
            service = "HyprlandService",
            "unavailable, using fallback window"
        );
        return fallback();
    };

    let runtime = Handle::current();
    match runtime.block_on(hyprland.active_window()) {
        Some(client) => (client.title.get(), client.class.get()),
        None => fallback(),
    }
}
