//! Render the markdown demo tab.

mod helpers;

use ratatui::layout::Rect;
use ratatui_toolkit::{AppTheme, MarkdownWidget};

use crate::app::App;

/// Render the markdown demo.
///
/// # Arguments
///
/// * `frame` - The frame to render into.
/// * `area` - The area to render in.
/// * `app` - The application state.
/// * `theme` - The application theme.
pub fn render_markdown_demo(
    frame: &mut ratatui::Frame,
    area: Rect,
    app: &mut App,
    theme: &AppTheme,
) {
    let content = app.markdown_source.content().unwrap_or("").to_string();
    let selection_active = app.markdown_selection.is_active();

    let widget = MarkdownWidget::new(
        &content,
        &mut app.markdown_scroll,
        &mut app.markdown_source,
        &mut app.markdown_cache,
        &app.markdown_display,
        &mut app.markdown_collapse,
        &mut app.markdown_expandable,
        &mut app.markdown_git_stats,
        &mut app.markdown_vim,
        &mut app.markdown_selection,
        &mut app.markdown_double_click,
    )
    .show_toc(true)
    .show_statusline(true)
    .show_scrollbar(true)
    .selection_active(selection_active)
    .toc_hovered(app.toc_hovered)
    .toc_hovered_entry(app.toc_hovered_entry)
    .toc_scroll_offset(app.toc_scroll_offset)
    .with_theme(theme);

    frame.render_widget(widget, area);

    // Store rendered lines for selection text extraction
    app.markdown_rendered_lines = app
        .markdown_cache
        .render_cache()
        .map(|c| c.lines.clone())
        .unwrap_or_default();

    // Store inner area for mouse event handling (account for border)
    app.markdown_inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
}
