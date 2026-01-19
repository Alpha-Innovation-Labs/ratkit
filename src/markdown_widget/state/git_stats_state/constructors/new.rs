//! Constructor for GitStatsState.

use crate::markdown_widget::state::git_stats_state::GitStatsState;

impl GitStatsState {
    /// Create a new git stats state with defaults.
    pub fn new() -> Self {
        Self {
            show: false,
            cache: None,
            last_update: None,
        }
    }
}
