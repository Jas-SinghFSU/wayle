//! Bar button variant with icon in a colored prefix block.

use std::sync::Arc;

use futures::StreamExt;
use gtk4::prelude::{OrientableExt, WidgetExt};
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, prelude::*};
use wayle_common::ConfigProperty;
use wayle_config::schemas::{
    bar::BorderLocation,
    styling::{ColorValue, CssToken, ThemeProvider},
};

use super::{
    shared::{resolve_color, setup_event_controllers},
    types::{BarButtonClass, BarButtonOutput, CommonBarButtonMsg},
};

/// Initialization data for BlockPrefixBarButton.
#[derive(Debug, Clone)]
pub struct BlockPrefixBarButtonInit {
    /// Icon name (symbolic icon).
    pub icon: String,
    /// Button label text.
    pub label: String,
    /// Optional tooltip.
    pub tooltip: Option<String>,
    /// Scroll sensitivity multiplier.
    pub scroll_sensitivity: f64,
    /// Configuration properties.
    pub config: BlockPrefixBarButtonConfig,
    /// Theme provider for determining color resolution strategy.
    pub theme_provider: ConfigProperty<ThemeProvider>,
    /// Border placement (global setting).
    pub border_location: ConfigProperty<BorderLocation>,
    /// Border width in pixels (global setting).
    pub border_width: ConfigProperty<u8>,
}

impl Default for BlockPrefixBarButtonInit {
    fn default() -> Self {
        Self {
            icon: String::new(),
            label: String::new(),
            tooltip: None,
            scroll_sensitivity: 1.0,
            config: BlockPrefixBarButtonConfig::default(),
            theme_provider: ConfigProperty::new(ThemeProvider::default()),
            border_location: ConfigProperty::new(BorderLocation::default()),
            border_width: ConfigProperty::new(1),
        }
    }
}

/// Runtime configuration for BlockPrefixBarButton.
///
/// Colors are per-module (inline CSS). Sizing is controlled globally via CSS variables.
#[derive(Clone)]
pub struct BlockPrefixBarButtonConfig {
    /// Whether to truncate label with ellipsis.
    pub truncation_enabled: Arc<ConfigProperty<bool>>,
    /// Maximum characters before truncation.
    pub truncation_size: Arc<ConfigProperty<u32>>,
    /// Whether to show the label.
    pub show_label: Arc<ConfigProperty<bool>>,
    /// Whether the button is visible.
    pub visible: Arc<ConfigProperty<bool>>,
    /// Whether orientation is vertical.
    pub vertical: Arc<ConfigProperty<bool>>,

    /// Icon color (per-module).
    pub icon_color: Arc<ConfigProperty<ColorValue>>,
    /// Label color (per-module).
    pub label_color: Arc<ConfigProperty<ColorValue>>,
    /// Icon container background (per-module).
    pub icon_background: Arc<ConfigProperty<ColorValue>>,
    /// Button background (per-module).
    pub button_background: Arc<ConfigProperty<ColorValue>>,
    /// Border color (per-module).
    pub border_color: Arc<ConfigProperty<ColorValue>>,
}

impl std::fmt::Debug for BlockPrefixBarButtonConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockPrefixBarButtonConfig")
            .field("show_label", &self.show_label.get())
            .field("visible", &self.visible.get())
            .field("vertical", &self.vertical.get())
            .finish_non_exhaustive()
    }
}

impl Default for BlockPrefixBarButtonConfig {
    fn default() -> Self {
        Self {
            truncation_enabled: Arc::new(ConfigProperty::new(false)),
            truncation_size: Arc::new(ConfigProperty::new(20)),
            show_label: Arc::new(ConfigProperty::new(true)),
            visible: Arc::new(ConfigProperty::new(true)),
            vertical: Arc::new(ConfigProperty::new(false)),
            icon_color: Arc::new(ConfigProperty::new(ColorValue::Token(CssToken::BgSurface))),
            label_color: Arc::new(ConfigProperty::new(ColorValue::Token(CssToken::Accent))),
            icon_background: Arc::new(ConfigProperty::new(ColorValue::Token(CssToken::Accent))),
            button_background: Arc::new(ConfigProperty::new(ColorValue::Token(
                CssToken::BgSurface,
            ))),
            border_color: Arc::new(ConfigProperty::new(ColorValue::Token(
                CssToken::BorderAccent,
            ))),
        }
    }
}

