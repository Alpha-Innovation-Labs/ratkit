//! Constructor with custom configuration.

use crate::{GitChangeSet, RepoWatchConfig, RepoWatcher};
use ratkit_file_watcher::FileWatcher;
use ratkit_git_watcher::GitWatcher;

impl RepoWatcher {
    /// Create a new repo watcher with custom configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the watcher.
    ///
    /// # Errors
    ///
    /// Returns a `notify::Error` if the watcher cannot be created.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_toolkit::services::repo_watcher::RepoWatchConfig;
    /// use ratatui_toolkit::services::repo_watcher::RepoWatcher;
    /// use ratatui_toolkit::services::file_watcher::{WatchConfig, WatchMode};
    ///
    /// let config = RepoWatchConfig::new().file_config(
    ///     WatchConfig::new().mode(WatchMode::Recursive).debounce_ms(150),
    /// );
    ///
    /// let watcher = RepoWatcher::with_config(config).unwrap();
    /// ```
    pub fn with_config(config: RepoWatchConfig) -> Result<Self, notify::Error> {
        let git_watcher = GitWatcher::with_config(config.git.clone())?;
        let file_watcher = FileWatcher::with_config(config.files.clone())?;

        Ok(Self {
            git_watcher,
            file_watcher,
            config,
            repo_path: None,
            change_set: GitChangeSet::default(),
            has_pending_changes: false,
        })
    }
}
