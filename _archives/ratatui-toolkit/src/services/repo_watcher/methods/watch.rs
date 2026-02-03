//! Watch a repository for changes.

use std::path::Path;

use crate::services::repo_watcher::helpers::collect_git_changes;
use crate::services::repo_watcher::RepoWatcher;

impl RepoWatcher {
    /// Start watching a repository for changes.
    ///
    /// This watches the `.git` directory for git state changes and the
    /// working tree for file edits. The initial change set is computed
    /// immediately after watching starts.
    ///
    /// # Arguments
    ///
    /// * `repo_path` - Path to the repository root (where `.git` is located).
    ///
    /// # Errors
    ///
    /// Returns a `notify::Error` if the path cannot be watched.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_toolkit::services::repo_watcher::RepoWatcher;
    /// use std::path::Path;
    ///
    /// let mut watcher = RepoWatcher::new().unwrap();
    /// watcher.watch(Path::new("/path/to/repo")).unwrap();
    /// ```
    pub fn watch(&mut self, repo_path: &Path) -> Result<(), notify::Error> {
        self.git_watcher.watch(repo_path)?;
        self.file_watcher.watch(repo_path)?;

        self.repo_path = Some(repo_path.to_path_buf());
        self.change_set = collect_git_changes(repo_path, self.config.include_untracked);
        self.has_pending_changes = true;

        Ok(())
    }
}
