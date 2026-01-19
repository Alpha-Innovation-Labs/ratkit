//! Check for git change events.

use std::sync::mpsc::TryRecvError;

use crate::services::git_watcher::helpers::is_relevant_git_event;
use crate::services::git_watcher::GitWatcher;

impl GitWatcher {
    /// Check if there are any pending git state changes.
    ///
    /// This is a non-blocking operation that returns `true` if any
    /// relevant git changes have been detected since the last check.
    ///
    /// # Returns
    ///
    /// `true` if git changes were detected, `false` otherwise.
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
    /// // In your event loop:
    /// if watcher.check_for_changes() {
    ///     println!("Git state changed, recompute stats!");
    /// }
    /// ```
    pub fn check_for_changes(&mut self) -> bool {
        // Drain all pending events
        loop {
            match self.rx.try_recv() {
                Ok(Ok(event)) => {
                    if is_relevant_git_event(&event) {
                        self.has_pending_changes = true;
                    }
                }
                Ok(Err(_)) => {
                    // Watcher error, ignore
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }

        // Return and reset the pending changes flag
        let had_changes = self.has_pending_changes;
        self.has_pending_changes = false;
        had_changes
    }

    /// Check if there are pending changes without consuming them.
    ///
    /// This is useful when you want to know if there are changes
    /// but don't want to reset the flag yet.
    ///
    /// # Returns
    ///
    /// `true` if git changes are pending, `false` otherwise.
    pub fn has_pending_changes(&self) -> bool {
        self.has_pending_changes
    }
}
