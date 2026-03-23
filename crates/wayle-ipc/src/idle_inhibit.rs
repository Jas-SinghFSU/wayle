//! D-Bus client proxy for the idle inhibit service.
#![allow(missing_docs)]

use zbus::{Result, proxy};

pub const SERVICE_NAME: &str = "com.wayle.IdleInhibit1";
pub const SERVICE_PATH: &str = "/com/wayle/IdleInhibit";

#[proxy(
    interface = "com.wayle.IdleInhibit1",
    default_service = "com.wayle.IdleInhibit1",
    default_path = "/com/wayle/IdleInhibit",
    gen_blocking = false
)]
pub trait IdleInhibit {
    async fn enable(&self, indefinite: bool) -> Result<()>;

    async fn disable(&self) -> Result<()>;

    async fn adjust_remaining(&self, delta_minutes: i32) -> Result<()>;

    async fn set_remaining(&self, minutes: u32) -> Result<()>;

    async fn adjust_duration(&self, delta_minutes: i32) -> Result<()>;

    async fn set_duration(&self, minutes: u32) -> Result<()>;

    #[zbus(property)]
    fn active(&self) -> Result<bool>;

    #[zbus(property)]
    fn duration(&self) -> Result<u32>;

    #[zbus(property)]
    fn remaining(&self) -> Result<u32>;

    #[zbus(property)]
    fn indefinite(&self) -> Result<bool>;
}
