//! Drain pending events without processing.

use super::super::MarkdownFileWatcher;

impl MarkdownFileWatcher {
    /// Drain all pending events without processing them.
    ///
    /// Useful when you want to clear the event queue without triggering
    /// any reloads (e.g., after programmatic file updates).
    pub fn drain_events(&self) {
        while self.rx.try_recv().is_ok() {}
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
    fn test_drain_events() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "initial").unwrap();

        let mut watcher = MarkdownFileWatcher::new().unwrap();
        watcher.watch(temp.path()).unwrap();

        // Modify the file
        thread::sleep(Duration::from_millis(50));
        let mut file = std::fs::File::create(temp.path()).unwrap();
        writeln!(file, "modified").unwrap();
        file.sync_all().unwrap();

        thread::sleep(Duration::from_millis(200));

        // Drain events
        watcher.drain_events();

        // Should be empty now
        assert!(!watcher.check_for_changes());
    }
}
