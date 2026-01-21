//! Shell startup: CSS, icons, actions, and bar initialization.

use std::collections::HashMap;

use gdk4::Display;
use gtk4::{
    CssProvider, STYLE_PROVIDER_PRIORITY_USER, Window, glib, prelude::ApplicationExt,
    style_context_add_provider_for_display,
};
use relm4::{
    Controller,
    actions::{RelmAction, RelmActionGroup},
    main_application,
    prelude::*,
};
use tracing::{info, warn};
use wayle_common::services;
use wayle_config::ConfigService;
use wayle_icons::IconRegistry;
use wayle_styling::compile;

use super::get_current_monitors;
use crate::shell::bar::{Bar, BarInit};

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");
relm4::new_stateless_action!(InspectorAction, AppActionGroup, "inspector");

pub(crate) fn init_icons() {
    if let Err(err) = IconRegistry::new().and_then(|r| r.init()) {
        warn!(error = %err, "Icon registry init failed");
    }
}

pub(crate) fn init_css_provider(display: &Display) -> CssProvider {
    let provider = CssProvider::new();

    let config_service = services::get::<ConfigService>();
    let config = config_service.config();
    let palette = config.styling.palette();
    let theme_provider = config.styling.theme_provider.get();

    let css_result = compile(&palette, &config.general, &config.bar, theme_provider);

    match css_result {
        Ok(css) => {
            provider.load_from_string(&css);
            info!("Initial CSS loaded");
        }
        Err(err) => warn!(error = %err, "CSS load failed"),
    }

    style_context_add_provider_for_display(display, &provider, STYLE_PROVIDER_PRIORITY_USER);

    provider
}

pub(crate) fn register_app_actions() {
    let quit_action: RelmAction<QuitAction> = RelmAction::new_stateless(|_| {
        info!("Quit action received");
        glib::idle_add_local_once(|| {
            main_application().quit();
        });
    });

    let inspector_action: RelmAction<InspectorAction> = RelmAction::new_stateless(|_| {
        info!("Inspector action received");
        Window::set_interactive_debugging(true);
    });

    let mut actions = RelmActionGroup::<AppActionGroup>::new();
    actions.add_action(quit_action);
    actions.add_action(inspector_action);
    actions.register_for_main_application();
}

pub(crate) fn create_bars() -> HashMap<String, Controller<Bar>> {
    let mut bars = HashMap::new();

    for (connector, monitor) in get_current_monitors() {
        let bar = Bar::builder().launch(BarInit { monitor }).detach();
        bars.insert(connector, bar);
    }

    bars
}
