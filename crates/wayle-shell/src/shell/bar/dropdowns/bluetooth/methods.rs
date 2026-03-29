use relm4::{factory::FactoryVecDequeGuard, gtk, prelude::*};
use tracing::warn;
use wayle_bluetooth::types::agent::PairingRequest;
use zbus::zvariant::OwnedObjectPath;

use super::{
    ACTION_TIMEOUT, BluetoothDropdown, SCAN_DURATION,
    device_item::{
        DeviceItem,
        messages::{DeviceItemInit, DeviceItemOutput},
    },
    helpers::{DeviceSnapshot, build_split_device_lists, resolve_device_display},
    messages::{BluetoothDropdownCmd, BluetoothDropdownMsg, DeviceActionMsg, PairingCardOutput},
    pairing_card::messages::PairingCardMsg,
    watchers,
};

impl BluetoothDropdown {
    pub(super) fn handle_bluetooth_toggled(
        &mut self,
        active: bool,
        sender: &ComponentSender<Self>,
    ) {
        self.enabled = active;
        if !active {
            self.scan_token.reset();
            self.scanning = false;
        }

        let Some(bluetooth) = self.bluetooth.clone() else {
            return;
        };
        sender.command(move |_out, _shutdown| async move {
            let result = if active {
                bluetooth.enable().await
            } else {
                bluetooth.disable().await
            };
            if let Err(err) = result {
                warn!(error = %err, "bluetooth toggle failed");
            }
        });
    }

    pub(super) fn handle_scan_requested(&mut self, sender: &ComponentSender<Self>) {
        let Some(bluetooth) = self.bluetooth.clone() else {
            return;
        };

        self.scanning = true;
        let token = self.scan_token.reset();

        sender.command(move |out, _shutdown| async move {
            if let Err(err) = bluetooth.start_timed_discovery(SCAN_DURATION).await {
                warn!(error = %err, "bluetooth scan failed");
            }
            tokio::select! {
                () = tokio::time::sleep(SCAN_DURATION) => {}
                () = token.cancelled() => {}
            }
            let _ = out.send(BluetoothDropdownCmd::ScanComplete);
        });
    }

