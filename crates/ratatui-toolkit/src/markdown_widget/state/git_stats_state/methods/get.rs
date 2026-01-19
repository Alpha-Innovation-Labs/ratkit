//! Git stats getter method for GitStatsState.

use crate::markdown_widget::foundation::types::GitStats;
use crate::markdown_widget::state::git_stats_state::GitStatsState;

impl GitStatsState {
    /// Get the cached git stats.
    ///
    /// # Returns
    ///
    /// The cached `GitStats` if available and git stats are enabled.
    pub fn get(&self) -> Option<GitStats> {
        if self.show {
            self.cache
        } else {
            None
        }
    }

    /// Get the cached git stats (alias for `get()`).
    ///
    /// # Returns
    ///
    /// The cached `GitStats` if available and git stats are enabled.
    pub fn git_stats(&self) -> Option<GitStats> {
        self.get()
    }

    /// Check if git stats display is enabled.
    pub fn is_enabled(&self) -> bool {
        self.show
    }
}
