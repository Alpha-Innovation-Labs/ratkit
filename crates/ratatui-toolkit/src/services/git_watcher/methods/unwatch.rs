//! Stop watching a git repository.

use notify::Watcher;
use std::path::Path;

use crate::services::git_watcher::GitWatcher;

impl GitWatcher {
    /// Stop watching a git repository.
    ///
    /// # Arguments
    ///
    /// * `repo_path` - Path to the repository root (where `.git` is located).
    ///
    /// # Errors
    ///
    /// Returns a `notify::Error` if the path cannot be unwatched.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_toolkit::services::git_watcher::GitWatcher;
    /// use std::path::Path;
    ///
    /// let mut watcher = GitWatcher::new().unwrap();
    /// let path = Path::new("/path/to/repo");
    /// watcher.watch(path).unwrap();
    /// // Later...
    /// watcher.unwatch(path).unwrap();
    /// ```
    pub fn unwatch(&mut self, repo_path: &Path) -> Result<(), notify::Error> {
        let git_dir = repo_path.join(".git");
        self.watcher.unwatch(&git_dir)?;
        self.repo_path = None;
        self.has_pending_changes = false;
        Ok(())
    }
}
