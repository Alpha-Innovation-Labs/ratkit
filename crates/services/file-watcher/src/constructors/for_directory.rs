//! Convenience constructor for directory watching.

use crate::{FileWatcher, WatchConfig, WatchMode};

impl FileWatcher {
    /// Create a file watcher preset for watching a directory recursively.
    ///
    /// Uses `WatchMode::Recursive` with 200ms debounce (longer to handle
    /// rapid changes during builds or git operations).
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
    /// let mut watcher = FileWatcher::for_directory().unwrap();
    /// watcher.watch(Path::new("./src")).unwrap();
    /// ```
    pub fn for_directory() -> Result<Self, notify::Error> {
        Self::with_config(WatchConfig {
            mode: WatchMode::Recursive,
            debounce_ms: 200,
        })
    }
}
