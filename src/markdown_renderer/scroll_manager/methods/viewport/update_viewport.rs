//! Update viewport method for MarkdownScrollManager.

use ratatui::layout::Rect;

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Update viewport dimensions.
    ///
    /// # Arguments
    ///
    /// * `area` - The new viewport area.
    pub fn update_viewport(&mut self, area: Rect) {
        self.viewport_height = area.height as usize;
    }
}
