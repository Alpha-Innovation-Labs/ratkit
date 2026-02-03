//! Get paths that have changed.

use std::path::PathBuf;

use crate::FileWatcher;

impl FileWatcher {
    /// Get the paths that have changed since the last call.
    ///
    /// This returns and clears the internal list of changed paths.
    /// Call [`check_for_changes`](Self::check_for_changes) first to
    /// populate this list.
    ///
    /// # Returns
    ///
    /// A vector of paths that have changed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_toolkit::services::file_watcher::FileWatcher;
    /// use std::path::Path;
    ///
    /// let mut watcher = FileWatcher::for_directory().unwrap();
    /// watcher.watch(Path::new("./src")).unwrap();
    ///
    /// // In your event loop:
    /// if watcher.check_for_changes() {
    ///     for path in watcher.get_changed_paths() {
    ///         println!("Changed: {}", path.display());
    ///     }
    /// }
    /// ```
    pub fn get_changed_paths(&mut self) -> Vec<PathBuf> {
        std::mem::take(&mut self.changed_paths)
    }

    /// Peek at the changed paths without clearing them.
    ///
    /// Unlike [`get_changed_paths`](Self::get_changed_paths), this does
    /// not clear the internal list.
    ///
    /// # Returns
    ///
    /// A slice of paths that have changed.
    pub fn peek_changed_paths(&self) -> &[PathBuf] {
        &self.changed_paths
    }
}
