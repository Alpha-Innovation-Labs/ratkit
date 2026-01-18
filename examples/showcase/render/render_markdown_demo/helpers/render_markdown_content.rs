//! Render markdown content with minimap and statusline.

use ratatui::{buffer::Buffer, layout::Rect, text::Line};
use ratatui_toolkit::{
    render_markdown_with_minimap, MarkdownRenderOptions, MarkdownScrollManager, SelectionState,
};

/// Render the markdown content with minimap and statusline.
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
/// * `minimap_hovered` - Whether the minimap is being hovered.
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
    minimap_hovered: bool,
) -> Vec<Line<'static>> {
    let render_options = MarkdownRenderOptions::default()
        .show_minimap(true)
        .minimap_width(12)
        .show_statusline(true)
        .selection_active(selection_active)
        .minimap_hovered(minimap_hovered);

    render_markdown_with_minimap(content, scroll, area, buf, is_dragging, selection, &render_options)
}
