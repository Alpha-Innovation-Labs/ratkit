//! Constructor for MarkdownScrollManager.

use std::collections::HashMap;
use crate::markdown_renderer::markdown_elements::CodeBlockTheme;
use super::super::MarkdownScrollManager;

impl MarkdownScrollManager {
    /// Create a new scroll manager with default settings.
    pub fn new() -> Self {
        Self {
            scroll_offset: 0,
            viewport_height: 20,
            total_lines: 0,
            current_line: 1,
            collapsed_sections: HashMap::new(),
            section_hierarchy: HashMap::new(),
            expandable_content: HashMap::new(),
            default_max_lines: 3,
            parsed_cache: None,
            render_cache: None,
            show_line_numbers: false,
            show_document_line_numbers: false,
            code_block_theme: CodeBlockTheme::default(),
            source: None,
            source_line_count: 0,
            show_git_stats: false,
            git_stats_cache: None,
            git_stats_last_update: None,
            pending_g_time: None,
        }
    }
}
