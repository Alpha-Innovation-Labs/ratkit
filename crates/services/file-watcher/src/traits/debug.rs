//! Debug trait implementation for FileWatcher.

use std::fmt;

use crate::FileWatcher;

impl fmt::Debug for FileWatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileWatcher")
            .field("config", &self.config)
            .field("changed_paths", &self.changed_paths)
            .finish_non_exhaustive()
    }
}
