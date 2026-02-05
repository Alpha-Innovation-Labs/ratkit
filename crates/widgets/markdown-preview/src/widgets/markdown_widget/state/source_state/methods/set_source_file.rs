//! Set source file method for SourceState.

use std::path::Path;

use crate::services::file_watcher::FileWatcher;
use crate::widgets::markdown_widget::foundation::source::MarkdownSource;
use crate::widgets::markdown_widget::state::source_state::SourceState;

impl SourceState {
    /// Set a file-based markdown source.
    ///
    /// This loads the file content and enables auto-reload support.
    /// Use `reload_source()` to check for and apply file changes.
    ///
    /// **Note:** Caller should invalidate any caches after calling this.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the markdown file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    pub fn set_source_file(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let source = MarkdownSource::from_file(path.as_ref())?;
        self.line_count = source.content().lines().count();
        self.source = Some(source);
        self.watch_path = Some(path.as_ref().to_path_buf());

        let mut watcher = FileWatcher::for_file()
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        watcher
            .watch(path.as_ref())
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        self.watcher = Some(watcher);
        Ok(())
    }
}
