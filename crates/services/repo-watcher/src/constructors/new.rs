//! Default constructor for RepoWatcher.

use crate::{RepoWatchConfig, RepoWatcher};

impl RepoWatcher {
    /// Create a new repo watcher with default configuration.
    ///
    /// # Errors
    ///
    /// Returns a `notify::Error` if the watcher cannot be created.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_toolkit::services::repo_watcher::RepoWatcher;
    ///
    /// let watcher = RepoWatcher::new().unwrap();
    /// ```
    pub fn new() -> Result<Self, notify::Error> {
        Self::with_config(RepoWatchConfig::default())
    }
}
