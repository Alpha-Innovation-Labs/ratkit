//! Default trait implementation for GitStatsState.

use crate::widgets::markdown_widget::state::git_stats_state::GitStatsState;

impl Default for GitStatsState {
    fn default() -> Self {
        Self::new()
    }
}
