//! Update git stats with GitWatcher integration.

use std::path::Path;
use std::time::Instant;

use crate::services::git_watcher::GitWatcher;
use crate::widgets::markdown_widget::foundation::types::GitStats;
use crate::widgets::markdown_widget::state::git_stats_state::helpers::compute_git_stats;
use crate::widgets::markdown_widget::state::git_stats_state::GitStatsState;

impl GitStatsState {
    /// Update git stats using a GitWatcher for change detection.
    ///
    /// This method only computes git stats when the watcher detects
    /// that the git repository state has changed, making it more
    /// efficient than time-based polling.
    ///
    /// # Arguments
    ///
    /// * `source_path` - The path to the source file, if any.
    /// * `watcher` - A mutable reference to the GitWatcher.
    ///
    /// # Returns
    ///
    /// `true` if stats were updated, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_toolkit::markdown_widget::state::git_stats_state::GitStatsState;
    /// use ratatui_toolkit::services::git_watcher::GitWatcher;
    /// use std::path::Path;
    ///
    /// let mut git_stats = GitStatsState::new();
    /// git_stats.set_show(true);
    ///
    /// let mut watcher = GitWatcher::new().unwrap();
    /// watcher.watch(Path::new(".")).unwrap();
    ///
    /// // In your event loop:
    /// if git_stats.update_if_changed(Some(Path::new("README.md")), &mut watcher) {
    ///     println!("Git stats updated!");
    /// }
    /// ```
    pub fn update_if_changed(
        &mut self,
        source_path: Option<&Path>,
        watcher: &mut GitWatcher,
    ) -> bool {
        if !self.show {
            return false;
        }

        // Only update if the watcher detected changes
        if watcher.check_for_changes() {
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

    /// Force update git stats immediately.
    ///
    /// This bypasses both time-based and watcher-based checks
    /// and immediately computes git stats.
    ///
    /// # Arguments
    ///
    /// * `source_path` - The path to the source file, if any.
    pub fn force_update(&mut self, source_path: Option<&Path>) {
        let (adds, modified, dels) = compute_git_stats(source_path);
        self.cache = Some(GitStats {
            additions: adds,
            modified,
            deletions: dels,
        });
        self.last_update = Some(Instant::now());
    }
}
