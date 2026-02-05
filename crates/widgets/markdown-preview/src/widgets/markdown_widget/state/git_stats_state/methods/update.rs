//! Update git stats method for GitStatsState.

use std::path::Path;
use std::time::Instant;

use crate::widgets::markdown_widget::foundation::types::GitStats;
use crate::widgets::markdown_widget::state::git_stats_state::helpers::compute_git_stats;
use crate::widgets::markdown_widget::state::git_stats_state::GitStatsState;

/// Update interval for git stats (in seconds).
const GIT_STATS_UPDATE_INTERVAL_SECS: u64 = 2;

impl GitStatsState {
    /// Update git stats if show is enabled and enough time has passed.
    ///
    /// This method should be called periodically (e.g., in the render loop).
    /// It only computes stats every 2 seconds to avoid excessive git calls.
    ///
    /// # Arguments
    ///
    /// * `source_path` - The path to the source file, if any.
    ///
    /// # Returns
    ///
    /// `true` if stats were updated, `false` otherwise.
    pub fn update(&mut self, source_path: Option<&Path>) -> bool {
        if !self.show {
            return false;
        }

        let should_update = match self.last_update {
            Some(last_update) => last_update.elapsed().as_secs() >= GIT_STATS_UPDATE_INTERVAL_SECS,
            None => true, // First update
        };

        if should_update {
            let (adds, modified, dels) = compute_git_stats(source_path);
            self.cache = Some(GitStats {
                additions: adds,
                modified,
                deletions: dels,
            });
            self.last_update = Some(Instant::now());
            true
        } else {
            false
        }
    }
}
