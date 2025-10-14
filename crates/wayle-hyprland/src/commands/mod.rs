mod command;
mod query;

use std::{env, path::PathBuf};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::UnixStream,
};

use crate::{Error, Result};

const END_OF_TRANSMISSION: u8 = b'\x04';

#[derive(Debug, Clone)]
pub(crate) struct CommandSocket {
    path: PathBuf,
}

impl CommandSocket {
    pub fn new() -> Result<Self> {
        let his = env::var("HYPRLAND_INSTANCE_SIGNATURE").map_err(|_| Error::HyprlandNotRunning)?;
        let runtime_dir = env::var("XDG_RUNTIME_DIR")
            .map_err(|_| Error::InvalidInstanceSignature("XDG_RUNTIME_DIR not set".to_string()))?;

        let socket_path = format!("{runtime_dir}/hypr/{his}/.socket.sock");

        Ok(Self {
            path: PathBuf::from(socket_path),
        })
    }

    pub async fn send(&self, command: &str) -> Result<String> {
        let mut stream = UnixStream::connect(&self.path).await?;
        stream.write_all(command.as_bytes()).await?;

        let mut reader = BufReader::new(stream);
        let mut response = Vec::new();

        reader
            .read_until(END_OF_TRANSMISSION, &mut response)
            .await?;

        if response.last() == Some(&END_OF_TRANSMISSION) {
            response.pop();
        }

        String::from_utf8(response).map_err(|e| Error::OperationFailed {
            operation: "send",
            reason: e.to_string(),
        })
    }
}
