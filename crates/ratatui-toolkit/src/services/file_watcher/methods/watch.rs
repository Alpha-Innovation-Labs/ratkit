//! Watch a path for changes.

use notify::{RecursiveMode, Watcher};
use std::path::Path;

use crate::services::file_watcher::{FileWatcher, WatchMode};

impl FileWatcher {
    /// Start watching a path for changes.
    ///
    /// The watch mode (recursive or non-recursive) is determined by the
    /// configuration used when creating the watcher.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file or directory to watch.
    ///
    /// # Errors
    ///
    /// Returns a `notify::Error` if the path cannot be watched.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_toolkit::services::file_watcher::FileWatcher;
    /// use std::path::Path;
    ///
    /// let mut watcher = FileWatcher::new().unwrap();
    /// watcher.watch(Path::new("README.md")).unwrap();
    /// ```
    pub fn watch(&mut self, path: &Path) -> Result<(), notify::Error> {
        let mode = match self.config.mode {
            WatchMode::File => RecursiveMode::NonRecursive,
            WatchMode::Recursive => RecursiveMode::Recursive,
        };
        self.watcher.watch(path, mode)
    }
}
