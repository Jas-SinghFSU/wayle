//! Bar button component with runtime-switchable visual variants.

use futures::StreamExt;
#[allow(deprecated)]
use gtk4::prelude::StyleContextExt;
use gtk4::prelude::{OrientableExt, WidgetExt};
use relm4::{ComponentParts, ComponentSender, gtk, prelude::*};
use wayle_common::ConfigProperty;
use wayle_config::schemas::styling::ThemeProvider;

use super::{
    shared::{resolve_color, setup_event_controllers},
    types::{
        BarButtonBehavior, BarButtonClass, BarButtonColors, BarButtonOutput, BarButtonVariant,
        BarSettings,
    },
};
use crate::utils::force_window_resize;

/// Initialization data for BarButton.
#[derive(Debug, Clone)]
pub struct BarButtonInit {
    /// Icon name (symbolic icon).
    pub icon: String,
    /// Button label text.
    pub label: String,
    /// Optional tooltip.
    pub tooltip: Option<String>,
    /// Module-specific color configuration.
    pub colors: BarButtonColors,
    /// Module-specific behavior configuration.
    pub behavior: BarButtonBehavior,
    /// Bar-wide settings.
    pub settings: BarSettings,
}

/// Input messages for BarButton.
#[derive(Debug)]
pub enum BarButtonInput {
    /// Update the icon.
    SetIcon(String),
    /// Update the label.
    SetLabel(String),
    /// Update the tooltip.
    SetTooltip(Option<String>),
    /// Config property changed.
    ConfigChanged,
}

/// Command outputs from async watchers.
#[derive(Debug)]
pub enum BarButtonCmd {
    VariantChanged(BarButtonVariant),
    ConfigChanged,
}

/// Bar button with switchable visual variants.
pub struct BarButton {
    icon: String,
    label: String,
    tooltip: Option<String>,
    variant: BarButtonVariant,
    colors: BarButtonColors,
    behavior: BarButtonBehavior,
    settings: BarSettings,
    css_provider: gtk::CssProvider,
}

impl BarButton {
    fn css_classes(&self) -> Vec<&'static str> {
        let mut classes = vec![BarButtonClass::BASE];

        classes.push(match self.variant {
            BarButtonVariant::Basic => "basic",
            BarButtonVariant::BlockPrefix => "block-prefix",
            BarButtonVariant::IconSquare => "icon-square",
        });

        if !self.behavior.show_label.get() {
            classes.push(BarButtonClass::ICON_ONLY);
        }
        if self.settings.is_vertical.get() {
            classes.push(BarButtonClass::VERTICAL);
        }
        if self.behavior.show_border.get()
            && let Some(border_class) = self.settings.border_location.get().css_class()
        {
            classes.push(border_class);
        }
        classes
    }

    fn orientation(&self) -> gtk::Orientation {
        if self.settings.is_vertical.get() {
            gtk::Orientation::Vertical
        } else {
            gtk::Orientation::Horizontal
        }
    }

    fn ellipsize(&self) -> gtk::pango::EllipsizeMode {
        if self.behavior.truncation_enabled.get() {
            gtk::pango::EllipsizeMode::End
        } else {
            gtk::pango::EllipsizeMode::None
        }
    }

    fn max_width_chars(&self) -> i32 {
        if self.behavior.truncation_enabled.get() {
            self.behavior.truncation_size.get() as i32
        } else {
            -1
        }
    }

    fn is_icon_only(&self) -> bool {
        !self.behavior.show_label.get()
    }

    fn icon_should_center(&self) -> bool {
        self.is_icon_only() || self.settings.is_vertical.get()
    }

    fn build_css(&self) -> String {
        let is_wayle = matches!(self.settings.theme_provider.get(), ThemeProvider::Wayle);

        let icon_color = resolve_color(&self.colors.icon_color, is_wayle);
        let label_color = resolve_color(&self.colors.label_color, is_wayle);
        let icon_bg = resolve_color(&self.colors.icon_background, is_wayle);
        let button_bg = resolve_color(&self.colors.button_background, is_wayle);
        let border_color = resolve_color(&self.colors.border_color, is_wayle);
        let border_width = self.settings.border_width.get();

        format!(
            "* {{ \
             --bar-btn-icon-color: {}; \
             --bar-btn-label-color: {}; \
             --bar-btn-icon-bg: {}; \
             --bar-btn-bg: {}; \
             --bar-btn-border-color: {}; \
             --bar-btn-border-width: {}px; \
             }}",
            icon_color, label_color, icon_bg, button_bg, border_color, border_width
        )
    }

    fn reload_css(&self) {
        self.css_provider.load_from_string(&self.build_css());
    }
}

