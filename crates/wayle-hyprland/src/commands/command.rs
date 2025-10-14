use std::time::Duration;

use super::CommandSocket;
use crate::Result;

pub enum OutputCommand<'a> {
    Create { backend: &'a str, name: &'a str },
    Remove { name: &'a str },
}

pub enum SetErrorCommand<'a> {
    Set { color: &'a str, message: &'a str },
    Disable,
}

pub enum DismissProps {
    All,
    Total(u32),
}

impl CommandSocket {
    pub(crate) async fn dispatch(&self, command: &str) -> Result<()> {
        todo!()
    }

    pub(crate) async fn keyword(&self, command: &str) -> Result<()> {
        todo!()
    }

    pub(crate) async fn reload(&self) -> Result<()> {
        todo!()
    }

    pub(crate) async fn kill(&self) -> Result<()> {
        todo!()
    }

    pub(crate) async fn set_cursor(&self, theme: &str, size: u8) -> Result<()> {
        todo!()
    }

    pub(crate) async fn output(&self, command: OutputCommand<'_>) -> Result<()> {
        todo!()
    }

    pub(crate) async fn switch_xkb_layout(&self, device: &str, command: &str) -> Result<()> {
        todo!()
    }

    pub(crate) async fn set_error(&self, command: SetErrorCommand<'_>) -> Result<()> {
        todo!()
    }

    pub(crate) async fn notify(
        &self,
        // -1 for none
        icon: Option<&str>,
        time: Duration,
        // 0 for default
        color: Option<&str>,
        message: &str,
    ) -> Result<()> {
        todo!()
    }

    pub(crate) async fn dismiss_notify(&self, props: DismissProps) -> Result<()> {
        todo!()
    }
}
