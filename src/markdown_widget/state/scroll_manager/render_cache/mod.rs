//! Cache for rendered markdown lines.
//!
//! This cache stores rendered lines that depend on rendering width.

pub mod constructors;
pub mod methods;

use crate::markdown_widget::foundation::elements::CodeBlockTheme;
use ratatui::text::Line;

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
