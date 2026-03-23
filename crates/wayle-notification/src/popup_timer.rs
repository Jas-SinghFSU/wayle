use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use wayle_core::Property;

use crate::core::notification::Notification;

struct PopupTimer {
    started_at: Instant,
    duration: Duration,
    cancel: CancellationToken,
}

impl PopupTimer {
    fn remaining(&self) -> Duration {
        self.duration.saturating_sub(self.started_at.elapsed())
    }
}

/// Manages pausable countdown timers for notification popups.
///
/// Each popup gets an independent timer that can be paused and resumed.
/// When a timer expires, the popup is removed from the visible list.
pub(crate) struct PopupTimerManager {
    timers: Mutex<HashMap<u32, PopupTimer>>,
    popups: Property<Vec<Arc<Notification>>>,
}

impl PopupTimerManager {
    pub(crate) fn new(popups: Property<Vec<Arc<Notification>>>) -> Self {
        Self {
            timers: Mutex::new(HashMap::new()),
            popups,
        }
    }

    /// Starts a countdown timer for a popup.
    ///
    /// If a timer already exists for this ID, it is cancelled first.
    pub(crate) fn start(self: &Arc<Self>, id: u32, duration: Duration) {
        let cancel = CancellationToken::new();

        {
            let Ok(mut timers) = self.timers.lock() else {
                return;
            };

            if let Some(existing) = timers.get(&id) {
                existing.cancel.cancel();
            }

            timers.insert(
                id,
                PopupTimer {
                    started_at: Instant::now(),
                    duration,
                    cancel: cancel.clone(),
                },
            );
        }

        let manager = Arc::clone(self);
        tokio::spawn(async move {
            tokio::select! {
                () = tokio::time::sleep(duration) => {
                    manager.remove_popup(id);
                }
                () = cancel.cancelled() => {}
            }
        });
    }

    /// Pauses the countdown for a popup.
    ///
    /// Captures remaining time so [`resume`] continues from where it left off.
    pub(crate) fn pause(&self, id: u32) {
        let Ok(mut timers) = self.timers.lock() else {
            return;
        };
        let Some(timer) = timers.get_mut(&id) else {
            return;
        };

        let remaining = timer.remaining();
        timer.cancel.cancel();
        timer.duration = remaining;
    }

    /// Resumes the countdown after a pause.
    ///
    /// Uses the remaining duration captured during [`pause`].
    /// Dismisses immediately if time expired while paused.
    pub(crate) fn resume(self: &Arc<Self>, id: u32) {
        let remaining = {
            let Ok(timers) = self.timers.lock() else {
                return;
            };
            let Some(timer) = timers.get(&id) else {
                return;
            };
            timer.duration
        };

        if remaining.is_zero() {
            self.remove_popup(id);
            return;
        }

        self.start(id, remaining);
    }

    /// Cancels and removes the timer for a popup.
    pub(crate) fn cancel(&self, id: u32) {
        if let Ok(mut timers) = self.timers.lock()
            && let Some(timer) = timers.remove(&id)
        {
            timer.cancel.cancel();
        }
    }

    fn remove_popup(&self, id: u32) {
        tracing::debug!(id = id, "popup timer expired");

        {
            let Ok(mut timers) = self.timers.lock() else {
                return;
            };
            timers.remove(&id);
        }

        let mut list = self.popups.get();
        list.retain(|popup| popup.id != id);
        self.popups.set(list);
    }
}
