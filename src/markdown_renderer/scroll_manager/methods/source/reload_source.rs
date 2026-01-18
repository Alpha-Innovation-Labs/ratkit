//! Reload source method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Reload the source content from disk (for file-based sources).
    ///
    /// This re-reads the file and invalidates caches if the content changed.
    /// For string-based sources, this is a no-op.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - Content changed and caches were invalidated.
    /// * `Ok(false)` - Content unchanged or source is string-based.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    pub fn reload_source(&mut self) -> std::io::Result<bool> {
        if let Some(ref mut source) = self.source {
            let changed = source.reload()?;
            if changed {
                self.invalidate_cache();
            }
            Ok(changed)
        } else {
            Ok(false)
        }
    }
}
