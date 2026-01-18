//! Stop watching a file.

use notify::Watcher;
use std::path::Path;

use super::super::MarkdownFileWatcher;

impl MarkdownFileWatcher {
    /// Stop watching a file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to stop watching.
    ///
    /// # Errors
    ///
    /// Returns a `notify::Error` if the path cannot be unwatched.
    pub fn unwatch(&mut self, path: &Path) -> Result<(), notify::Error> {
        self.watcher.unwatch(path)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::MarkdownFileWatcher;
    use tempfile::NamedTempFile;

    #[test]
    fn test_unwatch_file() {
        let temp = NamedTempFile::new().unwrap();
        let mut watcher = MarkdownFileWatcher::new().unwrap();
        watcher.watch(temp.path()).unwrap();
        assert!(watcher.unwatch(temp.path()).is_ok());
    }
}
