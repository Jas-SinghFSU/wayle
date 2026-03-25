//! Bar visibility domain logic.

use std::collections::HashSet;

use tracing::{instrument, warn};
use zbus::fdo;

use super::state::ShellIpcState;

/// Bar visibility logic. Validates connector names against known monitors
/// before mutating [`ShellIpcState::hidden_bars`].
pub(crate) struct BarVisibility {
    state: ShellIpcState,
}

impl BarVisibility {
    pub(crate) fn new(state: ShellIpcState) -> Self {
        Self { state }
    }

    fn is_known_connector(&self, monitor: &str) -> bool {
        self.state
            .connectors
            .get()
            .iter()
            .any(|connector| connector == monitor)
    }

    #[instrument(skip(self), fields(monitor))]
    pub(crate) fn hide(&self, monitor: &str) {
        let mut set = self.state.hidden_bars.get();

        if monitor.is_empty() {
            set.extend(self.state.connectors.get());
        } else if self.is_known_connector(monitor) {
            set.insert(monitor.to_owned());
        } else {
            warn!(monitor, "unknown connector, ignoring hide");
            return;
        }

        self.state.hidden_bars.set(set);
    }

    #[instrument(skip(self), fields(monitor))]
    pub(crate) fn show(&self, monitor: &str) {
        if monitor.is_empty() {
            self.state.hidden_bars.set(HashSet::new());
            return;
        }

        if !self.is_known_connector(monitor) {
            warn!(monitor, "unknown connector, ignoring show");
            return;
        }

        let mut set = self.state.hidden_bars.get();
        set.remove(monitor);
        self.state.hidden_bars.set(set);
    }

    #[instrument(skip(self), fields(monitor))]
    pub(crate) fn toggle(&self, monitor: &str) -> fdo::Result<()> {
        if monitor.is_empty() {
            return self.toggle_all();
        }

        if !self.is_known_connector(monitor) {
            warn!(monitor, "unknown connector, ignoring toggle");
            return Ok(());
        }

        let mut set = self.state.hidden_bars.get();

        if !set.remove(monitor) {
            set.insert(monitor.to_owned());
        }

        self.state.hidden_bars.set(set);
        Ok(())
    }

    fn toggle_all(&self) -> fdo::Result<()> {
        let set = self.state.hidden_bars.get();

        let hidden_bars = if set.is_empty() {
            HashSet::from_iter(self.state.connectors.get())
        } else {
            HashSet::new()
        };

        self.state.hidden_bars.set(hidden_bars);

        Ok(())
    }
}
