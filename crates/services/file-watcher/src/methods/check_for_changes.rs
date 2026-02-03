//! Check for file change events.

use std::sync::mpsc::TryRecvError;

use crate::helpers::is_relevant_event;
use crate::FileWatcher;

impl FileWatcher {
    /// Check if there are any pending file change events.
    ///
    /// This is a non-blocking operation that returns `true` if any
    /// relevant file modifications have been detected since the last check.
    /// Changed paths are stored internally and can be retrieved with
    /// [`get_changed_paths`](Self::get_changed_paths).
    ///
    /// # Returns
    ///
    /// `true` if file changes were detected, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_toolkit::services::file_watcher::FileWatcher;
    /// use std::path::Path;
    ///
    /// let mut watcher = FileWatcher::new().unwrap();
    /// watcher.watch(Path::new("README.md")).unwrap();
    ///
    /// // In your event loop:
    /// if watcher.check_for_changes() {
    ///     println!("Files changed!");
    ///     let paths = watcher.get_changed_paths();
    ///     for path in paths {
    ///         println!("  - {}", path.display());
    ///     }
    /// }
    /// ```
    pub fn check_for_changes(&mut self) -> bool {
        let mut has_changes = false;

        loop {
            match self.rx.try_recv() {
                Ok(Ok(event)) => {
                    if is_relevant_event(&event) {
                        has_changes = true;
                        // Collect the paths that changed
                        for path in event.paths {
                            if !self.changed_paths.contains(&path) {
                                self.changed_paths.push(path);
                            }
                        }
                    }
                }
                Ok(Err(_)) => {
                    // Watcher error, ignore
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }

        has_changes
    }
}
