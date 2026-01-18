//! Check for file change events.

use std::sync::mpsc::TryRecvError;

use super::super::helpers::is_relevant_event;
use super::super::MarkdownFileWatcher;

impl MarkdownFileWatcher {
    /// Check if there are any pending file change events.
    ///
    /// This is a non-blocking operation that returns `true` if any
    /// relevant file modifications have been detected since the last check.
    ///
    /// # Returns
    ///
    /// `true` if file changes were detected, `false` otherwise.
    pub fn check_for_changes(&self) -> bool {
        let mut has_changes = false;

        loop {
            match self.rx.try_recv() {
                Ok(Ok(event)) => {
                    if is_relevant_event(&event) {
                        has_changes = true;
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

#[cfg(test)]
mod tests {
    use super::super::super::MarkdownFileWatcher;
    use std::io::Write;
    use std::thread;
    use std::time::Duration;
    use tempfile::NamedTempFile;

    #[test]
    fn test_check_for_changes_empty() {
        let temp = NamedTempFile::new().unwrap();
        let mut watcher = MarkdownFileWatcher::new().unwrap();
        watcher.watch(temp.path()).unwrap();

        // No changes yet
        assert!(!watcher.check_for_changes());
    }

    #[test]
    fn test_detect_file_modification() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "initial content").unwrap();

        let mut watcher = MarkdownFileWatcher::new().unwrap();
        watcher.watch(temp.path()).unwrap();

        // Modify the file
        thread::sleep(Duration::from_millis(50));
        let mut file = std::fs::File::create(temp.path()).unwrap();
        writeln!(file, "modified content").unwrap();
        file.sync_all().unwrap();

        // Give the watcher time to detect the change
        thread::sleep(Duration::from_millis(200));

        // Should detect the change
        assert!(watcher.check_for_changes());
    }
}
