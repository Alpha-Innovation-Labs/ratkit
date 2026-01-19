//! Render markdown content with TOC and statusline.

use ratatui::{buffer::Buffer, layout::Rect, text::Line, widgets::Widget};
use ratatui_toolkit::{
    markdown_widget::state::{
        CacheState, CollapseState, DisplaySettings, ExpandableState, GitStatsState, ScrollState,
        SourceState, VimState,
    },
    AppTheme, DoubleClickState, MarkdownWidget, SelectionState,
};

/// Render the markdown content with TOC and statusline.
///
/// # Arguments
///
/// * `content` - The markdown content string.
/// * `scroll` - The scroll state.
/// * `source` - The source state.
/// * `cache` - The cache state.
/// * `display` - The display settings.
/// * `collapse` - The collapse state.
/// * `expandable` - The expandable state.
/// * `git_stats` - The git stats state.
/// * `vim` - The vim state.
/// * `selection` - The selection state.
/// * `double_click` - The double-click state.
/// * `area` - The area to render into.
/// * `buf` - The buffer to render to.
/// * `is_dragging` - Whether the divider is being dragged.
/// * `selection_active` - Whether selection mode is active.
/// * `toc_hovered` - Whether the TOC is being hovered (expands to show text).
/// * `toc_hovered_entry` - Index of the hovered TOC entry.
/// * `toc_scroll_offset` - Scroll offset for the TOC list.
/// * `app_theme` - The current application theme.
///
/// # Returns
///
/// All rendered lines for selection text extraction.
#[allow(clippy::too_many_arguments)]
pub fn render_markdown_content(
    content: &str,
    scroll: &mut ScrollState,
    source: &mut SourceState,
    cache: &mut CacheState,
    display: &DisplaySettings,
    collapse: &mut CollapseState,
    expandable: &mut ExpandableState,
    git_stats: &mut GitStatsState,
    vim: &mut VimState,
    selection: &mut SelectionState,
    double_click: &mut DoubleClickState,
    area: Rect,
    buf: &mut Buffer,
    is_dragging: bool,
    selection_active: bool,
    toc_hovered: bool,
    toc_hovered_entry: Option<usize>,
    toc_scroll_offset: usize,
    app_theme: &AppTheme,
) -> Vec<Line<'static>> {
    let widget = MarkdownWidget::new(
        content,
        scroll,
        source,
        cache,
        display,
        collapse,
        expandable,
        git_stats,
        vim,
        selection,
        double_click,
    )
    .show_toc(true)
    .show_statusline(true)
    .show_scrollbar(true)
    .selection_active(selection_active)
    .toc_hovered(toc_hovered)
    .toc_hovered_entry(toc_hovered_entry)
    .toc_scroll_offset(toc_scroll_offset)
    .is_resizing(is_dragging)
    .with_theme(app_theme);

    widget.render(area, buf);

    // Get rendered lines from cache for selection text extraction
    cache
        .render_cache()
        .map(|c| c.lines.clone())
        .unwrap_or_default()
}
