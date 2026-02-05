use std::sync::Arc;

use futures::stream::select;
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

pub(super) fn spawn_icon_watcher(
    sender: &FactorySender<SystrayItem>,
    item: &Arc<TrayItem>,
    cancel_token: CancellationToken,
) {
    let icon_name = item.icon_name.watch().skip(1).map(|_| ());
    let icon_pixmap = item.icon_pixmap.watch().skip(1).map(|_| ());
    let stream = select(icon_name, icon_pixmap);
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
                    sender.input(SystrayItemMsg::IconUpdated);
                }
            }
        }
    });
}
