//! Default constructor for GitWatcher.

use crate::{GitWatchConfig, GitWatcher};

impl GitWatcher {
    /// Create a new git watcher with default configuration.
    ///
    /// Uses 100ms debounce by default.
    ///
    /// # Errors
    ///
    /// Returns a `notify::Error` if the watcher cannot be created.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_toolkit::services::git_watcher::GitWatcher;
    ///
    /// let watcher = GitWatcher::new().unwrap();
    /// ```
    pub fn new() -> Result<Self, notify::Error> {
        Self::with_config(GitWatchConfig::default())
    }
}
