mod dispatcher;
pub mod layer;
pub mod monitor;
pub mod types;
pub mod window;
pub mod workspace;

use std::env;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::UnixStream,
    sync::broadcast::Sender,
};
use tracing::warn;
use types::ServiceNotification;

use crate::{Error, HyprlandEvent, Result};

pub(crate) async fn subscribe(
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let his = env::var("HYPRLAND_INSTANCE_SIGNATURE").map_err(|_| Error::HyprlandNotRunning)?;
    let runtime_dir = env::var("XDG_RUNTIME_DIR")
        .map_err(|_| Error::InvalidInstanceSignature("XDG_RUNTIME_DIR not set".to_string()))?;

    let socket_path = format!("{runtime_dir}/hypr/{his}/.socket2.sock");
    let event_stream =
        UnixStream::connect(&socket_path)
            .await
            .map_err(|e| Error::IpcConnectionFailed {
                socket_type: "event",
                reason: e.to_string(),
            })?;

    tokio::spawn(async move {
        let reader = BufReader::new(event_stream);
        let mut lines = reader.lines();

        loop {
            match lines.next_line().await {
                Ok(Some(line)) => {
                    let Some((event, data)) = line.split_once(">>") else {
                        warn!("Failed to parse Hyprland event: missing '>>' separator");
                        warn!("Data: {line}");
                        continue;
                    };

                    if let Err(e) =
                        dispatcher::dispatch(event, data, internal_tx.clone(), hyprland_tx.clone())
                            .await
                    {
                        warn!("Failed to handle event {event}: {e}");
                    }
                }
                Ok(None) => {
                    warn!("Hyprland event stream closed");
                    break;
                }
                Err(e) => {
                    warn!("Error reading event stream: {e}");
                    break;
                }
            }
        }
    });

    Ok(())
}
