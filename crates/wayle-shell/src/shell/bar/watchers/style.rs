use relm4::ComponentSender;
use tokio::sync::mpsc;
use wayle_common::{SubscribeChanges, services};
use wayle_config::ConfigService;

use crate::shell::bar::{Bar, BarCmd};

pub(crate) fn spawn(sender: &ComponentSender<Bar>) {
    let config = services::get::<ConfigService>().config().clone();
    let bar = &config.bar;

    let (tx, mut rx) = mpsc::unbounded_channel();

    bar.scale.subscribe_changes(tx.clone());
    bar.inset_edge.subscribe_changes(tx.clone());
    bar.inset_ends.subscribe_changes(tx.clone());
    bar.padding.subscribe_changes(tx.clone());
    bar.padding_ends.subscribe_changes(tx.clone());
    bar.module_gap.subscribe_changes(tx.clone());
    bar.button_group_module_gap.subscribe_changes(tx.clone());
    bar.bg.subscribe_changes(tx.clone());
    bar.background_opacity.subscribe_changes(tx.clone());
    bar.border_location.subscribe_changes(tx.clone());
    bar.border_width.subscribe_changes(tx.clone());
    bar.border_color.subscribe_changes(tx.clone());
    bar.shadow.subscribe_changes(tx);

    sender.command(move |out, shutdown| async move {
        let shutdown_fut = shutdown.wait();
        tokio::pin!(shutdown_fut);

        loop {
            tokio::select! {
                () = &mut shutdown_fut => break,
                Some(()) = rx.recv() => {
                    let _ = out.send(BarCmd::StyleChanged);
                }
            }
        }
    });
}
