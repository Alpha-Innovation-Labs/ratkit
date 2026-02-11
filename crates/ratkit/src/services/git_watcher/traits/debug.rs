//! Debug trait implementation for GitWatcher.

use std::fmt;

use crate::services::git_watcher::GitWatcher;

impl fmt::Debug for GitWatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GitWatcher")
            .field("config", &self.config)
            .field("repo_path", &self.repo_path)
            .field("has_pending_changes", &self.has_pending_changes)
            .finish_non_exhaustive()
    }
}
