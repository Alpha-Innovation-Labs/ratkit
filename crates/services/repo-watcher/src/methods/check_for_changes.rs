//! Check for repository change events.

use crate::helpers::collect_git_changes;
use crate::{GitChangeSet, RepoWatcher};

impl RepoWatcher {
    /// Check if there are any pending repository changes.
    ///
    /// This drains both the git watcher and file watcher. When changes
    /// are detected, the cached change set is refreshed.
    ///
    /// # Returns
    ///
    /// `true` if changes were detected, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_toolkit::services::repo_watcher::RepoWatcher;
    /// use std::path::Path;
    ///
    /// let mut watcher = RepoWatcher::new().unwrap();
    /// watcher.watch(Path::new("/path/to/repo")).unwrap();
    ///
    /// if watcher.check_for_changes() {
    ///     let changes = watcher.get_change_set();
    ///     println!("Changed files: {}", changes.all_paths().len());
    /// }
    /// ```
    pub fn check_for_changes(&mut self) -> bool {
        let mut has_changes = self.has_pending_changes;
        self.has_pending_changes = false;

        if self.git_watcher.check_for_changes() {
            has_changes = true;
        }

        if self.file_watcher.check_for_changes() {
            has_changes = true;
            let _ = self.file_watcher.get_changed_paths();
        }

        if has_changes {
            if let Some(repo_path) = self.repo_path.as_ref() {
                self.change_set = collect_git_changes(repo_path, self.config.include_untracked);
            } else {
                self.change_set = GitChangeSet::default();
            }
        }

        has_changes
    }
}
