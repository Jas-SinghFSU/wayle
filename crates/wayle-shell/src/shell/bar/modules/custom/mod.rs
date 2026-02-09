mod factory;
mod helpers;
mod messages;
mod watchers;

use gtk::prelude::*;
use relm4::prelude::*;
use tracing::debug;
use wayle_common::{ConfigProperty, WatcherToken};
use wayle_config::schemas::{
    modules::{CustomModuleDefinition, ExecutionMode},
    styling::{ColorValue, CssToken},
};
use wayle_widgets::{
    prelude::{
        BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput,
        BarButtonOutput,
    },
    utils::force_window_resize,
};

pub(crate) use self::{
    factory::Factory,
    messages::{CustomCmd, CustomInit, CustomMsg},
};

pub(crate) struct CustomModule {
    bar_button: Controller<BarButton>,
    definition: CustomModuleDefinition,
    poller_token: WatcherToken,
    watcher_token: WatcherToken,
    command_token: WatcherToken,
    scroll_debounce_token: WatcherToken,
    show_icon: ConfigProperty<bool>,
    show_label: ConfigProperty<bool>,
    show_border: ConfigProperty<bool>,
    label_max_chars: ConfigProperty<u32>,
    icon_color: ConfigProperty<ColorValue>,
    label_color: ConfigProperty<ColorValue>,
    icon_bg_color: ConfigProperty<ColorValue>,
    button_bg_color: ConfigProperty<ColorValue>,
    border_color: ConfigProperty<ColorValue>,
    dynamic_classes: Vec<String>,
    last_output: String,
}

#[relm4::component(pub(crate))]
impl Component for CustomModule {
    type Init = CustomInit;
    type Input = CustomMsg;
    type Output = ();
    type CommandOutput = CustomCmd;

    view! {
        gtk::Box {
            add_css_class: "custom",
            #[local_ref]
            bar_button -> gtk::MenuButton {},
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let definition = init.definition;

        let show_icon = ConfigProperty::new(definition.icon_show);
        let show_label = ConfigProperty::new(definition.label_show);
        let show_border = ConfigProperty::new(definition.border_show);
        let label_max_chars = ConfigProperty::new(definition.label_max_length);
        let icon_color = ConfigProperty::new(definition.icon_color.clone());
        let label_color = ConfigProperty::new(definition.label_color.clone());
        let icon_bg_color = ConfigProperty::new(definition.icon_bg_color.clone());
        let button_bg_color = ConfigProperty::new(definition.button_bg_color.clone());
        let border_color = ConfigProperty::new(definition.border_color.clone());

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: definition.icon_name.clone(),
                label: String::new(),
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: icon_color.clone(),
                    label_color: label_color.clone(),
                    icon_background: icon_bg_color.clone(),
                    button_background: button_bg_color.clone(),
                    border_color: border_color.clone(),
                    auto_icon_color: CssToken::Accent,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: label_max_chars.clone(),
                    show_icon: show_icon.clone(),
                    show_label: show_label.clone(),
                    show_border: show_border.clone(),
                    visible: ConfigProperty::new(true),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => CustomMsg::LeftClick,
                BarButtonOutput::RightClick => CustomMsg::RightClick,
                BarButtonOutput::MiddleClick => CustomMsg::MiddleClick,
                BarButtonOutput::ScrollUp => CustomMsg::ScrollUp,
                BarButtonOutput::ScrollDown => CustomMsg::ScrollDown,
            });

        let custom_modules = &init.config.config().modules.custom;
        let mut poller_token = WatcherToken::new();
        let mut watcher_token = WatcherToken::new();

        match definition.mode {
            ExecutionMode::Poll => {
                watchers::spawn_command_poller(&sender, &definition, poller_token.reset());
            }
            ExecutionMode::Watch => {
                watchers::spawn_command_watcher(&sender, &definition, watcher_token.reset());
            }
        }
        watchers::spawn_config_watcher(&sender, custom_modules, definition.id.clone());

        let model = Self {
            bar_button,
            definition,
            poller_token,
            watcher_token,
            command_token: WatcherToken::new(),
            scroll_debounce_token: WatcherToken::new(),
            show_icon,
            show_label,
            show_border,
            label_max_chars,
            icon_color,
            label_color,
            icon_bg_color,
            button_bg_color,
            border_color,
            dynamic_classes: Vec::new(),
            last_output: String::new(),
        };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        let is_scroll = matches!(msg, CustomMsg::ScrollUp | CustomMsg::ScrollDown);

        let action_cmd = match msg {
            CustomMsg::LeftClick => &self.definition.left_click,
            CustomMsg::RightClick => &self.definition.right_click,
            CustomMsg::MiddleClick => &self.definition.middle_click,
            CustomMsg::ScrollUp => &self.definition.scroll_up,
            CustomMsg::ScrollDown => &self.definition.scroll_down,
        };

        if !action_cmd.is_empty() {
            watchers::spawn_action(action_cmd);
        }

        let Some(on_action) = &self.definition.on_action else {
            return;
        };

