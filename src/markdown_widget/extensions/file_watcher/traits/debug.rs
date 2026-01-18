//! Debug trait implementation for MarkdownFileWatcher.

use super::super::MarkdownFileWatcher;

impl std::fmt::Debug for MarkdownFileWatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkdownFileWatcher")
            .field("watcher", &"RecommendedWatcher")
            .field("rx", &"Receiver<...>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::MarkdownFileWatcher;

    #[test]
    fn test_debug_impl() {
        let watcher = MarkdownFileWatcher::new().unwrap();
        let debug_str = format!("{:?}", watcher);
        assert!(debug_str.contains("MarkdownFileWatcher"));
    }
}
