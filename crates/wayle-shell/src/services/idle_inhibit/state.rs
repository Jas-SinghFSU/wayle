//! Shared reactive state for idle inhibit.

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use wayle_common::Property;

/// Reactive state for idle inhibition.
///
/// Properties auto-update when D-Bus methods are called. Modules watch these
/// properties to create/destroy the actual Wayland inhibitor.
#[derive(Clone)]
pub struct IdleInhibitState {
    /// Whether inhibition is currently active.
    pub active: Property<bool>,
    /// Duration in minutes (0 = indefinite). Persists across enable/disable.
    pub duration_mins: Property<u32>,
    /// Remaining seconds on timer, None when inactive or indefinite.
    pub remaining_secs: Property<Option<u32>>,
    timer_token: Arc<Mutex<CancellationToken>>,
}

impl IdleInhibitState {
    pub fn new(initial_duration_mins: u32) -> Self {
        Self {
            active: Property::new(false),
            duration_mins: Property::new(initial_duration_mins),
            remaining_secs: Property::new(None),
            timer_token: Arc::new(Mutex::new(CancellationToken::new())),
        }
    }

    pub fn indefinite(&self) -> bool {
        self.duration_mins.get() == 0
    }

    /// Enables idle inhibition.
    ///
    /// If `indefinite` is true, enables without a timer regardless of stored duration.
    /// Otherwise uses the stored `duration_mins` (0 = indefinite).
    pub fn enable(&self, indefinite: bool) {
        self.cancel_timer();

        let use_timer = !indefinite && self.duration_mins.get() > 0;
        if use_timer {
            self.remaining_secs.set(Some(self.duration_mins.get() * 60));
            self.start_timer();
        } else {
            self.remaining_secs.set(None);
        }
        self.active.set(true);
    }

    pub fn disable(&self) {
        self.cancel_timer();
        self.active.set(false);
        self.remaining_secs.set(None);
    }

    pub fn set_duration(&self, minutes: u32) {
        self.duration_mins.set(minutes);

        if self.active.get() {
            self.cancel_timer();
            if minutes == 0 {
                self.remaining_secs.set(None);
            } else {
                self.remaining_secs.set(Some(minutes * 60));
                self.start_timer();
            }
        }
    }

    pub fn adjust_duration(&self, delta_minutes: i32) {
        let current = self.duration_mins.get();
        let new_val = if delta_minutes >= 0 {
            current.saturating_add(delta_minutes as u32)
        } else {
            current.saturating_sub(delta_minutes.unsigned_abs())
        };
        self.set_duration(new_val);
    }

    pub fn adjust_remaining(&self, delta_minutes: i32) {
        if !self.active.get() || self.indefinite() {
            return;
        }
        let Some(remaining) = self.remaining_secs.get() else {
            return;
        };

        let duration_secs = self.duration_mins.get() * 60;
        let delta_secs = delta_minutes * 60;
        let new_remaining = if delta_secs >= 0 {
            remaining
                .saturating_add(delta_secs as u32)
                .min(duration_secs)
        } else {
            remaining.saturating_sub(delta_secs.unsigned_abs())
        };

        if new_remaining == 0 {
            self.disable();
        } else {
            self.remaining_secs.set(Some(new_remaining));
        }
    }

    pub fn set_remaining(&self, minutes: u32) {
        if !self.active.get() || self.indefinite() {
            return;
        }

        let duration_secs = self.duration_mins.get() * 60;
        let new_remaining = (minutes * 60).min(duration_secs);
        if new_remaining == 0 {
            self.disable();
        } else {
            self.remaining_secs.set(Some(new_remaining));
        }
    }

    fn start_timer(&self) {
        let token = CancellationToken::new();
        {
            let mut guard = self.timer_token.lock().unwrap_or_else(|e| e.into_inner());
            *guard = token.clone();
        }

        let state = self.clone();
        tokio::spawn(async move {
            let mut tick = interval(Duration::from_secs(1));
            tick.tick().await;

            loop {
                tokio::select! {
                    _ = token.cancelled() => break,
                    _ = tick.tick() => {
                        let Some(remaining) = state.remaining_secs.get() else {
                            break;
                        };
                        if remaining <= 1 {
                            state.disable();
                            break;
                        }
                        state.remaining_secs.set(Some(remaining - 1));
                    }
                }
            }
        });
    }

    fn cancel_timer(&self) {
        let guard = self.timer_token.lock().unwrap_or_else(|e| e.into_inner());
        guard.cancel();
    }
}

impl Default for IdleInhibitState {
    fn default() -> Self {
        Self::new(0)
    }
}
