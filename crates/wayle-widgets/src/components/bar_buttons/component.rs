//! Bar button compositor that manages variant switching.

use gtk4::prelude::Cast;
use relm4::{ComponentParts, ComponentSender, Controller, gtk, prelude::*};
use wayle_common::ConfigProperty;
use wayle_config::schemas::{bar::BorderLocation, styling::ThemeProvider};

use super::{
    basic::{BasicBarButton, BasicBarButtonConfig, BasicBarButtonInit},
    block_prefix::{BlockPrefixBarButton, BlockPrefixBarButtonConfig, BlockPrefixBarButtonInit},
    icon_square::{IconSquareBarButton, IconSquareBarButtonConfig, IconSquareBarButtonInit},
    types::{BarButtonOutput, BarButtonVariant, CommonBarButtonMsg},
};

/// Initialization data for the BarButton compositor.
#[derive(Debug, Clone)]
pub struct BarButtonInit {
    /// Icon name (symbolic icon).
    pub icon: String,
    /// Button label text.
    pub label: String,
    /// Optional tooltip.
    pub tooltip: Option<String>,
    /// Initial variant to display.
    pub variant: BarButtonVariant,
    /// Scroll sensitivity multiplier.
    pub scroll_sensitivity: f64,
    /// Variant-specific configuration.
    pub variant_config: BarButtonVariantConfig,
    /// Theme provider for determining color resolution strategy.
    pub theme_provider: ConfigProperty<ThemeProvider>,
    /// Border placement (global setting).
    pub border_location: ConfigProperty<BorderLocation>,
    /// Border width in pixels (global setting).
    pub border_width: ConfigProperty<u8>,
}

impl Default for BarButtonInit {
    fn default() -> Self {
        Self {
            icon: String::new(),
            label: String::new(),
            tooltip: None,
            variant: BarButtonVariant::default(),
            scroll_sensitivity: 1.0,
            variant_config: BarButtonVariantConfig::Basic(BasicBarButtonConfig::default()),
            theme_provider: ConfigProperty::new(ThemeProvider::default()),
            border_location: ConfigProperty::new(BorderLocation::default()),
            border_width: ConfigProperty::new(1),
        }
    }
}

/// Configuration for a specific variant.
#[derive(Debug, Clone)]
pub enum BarButtonVariantConfig {
    /// Configuration for Basic variant.
    Basic(BasicBarButtonConfig),
    /// Configuration for BlockPrefix variant.
    BlockPrefix(BlockPrefixBarButtonConfig),
    /// Configuration for IconSquare variant.
    IconSquare(IconSquareBarButtonConfig),
}

impl Default for BarButtonVariantConfig {
    fn default() -> Self {
        Self::Basic(BasicBarButtonConfig::default())
    }
}

/// Input messages for the BarButton compositor.
#[derive(Debug)]
pub enum BarButtonInput {
    /// Update the icon.
    SetIcon(String),
    /// Update the label.
    SetLabel(String),
    /// Update the tooltip.
    SetTooltip(Option<String>),
    /// Switch to a different variant.
    SetVariant(BarButtonVariant, BarButtonVariantConfig),
    /// Forwarded output from active sub-component.
    FromVariant(BarButtonOutput),
}

enum VariantController {
    Basic(Controller<BasicBarButton>),
    BlockPrefix(Controller<BlockPrefixBarButton>),
    IconSquare(Controller<IconSquareBarButton>),
}

impl VariantController {
    fn widget(&self) -> &gtk::Widget {
        match self {
            Self::Basic(ctrl) => ctrl.widget().upcast_ref(),
            Self::BlockPrefix(ctrl) => ctrl.widget().upcast_ref(),
            Self::IconSquare(ctrl) => ctrl.widget().upcast_ref(),
        }
    }

    fn variant(&self) -> BarButtonVariant {
        match self {
            Self::Basic(_) => BarButtonVariant::Basic,
            Self::BlockPrefix(_) => BarButtonVariant::BlockPrefix,
            Self::IconSquare(_) => BarButtonVariant::IconSquare,
        }
    }
}

/// BarButton compositor state.
pub struct BarButton {
    icon: String,
    label: String,
    tooltip: Option<String>,
    scroll_sensitivity: f64,
    active: VariantController,
    theme_provider: ConfigProperty<ThemeProvider>,
    border_location: ConfigProperty<BorderLocation>,
    border_width: ConfigProperty<u8>,
}

#[relm4::component(pub)]
impl Component for BarButton {
    type Init = BarButtonInit;
    type Input = BarButtonInput;
    type Output = BarButtonOutput;
    type CommandOutput = ();