#[relm4::component(pub)]
impl Component for BarButton {
    type Init = BarButtonInit;
    type Input = BarButtonInput;
    type Output = BarButtonOutput;
    type CommandOutput = BarButtonCmd;

    view! {
        #[root]
        gtk::MenuButton {
            set_always_show_arrow: false,
            set_cursor_from_name: Some("pointer"),

            #[watch]
            set_css_classes: &model.css_classes(),

            #[watch]
            set_visible: model.behavior.visible.get(),

            #[watch]
            set_tooltip_text: model.tooltip.as_deref(),

            #[wrap(Some)]
            set_child = &gtk::Box {
                add_css_class: "bar-button-content",

                #[watch]
                set_orientation: model.orientation(),

                gtk::Box {
                    add_css_class: "icon-container",

                    #[watch]
                    set_visible: model.behavior.show_icon.get(),

                    #[watch]
                    set_hexpand: model.icon_should_center(),

                    gtk::Image {
                        set_halign: gtk::Align::Center,

                        #[watch]
                        set_hexpand: model.icon_should_center(),

                        #[watch]
                        set_icon_name: Some(&model.icon),
                    },
                },

                gtk::Box {
                    add_css_class: "label-container",

                    #[watch]
                    set_visible: model.behavior.show_label.get(),

                    gtk::Label {
                        add_css_class: "bar-button-label",
                        set_valign: gtk::Align::Center,

                        #[watch]
                        set_label: &model.label,

                        #[watch]
                        set_ellipsize: model.ellipsize(),

                        #[watch]
                        set_max_width_chars: model.max_width_chars(),
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
        let css_provider = gtk::CssProvider::new();
        let scroll_sensitivity = init.settings.scroll_sensitivity;

        let model = BarButton {
            icon: init.icon,
            label: init.label,
            tooltip: init.tooltip,
            variant: init.settings.variant.get(),
            colors: init.colors,
            behavior: init.behavior,
            settings: init.settings,
            css_provider,
        };

        #[allow(deprecated)]
        root.style_context()
            .add_provider(&model.css_provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
        model.reload_css();

        let widgets = view_output!();

        setup_event_controllers(&root, sender.output_sender().clone(), scroll_sensitivity);
        Self::watch_variant(&model.settings.variant, &sender);
        Self::watch_config(&model, &sender);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            BarButtonInput::SetIcon(icon) => self.icon = icon,
            BarButtonInput::SetLabel(label) => self.label = label,
            BarButtonInput::SetTooltip(tooltip) => self.tooltip = tooltip,
            BarButtonInput::ConfigChanged => {}
        }
    }

    fn update_cmd(
        &mut self,
        msg: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match msg {
            BarButtonCmd::VariantChanged(variant) => {
                self.variant = variant;
            }
            BarButtonCmd::ConfigChanged => {
                self.reload_css();
                force_window_resize(root);
            }
        }
    }
}

impl BarButton {
    fn watch_variant(variant: &ConfigProperty<BarButtonVariant>, sender: &ComponentSender<Self>) {
        let variant = variant.clone();
        sender.command(move |out, shutdown| {
            shutdown
                .register(async move {
                    let mut stream = variant.watch();
                    stream.next().await;

                    while let Some(value) = stream.next().await {
                        if out.send(BarButtonCmd::VariantChanged(value)).is_err() {
                            break;
                        }
                    }
                })
                .drop_on_shutdown()
        });
    }

    fn watch_config(model: &BarButton, sender: &ComponentSender<Self>) {
        Self::watch_property(&model.behavior.show_icon, sender);
        Self::watch_property(&model.behavior.show_label, sender);
        Self::watch_property(&model.behavior.show_border, sender);
        Self::watch_property(&model.behavior.visible, sender);
        Self::watch_property(&model.behavior.truncation_enabled, sender);
        Self::watch_property(&model.behavior.truncation_size, sender);
        Self::watch_property(&model.colors.icon_color, sender);
        Self::watch_property(&model.colors.label_color, sender);
        Self::watch_property(&model.colors.icon_background, sender);
        Self::watch_property(&model.colors.button_background, sender);
        Self::watch_property(&model.colors.border_color, sender);
        Self::watch_property(&model.settings.border_location, sender);
        Self::watch_property(&model.settings.border_width, sender);
        Self::watch_property(&model.settings.theme_provider, sender);
        Self::watch_property(&model.settings.is_vertical, sender);
    }

    fn watch_property<T>(property: &ConfigProperty<T>, sender: &ComponentSender<Self>)
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
                        if out.send(BarButtonCmd::ConfigChanged).is_err() {
                            break;
                        }
                    }
                })
                .drop_on_shutdown()
        });
    }
}
