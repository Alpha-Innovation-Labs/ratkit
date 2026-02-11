//! Constructor with custom configuration.

use notify::{Config, RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::services::git_watcher::{GitWatchConfig, GitWatcher};

impl GitWatcher {
    /// Create a new git watcher with custom configuration.
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
    /// use ratatui_toolkit::services::git_watcher::{GitWatcher, GitWatchConfig};
    ///
    /// let config = GitWatchConfig::new()
    ///     .debounce_ms(200);
    ///
    /// let watcher = GitWatcher::with_config(config).unwrap();
    /// ```
    pub fn with_config(config: GitWatchConfig) -> Result<Self, notify::Error> {
        let (tx, rx) = channel();

        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default().with_poll_interval(Duration::from_millis(config.debounce_ms)),
        )?;

        Ok(Self {
            watcher,
            rx,
            config,
            repo_path: None,
            has_pending_changes: false,
        })
    }
}
