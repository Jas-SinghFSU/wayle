mod bar;
mod helpers;
pub(crate) mod services;

use std::time::Instant;

use console::style;
use gdk4::Display;
use gtk4::{CssProvider, glib};
use gtk4_layer_shell::{Layer, LayerShell};
use relm4::{gtk::prelude::*, prelude::*};
pub(crate) use services::ShellServices;
use tracing::info;

use crate::{startup::StartupTimer, watchers};

pub(crate) struct Shell {
    css_provider: CssProvider,
    bars: helpers::monitors::BarMap,
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
    LocationChanged,
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
        let bars = helpers::monitors::create_bars(&init.services);
        helpers::monitors::schedule_deferred_sync_if_needed(bars.len(), &sender);

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
            ShellCmd::LocationChanged => {
                self.recreate_bars();
            }
            ShellCmd::SyncMonitors {
                expected_count,
                attempt,
            } => {
                helpers::monitors::sync(
                    &mut self.bars,
                    &self.services,
                    expected_count,
                    attempt,
                    |expected, attempt| {
                        helpers::monitors::schedule_retry(expected, attempt, &sender);
                    },
                );
            }
        }
    }
}

impl Shell {
    fn recreate_bars(&mut self) {
        for controller in self.bars.values() {
            controller.widget().destroy();
        }
        self.bars.clear();
        self.bars = helpers::monitors::create_bars(&self.services);
        info!("Bars recreated for location change");
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
