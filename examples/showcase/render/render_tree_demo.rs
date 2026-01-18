//! Render the tree view demo tab.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui_toolkit::{TreeNode, TreeView};

use crate::app::App;

/// Render the tree view demo.
pub fn render_tree_demo(
    frame: &mut ratatui::Frame,
    area: Rect,
    app: &mut App,
    tree_nodes: &[TreeNode<String>],
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let tree = TreeView::new(tree_nodes.to_vec())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" TreeView - Component Browser "),
        )
        .highlight_style(Style::default().bg(Color::DarkGray))
        .render_fn(|data: &String, state| {
            let style = if state.is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            Line::styled(data.clone(), style)
        });

    frame.render_stateful_widget(tree, chunks[0], &mut app.tree_state);

    let info = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  TreeView Features",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  Navigation:"),
        Line::from("    j/↓  Move down"),
        Line::from("    k/↑  Move up"),
        Line::from("    l/→  Expand node"),
        Line::from("    h/←  Collapse node"),
        Line::from("    Enter Toggle expand"),
        Line::from(""),
        Line::from("  Features:"),
        Line::from("    • Generic data type"),
        Line::from("    • Custom render function"),
        Line::from("    • Configurable keybindings"),
        Line::from("    • Full-row selection"),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Controls "),
    );

    frame.render_widget(info, chunks[1]);
}