        if is_scroll {
            let token = self.scroll_debounce_token.reset();
            watchers::spawn_scroll_debounce(&sender, token);
        } else {
            let token = self.command_token.reset();
            watchers::run_command_async(&sender, &self.definition.id, on_action.clone(), token);
        }
    }

    fn update_cmd(&mut self, msg: CustomCmd, sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
            CustomCmd::PollTrigger => {
                if let Some(command) = &self.definition.command {
                    let token = self.command_token.reset();
                    watchers::run_command_async(
                        &sender,
                        &self.definition.id,
                        command.clone(),
                        token,
                    );
                }
            }
            CustomCmd::ScrollDebounceExpired => {
                if let Some(on_action) = &self.definition.on_action {
                    let token = self.command_token.reset();
                    watchers::run_command_async(
                        &sender,
                        &self.definition.id,
                        on_action.clone(),
                        token,
                    );
                }
            }
            CustomCmd::CommandCancelled => {}
            CustomCmd::CommandOutput(output) | CustomCmd::WatchOutput(output) => {
                self.apply_output(&output, root);
                force_window_resize(root);
            }
            CustomCmd::DefinitionRemoved => {
                self.handle_definition_removed(root);
            }
            CustomCmd::DefinitionChanged(boxed_definition) => {
                self.handle_definition_changed(&sender, root, *boxed_definition);
            }
        }
    }
}

impl CustomModule {
    fn handle_definition_removed(&mut self, root: &gtk::Box) {
        debug!(
            module_id = %self.definition.id,
            "custom module definition was removed; hiding module"
        );

        self.stop_execution_watchers();
        self.cancel_inflight_commands();
        if let Some(parent) = root.parent() {
            parent.set_visible(false);
        }
        force_window_resize(root);
    }

    fn handle_definition_changed(
        &mut self,
        sender: &ComponentSender<Self>,
        root: &gtk::Box,
        new_definition: CustomModuleDefinition,
    ) {
        if self.definition == new_definition {
            return;
        }

        let needs_restart = Self::execution_settings_changed(&self.definition, &new_definition);

        self.cancel_inflight_commands();

        self.apply_visual_properties(&new_definition);
        self.definition = new_definition;

        if needs_restart {
            self.restart_execution_watchers(sender);
        }

        self.refresh_from_last_output(root);
    }

    fn execution_settings_changed(
        current: &CustomModuleDefinition,
        next: &CustomModuleDefinition,
    ) -> bool {
        current.mode != next.mode
            || current.interval_ms != next.interval_ms
            || current.command != next.command
    }

    fn apply_visual_properties(&self, definition: &CustomModuleDefinition) {
        self.show_icon.set(definition.icon_show);
        self.show_label.set(definition.label_show);
        self.show_border.set(definition.border_show);
        self.label_max_chars.set(definition.label_max_length);
        self.icon_color.set(definition.icon_color.clone());
        self.label_color.set(definition.label_color.clone());
        self.icon_bg_color.set(definition.icon_bg_color.clone());
        self.button_bg_color.set(definition.button_bg_color.clone());
        self.border_color.set(definition.border_color.clone());
    }

    fn stop_execution_watchers(&mut self) {
        self.poller_token.reset();
        self.watcher_token.reset();
    }

    fn restart_execution_watchers(&mut self, sender: &ComponentSender<Self>) {
        match self.definition.mode {
            ExecutionMode::Poll => {
                self.watcher_token.reset();
                watchers::spawn_command_poller(sender, &self.definition, self.poller_token.reset());
            }
            ExecutionMode::Watch => {
                self.poller_token.reset();
                watchers::spawn_command_watcher(
                    sender,
                    &self.definition,
                    self.watcher_token.reset(),
                );
            }
        }
    }

    fn cancel_inflight_commands(&mut self) {
        self.command_token.reset();
        self.scroll_debounce_token.reset();
    }

    fn refresh_from_last_output(&mut self, root: &gtk::Box) {
        let last_output = self.last_output.clone();
        self.apply_output(&last_output, root);
        force_window_resize(root);
    }

    fn apply_output(&mut self, output: &str, root: &gtk::Box) {
        self.last_output = output.to_string();

        let parsed = helpers::ParsedOutput::parse(output);
        let label = helpers::format_label(&self.definition, &parsed);
        let icon = helpers::resolve_icon(&self.definition, &parsed);
        let tooltip = helpers::format_tooltip(&self.definition, &parsed);
        let is_visible = !helpers::should_hide(&parsed.raw, self.definition.hide_if_empty);
        let new_classes = helpers::resolve_classes(&self.definition, &parsed);

        self.bar_button.emit(BarButtonInput::SetLabel(label));
        self.bar_button.emit(BarButtonInput::SetIcon(icon));
        self.bar_button.emit(BarButtonInput::SetTooltip(tooltip));
        if let Some(parent) = root.parent() {
            parent.set_visible(is_visible);
        }

        for old_class in &self.dynamic_classes {
            if !new_classes.contains(old_class) {
                root.remove_css_class(old_class);
            }
        }
        for new_class in &new_classes {
            if !self.dynamic_classes.contains(new_class) {
                root.add_css_class(new_class);
            }
        }
        self.dynamic_classes = new_classes;
    }
}
