use std::{path::Path, sync::Arc};

use notify::{Event, Watcher, event::EventKind};
use relm4::ComponentSender;
use tokio::sync::mpsc;
use tracing::error;

use super::{UserSessionSection, messages::UserSessionCmd};

pub(super) fn spawn_face_watcher(sender: &ComponentSender<UserSessionSection>, face_path: &Path) {
    let (tx, mut rx) = mpsc::unbounded_channel();

    let mut watcher = match notify::recommended_watcher(move |result: Result<Event, _>| {
        if let Ok(event) = result
            && matches!(
                event.kind,
                EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
            )
        {
            let _ = tx.send(());
        }
    }) {
        Ok(watcher) => watcher,
        Err(err) => {
            error!(error = %err, "cannot create .face watcher");
            return;
        }
    };

    let parent = face_path.parent().unwrap_or(face_path);
    if let Err(err) = watcher.watch(parent, notify::RecursiveMode::NonRecursive) {
        error!(error = %err, "cannot watch home directory for .face");
        return;
    }

    let watcher = Arc::new(watcher);
    let face_path = face_path.to_path_buf();

    sender.command(move |out, shutdown| async move {
        let _watcher = watcher;
        let shutdown_fut = shutdown.wait();
        tokio::pin!(shutdown_fut);

        loop {
            tokio::select! {
                () = &mut shutdown_fut => break,
                received = rx.recv() => {
                    let Some(()) = received else { break };
                    let exists = face_path.exists();
                    let _ = out.send(UserSessionCmd::FaceChanged(exists));
                }
            }
        }
    });
}
