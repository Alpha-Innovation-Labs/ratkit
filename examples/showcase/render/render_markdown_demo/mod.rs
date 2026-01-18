//! Render the markdown demo tab.

mod helpers;

use ratatui::{
    layout::Rect,
    widgets::{Block, BorderType, Borders, Widget},
};
use ratatui_toolkit::AppTheme;

use crate::app::App;
use helpers::{
    calculate_split_areas, get_border_style, get_markdown_title, render_controls_panel,
    render_markdown_content,
};

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
    app.markdown_split.update_divider_position(area);
    let areas = calculate_split_areas(area, app.markdown_split.split_percent);
    let selection_active = app.markdown_selection.is_active();
    let border_style = get_border_style(
        theme,
        selection_active,
        app.markdown_split.is_hovering,
        app.markdown_split.is_dragging,
    );

    // Create and render block for left panel
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(get_markdown_title(selection_active))
        .border_style(border_style);
    let inner_area = block.inner(areas.left);
    block.render(areas.left, frame.buffer_mut());

    // Render markdown content with TOC and statusline using the current theme
    app.markdown_rendered_lines = render_markdown_content(
        &app.markdown_scroll.content().unwrap_or("").to_string(),
        &mut app.markdown_scroll,
        inner_area,
        frame.buffer_mut(),
        app.markdown_split.is_dragging,
        &app.markdown_selection,
        selection_active,
        app.toc_hovered,
        app.toc_hovered_entry,
        app.toc_scroll_offset,
        theme,
    );

    render_controls_panel(frame, areas.right, border_style);
}
