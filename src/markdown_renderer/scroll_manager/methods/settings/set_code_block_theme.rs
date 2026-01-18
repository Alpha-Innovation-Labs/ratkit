//! Set code block theme method for MarkdownScrollManager.

use crate::markdown_renderer::markdown_elements::CodeBlockTheme;
use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Set the code block color theme.
    ///
    /// # Arguments
    ///
    /// * `theme` - The theme to use for code blocks.
    pub fn set_code_block_theme(&mut self, theme: CodeBlockTheme) {
        if self.code_block_theme != theme {
            self.code_block_theme = theme;
            self.invalidate_cache();
        }
    }
}
