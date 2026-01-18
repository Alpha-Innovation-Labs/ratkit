//! Render markdown content with TOC and statusline.

use ratatui::{buffer::Buffer, layout::Rect, text::Line};
use ratatui_toolkit::{
    render_markdown_with_minimap, AppTheme, MarkdownRenderOptions, MarkdownScrollManager,
    SelectionState,
};

/// Render the markdown content with TOC and statusline.
///
/// # Arguments
///
/// * `content` - The markdown content string.
/// * `scroll` - The scroll manager.
/// * `area` - The area to render into.
/// * `buf` - The buffer to render to.
/// * `is_dragging` - Whether the divider is being dragged.
/// * `selection` - The selection state.
/// * `selection_active` - Whether selection mode is active.
/// * `toc_hovered` - Whether the TOC is being hovered (expands to show text).
/// * `toc_hovered_entry` - Index of the hovered TOC entry.
/// * `toc_scroll_offset` - Scroll offset for the TOC list.
/// * `app_theme` - The current application theme.
///
/// # Returns
///
/// All rendered lines for selection text extraction.
pub fn render_markdown_content(
    content: &str,
    scroll: &mut MarkdownScrollManager,
    area: Rect,
    buf: &mut Buffer,
    is_dragging: bool,
    selection: &SelectionState,
    selection_active: bool,
    toc_hovered: bool,
    toc_hovered_entry: Option<usize>,
    toc_scroll_offset: usize,
    app_theme: &AppTheme,
) -> Vec<Line<'static>> {
    let render_options = MarkdownRenderOptions::default()
        .show_toc(true)
        .show_statusline(true)
        .selection_active(selection_active)
        .toc_hovered(toc_hovered)
        .toc_hovered_entry(toc_hovered_entry)
        .toc_scroll_offset(toc_scroll_offset)
        .with_theme(app_theme);

    render_markdown_with_minimap(
        content,
        scroll,
        area,
        buf,
        is_dragging,
        selection,
        &render_options,
    )
}
