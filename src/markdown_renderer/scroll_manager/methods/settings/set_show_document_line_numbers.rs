//! Set show document line numbers method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Enable or disable line numbers for the entire document.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show document line numbers.
    pub fn set_show_document_line_numbers(&mut self, show: bool) {
        if self.show_document_line_numbers != show {
            self.show_document_line_numbers = show;
            self.invalidate_cache();
        }
    }
}
