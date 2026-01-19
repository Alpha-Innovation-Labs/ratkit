//! Convenience constructor for single file watching.

use crate::services::file_watcher::{FileWatcher, WatchConfig, WatchMode};

impl FileWatcher {
    /// Create a file watcher preset for watching a single file.
    ///
    /// Uses `WatchMode::File` (non-recursive) with 100ms debounce.
    ///
    /// # Errors
    ///
    /// Returns a `notify::Error` if the watcher cannot be created.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_toolkit::services::file_watcher::FileWatcher;
    /// use std::path::Path;
    ///
    /// let mut watcher = FileWatcher::for_file().unwrap();
    /// watcher.watch(Path::new("config.toml")).unwrap();
    /// ```
    pub fn for_file() -> Result<Self, notify::Error> {
        Self::with_config(WatchConfig {
            mode: WatchMode::File,
            debounce_ms: 100,
        })
    }
}
