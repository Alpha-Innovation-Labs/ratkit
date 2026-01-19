//! Reload source method for SourceState.

use crate::markdown_widget::state::source_state::SourceState;

impl SourceState {
    /// Reload the source content from disk (for file-based sources).
    ///
    /// This re-reads the file. The caller should check the return value
    /// and invalidate caches if content changed.
    ///
    /// For string-based sources, this is a no-op.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - Content changed, caller should invalidate caches.
    /// * `Ok(false)` - Content unchanged or source is string-based.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    pub fn reload_source(&mut self) -> std::io::Result<bool> {
        if let Some(ref mut source) = self.source {
            let changed = source.reload()?;
            if changed {
                self.line_count = source.content().lines().count();
            }
            Ok(changed)
        } else {
            Ok(false)
        }
    }
}
