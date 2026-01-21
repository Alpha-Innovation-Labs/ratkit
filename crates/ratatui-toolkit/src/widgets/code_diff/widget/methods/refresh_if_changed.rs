//! Refresh diff when repository changes are detected.

use super::super::CodeDiff;
use crate::services::repo_watcher::RepoWatcher;

impl CodeDiff {
    /// Refresh the diff if the repository watcher detected changes.
    ///
    /// Returns `true` when a refresh occurred.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ratatui_toolkit::code_diff::CodeDiff;
    /// use ratatui_toolkit::services::repo_watcher::RepoWatcher;
    /// use std::path::Path;
    ///
    /// let mut diff = CodeDiff::from_git();
    /// let mut watcher = RepoWatcher::new().unwrap();
    /// watcher.watch(Path::new("."))?;
    ///
    /// if diff.refresh_if_changed(&mut watcher) {
    ///     println!("Diff updated");
    /// }
    /// # Ok::<(), notify::Error>(())
    /// ```
    pub fn refresh_if_changed(&mut self, watcher: &mut RepoWatcher) -> bool {
        if watcher.check_for_changes() {
            self.refresh();
            true
        } else {
            false
        }
    }
}
