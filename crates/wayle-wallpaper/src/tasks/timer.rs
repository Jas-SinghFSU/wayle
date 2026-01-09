//! Single timer for wallpaper cycling.

use std::time::Duration;

use tokio::time::{Instant, sleep_until};

/// Single timer for cycling interval.
///
/// Only one timer can be active at a time.
pub struct CyclingTimer {
    fires_at: Option<Instant>,
}

impl CyclingTimer {
    /// Creates a new timer with no scheduled fire time.
    pub fn new() -> Self {
        Self { fires_at: None }
    }

    /// Schedules the timer to fire after the given delay.
    ///
    /// Replaces any existing scheduled time.
    pub fn schedule(&mut self, delay: Duration) {
        self.fires_at = Some(Instant::now() + delay);
    }

    /// Cancels the scheduled timer.
    pub fn cancel(&mut self) {
        self.fires_at = None;
    }

    /// Returns whether a timer is scheduled.
    pub fn is_scheduled(&self) -> bool {
        self.fires_at.is_some()
    }

    /// Waits for the timer to fire.
    ///
    /// Returns `None` immediately if no timer is scheduled.
    /// After firing, the timer is cleared and must be rescheduled.
    pub async fn wait(&mut self) -> Option<()> {
        let fires_at = self.fires_at.take()?;
        sleep_until(fires_at).await;
        Some(())
    }
}

impl Default for CyclingTimer {
    fn default() -> Self {
        Self::new()
    }
}
