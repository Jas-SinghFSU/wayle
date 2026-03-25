use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    os::fd::AsFd,
    slice,
    sync::Arc,
};

use nix::{
    errno::Errno,
    poll::{PollFd, PollFlags, PollTimeout, poll},
};
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};
use wayle_traits::ModelMonitoring;

use super::BacklightDevice;
use crate::{
    Error,
    backend::{
        sysfs,
        types::{BrightnessEvent, EventSender},
    },
};

const POLL_TIMEOUT_MS: u16 = 1000;

impl ModelMonitoring for BacklightDevice {
    type Error = Error;

    /// Spawns a blocking thread that watches `actual_brightness` via `poll(POLLPRI)`.
    ///
    /// inotify doesn't work on sysfs - the kernel uses `sysfs_notify()`
    /// which only triggers `POLLPRI`, not filesystem change events.
    async fn start_monitoring(self: Arc<Self>) -> Result<(), Error> {
        let Some(ref token) = self.cancellation_token else {
            return Ok(());
        };

        let Some(ref event_tx) = self.event_tx else {
            return Ok(());
        };

        let name = self.name.to_string();
        let event_tx = event_tx.clone();
        let token = token.clone();

        tokio::task::spawn_blocking(move || {
            if let Err(err) = watch_loop(&name, &event_tx, &token) {
                warn!(device = name, error = %err, "brightness watcher stopped");
            }
        });

        Ok(())
    }
}

fn watch_loop(name: &str, event_tx: &EventSender, token: &CancellationToken) -> Result<(), Error> {
    let path = sysfs::brightness_path(name);
    let path_str = path.display().to_string();

    let mut file = File::open(&path).map_err(|source| Error::WatchFailed {
        path: path_str.clone(),
        source,
    })?;

    if let Err(err) = file.read_to_string(&mut String::new()) {
        warn!(device = name, error = %err, "initial sysfs read failed");
    }

    debug!(device = name, "brightness watcher started");

    loop {
        if token.is_cancelled() {
            debug!(device = name, "brightness watcher cancelled");
            return Ok(());
        }

        let Some(poll_flags) = poll_once(&file, &path_str)? else {
            continue;
        };

        handle_poll_flags(&mut file, name, event_tx, poll_flags)?;
    }
}

fn poll_once(file: &File, path_str: &str) -> Result<Option<PollFlags>, Error> {
    let borrowed_fd = file.as_fd();
    let flags = PollFlags::POLLPRI | PollFlags::POLLERR;
    let mut poll_fd = PollFd::new(borrowed_fd, flags);

    let poll_result = poll(
        slice::from_mut(&mut poll_fd),
        PollTimeout::from(POLL_TIMEOUT_MS),
    );

    match poll_result {
        Ok(0) => Ok(None),
        Ok(_) => Ok(Some(poll_fd.revents().unwrap_or(PollFlags::empty()))),
        Err(Errno::EINTR) => Ok(None),

        Err(err) => {
            warn!(error = %err, "poll failed");

            Err(Error::WatchFailed {
                path: path_str.to_owned(),
                source: io::Error::from(err),
            })
        }
    }
}

fn handle_poll_flags(
    file: &mut File,
    name: &str,
    event_tx: &EventSender,
    poll_flags: PollFlags,
) -> Result<(), Error> {
    // sysfs sets POLLERR alongside POLLPRI for normal changes,
    // so check POLLPRI first to avoid mistaking a change for removal.
    if poll_flags.contains(PollFlags::POLLPRI) {
        if let Err(err) = file.seek(SeekFrom::Start(0)) {
            warn!(device = name, error = %err, "seek failed on brightness file");
            return Ok(());
        }

        let _ = file.read_to_string(&mut String::new());

        emit_brightness_change(name, event_tx);
        return Ok(());
    }

    if poll_flags.contains(PollFlags::POLLERR) {
        debug!(device = name, "POLLERR: device likely removed");
        let _ = event_tx.send(BrightnessEvent::DeviceRemoved(name.to_owned()));
        return Err(Error::NoDevices);
    }

    Ok(())
}

fn emit_brightness_change(name: &str, event_tx: &EventSender) {
    let info = match sysfs::read_device(name) {
        Ok(info) => info,

        Err(err) => {
            warn!(device = name, error = %err, "cannot read device state after brightness change");
            return;
        }
    };

    let _ = event_tx.send(BrightnessEvent::DeviceChanged(info));
}
