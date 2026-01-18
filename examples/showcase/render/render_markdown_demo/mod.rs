//! Render the markdown demo tab.

mod helpers;

use ratatui::{
    layout::Rect,
    widgets::{Block, BorderType, Borders, Widget},
};

use crate::app::App;
use helpers::{
    calculate_split_areas, get_border_style, get_markdown_title, render_controls_panel,
    render_markdown_content,
};

/// Render the markdown demo.
pub fn render_markdown_demo(frame: &mut ratatui::Frame, area: Rect, app: &mut App) {
    app.markdown_split.update_divider_position(area);
    let areas = calculate_split_areas(area, app.markdown_split.split_percent);
    let selection_active = app.markdown_selection.is_active();
    let border_style = get_border_style(
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

    // Render markdown content with minimap and statusline
    app.markdown_rendered_lines = render_markdown_content(
        &app.markdown_scroll.content().unwrap_or("").to_string(),
        &mut app.markdown_scroll,
        inner_area,
        frame.buffer_mut(),
        app.markdown_split.is_dragging,
        &app.markdown_selection,
        selection_active,
        app.minimap_hovered,
    );

    render_controls_panel(frame, areas.right, border_style);
}
