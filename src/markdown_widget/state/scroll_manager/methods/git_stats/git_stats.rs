//! Git stats getter method for MarkdownScrollManager.

use crate::markdown_widget::foundation::types::GitStats;
use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Get the cached git stats.
    ///
    /// # Returns
    ///
    /// The cached `GitStats` if available and git stats are enabled.
    pub fn git_stats(&self) -> Option<GitStats> {
        if self.show_git_stats {
            self.git_stats_cache
        } else {
            None
        }
    }
}
