//! Scroll manager for markdown rendering
//!
//! Provides utilities to manage scroll offset, handle mouse scroll events,
//! and track collapse/expand state for markdown sections.

mod constructors;
mod methods;
mod traits;

use super::markdown_elements::{CodeBlockTheme, MarkdownElement};
use super::markdown_source::MarkdownSource;
use super::markdown_widget::GitStats;
use ratatui::text::Line;
use std::collections::HashMap;
use std::time::Instant;

pub use constructors::*;
pub use methods::*;
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
    pub code_block_theme: CodeBlockTheme,
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
    /// TOC scroll offset (for scrolling within the TOC list).
    pub toc_scroll_offset: usize,
    /// Currently hovered TOC entry index.
    pub toc_hovered_entry: Option<usize>,
    /// Whether the TOC is currently being hovered (expanded).
    pub toc_hovered: bool,
}

/// Cache for parsed markdown (doesn't depend on width).
#[derive(Debug, Clone)]
pub struct ParsedCache {
    /// Hash of the content that was parsed.
    pub content_hash: u64,
    /// Parsed markdown elements.
    pub elements: Vec<MarkdownElement>,
}

/// Cache for rendered markdown lines (depends on width).
#[derive(Debug, Clone)]
pub struct RenderCache {
    /// Hash of the content that was rendered.
    pub content_hash: u64,
    /// Width used for rendering.
    pub width: usize,
    /// Whether line numbers were shown.
    pub show_line_numbers: bool,
    /// Theme used for rendering.
    pub theme: CodeBlockTheme,
    /// Hash of the app theme (for cache invalidation on theme change).
    pub app_theme_hash: u64,
    /// Cached rendered lines.
    pub lines: Vec<Line<'static>>,
    /// Line boundaries: (start_visual_idx, visual_line_count) for each logical line.
    pub line_boundaries: Vec<(usize, usize)>,
}

/// State for expandable content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpandableState {
    /// Whether the content is collapsed (showing limited lines).
    pub collapsed: bool,
    /// Maximum number of visible lines when collapsed.
    pub max_lines: usize,
}

impl ExpandableState {
    /// Create a new expandable state.
    pub fn new(collapsed: bool, max_lines: usize) -> Self {
        Self {
            collapsed,
            max_lines: max_lines.max(1),
        }
    }
}
