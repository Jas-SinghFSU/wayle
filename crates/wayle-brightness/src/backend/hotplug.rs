use std::{os::fd::AsFd, slice, time::Duration};

use nix::poll::{PollFd, PollFlags, PollTimeout, poll};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, warn};

use super::{
    sysfs,
    types::{BrightnessEvent, EventSender},
};
use crate::Error;

const BACKLIGHT_SUBSYSTEM: &str = "backlight";
const POLL_TIMEOUT: Duration = Duration::from_secs(2);

/// Watches for backlight device add/remove via udev.
pub(crate) fn spawn(event_tx: EventSender, token: CancellationToken) -> JoinHandle<()> {
    tokio::task::spawn_blocking(move || {
        if let Err(err) = monitor_loop(&event_tx, &token) {
            warn!(error = %err, "udev backlight monitor stopped");
        }
    })
}

fn monitor_loop(event_tx: &EventSender, token: &CancellationToken) -> Result<(), Error> {
    let socket = match build_monitor_socket() {
        Some(socket) => socket,
        None => return Ok(()),
    };

    info!("udev backlight hotplug monitor started");

    let timeout_ms = POLL_TIMEOUT.as_millis() as u16;

    loop {
        if token.is_cancelled() {
            debug!("udev monitor cancelled");
            return Ok(());
        }

        let fd = socket.as_fd();
        let mut poll_fd = PollFd::new(fd, PollFlags::POLLIN);

        match poll(slice::from_mut(&mut poll_fd), PollTimeout::from(timeout_ms)) {
            Ok(0) => continue,
            Err(_) => continue,
            Ok(_) => {}
        }

        for event in socket.iter() {
            let Some(device_name) = event.sysname().to_str() else {
                continue;
            };

            handle_event(device_name, &event, event_tx);
        }
    }
}

fn handle_event(device_name: &str, event: &udev::Event, event_tx: &EventSender) {
    match event.action() {
        Some(action) if action == "add" => {
            handle_device_added(device_name, event_tx);
        }

        Some(action) if action == "remove" => {
            debug!(device = device_name, "backlight device removed (udev)");
            let _ = event_tx.send(BrightnessEvent::DeviceRemoved(device_name.to_owned()));
        }

        _ => {}
    }
}

fn build_monitor_socket() -> Option<udev::MonitorSocket> {
    let monitor = udev::MonitorBuilder::new()
        .and_then(|builder| builder.match_subsystem(BACKLIGHT_SUBSYSTEM))
        .and_then(|builder| builder.listen());

    match monitor {
        Ok(socket) => Some(socket),

        Err(err) => {
            warn!(
                error = %err,
                "cannot create udev monitor for backlight hotplug"
            );
            None
        }
    }
}

fn handle_device_added(device_name: &str, event_tx: &EventSender) {
    let device_info = match sysfs::read_device(device_name) {
        Ok(info) => info,

        Err(err) => {
            warn!(
                device = device_name,
                error = %err,
                "cannot read newly added backlight device"
            );
            return;
        }
    };

    info!(device = device_name, "backlight device added (udev)");
    let _ = event_tx.send(BrightnessEvent::DeviceAdded(device_info));
}
