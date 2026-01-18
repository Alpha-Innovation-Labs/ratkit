//! Line up method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Move current line up (for keyboard navigation).
    pub fn line_up(&mut self) {
        if self.current_line > 1 {
            self.current_line -= 1;
        }
        self.adjust_scroll_for_current_line();
    }
}
