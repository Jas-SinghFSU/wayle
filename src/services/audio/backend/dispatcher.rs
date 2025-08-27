use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

use libpulse_binding::context::Context;

use super::{
    commands::{device, server, stream},
    types::{
        DefaultDevice, DeviceStore, EventSender, ExternalCommand, InternalCommand, StreamStore,
    },
};

/// Rate limiter for volume operations to prevent overwhelming PulseAudio
///
/// Enforces a global rate limit on volume changes to prevent overwhelming
/// PulseAudio's command dispatcher with rapid updates.
pub(super) struct VolumeRateLimiter {
    last_volume_change: Mutex<Instant>,
    min_interval: Duration,
}

impl VolumeRateLimiter {
    /// Create a new rate limiter with 30ms minimum interval
    pub fn new() -> Self {
        Self {
            last_volume_change: Mutex::new(Instant::now() - Duration::from_secs(1)),
            min_interval: Duration::from_millis(30),
        }
    }

    /// Check if a volume operation should be processed
    ///
    /// Returns true if enough time has passed since the last operation,
    /// false if the operation should be dropped to avoid overwhelming PulseAudio.
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

/// Handle internal PulseAudio commands (event-driven)
#[allow(clippy::too_many_arguments)]
pub(super) fn handle_internal_command(
    context: &mut Context,
    command: InternalCommand,
    devices: &DeviceStore,
    streams: &StreamStore,
    events_tx: &EventSender,
    default_input: &DefaultDevice,
    default_output: &DefaultDevice,
) {
    match command {
        InternalCommand::RefreshDevices => {
            device::trigger_device_discovery(context, devices, events_tx);
        }
        InternalCommand::RefreshStreams => {
            stream::trigger_stream_discovery(context, streams, events_tx);
        }
        InternalCommand::RefreshServerInfo => {
            server::trigger_server_info_query(
                context,
                devices,
                events_tx,
                default_input,
                default_output,
            );
        }
        InternalCommand::RefreshDevice {
            device_key,
            facility,
        } => {
            device::trigger_device_refresh(context, devices, events_tx, device_key, facility);
        }
        InternalCommand::RefreshStream {
            stream_key,
            facility,
        } => {
            stream::trigger_stream_refresh(context, streams, events_tx, stream_key, facility);
        }
    }
}

/// Handle external PulseAudio commands (user-initiated)
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
        ExternalCommand::Shutdown => {
            // Shutdown handled in main loop
        }
    }
}
