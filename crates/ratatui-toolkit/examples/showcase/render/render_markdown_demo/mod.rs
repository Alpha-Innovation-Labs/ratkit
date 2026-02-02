//! Render the markdown demo tab.

mod helpers;

use ratatui::layout::Rect;
use ratatui_toolkit::AppTheme;

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
    let widget = app
        .markdown_widget
        .show_toc(true)
        .show_statusline(true)
        .show_scrollbar(true)
        .with_theme(theme);

    frame.render_widget(widget, area);

    app.markdown_widget
        .set_rendered_lines(app.markdown_widget.rendered_lines().clone());
}

fn calculate_inner_area(area: Rect, show_statusline: bool) -> Rect {
    let border_offset = 1;
    let statusline_offset = if show_statusline && area.height > 1 {
        1
    } else {
        0
    };
    Rect {
        x: area.x + border_offset,
        y: area.y + border_offset,
        width: area.width.saturating_sub(2 * border_offset),
        height: area
            .height
            .saturating_sub(2 * border_offset + statusline_offset),
    }
}
