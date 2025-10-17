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
    pub fn new() -> Result<Self> {
        let his = env::var("HYPRLAND_INSTANCE_SIGNATURE").map_err(|_| Error::HyprlandNotRunning)?;
        let runtime_dir = env::var("XDG_RUNTIME_DIR")
            .map_err(|_| Error::InvalidInstanceSignature("XDG_RUNTIME_DIR not set".to_string()))?;

        let socket_path = format!("{runtime_dir}/hypr/{his}/.socket.sock");

        Ok(Self {
            path: PathBuf::from(socket_path),
        })
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

        String::from_utf8(response).map_err(|e| Error::OperationFailed {
            operation: "send",
            reason: e.to_string(),
        })
    }
}

#[cfg(test)]
#[allow(unsafe_code)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn new_succeeds_with_valid_env_vars() {
        unsafe {
            env::set_var("HYPRLAND_INSTANCE_SIGNATURE", "test_signature");
            env::set_var("XDG_RUNTIME_DIR", "/tmp");
        }

        let result = HyprMessenger::new();

        assert!(result.is_ok());
        let messenger = result.unwrap();
        assert_eq!(
            messenger.path,
            PathBuf::from("/tmp/hypr/test_signature/.socket.sock")
        );

        unsafe {
            env::remove_var("HYPRLAND_INSTANCE_SIGNATURE");
            env::remove_var("XDG_RUNTIME_DIR");
        }
    }

    #[test]
    fn new_fails_when_hyprland_instance_signature_missing() {
        unsafe {
            env::remove_var("HYPRLAND_INSTANCE_SIGNATURE");
            env::set_var("XDG_RUNTIME_DIR", "/tmp");
        }

        let result = HyprMessenger::new();

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::HyprlandNotRunning));

        unsafe {
            env::remove_var("XDG_RUNTIME_DIR");
        }
    }

    #[test]
    fn new_fails_when_xdg_runtime_dir_missing() {
        unsafe {
            env::set_var("HYPRLAND_INSTANCE_SIGNATURE", "test_signature");
            env::remove_var("XDG_RUNTIME_DIR");
        }

        let result = HyprMessenger::new();

        assert!(result.is_err());
        let error = result.unwrap_err();
        if let Error::InvalidInstanceSignature(msg) = error {
            assert_eq!(msg, "XDG_RUNTIME_DIR not set");
        } else {
            panic!("Expected InvalidInstanceSignature error");
        }

        unsafe {
            env::remove_var("HYPRLAND_INSTANCE_SIGNATURE");
        }
    }
}
