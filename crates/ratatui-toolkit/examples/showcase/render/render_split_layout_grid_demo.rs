//! Render split layout grid demo.

use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui_toolkit::AppTheme;

use crate::app::App;

/// Render split layout grid demo.
pub fn render_split_layout_grid_demo(
    frame: &mut ratatui::Frame,
    area: Rect,
    app: &mut App,
    theme: &AppTheme,
) {
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" Split Layout Grid (5 panes) ")
        .border_style(Style::default().fg(theme.primary));
    let inner_area = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    app.grid_row_split.update_divider_position(inner_area);
    app.grid_left_split.update_divider_position(inner_area);

    let left_width =
        (inner_area.width as u32 * app.grid_left_split.split_percent as u32 / 100) as u16;
    let left_area = Rect {
        x: inner_area.x,
        y: inner_area.y,
        width: left_width,
        height: inner_area.height,
    };
    let right_area = Rect {
        x: inner_area.x + left_width,
        y: inner_area.y,
        width: inner_area.width.saturating_sub(left_width),
        height: inner_area.height,
    };

    app.grid_right_split.update_divider_position(right_area);

    let top_height =
        (inner_area.height as u32 * app.grid_row_split.split_percent as u32 / 100) as u16;
    let top_area = Rect {
        x: inner_area.x,
        y: inner_area.y,
        width: inner_area.width,
        height: top_height,
    };
    let bottom_area = Rect {
        x: inner_area.x,
        y: inner_area.y + top_height,
        width: inner_area.width,
        height: inner_area.height.saturating_sub(top_height),
    };

    let top_left_width =
        (top_area.width as u32 * app.grid_left_split.split_percent as u32 / 100) as u16;
    let top_left_area = Rect {
        x: top_area.x,
        y: top_area.y,
        width: top_left_width,
        height: top_area.height,
    };
    let top_right_container = Rect {
        x: top_area.x + top_left_width,
        y: top_area.y,
        width: top_area.width.saturating_sub(top_left_width),
        height: top_area.height,
    };
    let top_middle_width =
        (top_right_container.width as u32 * app.grid_right_split.split_percent as u32 / 100) as u16;
    let top_middle_area = Rect {
        x: top_right_container.x,
        y: top_right_container.y,
        width: top_middle_width,
        height: top_right_container.height,
    };
    let top_right_area = Rect {
        x: top_right_container.x + top_middle_width,
        y: top_right_container.y,
        width: top_right_container.width.saturating_sub(top_middle_width),
        height: top_right_container.height,
    };

    let bottom_left_width =
        (bottom_area.width as u32 * app.grid_left_split.split_percent as u32 / 100) as u16;
    let bottom_left_area = Rect {
        x: bottom_area.x,
        y: bottom_area.y,
        width: bottom_left_width,
        height: bottom_area.height,
    };
    let bottom_right_container = Rect {
        x: bottom_area.x + bottom_left_width,
        y: bottom_area.y,
        width: bottom_area.width.saturating_sub(bottom_left_width),
        height: bottom_area.height,
    };
    let bottom_right_area = Rect {
        x: bottom_right_container.x,
        y: bottom_right_container.y,
        width: bottom_right_container.width,
        height: bottom_right_container.height,
    };
    let pane_labels = [
        "Pane 0\nTop Left",
        "Pane 1\nBottom Left",
        "Pane 2\nTop Middle",
        "Pane 3\nTop Right",
        "Pane 4\nBottom Right",
    ];

    // Render content in each pane
    let pane_areas = [
        top_left_area,
        bottom_left_area,
        top_middle_area,
        top_right_area,
        bottom_right_area,
    ];

    for (index, pane_area) in pane_areas.iter().enumerate() {
        let label = pane_labels.get(index).unwrap_or(&"Unknown");

        let content = Paragraph::new(vec![
            Line::from(vec![Span::styled(
                *label,
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from("Drag borders"),
            Line::from("to resize panes"),
        ])
        .style(Style::default().fg(theme.text));

        let pane_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme.primary));
        frame.render_widget(pane_block, *pane_area);
        frame.render_widget(content, *pane_area);
    }
}
