//! Reload content if the file watcher detected changes.

use crate::widgets::markdown_widget::state::source_state::SourceState;

impl SourceState {
    /// Reload the source content if the watcher detected changes.
    ///
    /// Returns `Ok(true)` when content changed and was reloaded.
    pub fn reload_if_changed(&mut self) -> std::io::Result<bool> {
        let Some(watcher) = self.watcher.as_mut() else {
            return Ok(false);
        };

        if !watcher.check_for_changes() {
            return Ok(false);
        }

        self.reload_source()
    }
}
