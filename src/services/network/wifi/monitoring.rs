use std::sync::Arc;

use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use zbus::{Connection, proxy::PropertyStream, zvariant::OwnedObjectPath};

use super::Wifi;
use crate::{
    remove_and_cancel,
    services::{
        common::Property,
        network::{
            AccessPoint, AccessPointProxy, DeviceProxy, NMDeviceState, NetworkError,
            NetworkManagerProxy, NetworkStatus, SSID,
            core::{access_point::LiveAccessPointParams, device::wifi::DeviceWifi},
            wireless::DeviceWirelessProxy,
        },
        traits::{ModelMonitoring, Reactive},
    },
};

type SsidStream = PropertyStream<'static, Vec<u8>>;
type StrengthStream = PropertyStream<'static, u8>;

impl ModelMonitoring for Wifi {
    type Error = NetworkError;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let device_arc = Arc::new(self.device.clone());
        device_arc.start_monitoring().await?;

        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(NetworkError::OperationFailed {
                operation: "start_monitoring",
                reason: String::from("A cancellation_token was not found."),
            });
        };

        let access_points = &self.access_points;
        let device = &self.device;

        populate_existing_access_points(
            &self.connection,
            device,
            access_points,
            cancellation_token,
        )
        .await;

        let cancel_token = cancellation_token.clone();

        tokio::spawn(async move {
            let _ = monitor_wifi(self, cancel_token).await;
        });

        Ok(())
    }
}

async fn populate_existing_access_points(
    connection: &Connection,
    device: &DeviceWifi,
    access_points: &Property<Vec<Arc<AccessPoint>>>,
    cancellation_token: &CancellationToken,
) {
    let existing_paths = device.access_points.get();
    let mut initial_aps = Vec::with_capacity(existing_paths.len());

    for ap_path in existing_paths {
        let Ok(path) = OwnedObjectPath::try_from(ap_path.as_str()) else {
            continue;
        };

        if let Ok(ap) = AccessPoint::get_live(LiveAccessPointParams {
            connection,
            path,
            cancellation_token: cancellation_token.child_token(),
        })
        .await
        {
            initial_aps.push(ap);
        }
    }

    if !initial_aps.is_empty() {
        access_points.set(initial_aps);
    }
}

async fn monitor_wifi(
    wifi: Arc<Wifi>,
    cancellation_token: CancellationToken,
) -> Result<(), NetworkError> {
    let access_points_prop = wifi.access_points.clone();
    let device_prop = wifi.device.clone();
    let enabled_state_prop = wifi.enabled.clone();
    let ssid_prop = wifi.ssid.clone();
    let strength_prop = wifi.strength.clone();
    let connectivity_prop = wifi.connectivity.clone();

    let wireless_proxy =
        DeviceWirelessProxy::new(&wifi.connection, device_prop.object_path.clone())
            .await
            .map_err(NetworkError::DbusError)?;
    let device_proxy = DeviceProxy::new(&wifi.connection, device_prop.object_path.clone())
        .await
        .map_err(NetworkError::DbusError)?;
    let nm_proxy = NetworkManagerProxy::new(&wifi.connection)
        .await
        .map_err(NetworkError::DbusError)?;

    let mut ap_added = wireless_proxy
        .receive_access_point_added()
        .await
        .map_err(NetworkError::DbusError)?;
    let mut ap_removed = wireless_proxy
        .receive_access_point_removed()
        .await
        .map_err(NetworkError::DbusError)?;
    let mut enabled_changed = nm_proxy.receive_wireless_enabled_changed().await;
    let mut access_point_changed = wireless_proxy.receive_active_access_point_changed().await;
    let mut connectivity_changed = device_proxy.receive_state_changed().await;

    let (mut ap_ssid_stream, mut ap_strength_stream) = handle_access_point_changed(
        &wifi.connection,
        device_prop.active_access_point.get(),
        &ssid_prop,
        &strength_prop,
    )
    .await;

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("WifiMonitor cancelled");
                    return;
                }

                Some(added) = ap_added.next() => {
                    if let Ok(args) = added.args() {
                        handle_ap_added(&wifi.connection, args.access_point, &access_points_prop, &cancellation_token).await;
                    }
                }

                Some(removed) = ap_removed.next() => {
                    if let Ok(args) = removed.args() {
                        handle_ap_removed(&args.access_point, &access_points_prop);
                    }
                }

                Some(change) = enabled_changed.next() => {
                    if let Ok(new_state) = change.get().await {
                        enabled_state_prop.set(new_state);
                    }
                }

                Some(change) = access_point_changed.next() => {
                    let Ok(new_ap_path) = change.get().await else {
                        continue;
                    };

                    let (new_ssid_stream, new_strength_stream) =
                        handle_access_point_changed(
                            &wifi.connection,
                            new_ap_path,
                            &ssid_prop,
                            &strength_prop
                        ).await;

                    ap_ssid_stream = new_ssid_stream;
                    ap_strength_stream = new_strength_stream;
                }

                Some(change) = async { ap_ssid_stream.as_mut()?.next().await } => {
                    if let Ok(new_ssid) = change.get().await {
                        ssid_prop.set(Some(SSID::new(new_ssid).to_string()));
                    }
                }

                Some(change) = async { ap_strength_stream.as_mut()?.next().await } => {
                    if let Ok(new_strength) = change.get().await {
                        strength_prop.set(Some(new_strength));
                    }
                }

                Some(change) = connectivity_changed.next() => {
                    if let Ok(new_connectivity) = change.get().await {
                        let device_state = NMDeviceState::from_u32(new_connectivity);
                        connectivity_prop.set(NetworkStatus::from_device_state(device_state));
                    }
                }

                else => {
                    break;
                }
            }
        }
    });

    Ok(())
}

async fn handle_ap_added(
    connection: &Connection,
    ap_path: OwnedObjectPath,
    access_points: &Property<Vec<Arc<AccessPoint>>>,
    cancellation_token: &CancellationToken,
) {
    if let Ok(new_ap) = AccessPoint::get_live(LiveAccessPointParams {
        connection,
        path: ap_path,
        cancellation_token: cancellation_token.child_token(),
    })
    .await
    {
        let mut aps = access_points.get();
        aps.push(new_ap);
        access_points.set(aps);
    }
}

fn handle_ap_removed(ap_path: &OwnedObjectPath, access_points: &Property<Vec<Arc<AccessPoint>>>) {
    remove_and_cancel!(access_points, ap_path.clone());
}

async fn handle_access_point_changed(
    connection: &Connection,
    new_ap_path: OwnedObjectPath,
    ssid_prop: &Property<Option<String>>,
    strength_prop: &Property<Option<u8>>,
) -> (Option<SsidStream>, Option<StrengthStream>) {
    if new_ap_path.is_empty() || new_ap_path == OwnedObjectPath::default() {
        ssid_prop.set(None);
        strength_prop.set(None);
        return (None, None);
    }

    match AccessPointProxy::new(connection, new_ap_path).await {
        Ok(ap_proxy) => {
            if let Ok(raw_ssid) = ap_proxy.ssid().await {
                ssid_prop.set(Some(SSID::new(raw_ssid).to_string()));
            }

            if let Ok(strength) = ap_proxy.strength().await {
                strength_prop.set(Some(strength));
            }

            (
                Some(ap_proxy.receive_ssid_changed().await),
                Some(ap_proxy.receive_strength_changed().await),
            )
        }
        Err(_) => {
            ssid_prop.set(None);
            strength_prop.set(None);
            (None, None)
        }
    }
}
