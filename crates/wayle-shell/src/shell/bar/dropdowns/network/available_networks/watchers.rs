use std::{collections::HashSet, ops::ControlFlow, sync::Arc, time::Duration};

use futures::StreamExt;
use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};
use wayle_common::{watch_async, watch_cancellable};
use wayle_network::{
    core::{device::DeviceStateChangedEvent, settings::Settings},
    types::states::{NMDeviceState, NMDeviceStateReason},
    wifi::Wifi,
};

use crate::{
    i18n::t,
    shell::bar::dropdowns::network::available_networks::{
        AvailableNetworks, messages::AvailableNetworksCmd,
    },
};

const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

pub(super) fn spawn(
    sender: &ComponentSender<AvailableNetworks>,
    wifi: &Arc<Wifi>,
    token: CancellationToken,
) {
    let access_points = wifi.access_points.clone();
    watch_cancellable!(sender, token, [access_points.watch()], |out| {
        let _ = out.send(AvailableNetworksCmd::AccessPointsChanged);
    });
}

/// Watches for changes to saved network profiles and extracts known WiFi SSIDs.
pub(super) fn spawn_settings_watcher(
    sender: &ComponentSender<AvailableNetworks>,
    settings: &Arc<Settings>,
) {
    let settings = settings.clone();
    watch_async!(sender, [settings.connections.watch()], |out| async {
        let known_ssids = extract_known_ssids(&settings).await;
        let _ = out.send(AvailableNetworksCmd::KnownSsidsUpdated(known_ssids));
    });
}

async fn extract_known_ssids(settings: &Settings) -> HashSet<String> {
    let mut known = HashSet::new();
    for connection in settings.connections.get() {
        if let Some(ssid) = connection.wifi_ssid().await {
            known.insert(ssid.to_string_lossy());
        }
    }
    known
}

pub(super) fn spawn_connection_watcher(
    sender: &ComponentSender<AvailableNetworks>,
    wifi: &Arc<Wifi>,
    token: CancellationToken,
) {
    let wifi = wifi.clone();

    sender.command(move |out, shutdown| async move {
        let device_stream = match wifi.device.core.device_state_changed_signal().await {
            Ok(stream) => stream,
            Err(err) => {
                warn!(error = %err, "failed to subscribe to device state changes");
                let _ = out.send(AvailableNetworksCmd::ConnectionFailed(t!(
                    "dropdown-network-error-generic"
                )));
                return;
            }
        };

        tokio::select! {
            () = shutdown.wait() => {}
            () = token.cancelled() => {
                let _ = out.send(AvailableNetworksCmd::ScanComplete);
            }
            () = monitor_connection(device_stream, &out) => {}
        }
    });
}

async fn monitor_connection(
    device_stream: impl futures::Stream<Item = DeviceStateChangedEvent>,
    out: &relm4::Sender<AvailableNetworksCmd>,
) {
    let timeout = tokio::time::sleep(CONNECTION_TIMEOUT);
    tokio::pin!(timeout);
    tokio::pin!(device_stream);

    loop {
        tokio::select! {
            () = &mut timeout => {
                let _ = out.send(AvailableNetworksCmd::ConnectionTimedOut);
                return;
            }
            event = device_stream.next() => {
                let Some(event) = event else {
                    let _ = out.send(AvailableNetworksCmd::ConnectionFailed(
                        t!("dropdown-network-error-generic"),
                    ));
                    return;
                };

                if handle_device_state_change(&event, out).is_break() {
                    return;
                }
            }
        }
    }
}

fn handle_device_state_change(
    event: &DeviceStateChangedEvent,
    out: &relm4::Sender<AvailableNetworksCmd>,
) -> ControlFlow<()> {
    debug!(
        new_state = ?event.new_state,
        old_state = ?event.old_state,
        reason = ?event.reason,
        "device state changed"
    );

    match event.new_state {
        NMDeviceState::Activated => {
            let _ = out.send(AvailableNetworksCmd::ConnectionActivated);
            ControlFlow::Break(())
        }
        NMDeviceState::Failed => {
            warn!(reason = ?event.reason, "wifi connection failed");
            let _ = out.send(translate_device_failure(event.reason));
            ControlFlow::Break(())
        }
        NMDeviceState::Disconnected if is_transient_disconnect(event.reason) => {
            debug!(
                reason = ?event.reason,
                "transient disconnect during connection, continuing"
            );
            ControlFlow::Continue(())
        }
        NMDeviceState::Disconnected => {
            warn!(reason = ?event.reason, "wifi connection failed (disconnected)");
            let _ = out.send(translate_device_failure(event.reason));
            ControlFlow::Break(())
        }
        _ => {
            if let Some(step) = translate_connection_step(event.new_state) {
                let _ = out.send(AvailableNetworksCmd::ConnectionProgress(step));
            }
            ControlFlow::Continue(())
        }
    }
}

fn translate_connection_step(state: NMDeviceState) -> Option<String> {
    match state {
        NMDeviceState::Prepare => Some(t!("dropdown-network-step-preparing")),
        NMDeviceState::Config => Some(t!("dropdown-network-step-configuring")),
        NMDeviceState::NeedAuth => Some(t!("dropdown-network-step-authenticating")),
        NMDeviceState::IpConfig => Some(t!("dropdown-network-step-obtaining-ip")),
        NMDeviceState::IpCheck | NMDeviceState::Secondaries => {
            Some(t!("dropdown-network-step-verifying"))
        }
        _ => None,
    }
}

fn is_transient_disconnect(reason: NMDeviceStateReason) -> bool {
    matches!(
        reason,
        NMDeviceStateReason::None
            | NMDeviceStateReason::Unknown
            | NMDeviceStateReason::NewActivation
    )
}

fn translate_device_failure(reason: NMDeviceStateReason) -> AvailableNetworksCmd {
    match reason {
        NMDeviceStateReason::NoSecrets
        | NMDeviceStateReason::SupplicantDisconnect
        | NMDeviceStateReason::SupplicantConfigFailed
        | NMDeviceStateReason::SupplicantFailed => AvailableNetworksCmd::ConnectionAuthFailed,

        NMDeviceStateReason::SupplicantTimeout => AvailableNetworksCmd::ConnectionTimedOut,

        NMDeviceStateReason::SsidNotFound => {
            AvailableNetworksCmd::ConnectionFailed(t!("dropdown-network-error-not-found"))
        }

        NMDeviceStateReason::IpConfigUnavailable | NMDeviceStateReason::IpConfigExpired => {
            AvailableNetworksCmd::ConnectionFailed(t!("dropdown-network-error-ip-config"))
        }

        NMDeviceStateReason::UserRequested | NMDeviceStateReason::NewActivation => {
            AvailableNetworksCmd::ScanComplete
        }

        _ => AvailableNetworksCmd::ConnectionFailed(t!("dropdown-network-error-generic")),
    }
}
