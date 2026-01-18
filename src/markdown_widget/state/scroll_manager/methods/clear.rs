//! Clear method for MarkdownScrollManager.

use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Clear all scroll and collapse state.
    ///
    /// This resets:
    /// - Scroll offset to 0
    /// - Total lines to 0
    /// - Current line to 1
    /// - All collapsed sections
    /// - Section hierarchy
    /// - Expandable content state
    /// - Parsed and render caches
    ///
    /// Note: This does NOT clear the source. Use `set_source_string` or
    /// `set_source_file` to change the source.
    pub fn clear(&mut self) {
        self.scroll_offset = 0;
        self.total_lines = 0;
        self.current_line = 1;
        self.collapsed_sections.clear();
        self.section_hierarchy.clear();
        self.expandable_content.clear();
        self.parsed_cache = None;
        self.render_cache = None;
    }
}
