use std::path::PathBuf;

/// State for a single monitor.
///
/// Tracks the current wallpaper and cycling position for one monitor.
/// All configuration (fit mode, cycling config, etc.) is shared at the
/// service level.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonitorState {
    /// Current wallpaper displayed on this monitor.
    pub wallpaper: Option<PathBuf>,
    /// Position in the cycling image list (only used when cycling is active).
    pub cycle_index: usize,
}

impl MonitorState {
    /// Creates a new monitor state with no wallpaper.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new monitor state with a specific cycle index.
    pub fn with_cycle_index(cycle_index: usize) -> Self {
        Self {
            wallpaper: None,
            cycle_index,
        }
    }

    /// Advances the cycle index, wrapping around at the given count.
    pub fn advance(&mut self, image_count: usize) {
        if image_count > 0 {
            self.cycle_index = (self.cycle_index + 1) % image_count;
        }
    }

    /// Goes back in the cycle, wrapping around at the given count.
    pub fn previous(&mut self, image_count: usize) {
        if image_count > 0 {
            self.cycle_index = if self.cycle_index == 0 {
                image_count - 1
            } else {
                self.cycle_index - 1
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advance_increments_index() {
        let mut state = MonitorState::with_cycle_index(0);

        state.advance(5);
        assert_eq!(state.cycle_index, 1);

        state.advance(5);
        assert_eq!(state.cycle_index, 2);
    }

    #[test]
    fn advance_wraps_at_count() {
        let mut state = MonitorState::with_cycle_index(4);

        state.advance(5);

        assert_eq!(state.cycle_index, 0);
    }

    #[test]
    fn previous_decrements_index() {
        let mut state = MonitorState::with_cycle_index(3);

        state.previous(5);
        assert_eq!(state.cycle_index, 2);

        state.previous(5);
        assert_eq!(state.cycle_index, 1);
    }

    #[test]
    fn previous_wraps_at_zero() {
        let mut state = MonitorState::with_cycle_index(0);

        state.previous(5);

        assert_eq!(state.cycle_index, 4);
    }

    #[test]
    fn advance_does_nothing_for_zero_count() {
        let mut state = MonitorState::with_cycle_index(0);

        state.advance(0);

        assert_eq!(state.cycle_index, 0);
    }

    #[test]
    fn previous_does_nothing_for_zero_count() {
        let mut state = MonitorState::with_cycle_index(0);

        state.previous(0);

        assert_eq!(state.cycle_index, 0);
    }
}
