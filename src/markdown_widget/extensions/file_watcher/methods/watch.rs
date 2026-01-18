//! Watch a file for changes.

use notify::{RecursiveMode, Watcher};
use std::path::Path;

use super::super::MarkdownFileWatcher;

impl MarkdownFileWatcher {
    /// Start watching a file for changes.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to watch.
    ///
    /// # Errors
    ///
    /// Returns a `notify::Error` if the path cannot be watched.
    pub fn watch(&mut self, path: &Path) -> Result<(), notify::Error> {
        self.watcher.watch(path, RecursiveMode::NonRecursive)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::MarkdownFileWatcher;
    use tempfile::NamedTempFile;

    #[test]
    fn test_watch_file() {
        let temp = NamedTempFile::new().unwrap();
        let mut watcher = MarkdownFileWatcher::new().unwrap();
        assert!(watcher.watch(temp.path()).is_ok());
    }
}
