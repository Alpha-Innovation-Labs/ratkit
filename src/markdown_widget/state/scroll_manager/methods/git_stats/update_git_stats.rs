//! Update git stats method for MarkdownScrollManager.

use std::time::Instant;

use crate::markdown_widget::foundation::types::GitStats;
use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;

use super::helpers::compute_git_stats;

/// Update interval for git stats (in seconds).
const GIT_STATS_UPDATE_INTERVAL_SECS: u64 = 2;

impl MarkdownScrollManager {
    /// Update git stats if show_git_stats is enabled and enough time has passed.
    ///
    /// This method should be called periodically (e.g., in the render loop).
    /// It only computes stats every 2 seconds to avoid excessive git calls.
    ///
    /// # Returns
    ///
    /// `true` if stats were updated, `false` otherwise.
    pub fn update_git_stats(&mut self) -> bool {
        if !self.show_git_stats {
            return false;
        }

        let should_update = match self.git_stats_last_update {
            Some(last_update) => last_update.elapsed().as_secs() >= GIT_STATS_UPDATE_INTERVAL_SECS,
            None => true, // First update
        };

        if should_update {
            let (adds, modified, dels) = compute_git_stats(self.source_path());
            self.git_stats_cache = Some(GitStats {
                additions: adds,
                modified,
                deletions: dels,
            });
            self.git_stats_last_update = Some(Instant::now());
            true
        } else {
            false
        }
    }
}
