//! Drain pending events without processing.

use std::sync::mpsc::TryRecvError;

use crate::services::git_watcher::GitWatcher;

impl GitWatcher {
    /// Drain all pending events without processing them.
    ///
    /// This clears the event queue and resets the pending changes flag,
    /// useful when you want to ignore accumulated changes
    /// (e.g., after a batch operation).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_toolkit::services::git_watcher::GitWatcher;
    /// use std::path::Path;
    ///
    /// let mut watcher = GitWatcher::new().unwrap();
    /// watcher.watch(Path::new("/path/to/repo")).unwrap();
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
        self.has_pending_changes = false;
    }
}
