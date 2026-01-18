//! Set default max lines method for MarkdownScrollManager.

use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Set the default max lines for expandable content.
    ///
    /// # Arguments
    ///
    /// * `max_lines` - Default maximum visible lines when collapsed (minimum 1).
    pub fn set_default_max_lines(&mut self, max_lines: usize) {
        self.default_max_lines = max_lines.max(1);
    }
}
