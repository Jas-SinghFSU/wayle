mod command;
pub(crate) mod events;
mod query;
mod types;

use std::{env, path::PathBuf};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::UnixStream,
};
use tracing::instrument;
pub use types::*;

use crate::{Error, Result};

const END_OF_TRANSMISSION: u8 = b'\x04';

#[derive(Debug, Clone)]
pub(crate) struct HyprMessenger {
    path: PathBuf,
}

impl HyprMessenger {
    pub fn new(instance_signature: &str, runtime_dir: &str) -> Self {
        Self {
            path: PathBuf::from(format!(
                "{runtime_dir}/hypr/{instance_signature}/.socket.sock"
            )),
        }
    }

    pub fn from_env() -> Result<Self> {
        let signature =
            env::var("HYPRLAND_INSTANCE_SIGNATURE").map_err(|_| Error::HyprlandNotRunning)?;

        let runtime_dir = env::var("XDG_RUNTIME_DIR")
            .map_err(|_| Error::InvalidInstanceSignature("XDG_RUNTIME_DIR not set".to_string()))?;

        Ok(Self::new(&signature, &runtime_dir))
    }

    #[instrument(skip(self), fields(command = %command), err)]
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

        String::from_utf8(response).map_err(Error::ResponseDecodeError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_correct_socket_path() {
        let messenger = HyprMessenger::new("test_signature", "/tmp");

        assert_eq!(
            messenger.path,
            PathBuf::from("/tmp/hypr/test_signature/.socket.sock")
        );
    }
}
