use std::sync::{Arc, Weak};

use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error};
use zbus::{Connection, proxy::PropertyStream, zvariant::OwnedObjectPath};

use super::Wifi;
use crate::{
    remove_and_cancel,
    services::{
        common::Property,
        network::{
            core::{
                access_point::{
                    AccessPoint,
                    types::{LiveAccessPointParams, Ssid},
                },
                device::wifi::DeviceWifi,
            },
            error::Error,
            proxy::{
                access_point::AccessPointProxy,
                devices::{DeviceProxy, wireless::DeviceWirelessProxy},
                manager::NetworkManagerProxy,
            },
            types::states::{NMDeviceState, NetworkStatus},
        },
        traits::{ModelMonitoring, Reactive},
    },
};

type SsidStream = PropertyStream<'static, Vec<u8>>;
type StrengthStream = PropertyStream<'static, u8>;

impl ModelMonitoring for Wifi {
    type Error = Error;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let device_arc = Arc::new(self.device.clone());
        device_arc.start_monitoring().await?;

        let Some(ref cancellation_token) = self.device.core.cancellation_token else {
            return Err(Error::OperationFailed {
                operation: "start_monitoring",
                reason: String::from("A cancellation_token was not found."),
            });
        };

        let access_points = &self.access_points;
        let device = &self.device;

        populate_existing_access_points(
            &self.device.core.connection,
            device,
            access_points,
            cancellation_token,
        )
        .await;

        let cancel_token = cancellation_token.clone();
        let weak_self = Arc::downgrade(&self);

        let wireless_proxy = DeviceWirelessProxy::new(
            &self.device.core.connection,
            self.device.core.object_path.clone(),
        )
        .await
        .map_err(Error::DbusError)?;
        let device_proxy = DeviceProxy::new(
            &self.device.core.connection,
            self.device.core.object_path.clone(),
        )
        .await
        .map_err(Error::DbusError)?;
        let nm_proxy = NetworkManagerProxy::new(&self.device.core.connection)
            .await
            .map_err(Error::DbusError)?;

        tokio::spawn(async move {
            let _ = monitor_wifi(
                weak_self,
                wireless_proxy,
                device_proxy,
                nm_proxy,
                cancel_token,
            )
            .await;
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
            cancellation_token,
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
    weak_wifi: Weak<Wifi>,
    wireless_proxy: DeviceWirelessProxy<'static>,
    device_proxy: DeviceProxy<'static>,
    nm_proxy: NetworkManagerProxy<'static>,
    cancellation_token: CancellationToken,
) -> Result<(), Error> {
    let mut ap_added = wireless_proxy
        .receive_access_point_added()
        .await
        .map_err(Error::DbusError)?;
    let mut ap_removed = wireless_proxy
        .receive_access_point_removed()
        .await
        .map_err(Error::DbusError)?;
    let mut enabled_changed = nm_proxy.receive_wireless_enabled_changed().await;
    let mut access_point_changed = wireless_proxy.receive_active_access_point_changed().await;
    let mut connectivity_changed = device_proxy.receive_state_changed().await;

    let (mut ap_ssid_stream, mut ap_strength_stream) = {
        let Some(wifi) = weak_wifi.upgrade() else {
            error!("Failed to upgrade weak wifi reference.");
            error!("Access Point monitoring may be degraded");
            return Err(Error::OperationFailed {
                operation: "monitor_wifi",
                reason: String::from("Failed to upgrade weak_wifi"),
            });
        };

        handle_access_point_changed(
            &wifi.device.core.connection,
            wifi.device.active_access_point.get(),
            &wifi.ssid,
            &wifi.strength,
        )
        .await
    };

    tokio::spawn(async move {
        loop {
            let Some(wifi) = weak_wifi.upgrade() else {
                return;
            };

            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("WifiMonitor cancelled");
                    return;
                }

                Some(added) = ap_added.next() => {
                    if let Ok(args) = added.args() {
                        handle_ap_added(
                            &wifi.device.core.connection,
                            args.access_point,
                            &wifi.access_points,
                            &cancellation_token
                        ).await;
                    }
                }

                Some(removed) = ap_removed.next() => {
                    if let Ok(args) = removed.args() {
                        handle_ap_removed(&args.access_point, &wifi.access_points);
                    }
                }

                Some(change) = enabled_changed.next() => {
                    if let Ok(new_state) = change.get().await {
                        wifi.enabled.set(new_state);
                    }
                }

                Some(change) = access_point_changed.next() => {
                    let Ok(new_ap_path) = change.get().await else {
                        continue;
                    };

                    let (new_ssid_stream, new_strength_stream) =
                        handle_access_point_changed(
                            &wifi.device.core.connection,
                            new_ap_path,
                            &wifi.ssid,
                            &wifi.strength
                        ).await;

                    ap_ssid_stream = new_ssid_stream;
                    ap_strength_stream = new_strength_stream;
                }

                Some(change) = async { ap_ssid_stream.as_mut()?.next().await } => {
                    if let Ok(new_ssid) = change.get().await {
                        wifi.ssid.set(Some(Ssid::new(new_ssid).to_string()));
                    }
                }

                Some(change) = async { ap_strength_stream.as_mut()?.next().await } => {
                    if let Ok(new_strength) = change.get().await {
                        wifi.strength.set(Some(new_strength));
                    }
                }

                Some(change) = connectivity_changed.next() => {
                    if let Ok(new_connectivity) = change.get().await {
                        let device_state = NMDeviceState::from_u32(new_connectivity);
                        wifi.connectivity.set(NetworkStatus::from_device_state(device_state));
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
        cancellation_token,
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
                ssid_prop.set(Some(Ssid::new(raw_ssid).to_string()));
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