/// Input messages for BlockPrefixBarButton.
#[derive(Debug)]
pub enum BlockPrefixBarButtonInput {
    /// Update the icon.
    SetIcon(String),
    /// Update the label.
    SetLabel(String),
    /// Update the tooltip.
    SetTooltip(Option<String>),
    /// Config property changed.
    ConfigChanged,
}

impl From<CommonBarButtonMsg> for BlockPrefixBarButtonInput {
    fn from(msg: CommonBarButtonMsg) -> Self {
        match msg {
            CommonBarButtonMsg::SetIcon(s) => Self::SetIcon(s),
            CommonBarButtonMsg::SetLabel(s) => Self::SetLabel(s),
            CommonBarButtonMsg::SetTooltip(t) => Self::SetTooltip(t),
            CommonBarButtonMsg::ConfigChanged => Self::ConfigChanged,
        }
    }
}

/// Command output for config watchers.
#[derive(Debug)]
pub enum BlockPrefixBarButtonCmdOutput {
    ConfigChanged,
}

/// BlockPrefixBarButton component state.
pub struct BlockPrefixBarButton {
    icon: String,
    label: String,
    tooltip: Option<String>,
    config: BlockPrefixBarButtonConfig,
    theme_provider: ConfigProperty<ThemeProvider>,
    border_location: ConfigProperty<BorderLocation>,
    border_width: ConfigProperty<u8>,
}

impl BlockPrefixBarButton {
    fn root_css_classes(&self) -> Vec<&'static str> {
        let mut classes = vec![BarButtonClass::BASE, "block-prefix"];
        if !self.config.show_label.get() {
            classes.push(BarButtonClass::ICON_ONLY);
        }
        if self.config.vertical.get() {
            classes.push(BarButtonClass::VERTICAL);
        }
        if let Some(border_class) = self.border_location.get().css_class() {
            classes.push(border_class);
        }
        classes
    }
}

#[relm4::component(pub)]
impl Component for BlockPrefixBarButton {
    type Init = BlockPrefixBarButtonInit;
    type Input = BlockPrefixBarButtonInput;
    type Output = BarButtonOutput;
    type CommandOutput = BlockPrefixBarButtonCmdOutput;

