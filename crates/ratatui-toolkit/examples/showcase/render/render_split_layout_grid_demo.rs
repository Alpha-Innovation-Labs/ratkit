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
        .title(" ResizableGrid (5 panes) ")
        .border_style(Style::default().fg(theme.primary));
    let inner_area = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    // Get pane layouts from the grid
    let pane_layouts = app.grid_split.layout_panes(inner_area);
    let divider_layouts = app.grid_split.layout_dividers(inner_area);

    let pane_labels = [
        "Pane 0\n(Top Left)",
        "Pane 1\n(Bottom Left)",
        "Pane 2\n(Top Middle)",
        "Pane 3\n(Top Right)",
        "Pane 4\n(Bottom Right)",
    ];

    // Render content in each pane
    for (index, pane_layout) in pane_layouts.iter().enumerate() {
        let label = pane_labels.get(index).unwrap_or(&"Unknown");
        let pane_area = pane_layout.area();

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
        frame.render_widget(pane_block, pane_area);
        frame.render_widget(content, pane_area);
    }

    // Render divider highlights when hovering/dragging
    for divider in &divider_layouts {
        let is_active = app.grid_split.hovered_split == Some(divider.split_index())
            || app.grid_split.dragging_split == Some(divider.split_index());

        if is_active {
            let divider_style = if app.grid_split.dragging_split == Some(divider.split_index()) {
                Style::default().fg(theme.warning)
            } else {
                Style::default().fg(theme.secondary)
            };

            let rect = divider.area();
            match divider.axis() {
                ratatui_toolkit::primitives::resizable_grid::SplitAxis::Vertical => {
                    let divider_x = rect.x.saturating_add(
                        ((rect.width as u32 * divider.ratio() as u32) / 100) as u16,
                    );
                    for y in rect.top()..rect.bottom() {
                        if let Some(cell) = frame.buffer_mut().cell_mut((divider_x, y)) {
                            cell.set_style(divider_style);
                            cell.set_char('│');
                        }
                    }
                }
                ratatui_toolkit::primitives::resizable_grid::SplitAxis::Horizontal => {
                    let divider_y = rect.y.saturating_add(
                        ((rect.height as u32 * divider.ratio() as u32) / 100) as u16,
                    );
                    for x in rect.left()..rect.right() {
                        if let Some(cell) = frame.buffer_mut().cell_mut((x, divider_y)) {
                            cell.set_style(divider_style);
                            cell.set_char('─');
                        }
                    }
                }
            }
        }
    }
}
