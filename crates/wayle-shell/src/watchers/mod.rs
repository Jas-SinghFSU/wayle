//! Shell watchers for reactive state synchronization.
//!
//! Spawns background tasks that watch service state and trigger shell updates.

mod css;
mod monitors;

use relm4::ComponentSender;

use crate::shell::Shell;

/// Initializes all shell watchers.
///
/// Call this once during shell initialization to spawn all background
/// watching tasks.
pub fn init(sender: &ComponentSender<Shell>) {
    css::spawn(sender);
    monitors::spawn(sender);
}
