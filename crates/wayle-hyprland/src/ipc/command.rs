use std::time::Duration;

use tracing::instrument;

use super::{DismissProps, HyprMessenger, OutputCommand, SetErrorCommand};
use crate::Result;

impl HyprMessenger {
    #[instrument(skip(self), fields(command = %command), err)]
    pub(crate) async fn dispatch(&self, command: &str) -> Result<String> {
        self.send(&format!("dispatch {command}")).await
    }

    #[instrument(skip(self), fields(command = %command), err)]
    pub(crate) async fn keyword(&self, command: &str) -> Result<String> {
        self.send(&format!("keyword {command}")).await
    }

    #[instrument(skip(self), err)]
    pub(crate) async fn reload(&self) -> Result<String> {
        self.send("reload").await
    }

    #[instrument(skip(self), err)]
    pub(crate) async fn kill(&self) -> Result<String> {
        self.send("kill").await
    }

    #[instrument(skip(self), fields(theme = %theme, size = %size), err)]
    pub(crate) async fn set_cursor(&self, theme: &str, size: u8) -> Result<String> {
        self.send(&format!("setcursor {theme} {size}")).await
    }

    #[instrument(skip(self), fields(command = ?command), err)]
    pub(crate) async fn output(&self, command: OutputCommand<'_>) -> Result<String> {
        match command {
            OutputCommand::Create { backend, name } => {
                self.send(&format!("output create {backend} {name}")).await
            }
            OutputCommand::Remove { name } => self.send(&format!("output remove {name}")).await,
        }
    }

    #[instrument(skip(self), fields(device = %device, command = %command), err)]
    pub(crate) async fn switch_xkb_layout(&self, device: &str, command: &str) -> Result<String> {
        self.send(&format!("switchxkblayout {device} {command}"))
            .await
    }

    #[instrument(skip(self), fields(command = ?command), err)]
    pub(crate) async fn set_error(&self, command: SetErrorCommand<'_>) -> Result<String> {
        match command {
            SetErrorCommand::Set { color, message } => {
                self.send(&format!("seterror '{color}' {message}")).await
            }
            SetErrorCommand::Disable => self.send("seterror disable").await,
        }
    }

    #[instrument(
        skip(self, message),
        fields(
            icon = ?icon,
            time_ms = %time.as_millis(),
            color = ?color
        ),
        err
    )]
    pub(crate) async fn notify(
        &self,
        icon: Option<&str>,
        time: Duration,
        color: Option<&str>,
        message: &str,
    ) -> Result<String> {
        let icon = icon.unwrap_or("-1");
        let color = color.unwrap_or("0");

        self.send(&format!(
            "notify {} {} '{}' '{}'",
            icon,
            time.as_millis(),
            color,
            message
        ))
        .await
    }

    #[instrument(skip(self), fields(props = ?props), err)]
    pub(crate) async fn dismiss_notify(&self, props: DismissProps) -> Result<String> {
        match props {
            DismissProps::Total(n) => self.send(&format!("dismissnotify {n}")).await,
            DismissProps::All => self.send("dismissnotify").await,
        }
    }
}
