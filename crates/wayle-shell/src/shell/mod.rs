mod bar;
mod helpers;
pub(crate) mod services;

use std::{
    collections::{HashMap, HashSet, hash_map::Entry},
    time::{Duration, Instant},
};

use console::style;
use gdk4::Display;
use gtk4::{CssProvider, glib};
use gtk4_layer_shell::{Layer, LayerShell};
use relm4::{
    Controller,
    gtk::{gdk, prelude::*},
    prelude::*,
};
pub(crate) use services::ShellServices;
use tracing::{debug, info, warn};

use crate::{
    shell::bar::{Bar, BarInit},
    startup::StartupTimer,
    watchers,
};

/// Monitor sync retry limit before giving up.
const MAX_SYNC_RETRIES: u32 = 5;
/// Initial delay between sync retries (doubles each attempt).
const BASE_RETRY_DELAY_MS: u64 = 50;

pub(crate) struct Shell {
    css_provider: CssProvider,
    bars: HashMap<String, Controller<Bar>>,
    services: ShellServices,
}

pub(crate) struct ShellInit {
    pub(crate) timer: StartupTimer,
    pub(crate) services: ShellServices,
}

#[derive(Debug)]
pub(crate) enum ShellInput {
    ReloadCss(String),
}

#[derive(Debug)]
pub(crate) enum ShellCmd {
    CssRecompiled(String),
    SyncMonitors { expected_count: u32, attempt: u32 },
}

#[relm4::component(pub(crate))]
impl Component for Shell {
    type Init = ShellInit;
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
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        init.timer.print_gtk_overhead();
        let start = Instant::now();

        root.init_layer_shell();
        root.set_layer(Layer::Background);
        root.set_default_size(1, 1);
        root.set_visible(false);

        let display = Display::default().expect("No display");

        helpers::init_icons();
        helpers::register_app_actions();
        watchers::init(&sender, &init.services);

        let css_provider = helpers::init_css_provider(&display, &init.services.config);
        let bars = helpers::create_bars(&init.services);

        let elapsed = start.elapsed();
        eprintln!(
            "{} Shell ({}ms)",
            style("✓").green().bold(),
            elapsed.as_millis()
        );
        info!(elapsed_ms = elapsed.as_millis(), "Shell initialized");

        init.timer.finish();

        let model = Shell {
            css_provider,
            bars,
            services: init.services,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: ShellInput, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            ShellInput::ReloadCss(css) => {
                self.css_provider.load_from_string(&css);

                for bar in self.bars.values() {
                    let window = bar.widget().clone();
                    glib::idle_add_local_once(move || {
                        trigger_layer_shell_reconfigure(&window);
                    });
                }

                info!("CSS reloaded");
            }
        }
    }

    fn update_cmd(&mut self, msg: ShellCmd, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            ShellCmd::CssRecompiled(css) => {
                sender.input(ShellInput::ReloadCss(css));
            }
            ShellCmd::SyncMonitors {
                expected_count,
                attempt,
            } => {
                self.sync_monitors(expected_count, attempt, &sender);
            }
        }
    }
}

impl Shell {
    #[allow(clippy::expect_used)]
    fn sync_monitors(&mut self, expected_count: u32, attempt: u32, sender: &ComponentSender<Self>) {
        let monitors = helpers::get_current_monitors();
        let found_count = monitors.len() as u32;

        debug!(expected_count, found_count, attempt, "Syncing monitors");

        if found_count < expected_count {
            if attempt < MAX_SYNC_RETRIES {
                self.schedule_retry(expected_count, attempt, sender);
                return;
            }
            warn!(
                found_count,
                expected_count, "Monitor sync incomplete after max retries"
            );
        }

        self.update_bars(monitors);
    }

    fn schedule_retry(&self, expected_count: u32, attempt: u32, sender: &ComponentSender<Self>) {
        let delay_ms = BASE_RETRY_DELAY_MS * (1 << attempt);
        let next_attempt = attempt + 1;

        debug!(delay_ms, next_attempt, "Scheduling monitor sync retry");

        let cmd_sender = sender.command_sender().clone();
        glib::timeout_add_local_once(Duration::from_millis(delay_ms), move || {
            let _ = cmd_sender.send(ShellCmd::SyncMonitors {
                expected_count,
                attempt: next_attempt,
            });
        });
    }

    #[allow(clippy::cognitive_complexity)]
    fn update_bars(&mut self, monitors: Vec<(String, gdk::Monitor)>) {
        let current: HashSet<&str> = monitors.iter().map(|(c, _)| c.as_str()).collect();
        debug!(?current, "Updating bars");

        let stale: Vec<String> = self
            .bars
            .keys()
            .filter(|c| !current.contains(c.as_str()))
            .cloned()
            .collect();

        for connector in stale {
            if let Some(_orphan) = self.bars.remove(&connector) {
                info!(connector = %connector, "Removing bar for disconnected monitor");
            }
        }

        for (connector, monitor) in monitors {
            if let Entry::Vacant(entry) = self.bars.entry(connector) {
                info!(connector = %entry.key(), "Creating bar for monitor");
                let bar = Bar::builder()
                    .launch(BarInit {
                        monitor,
                        services: self.services.clone(),
                    })
                    .detach();
                entry.insert(bar);
            }
        }

        debug!(bar_count = self.bars.len(), "Bar sync complete");
    }
}

/// Resets a layer-shell window's cached size so GTK recalculates from content.
///
/// The exclusive zone is managed separately via a tick callback on each bar,
/// so this poke does not cause compositor flicker.
fn trigger_layer_shell_reconfigure(window: &gtk4::Window) {
    window.set_default_size(1, 1);
    window.set_default_size(0, 0);
}
