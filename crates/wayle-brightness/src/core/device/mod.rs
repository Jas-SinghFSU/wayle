mod monitoring;

use derive_more::Debug;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;
use wayle_common::Property;

use crate::{
    Error,
    backend::types::{Command, CommandSender, EventSender},
    types::{BacklightInfo, BacklightType, DeviceName, Percentage},
};

/// A discovered backlight device.
#[derive(Debug)]
pub struct BacklightDevice {
    #[debug(skip)]
    command_tx: CommandSender,

    #[debug(skip)]
    pub(crate) cancellation_token: Option<CancellationToken>,

    #[debug(skip)]
    pub(crate) event_tx: Option<EventSender>,

    /// e.g., `intel_backlight`, `amdgpu_bl0`.
    pub name: DeviceName,

    /// Controls primary device selection priority.
    pub backlight_type: BacklightType,

    /// Hardware-reported ceiling for raw brightness writes.
    pub max_brightness: u32,

    /// Raw brightness value, updates automatically via sysfs polling.
    pub brightness: Property<u32>,
}

impl BacklightDevice {
    pub(crate) fn from_info(
        info: &BacklightInfo,
        command_tx: CommandSender,
        event_tx: Option<EventSender>,
        cancellation_token: Option<CancellationToken>,
    ) -> Self {
        Self {
            command_tx,
            cancellation_token,
            event_tx,
            name: info.name.clone(),
            backlight_type: info.backlight_type,
            max_brightness: info.max_brightness,
            brightness: Property::new(info.brightness),
        }
    }

    pub(crate) fn update_brightness(&self, value: u32) {
        self.brightness.set(value);
    }

    /// Sets brightness to the given raw value, clamped to `max_brightness`.
    ///
    /// # Errors
    ///
    /// Returns error if the write fails via both logind and sysfs.
    pub async fn set_brightness(&self, value: u32) -> Result<(), Error> {
        let clamped = value.min(self.max_brightness);

        let (tx, rx) = oneshot::channel();

        self.command_tx
            .send(Command::SetBrightness {
                name: self.name.to_string(),
                value: clamped,
                responder: tx,
            })
            .map_err(|_| Error::CommandChannelDisconnected)?;

        rx.await.map_err(|_| Error::CommandChannelDisconnected)?
    }

    /// Computes percentage from raw brightness and max.
    pub fn percentage(&self) -> Percentage {
        if self.max_brightness == 0 {
            return Percentage::new(0.0);
        }

        let value = (f64::from(self.brightness.get()) / f64::from(self.max_brightness)) * 100.0;
        Percentage::new(value)
    }

    /// Sets brightness by percentage, rounded to nearest raw value.
    ///
    /// # Errors
    ///
    /// Returns error if the write fails via both logind and sysfs.
    pub async fn set_percentage(&self, percent: Percentage) -> Result<(), Error> {
        let raw = (percent.fraction() * f64::from(self.max_brightness)).round() as u32;
        self.set_brightness(raw).await
    }
}

impl PartialEq for BacklightDevice {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
