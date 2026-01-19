//! Drain pending events without processing.

use std::sync::mpsc::TryRecvError;

use crate::services::file_watcher::FileWatcher;

impl FileWatcher {
    /// Drain all pending events without processing them.
    ///
    /// This clears the event queue and the changed paths list,
    /// useful when you want to ignore accumulated changes
    /// (e.g., after a batch operation).
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
    /// // After some operation that causes many changes:
    /// watcher.drain_events();
    /// ```
    pub fn drain_events(&mut self) {
        loop {
            match self.rx.try_recv() {
                Ok(_) => continue,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }
        self.changed_paths.clear();
    }
}
