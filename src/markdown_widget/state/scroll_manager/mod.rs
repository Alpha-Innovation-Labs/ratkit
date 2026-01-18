//! Scroll manager for markdown rendering.
//!
//! Provides utilities to manage scroll offset, handle mouse scroll events,
//! and track collapse/expand state for markdown sections.

pub mod constructors;
pub mod expandable_state;
pub mod methods;
pub mod parsed_cache;
pub mod render_cache;
pub mod traits;

use crate::markdown_widget::foundation::source::MarkdownSource;
use crate::markdown_widget::foundation::types::GitStats;
use std::collections::HashMap;
use std::time::Instant;

pub use constructors::*;
pub use expandable_state::ExpandableState;
pub use methods::*;
pub use parsed_cache::ParsedCache;
pub use render_cache::RenderCache;
pub use traits::*;

/// Manages scroll state for markdown rendering.
#[derive(Debug, Clone)]
pub struct MarkdownScrollManager {
    /// Current scroll offset (0-indexed, first visible line index).
    pub scroll_offset: usize,
    /// Height of viewport (number of visible lines).
    pub viewport_height: usize,
    /// Total number of lines in document.
    pub total_lines: usize,
    /// Currently selected line (1-indexed, for highlighting).
    pub current_line: usize,
    /// Section collapse state: section_id -> is_collapsed.
    pub collapsed_sections: HashMap<usize, bool>,
    /// Section hierarchy: section_id -> (level, parent_section_id).
    pub section_hierarchy: HashMap<usize, (u8, Option<usize>)>,
    /// Expandable content state: content_id -> { collapsed, max_lines }.
    pub expandable_content: HashMap<String, ExpandableState>,
    /// Default max lines for expandable content.
    pub default_max_lines: usize,
    /// Cache for parsed markdown elements (doesn't depend on width).
    pub(crate) parsed_cache: Option<ParsedCache>,
    /// Cache for rendered lines (depends on width).
    pub(crate) render_cache: Option<RenderCache>,
    /// Whether to show line numbers in code blocks.
    pub show_line_numbers: bool,
    /// Whether to show line numbers for the entire document.
    pub show_document_line_numbers: bool,
    /// Color theme for code blocks.
    pub code_block_theme: crate::markdown_widget::foundation::elements::CodeBlockTheme,
    /// Optional markdown source (string or file-based with auto-reload support).
    source: Option<MarkdownSource>,
    /// Source file line count (for accurate status bar display).
    pub source_line_count: usize,
    /// Whether to show git stats in the statusline.
    pub show_git_stats: bool,
    /// Cached git stats for the source file.
    pub(crate) git_stats_cache: Option<GitStats>,
    /// Last time git stats were updated.
    pub(crate) git_stats_last_update: Option<Instant>,
    /// Pending 'g' keypress time for vim-style gg (go to top).
    pub(crate) pending_g_time: Option<Instant>,
}