    pub(super) fn build_device_list(sender: &ComponentSender<Self>) -> FactoryVecDeque<DeviceItem> {
        FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |output| match output {
                DeviceItemOutput::Connect(path) => {
                    BluetoothDropdownMsg::DeviceAction(DeviceActionMsg::Connect(path))
                }
                DeviceItemOutput::Disconnect(path) => {
                    BluetoothDropdownMsg::DeviceAction(DeviceActionMsg::Disconnect(path))
                }
                DeviceItemOutput::Forget(path) => {
                    BluetoothDropdownMsg::DeviceAction(DeviceActionMsg::Forget(path))
                }
            })
    }

    pub(super) fn reset_device_watchers(&mut self, sender: &ComponentSender<Self>) {
        let Some(bluetooth) = &self.bluetooth else {
            return;
        };

        let token = self.device_watcher.reset();
        watchers::spawn_device_watchers(sender, bluetooth, token);
    }

    pub(super) fn rebuild_device_lists(&mut self) {
        let Some(bluetooth) = &self.bluetooth else {
            return;
        };

        let devices = bluetooth.devices.get();
        let lists = build_split_device_lists(&devices);

        reconcile_list(&mut self.my_devices.guard(), &lists.my_devices);
        reconcile_list(
            &mut self.available_devices.guard(),
            &lists.available_devices,
        );
    }

    pub(super) fn handle_device_action(
        &self,
        action: DeviceActionMsg,
        sender: &ComponentSender<Self>,
    ) {
        let Some(bluetooth) = self.bluetooth.clone() else {
            return;
        };
        let path = match &action {
            DeviceActionMsg::Connect(path)
            | DeviceActionMsg::Disconnect(path)
            | DeviceActionMsg::Forget(path) => path.clone(),
        };

        sender.command(move |out, _shutdown| async move {
            let Ok(device) = bluetooth.device(path.clone()).await else {
                let _ = out.send(BluetoothDropdownCmd::DeviceActionFailed(path));
                return;
            };

            let action_fut = async {
                match action {
                    DeviceActionMsg::Connect(_) => device.connect().await,
                    DeviceActionMsg::Disconnect(_) => device.disconnect().await,
                    DeviceActionMsg::Forget(_) => device.forget().await,
                }
            };

            match tokio::time::timeout(ACTION_TIMEOUT, action_fut).await {
                Ok(Ok(())) => {}
                Ok(Err(err)) => {
                    warn!(error = %err, "bluetooth device action failed");
                    let _ = out.send(BluetoothDropdownCmd::DeviceActionFailed(path));
                }
                Err(_elapsed) => {
                    warn!("bluetooth device action timed out");
                    let _ = out.send(BluetoothDropdownCmd::DeviceActionFailed(path));
                }
            }
        });
    }

    pub(super) fn clear_device_pending(&mut self, path: &OwnedObjectPath) {
        clear_pending_in_factory(&mut self.my_devices.guard(), path);
        clear_pending_in_factory(&mut self.available_devices.guard(), path);
    }

    pub(super) fn handle_pairing_request(&mut self, request: Option<PairingRequest>) {
        match request {
            Some(request) => {
                let device_path = match &request {
                    PairingRequest::RequestPinCode { device_path } => device_path,
                    PairingRequest::DisplayPinCode { device_path, .. } => device_path,
                    PairingRequest::RequestPasskey { device_path } => device_path,
                    PairingRequest::DisplayPasskey { device_path, .. } => device_path,
                    PairingRequest::RequestConfirmation { device_path, .. } => device_path,
                    PairingRequest::RequestAuthorization { device_path } => device_path,
                    PairingRequest::RequestServiceAuthorization { device_path, .. } => device_path,
                };

                let Some(bluetooth) = &self.bluetooth else {
                    return;
                };

                let devices = bluetooth.devices.get();
                let matching_device = devices
                    .iter()
                    .find(|device| &device.object_path == device_path);

                let display = matching_device
                    .map(|device| resolve_device_display(device))
                    .unwrap_or_default();

                self.pairing_card.emit(PairingCardMsg::SetRequest {
                    request,
                    device_name: display.name,
                    device_icon: display.icon,
                    device_type_key: display.device_type_key,
                });
            }
            None => {
                self.pairing_card.emit(PairingCardMsg::Clear);
            }
        }
    }

    pub(super) fn handle_pairing_output(
        &mut self,
        output: PairingCardOutput,
        sender: &ComponentSender<Self>,
    ) {
        self.pairing_card.emit(PairingCardMsg::Clear);

        if matches!(output, PairingCardOutput::Cancelled) {
            let Some(bluetooth) = self.bluetooth.clone() else {
                return;
            };
            sender.command(move |_out, _shutdown| async move {
                bluetooth.cancel_pending_request().await;
            });
            return;
        }

        let Some(bluetooth) = self.bluetooth.clone() else {
            return;
        };

        sender.command(move |_out, _shutdown| async move {
            let result = match output {
                PairingCardOutput::PinSubmitted(pin)
                | PairingCardOutput::LegacyPinSubmitted(pin) => bluetooth.provide_pin(pin).await,
                PairingCardOutput::PasskeyConfirmed => bluetooth.provide_confirmation(true).await,
                PairingCardOutput::PasskeyRejected => bluetooth.provide_confirmation(false).await,
                PairingCardOutput::AuthorizationAccepted => {
                    bluetooth.provide_authorization(true).await
                }
                PairingCardOutput::AuthorizationRejected => {
                    bluetooth.provide_authorization(false).await
                }
                PairingCardOutput::ServiceAuthorizationAccepted => {
                    bluetooth.provide_service_authorization(true).await
                }
                PairingCardOutput::ServiceAuthorizationRejected => {
                    bluetooth.provide_service_authorization(false).await
                }
                PairingCardOutput::Cancelled => return,
            };
            if let Err(err) = result {
                warn!(
                    error = %err,
                    "pairing response failed"
                );
            }
        });
    }
}

fn reconcile_list(
    guard: &mut FactoryVecDequeGuard<'_, DeviceItem>,
    new_snapshots: &[DeviceSnapshot],
) {
    let old_paths: Vec<_> = (0..guard.len())
        .filter_map(|idx| guard.get(idx).map(|item| item.device_path.clone()))
        .collect();

    if try_reconcile(guard, &old_paths, new_snapshots) {
        return;
    }

    guard.clear();
    for snapshot in new_snapshots {
        guard.push_back(DeviceItemInit {
            snapshot: snapshot.clone(),
        });
    }
}

fn clear_pending_in_factory(
    guard: &mut FactoryVecDequeGuard<'_, DeviceItem>,
    path: &OwnedObjectPath,
) {
    let Some(idx) =
        (0..guard.len()).find(|&idx| guard.get(idx).is_some_and(|item| item.device_path == *path))
    else {
        return;
    };

    if let Some(item) = guard.get_mut(idx) {
        item.clear_pending();
    }
}

fn try_reconcile(
    guard: &mut FactoryVecDequeGuard<'_, DeviceItem>,
    old_paths: &[OwnedObjectPath],
    new_snapshots: &[DeviceSnapshot],
) -> bool {
    let mut old_iter = old_paths.iter().peekable();
    for snapshot in new_snapshots {
        if old_iter.peek() == Some(&&snapshot.device.object_path) {
            old_iter.next();
        }
    }

    if old_iter.peek().is_some() {
        return false;
    }

    let mut old_idx = 0;
    for (new_idx, snapshot) in new_snapshots.iter().enumerate() {
        if old_idx < old_paths.len() && old_paths[old_idx] == snapshot.device.object_path {
            let needs_update = guard
                .get(new_idx)
                .is_some_and(|item| item.differs_from(snapshot));
            if needs_update && let Some(item) = guard.get_mut(new_idx) {
                item.update_from_snapshot(snapshot.clone());
            }
            old_idx += 1;
        } else {
            guard.insert(
                new_idx,
                DeviceItemInit {
                    snapshot: snapshot.clone(),
                },
            );
        }
    }

    true
}
