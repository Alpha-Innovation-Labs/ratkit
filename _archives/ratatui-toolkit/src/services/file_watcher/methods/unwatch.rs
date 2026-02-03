//! Stop watching a path.

use notify::Watcher;
use std::path::Path;

use crate::services::file_watcher::FileWatcher;

impl FileWatcher {
    /// Stop watching a path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to stop watching.
    ///
    /// # Errors
    ///
    /// Returns a `notify::Error` if the path cannot be unwatched.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_toolkit::services::file_watcher::FileWatcher;
    /// use std::path::Path;
    ///
    /// let mut watcher = FileWatcher::new().unwrap();
    /// let path = Path::new("README.md");
    /// watcher.watch(path).unwrap();
    /// // Later...
    /// watcher.unwatch(path).unwrap();
    /// ```
    pub fn unwatch(&mut self, path: &Path) -> Result<(), notify::Error> {
        self.watcher.unwatch(path)
    }
}
