//! D-Bus interface for idle inhibit control.

use tracing::instrument;
use zbus::{fdo, interface};

use super::state::IdleInhibitState;

pub const SERVICE_NAME: &str = "com.wayle.IdleInhibit1";
pub const SERVICE_PATH: &str = "/com/wayle/IdleInhibit";

pub struct IdleInhibitDaemon {
    state: IdleInhibitState,
}

impl IdleInhibitDaemon {
    pub fn new(state: IdleInhibitState) -> Self {
        Self { state }
    }
}

#[interface(name = "com.wayle.IdleInhibit1")]
impl IdleInhibitDaemon {
    /// Enable idle inhibition.
    ///
    /// If `indefinite` is true, enables without a timer for this session.
    /// Otherwise uses the stored duration.
    #[instrument(skip(self), fields(indefinite))]
    pub async fn enable(&self, indefinite: bool) {
        self.state.enable(indefinite);
    }

    #[instrument(skip(self))]
    pub async fn disable(&self) {
        self.state.disable();
    }

    #[instrument(skip(self), fields(delta_minutes))]
    pub async fn adjust_remaining(&self, delta_minutes: i32) -> fdo::Result<()> {
        if !self.state.active.get() {
            return Err(fdo::Error::Failed("idle inhibit is not active".to_string()));
        }
        if self.state.indefinite() {
            return Err(fdo::Error::Failed(
                "cannot adjust timer in indefinite mode".to_string(),
            ));
        }
        self.state.adjust_remaining(delta_minutes);
        Ok(())
    }

    #[instrument(skip(self), fields(minutes))]
    pub async fn set_remaining(&self, minutes: u32) -> fdo::Result<()> {
        if !self.state.active.get() {
            return Err(fdo::Error::Failed("idle inhibit is not active".to_string()));
        }
        if self.state.indefinite() {
            return Err(fdo::Error::Failed(
                "cannot set timer in indefinite mode".to_string(),
            ));
        }
        self.state.set_remaining(minutes);
        Ok(())
    }

    /// Set the duration in minutes (0 = indefinite).
    /// If active, also resets remaining time to the new duration.
    #[instrument(skip(self), fields(minutes))]
    pub async fn set_duration(&self, minutes: u32) {
        self.state.set_duration(minutes);
    }

    #[instrument(skip(self), fields(delta_minutes))]
    pub async fn adjust_duration(&self, delta_minutes: i32) {
        self.state.adjust_duration(delta_minutes);
    }

    #[zbus(property)]
    pub async fn active(&self) -> bool {
        self.state.active.get()
    }

    /// Duration in minutes (0 = indefinite).
    #[zbus(property)]
    pub async fn duration(&self) -> u32 {
        self.state.duration_mins.get()
    }

    /// Remaining seconds (0 when inactive or indefinite).
    #[zbus(property)]
    pub async fn remaining(&self) -> u32 {
        self.state.remaining_secs.get().unwrap_or(0)
    }

    #[zbus(property)]
    pub async fn indefinite(&self) -> bool {
        self.state.indefinite()
    }
}
