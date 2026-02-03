//! Access the current change set.

use crate::{GitChangeSet, RepoWatcher};

impl RepoWatcher {
    /// Get the current change set and clear the cached copy.
    ///
    /// This returns the last computed change set. Call
    /// [`check_for_changes`](Self::check_for_changes) to refresh it.
    pub fn get_change_set(&mut self) -> GitChangeSet {
        std::mem::take(&mut self.change_set)
    }

    /// Peek at the current change set without clearing it.
    pub fn peek_change_set(&self) -> &GitChangeSet {
        &self.change_set
    }
}
