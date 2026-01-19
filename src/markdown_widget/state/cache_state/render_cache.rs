//! Cache for rendered markdown lines.
//!
//! This cache stores rendered lines that depend on rendering width.

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
    /// Whether heading collapse indicators were shown.
    pub show_heading_collapse: bool,
    /// Cached rendered lines.
    pub lines: Vec<Line<'static>>,
    /// Line boundaries: (start_visual_idx, visual_line_count) for each logical line.
    pub line_boundaries: Vec<(usize, usize)>,
}

impl RenderCache {
    /// Create a new render cache.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        content_hash: u64,
        width: usize,
        show_line_numbers: bool,
        theme: CodeBlockTheme,
        app_theme_hash: u64,
        show_heading_collapse: bool,
        lines: Vec<Line<'static>>,
        line_boundaries: Vec<(usize, usize)>,
    ) -> Self {
        Self {
            content_hash,
            width,
            show_line_numbers,
            theme,
            app_theme_hash,
            show_heading_collapse,
            lines,
            line_boundaries,
        }
    }
}