    view! {
        #[root]
        gtk::MenuButton {
            set_always_show_arrow: false,
            set_cursor_from_name: Some("pointer"),

            #[watch]
            set_css_classes: &model.root_css_classes(),

            #[watch]
            set_visible: model.config.visible.get(),

            #[watch]
            set_tooltip_text: model.tooltip.as_deref(),

            #[wrap(Some)]
            #[name = "content_box"]
            set_child = &gtk::Box {
                add_css_class: "bar-button-content",

                #[watch]
                set_orientation: if model.config.vertical.get() {
                    gtk::Orientation::Vertical
                } else {
                    gtk::Orientation::Horizontal
                },

                #[watch]
                set_valign: if model.config.vertical.get() {
                    gtk::Align::Start
                } else {
                    gtk::Align::Fill
                },

                #[name = "icon_container"]
                gtk::Box {
                    add_css_class: "icon-container",

                    #[watch]
                    set_halign: if model.config.vertical.get() {
                        gtk::Align::Fill
                    } else {
                        gtk::Align::Start
                    },

                    #[watch]
                    set_valign: if model.config.vertical.get() {
                        gtk::Align::Start
                    } else {
                        gtk::Align::Fill
                    },

                    #[name = "icon"]
                    gtk::Image {
                        set_valign: gtk::Align::Center,
                        set_halign: gtk::Align::Center,
                        set_hexpand: true,

                        #[watch]
                        set_icon_name: Some(&model.icon),
                    },
                },

                #[name = "label_container"]
                gtk::Box {
                    add_css_class: "label-container",

                    #[watch]
                    set_halign: if model.config.vertical.get() {
                        gtk::Align::Fill
                    } else {
                        gtk::Align::Center
                    },

                    #[watch]
                    set_valign: if model.config.vertical.get() {
                        gtk::Align::Start
                    } else {
                        gtk::Align::Fill
                    },

                    set_hexpand: true,

                    #[watch]
                    set_visible: model.config.show_label.get(),

                    #[name = "label_widget"]
                    gtk::Label {
                        add_css_class: "bar-button-label",
                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::BaselineCenter,
                        set_hexpand: true,

                        #[watch]
                        set_label: &model.label,

                        #[watch]
                        set_ellipsize: if model.config.truncation_enabled.get() {
                            gtk::pango::EllipsizeMode::End
                        } else {
                            gtk::pango::EllipsizeMode::None
                        },

                        #[watch]
                        set_max_width_chars: if model.config.truncation_enabled.get() {
                            model.config.truncation_size.get() as i32
                        } else {
                            -1
                        },
                    },
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = BlockPrefixBarButton {
            icon: init.icon,
            label: init.label,
            tooltip: init.tooltip,
            config: init.config,
            theme_provider: init.theme_provider,
            border_location: init.border_location,
            border_width: init.border_width,
        };

        let widgets = view_output!();

        Self::apply_inline_css(&root, &model);
        setup_event_controllers(
            &root,
            sender.output_sender().clone(),
            init.scroll_sensitivity,
        );
        Self::setup_config_watchers(&model.config, &sender);
        Self::watch_model_property(model.border_location.clone(), &sender);
        Self::watch_model_property(model.border_width.clone(), &sender);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
            BlockPrefixBarButtonInput::SetIcon(icon) => self.icon = icon,
            BlockPrefixBarButtonInput::SetLabel(label) => self.label = label,
            BlockPrefixBarButtonInput::SetTooltip(tooltip) => self.tooltip = tooltip,
            BlockPrefixBarButtonInput::ConfigChanged => {
                Self::apply_inline_css(root, self);
            }
        }
    }

    fn update_cmd(
        &mut self,
        msg: Self::CommandOutput,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            BlockPrefixBarButtonCmdOutput::ConfigChanged => {
                sender.input(BlockPrefixBarButtonInput::ConfigChanged);
            }
        }
    }
}

impl BlockPrefixBarButton {
    fn apply_inline_css(root: &gtk::MenuButton, model: &Self) {
        let is_wayle_themed = matches!(model.theme_provider.get(), ThemeProvider::Wayle);

        let icon_color = resolve_color(&model.config.icon_color, is_wayle_themed);
        let label_color = resolve_color(&model.config.label_color, is_wayle_themed);
        let icon_bg = resolve_color(&model.config.icon_background, is_wayle_themed);
        let button_bg = resolve_color(&model.config.button_background, is_wayle_themed);
        let border_color = resolve_color(&model.config.border_color, is_wayle_themed);
        let border_width = model.border_width.get();

        let css = format!(
            "--bar-btn-icon-color: {}; \
             --bar-btn-label-color: {}; \
             --bar-btn-icon-bg: {}; \
             --bar-btn-bg: {}; \
             --bar-btn-border-color: {}; \
             --bar-btn-border-width: {}px;",
            icon_color, label_color, icon_bg, button_bg, border_color, border_width
        );

        root.inline_css(&css);
    }

    fn setup_config_watchers(config: &BlockPrefixBarButtonConfig, sender: &ComponentSender<Self>) {
        Self::watch_property(&config.truncation_enabled, sender);
        Self::watch_property(&config.truncation_size, sender);
        Self::watch_property(&config.show_label, sender);
        Self::watch_property(&config.visible, sender);
        Self::watch_property(&config.vertical, sender);
        Self::watch_property(&config.icon_color, sender);
        Self::watch_property(&config.label_color, sender);
        Self::watch_property(&config.icon_background, sender);
        Self::watch_property(&config.button_background, sender);
        Self::watch_property(&config.border_color, sender);
    }

    fn watch_property<T>(property: &Arc<ConfigProperty<T>>, sender: &ComponentSender<Self>)
    where
        T: Clone + Send + Sync + PartialEq + 'static,
    {
        let property = property.clone();
        sender.command(move |out, shutdown| {
            shutdown
                .register(async move {
                    let mut stream = property.watch();
                    stream.next().await;

                    while (stream.next().await).is_some() {
                        if out
                            .send(BlockPrefixBarButtonCmdOutput::ConfigChanged)
                            .is_err()
                        {
                            break;
                        }
                    }
                })
                .drop_on_shutdown()
        });
    }

    fn watch_model_property<T>(property: ConfigProperty<T>, sender: &ComponentSender<Self>)
    where
        T: Clone + Send + Sync + PartialEq + 'static,
    {
        sender.command(move |out, shutdown| {
            shutdown
                .register(async move {
                    let mut stream = property.watch();
                    stream.next().await;

                    while (stream.next().await).is_some() {
                        if out
                            .send(BlockPrefixBarButtonCmdOutput::ConfigChanged)
                            .is_err()
                        {
                            break;
                        }
                    }
                })
                .drop_on_shutdown()
        });
    }
}
