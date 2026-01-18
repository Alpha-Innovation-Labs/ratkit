//! Default trait implementation for MarkdownScrollManager.

use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;

impl Default for MarkdownScrollManager {
    fn default() -> Self {
        Self::new()
    }
}
