mod dispatcher;
pub(crate) mod layer;
pub(crate) mod monitor;
pub(crate) mod types;
pub(crate) mod window;
pub(crate) mod workspace;

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
            .map_err(|source| Error::IpcConnectionFailed {
                socket_type: "event",
                source,
            })?;

    tokio::spawn(async move {
        let reader = BufReader::new(event_stream);
        let mut lines = reader.lines();

        loop {
            match lines.next_line().await {
                Ok(Some(line)) => {
                    let Some((event, data)) = line.split_once(">>") else {
                        warn!(raw_data = %line, "cannot parse hyprland event: missing '>>' separator");
                        continue;
                    };

                    if let Err(e) =
                        dispatcher::dispatch(event, data, internal_tx.clone(), hyprland_tx.clone())
                            .await
                    {
                        warn!(error = %e, event, "cannot handle event");
                    }
                }
                Ok(None) => {
                    warn!("Hyprland event stream closed");
                    break;
                }
                Err(e) => {
                    warn!(error = %e, "Error reading event stream");
                    break;
                }
            }
        }
    });

    Ok(())
}
