//! Default constructor for FileWatcher.

use notify::{Config, RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::services::file_watcher::{FileWatcher, WatchConfig};

impl FileWatcher {
    /// Create a new file watcher with default configuration.
    ///
    /// Uses `WatchMode::File` (non-recursive) and 100ms debounce by default.
    ///
    /// # Errors
    ///
    /// Returns a `notify::Error` if the watcher cannot be created.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_toolkit::services::file_watcher::FileWatcher;
    ///
    /// let watcher = FileWatcher::new().unwrap();
    /// ```
    pub fn new() -> Result<Self, notify::Error> {
        Self::with_config(WatchConfig::default())
    }
}
