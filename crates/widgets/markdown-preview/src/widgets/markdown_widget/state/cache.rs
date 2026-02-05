//! Cache state for markdown widget.
//!
//! Manages parsed and rendered markdown caches for efficient rendering.

use crate::widgets::markdown_widget::foundation::elements::{CodeBlockTheme, MarkdownElement};
use ratatui::text::Line;

/// Cache state for markdown rendering.
///
/// Maintains two levels of caching:
/// - Parsed cache: Content-dependent, width-independent
/// - Render cache: Content and width-dependent
#[derive(Debug, Clone)]
pub struct CacheState {
    /// Cache for parsed markdown elements (doesn't depend on width).
    pub(crate) parsed: Option<ParsedCache>,
    /// Cache for rendered lines (depends on width).
    pub(crate) render: Option<RenderCache>,
}

impl CacheState {
    /// Create a new cache state with empty caches.
    pub fn new() -> Self {
        Self {
            parsed: None,
            render: None,
        }
    }

    /// Clear the render cache (e.g., when exiting filter mode).
    pub fn clear_render_cache(&mut self) {
        self.render = None;
    }

    /// Invalidate both parsed and render caches.
    ///
    /// Call this when content changes.
    pub fn invalidate(&mut self) {
        self.parsed = None;
        self.render = None;
    }

    /// Invalidate only the render cache.
    ///
    /// Call this when width changes but content is the same.
    pub fn invalidate_render(&mut self) {
        self.render = None;
    }

    /// Get a reference to the parsed cache if it exists.
    pub fn parsed_cache(&self) -> Option<&ParsedCache> {
        self.parsed.as_ref()
    }

    /// Get a reference to the render cache if it exists.
    ///
    /// The render cache contains the rendered lines from the last render operation.
    /// This is useful for extracting text for copy operations.
    pub fn render_cache(&self) -> Option<&RenderCache> {
        self.render.as_ref()
    }

    /// Set the parsed cache.
    pub fn set_parsed(&mut self, cache: ParsedCache) {
        self.parsed = Some(cache);
    }

    /// Set the render cache.
    pub fn set_render(&mut self, cache: RenderCache) {
        self.render = Some(cache);
    }
}

impl Default for CacheState {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache for parsed markdown (doesn't depend on width).
#[derive(Debug, Clone)]
pub struct ParsedCache {
    /// Hash of the content that was parsed.
    pub content_hash: u64,
    /// Parsed markdown elements.
    pub elements: Vec<MarkdownElement>,
}

impl ParsedCache {
    /// Create a new parsed cache.
    pub fn new(content_hash: u64, elements: Vec<MarkdownElement>) -> Self {
        Self {
            content_hash,
            elements,
        }
    }
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
