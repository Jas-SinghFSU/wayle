use std::time::Duration;

use tokio::time::{Instant, sleep_until};

pub(super) struct CyclingTimer {
    fires_at: Option<Instant>,
}

impl CyclingTimer {
    pub fn new() -> Self {
        Self { fires_at: None }
    }

    pub fn schedule(&mut self, delay: Duration) {
        self.fires_at = Some(Instant::now() + delay);
    }

    pub fn cancel(&mut self) {
        self.fires_at = None;
    }

    pub fn is_scheduled(&self) -> bool {
        self.fires_at.is_some()
    }

    pub async fn wait(&mut self) -> Option<()> {
        let fires_at = self.fires_at?;
        sleep_until(fires_at).await;
        self.fires_at = None;
        Some(())
    }
}

impl Default for CyclingTimer {
    fn default() -> Self {
        Self::new()
    }
}
