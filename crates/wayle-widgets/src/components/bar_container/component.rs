//! Bar container Relm4 component.

#[allow(deprecated)]
use gtk4::prelude::StyleContextExt;
use gtk4::prelude::{OrientableExt, WidgetExt};
use relm4::{ComponentParts, ComponentSender, gtk, prelude::*};
use wayle_common::ConfigProperty;
use wayle_config::schemas::{bar::BorderLocation, styling::ThemeProvider};

use super::{
    helpers::{compute_css_classes, compute_orientation},
    types::{BarContainerBehavior, BarContainerColors, BarContainerInit},
    watchers::spawn_orientation_watcher,
};
use crate::{styling::InlineStyling, utils::force_window_resize};

/// Input messages for BarContainer.
#[derive(Debug)]
pub enum BarContainerInput {}

/// Command outputs from async watchers.
#[derive(Debug)]
pub enum BarContainerCmd {
    /// Bar orientation changed.
    OrientationChanged(bool),
    /// Config affecting style changed.
    ConfigChanged,
}

/// Passthrough container for bar modules with custom children.
///
/// Unlike `BarButton`, this component does not handle clicks - children
/// are responsible for their own event handling.
pub struct BarContainer {
    is_vertical: bool,
    pub(super) colors: BarContainerColors,
    pub(super) behavior: BarContainerBehavior,
    pub(super) theme_provider: ConfigProperty<ThemeProvider>,
    pub(super) border_width: ConfigProperty<u8>,
    pub(super) border_location: ConfigProperty<BorderLocation>,
    pub(super) css_provider: gtk::CssProvider,
}

#[relm4::component(pub)]
impl Component for BarContainer {
    type Init = BarContainerInit;
    type Input = BarContainerInput;
    type Output = ();
    type CommandOutput = BarContainerCmd;

    view! {
        #[root]
        gtk::Box {
            #[watch]
            set_css_classes: &compute_css_classes(
                model.is_vertical,
                model.behavior.show_border.get(),
                model.border_location.get(),
            ),

            #[watch]
            set_orientation: compute_orientation(model.is_vertical),

            #[watch]
            set_hexpand: model.is_vertical,

            #[watch]
            set_vexpand: !model.is_vertical,
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let css_provider = gtk::CssProvider::new();

        let model = BarContainer {
            is_vertical: init.is_vertical.get(),
            colors: init.colors,
            behavior: init.behavior,
            theme_provider: init.theme_provider.clone(),
            border_width: init.border_width.clone(),
            border_location: init.border_location.clone(),
            css_provider,
        };

        #[allow(deprecated)]
        root.style_context()
            .add_provider(&model.css_provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
        model.reload_css();

        let widgets = view_output!();

        spawn_orientation_watcher(&init.is_vertical, &sender);
        model.spawn_style_watcher(&sender);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {}

    fn update_cmd(
        &mut self,
        msg: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match msg {
            BarContainerCmd::OrientationChanged(vertical) => {
                self.is_vertical = vertical;
            }
            BarContainerCmd::ConfigChanged => {
                self.reload_css();
                force_window_resize(root);
            }
        }
    }
}
