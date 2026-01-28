use std::sync::Arc;

use futures::StreamExt;
use relm4::prelude::FactorySender;
use tokio_util::sync::CancellationToken;
use wayle_systray::core::item::TrayItem;

use super::{SystrayItem, SystrayItemMsg};

pub(super) fn spawn_menu_watcher(
    sender: &FactorySender<SystrayItem>,
    item: &Arc<TrayItem>,
    cancel_token: CancellationToken,
) {
    let stream = item.menu.watch().skip(1);
    let sender = sender.clone();

    relm4::spawn_local(async move {
        futures::pin_mut!(stream);

        loop {
            tokio::select! {
                () = cancel_token.cancelled() => break,
                result = stream.next() => {
                    if result.is_none() {
                        break;
                    }
                    sender.input(SystrayItemMsg::MenuUpdated);
                }
            }
        }
    });
}
