//! Bar button component with runtime-switchable visual variants.

use std::borrow::Cow;

#[allow(deprecated)]
use gtk4::prelude::StyleContextExt;
use gtk4::prelude::{OrientableExt, WidgetExt};
use relm4::{ComponentParts, ComponentSender, gtk, prelude::*};
use wayle_common::{ConfigProperty, watch};
use wayle_config::schemas::styling::CssToken;

use super::{
    shared::setup_event_controllers,
    types::{
        BarButtonBehavior, BarButtonClass, BarButtonColors, BarButtonOutput, BarButtonVariant,
        BarSettings,
    },
};
use crate::{styling::InlineStyling, utils::force_window_resize};

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
    pub(super) variant: BarButtonVariant,
    pub(super) colors: BarButtonColors,
    pub(super) behavior: BarButtonBehavior,
    pub(super) settings: BarSettings,
    pub(super) css_provider: gtk::CssProvider,
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

            #[watch]
            set_hexpand: model.settings.is_vertical.get(),

            #[wrap(Some)]
            set_child = &gtk::Box {
                add_css_class: "bar-button-content",

                #[watch]
                set_hexpand: model.settings.is_vertical.get(),

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

                    #[watch]
                    set_hexpand: model.settings.is_vertical.get(),

                    gtk::Label {
                        add_css_class: "bar-button-label",
                        set_align: gtk::Align::Center,

                        #[watch]
                        set_hexpand: model.settings.is_vertical.get(),

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
        model.spawn_style_watcher(&sender);

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
                self.reload_css();
            }
            BarButtonCmd::ConfigChanged => {
                self.reload_css();
                force_window_resize(root);
            }
        }
    }
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
        if !self.behavior.show_icon.get() {
            classes.push(BarButtonClass::LABEL_ONLY);
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
        if self.behavior.label_max_chars.get() > 0 {
            gtk::pango::EllipsizeMode::End
        } else {
            gtk::pango::EllipsizeMode::None
        }
    }

    fn max_width_chars(&self) -> i32 {
        let max = self.behavior.label_max_chars.get();
        if max > 0 { max as i32 } else { -1 }
    }

    fn is_icon_only(&self) -> bool {
        !self.behavior.show_label.get()
    }

    fn icon_should_center(&self) -> bool {
        self.is_icon_only() || self.settings.is_vertical.get()
    }

    pub(super) fn resolve_icon_color(&self, is_wayle_themed: bool) -> Cow<'static, str> {
        let color = if is_wayle_themed {
            self.colors.icon_color.get()
        } else {
            self.colors.icon_color.default().clone()
        };

        if color.is_auto() {
            let token = match self.variant {
                BarButtonVariant::Basic => self.colors.auto_icon_color,
                BarButtonVariant::BlockPrefix | BarButtonVariant::IconSquare => {
                    CssToken::FgOnAccent
                }
            };
            Cow::Borrowed(token.css_var())
        } else {
            color.to_css()
        }
    }

    fn watch_variant(variant: &ConfigProperty<BarButtonVariant>, sender: &ComponentSender<Self>) {
        let variant = variant.clone();
        watch!(sender, [variant.watch()], |out| {
            let _ = out.send(BarButtonCmd::VariantChanged(variant.get()));
        });
    }
}
