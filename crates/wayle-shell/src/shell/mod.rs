//! Root shell component that orchestrates bars and global styling.
mod bar;
mod helpers;

use std::collections::HashMap;

use gdk4::Display;
use gtk4::{CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk4_layer_shell::{Layer, LayerShell};
use relm4::{
    actions::{RelmAction, RelmActionGroup},
    gtk::{gdk, prelude::*},
    prelude::*,
};
use tracing::info;

use crate::{
    shell::{
        bar::{Bar, BarInit},
        helpers::get_current_monitors,
    },
    watchers,
};

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

/// Root shell component managing the desktop environment.
pub struct Shell {
    css_provider: CssProvider,
    bars: HashMap<String, Controller<Bar>>,
}

/// Input messages for Shell.
#[derive(Debug)]
pub enum ShellInput {
    /// Reload CSS with new compiled stylesheet.
    ReloadCss(String),
}

/// Command outputs from async operations.
#[derive(Debug)]
pub enum ShellCmd {
    /// CSS recompilation completed.
    CssRecompiled(String),
    MonitorsChanged,
}

#[relm4::component(pub)]
impl Component for Shell {
    type Init = ();
    type Input = ShellInput;
    type Output = ();
    type CommandOutput = ShellCmd;

    view! {
        #[root]
        gtk::Window {
            set_decorated: false,
        }
    }

    #[allow(clippy::expect_used)]
    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        root.init_layer_shell();
        root.set_layer(Layer::Background);
        root.set_default_size(0, 0);
        root.set_visible(false);

        let css_provider = CssProvider::new();

        gtk4::style_context_add_provider_for_display(
            &Display::default().expect("No display available"),
            &css_provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let quit_action: RelmAction<QuitAction> = RelmAction::new_stateless(|_| {
            info!("Quit action received, shutting down");
            relm4::main_application().quit();
        });

        let mut actions = RelmActionGroup::<AppActionGroup>::new();
        actions.add_action(quit_action);
        actions.register_for_main_application();

        info!("Shell initialized with CSS provider and app actions");

        info!("Initializing Wayle shell watchers...");
        watchers::init(&sender);
        info!("Watchers initialized");

        let mut bars = HashMap::new();
        let connectors = get_current_monitors();

        for connector in connectors {
            let (connector_name, monitor) = connector;
            if !bars.contains_key(&connector_name) {
                let bar = Bar::builder().launch(BarInit { monitor }).detach();
                bars.insert(connector_name.clone(), bar);
            }
        }

        let model = Shell { css_provider, bars };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: ShellInput, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            ShellInput::ReloadCss(css) => {
                self.css_provider.load_from_string(&css);
                info!("CSS reloaded");
            }
        }
    }

    #[allow(clippy::expect_used)]
    fn update_cmd(&mut self, msg: ShellCmd, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            ShellCmd::CssRecompiled(css) => {
                sender.input(ShellInput::ReloadCss(css));
            }
            ShellCmd::MonitorsChanged => {
                let display = gdk::Display::default().expect("display");
                let current_monitors: Vec<(String, gdk::Monitor)> =
                    (0..display.monitors().n_items())
                        .filter_map(|i| display.monitors().item(i))
                        .filter_map(|obj| obj.downcast::<gdk::Monitor>().ok())
                        .filter_map(|m| m.connector().map(|c| (c.to_string(), m)))
                        .collect();

                self.bars
                    .retain(|connector, _| current_monitors.iter().any(|(c, _)| c == connector));

                for connector in current_monitors {
                    let (connector_name, monitor) = connector;
                    if !self.bars.contains_key(&connector_name) {
                        let bar = Bar::builder().launch(BarInit { monitor }).detach();
                        self.bars.insert(connector_name.clone(), bar);
                    }
                }
            }
        }
    }
}
