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
    app.grid_content_area = Some(area);

    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" ResizableGrid (5 panes) ")
        .border_style(Style::default().fg(theme.primary));
    let inner_area = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    let pane_layouts = app.grid_split_widget.pane_layouts(inner_area);

    let pane_labels = [
        "Pane 0\n(Top Left)",
        "Pane 1\n(Bottom Left)",
        "Pane 2\n(Top Middle)",
        "Pane 3\n(Top Right)",
        "Pane 4\n(Bottom Right)",
    ];

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

    frame.render_widget(app.grid_split_widget.clone(), inner_area);
}
