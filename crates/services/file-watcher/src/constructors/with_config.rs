//! Constructor with custom configuration.

use notify::{Config, RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::{FileWatcher, WatchConfig};

impl FileWatcher {
    /// Create a new file watcher with custom configuration.
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
    /// use ratatui_toolkit::services::file_watcher::{FileWatcher, WatchConfig, WatchMode};
    ///
    /// let config = WatchConfig::new()
    ///     .mode(WatchMode::Recursive)
    ///     .debounce_ms(200);
    ///
    /// let watcher = FileWatcher::with_config(config).unwrap();
    /// ```
    pub fn with_config(config: WatchConfig) -> Result<Self, notify::Error> {
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
            changed_paths: Vec::new(),
        })
    }
}
