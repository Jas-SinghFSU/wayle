use gdk4::{gio::prelude::ListModelExt, prelude::DisplayExt};
use relm4::{ComponentSender, gtk::gdk};
use tracing::debug;

use crate::shell::{Shell, ShellCmd};

/// Spawns the GTK4 monitor watcher
///
/// Watches when monitors get added/removed
#[allow(clippy::expect_used)]
pub fn spawn(sender: &ComponentSender<Shell>) {
    let display = gdk::Display::default().expect("No GDK display found...");
    let monitors = display.monitors();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    monitors.connect_items_changed(move |_, _, _, _| {
        debug!("Monitors changed...");

        let _ = tx.send(());
    });

    sender.command(move |out, shutdown| async move {
        let shutdown_fut = shutdown.wait();
        tokio::pin!(shutdown_fut);
        loop {
            tokio::select! {
                _ =  &mut shutdown_fut => break,
                Some(()) = rx.recv() => {
                    let _ = out.send(ShellCmd::MonitorsChanged);
                }
            }
        }
    });
}
