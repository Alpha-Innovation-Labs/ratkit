//! Debug trait implementation for RepoWatcher.

use std::fmt;

use crate::services::repo_watcher::RepoWatcher;

impl fmt::Debug for RepoWatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RepoWatcher")
            .field("config", &self.config)
            .field("repo_path", &self.repo_path)
            .field("has_pending_changes", &self.has_pending_changes)
            .field("change_set", &self.change_set)
            .finish_non_exhaustive()
    }
}