    view! {
        #[root]
        #[name = "stack"]
        gtk::Stack {
            set_transition_type: gtk::StackTransitionType::Crossfade,
            set_transition_duration: 150,
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let icon = init.icon.clone();
        let label = init.label.clone();
        let tooltip = init.tooltip.clone();
        let scroll_sensitivity = init.scroll_sensitivity;

        let active = Self::create_variant(
            init.variant,
            &init.variant_config,
            &icon,
            &label,
            tooltip.clone(),
            scroll_sensitivity,
            &sender,
            &init.theme_provider,
            &init.border_location,
            &init.border_width,
        );

        root.add_child(active.widget());
        root.set_visible_child(active.widget());

        let model = BarButton {
            icon,
            label,
            tooltip,
            scroll_sensitivity,
            active,
            theme_provider: init.theme_provider,
            border_location: init.border_location,
            border_width: init.border_width,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
            BarButtonInput::SetIcon(icon) => {
                self.icon = icon.clone();
                self.forward_to_active(CommonBarButtonMsg::SetIcon(icon));
            }
            BarButtonInput::SetLabel(label) => {
                self.label = label.clone();
                self.forward_to_active(CommonBarButtonMsg::SetLabel(label));
            }
            BarButtonInput::SetTooltip(tooltip) => {
                self.tooltip = tooltip.clone();
                self.forward_to_active(CommonBarButtonMsg::SetTooltip(tooltip));
            }
            BarButtonInput::SetVariant(variant, config) => {
                if self.active.variant() != variant {
                    self.swap_variant(variant, config, root, &sender);
                }
            }
            BarButtonInput::FromVariant(output) => {
                let _ = sender.output(output);
            }
        }
    }
}

impl BarButton {
    #[allow(clippy::too_many_arguments)]
    fn create_variant(
        variant: BarButtonVariant,
        config: &BarButtonVariantConfig,
        icon: &str,
        label: &str,
        tooltip: Option<String>,
        scroll_sensitivity: f64,
        sender: &ComponentSender<Self>,
        theme_provider: &ConfigProperty<ThemeProvider>,
        border_location: &ConfigProperty<BorderLocation>,
        border_width: &ConfigProperty<u8>,
    ) -> VariantController {
        match variant {
            BarButtonVariant::Basic => {
                let basic_config = match config {
                    BarButtonVariantConfig::Basic(c) => c.clone(),
                    _ => BasicBarButtonConfig::default(),
                };

                let ctrl = BasicBarButton::builder()
                    .launch(BasicBarButtonInit {
                        icon: icon.to_string(),
                        label: label.to_string(),
                        tooltip,
                        scroll_sensitivity,
                        config: basic_config,
                        theme_provider: theme_provider.clone(),
                        border_location: border_location.clone(),
                        border_width: border_width.clone(),
                    })
                    .forward(sender.input_sender(), BarButtonInput::FromVariant);

                VariantController::Basic(ctrl)
            }

            BarButtonVariant::BlockPrefix => {
                let block_config = match config {
                    BarButtonVariantConfig::BlockPrefix(c) => c.clone(),
                    _ => BlockPrefixBarButtonConfig::default(),
                };

                let ctrl = BlockPrefixBarButton::builder()
                    .launch(BlockPrefixBarButtonInit {
                        icon: icon.to_string(),
                        label: label.to_string(),
                        tooltip,
                        scroll_sensitivity,
                        config: block_config,
                        theme_provider: theme_provider.clone(),
                        border_location: border_location.clone(),
                        border_width: border_width.clone(),
                    })
                    .forward(sender.input_sender(), BarButtonInput::FromVariant);

                VariantController::BlockPrefix(ctrl)
            }

            BarButtonVariant::IconSquare => {
                let square_config = match config {
                    BarButtonVariantConfig::IconSquare(c) => c.clone(),
                    _ => IconSquareBarButtonConfig::default(),
                };

                let ctrl = IconSquareBarButton::builder()
                    .launch(IconSquareBarButtonInit {
                        icon: icon.to_string(),
                        label: label.to_string(),
                        tooltip,
                        scroll_sensitivity,
                        config: square_config,
                        theme_provider: theme_provider.clone(),
                        border_location: border_location.clone(),
                        border_width: border_width.clone(),
                    })
                    .forward(sender.input_sender(), BarButtonInput::FromVariant);

                VariantController::IconSquare(ctrl)
            }
        }
    }

    fn swap_variant(
        &mut self,
        variant: BarButtonVariant,
        config: BarButtonVariantConfig,
        stack: &gtk::Stack,
        sender: &ComponentSender<Self>,
    ) {
        let new_controller = Self::create_variant(
            variant,
            &config,
            &self.icon,
            &self.label,
            self.tooltip.clone(),
            self.scroll_sensitivity,
            sender,
            &self.theme_provider,
            &self.border_location,
            &self.border_width,
        );

        stack.add_child(new_controller.widget());
        stack.set_visible_child(new_controller.widget());

        let old_widget = self.active.widget().clone();
        self.active = new_controller;

        stack.remove(&old_widget);
    }

    fn forward_to_active(&self, msg: CommonBarButtonMsg) {
        match &self.active {
            VariantController::Basic(ctrl) => ctrl.emit(msg.into()),
            VariantController::BlockPrefix(ctrl) => ctrl.emit(msg.into()),
            VariantController::IconSquare(ctrl) => ctrl.emit(msg.into()),
        }
    }
}
