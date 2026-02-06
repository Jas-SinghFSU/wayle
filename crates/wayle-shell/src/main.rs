//! Wayle desktop shell - a GTK4/Relm4 status bar for Wayland compositors.

use std::error::Error;

use relm4::RelmApp;
use tokio::runtime::Runtime;
use tracing::info;

mod bootstrap;
mod i18n;
mod services;
mod shell;
mod startup;
mod tracing_init;
mod watchers;

use shell::{Shell, ShellInit};

fn main() -> Result<(), Box<dyn Error>> {
    tracing_init::init()?;

    let runtime = Runtime::new()?;
    let _guard = runtime.enter();

    if runtime.block_on(bootstrap::is_already_running()) {
        eprintln!("Wayle shell is already running");
        return Ok(());
    }

    let (timer, services) = runtime.block_on(bootstrap::init_services())?;

    let app = RelmApp::new("com.wayle.shell").visible_on_activate(false);
    app.run::<Shell>(ShellInit { timer, services });

    info!("Wayle shell stopped");

    runtime.shutdown_background();
    Ok(())
}
