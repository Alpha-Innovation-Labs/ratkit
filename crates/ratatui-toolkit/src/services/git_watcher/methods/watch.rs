//! Watch a git repository for changes.

use notify::{RecursiveMode, Watcher};
use std::path::Path;

use crate::services::git_watcher::GitWatcher;

impl GitWatcher {
    /// Start watching a git repository for changes.
    ///
    /// Watches the `.git` directory recursively to detect any changes
    /// to the repository state (commits, staging, branches, etc.).
    ///
    /// # Arguments
    ///
    /// * `repo_path` - Path to the repository root (where `.git` is located).
    ///
    /// # Errors
    ///
    /// Returns a `notify::Error` if the path cannot be watched or
    /// if the `.git` directory doesn't exist.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_toolkit::services::git_watcher::GitWatcher;
    /// use std::path::Path;
    ///
    /// let mut watcher = GitWatcher::new().unwrap();
    /// watcher.watch(Path::new("/path/to/repo")).unwrap();
    /// ```
    pub fn watch(&mut self, repo_path: &Path) -> Result<(), notify::Error> {
        let git_dir = repo_path.join(".git");

        // Watch the .git directory recursively
        self.watcher.watch(&git_dir, RecursiveMode::Recursive)?;

        self.repo_path = Some(repo_path.to_path_buf());
        // Mark that we have pending changes on initial watch so stats are computed
        self.has_pending_changes = true;

        Ok(())
    }
}
