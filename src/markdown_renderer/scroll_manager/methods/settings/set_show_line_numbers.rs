//! Set show line numbers method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Enable or disable line numbers in code blocks.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show line numbers.
    pub fn set_show_line_numbers(&mut self, show: bool) {
        if self.show_line_numbers != show {
            self.show_line_numbers = show;
            self.invalidate_cache();
        }
    }
}
