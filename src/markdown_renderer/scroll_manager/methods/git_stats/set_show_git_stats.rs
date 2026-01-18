//! Set show git stats method for MarkdownScrollManager.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Enable or disable git stats display.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show git stats in the statusline.
    pub fn set_show_git_stats(&mut self, show: bool) {
        self.show_git_stats = show;
        if show && self.git_stats_cache.is_none() {
            // Trigger immediate update when enabled
            self.git_stats_last_update = None;
        }
    }
}
