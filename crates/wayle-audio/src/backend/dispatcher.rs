use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

use libpulse_binding::context::Context;

use super::{
    commands::{device, server, stream},
    types::{
        DefaultDevice, DeviceStore, EventSender, ExternalCommand, InternalRefresh, StreamStore,
    },
};

pub(super) struct VolumeRateLimiter {
    last_volume_change: Mutex<Instant>,
    min_interval: Duration,
}

impl VolumeRateLimiter {
    pub fn new() -> Self {
        Self {
            last_volume_change: Mutex::new(Instant::now() - Duration::from_secs(1)),
            min_interval: Duration::from_millis(30),
        }
    }

    pub fn should_process(&self) -> bool {
        let Ok(mut last) = self.last_volume_change.lock() else {
            return true;
        };

        let now = Instant::now();

        if now.duration_since(*last) < self.min_interval {
            return false;
        }

        *last = now;
        true
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn handle_internal_command(
    context: &mut Context,
    command: InternalRefresh,
    devices: &DeviceStore,
    streams: &StreamStore,
    events_tx: &EventSender,
    default_input: &DefaultDevice,
    default_output: &DefaultDevice,
) {
    match command {
        InternalRefresh::Devices => {
            device::trigger_discovery(context, devices, events_tx);
        }
        InternalRefresh::Streams => {
            stream::trigger_discovery(context, streams, events_tx);
        }
        InternalRefresh::ServerInfo => {
            server::trigger_info_query(context, devices, events_tx, default_input, default_output);
        }
        InternalRefresh::Device {
            device_key,
            facility,
        } => {
            device::trigger_refresh(context, devices, events_tx, device_key, facility);
        }
        InternalRefresh::Stream {
            stream_key,
            facility,
        } => {
            stream::trigger_refresh(context, streams, events_tx, stream_key, facility);
        }
    }
}

pub(super) fn handle_external_command(
    context: &mut Context,
    command: ExternalCommand,
    devices: &DeviceStore,
    streams: &StreamStore,
    rate_limiter: &VolumeRateLimiter,
) {
    match command {
        ExternalCommand::SetDeviceVolume { device_key, volume } => {
            if rate_limiter.should_process() {
                device::set_device_volume(context, device_key, volume, devices);
            }
        }
        ExternalCommand::SetDeviceMute { device_key, muted } => {
            device::set_device_mute(context, device_key, muted, devices);
        }
        ExternalCommand::SetDefaultInput { device_key } => {
            server::set_default_input(context, device_key, devices);
        }
        ExternalCommand::SetDefaultOutput { device_key } => {
            server::set_default_output(context, device_key, devices);
        }
        ExternalCommand::SetStreamVolume { stream_key, volume } => {
            if rate_limiter.should_process() {
                stream::set_stream_volume(context, stream_key, volume, streams);
            }
        }
        ExternalCommand::SetStreamMute { stream_key, muted } => {
            stream::set_stream_mute(context, stream_key, muted, streams);
        }
        ExternalCommand::MoveStream {
            stream_key,
            device_key,
        } => {
            stream::move_stream(context, stream_key, device_key, streams);
        }
        ExternalCommand::SetPort { device_key, port } => {
            device::set_device_port(context, device_key, port, devices);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn should_process_returns_true_on_first_call() {
        let limiter = VolumeRateLimiter::new();

        let result = limiter.should_process();

        assert!(result);
    }

    #[test]
    fn should_process_returns_false_when_called_too_quickly() {
        let limiter = VolumeRateLimiter::new();

        limiter.should_process();
        let result = limiter.should_process();

        assert!(!result);
    }

    #[test]
    fn should_process_returns_true_after_minimum_interval() {
        let limiter = VolumeRateLimiter::new();

        limiter.should_process();
        thread::sleep(Duration::from_millis(35));
        let result = limiter.should_process();

        assert!(result);
    }

    #[test]
    fn should_process_returns_true_on_lock_poisoned() {
        use std::sync::{Arc, Barrier};

        let limiter = Arc::new(VolumeRateLimiter::new());
        let limiter_clone = Arc::clone(&limiter);
        let barrier = Arc::new(Barrier::new(2));
        let barrier_clone = Arc::clone(&barrier);

        let handle = thread::spawn(move || {
            let _guard = limiter_clone.last_volume_change.lock().unwrap();
            barrier_clone.wait();
            panic!("Intentional panic to poison lock");
        });

        barrier.wait();
        thread::sleep(Duration::from_millis(10));
        let _ = handle.join();

        let result = limiter.should_process();

        assert!(result);
    }
}
