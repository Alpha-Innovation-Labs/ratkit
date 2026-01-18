//! Set source file method for MarkdownScrollManager.

use std::path::Path;

use crate::markdown_renderer::markdown_source::MarkdownSource;
use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Set a file-based markdown source.
    ///
    /// This loads the file content and enables auto-reload support.
    /// Use `reload_source()` to check for and apply file changes.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the markdown file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    pub fn set_source_file(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        self.source = Some(MarkdownSource::from_file(path)?);
        self.invalidate_cache();
        Ok(())
    }
}
