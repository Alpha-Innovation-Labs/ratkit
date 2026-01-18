//! Constructor for MarkdownFileWatcher.

use notify::{Config, RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

use super::super::MarkdownFileWatcher;

impl MarkdownFileWatcher {
    /// Create a new file watcher.
    ///
    /// # Errors
    ///
    /// Returns a `notify::Error` if the watcher cannot be created.
    pub fn new() -> Result<Self, notify::Error> {
        let (tx, rx) = channel();

        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default().with_poll_interval(Duration::from_millis(100)),
        )?;

        Ok(Self { watcher, rx })
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::MarkdownFileWatcher;

    #[test]
    fn test_watcher_creation() {
        let watcher = MarkdownFileWatcher::new();
        assert!(watcher.is_ok());
    }
}
