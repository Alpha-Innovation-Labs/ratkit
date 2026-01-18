//! Constructor for RenderCache.

use crate::markdown_widget::foundation::elements::CodeBlockTheme;
use crate::markdown_widget::state::scroll_manager::render_cache::RenderCache;
use ratatui::text::Line;

impl RenderCache {
    /// Create a new render cache.
    ///
    /// # Arguments
    ///
    /// * `content_hash` - Hash of the content that was rendered.
    /// * `width` - Width used for rendering.
    /// * `show_line_numbers` - Whether line numbers were shown.
    /// * `theme` - Theme used for rendering.
    /// * `app_theme_hash` - Hash of the app theme.
    /// * `lines` - Cached rendered lines.
    /// * `line_boundaries` - Line boundaries for each logical line.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        content_hash: u64,
        width: usize,
        show_line_numbers: bool,
        theme: CodeBlockTheme,
        app_theme_hash: u64,
        lines: Vec<Line<'static>>,
        line_boundaries: Vec<(usize, usize)>,
    ) -> Self {
        Self {
            content_hash,
            width,
            show_line_numbers,
            theme,
            app_theme_hash,
            lines,
            line_boundaries,
        }
    }
}
