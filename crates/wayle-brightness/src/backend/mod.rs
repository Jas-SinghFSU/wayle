pub(crate) mod hotplug;
pub(crate) mod logind;
pub(crate) mod sysfs;
pub(crate) mod types;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info};

use self::types::{BrightnessEvent, Command, CommandReceiver, EventSender};
use crate::{Error, types::BacklightInfo};

pub(crate) fn start(
    initial_devices: Vec<BacklightInfo>,
    mut command_rx: CommandReceiver,
    event_tx: EventSender,
    token: CancellationToken,
) -> JoinHandle<Result<(), Error>> {
    tokio::spawn(async move {
        let logind_connection = logind::connect().await;

        info!(
            count = initial_devices.len(),
            "backlight devices discovered"
        );

        for device in &initial_devices {
            debug!(
                name = %device.name,
                backlight_type = ?device.backlight_type,
                brightness = device.brightness,
                max = device.max_brightness,
                "backlight device found"
            );

            let _ = event_tx.send(BrightnessEvent::DeviceAdded(device.clone()));
        }

        let _hotplug_handle = hotplug::spawn(event_tx.clone(), token.child_token());

        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    info!("brightness backend stopping");
                    return Ok(());
                }

                Some(command) = command_rx.recv() => {
                    match command {
                        Command::SetBrightness { name, value, responder } => {
                            let result = logind::set_brightness(
                                &logind_connection,
                                &name,
                                value,
                            ).await;

                            let _ = responder.send(result);
                        }
                    }
                }
            }
        }
    })
}
