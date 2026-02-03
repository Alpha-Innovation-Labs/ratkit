//! Render the combined trees demo tab.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui_toolkit::AppTheme;

use crate::app::{App, TreePaneFocus};

/// Render the combined trees demo.
pub fn render_trees_demo(frame: &mut ratatui::Frame, area: Rect, app: &mut App, theme: &AppTheme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(7)])
        .split(area);

    let tree_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    let focus_style = Style::default()
        .fg(theme.primary)
        .add_modifier(Modifier::BOLD);
    let file_tree_focused = app.tree_focus == TreePaneFocus::FileTree;
    let component_tree_focused = app.tree_focus == TreePaneFocus::ComponentTree;

    let file_tree_title = if file_tree_focused {
        " FileSystemTree (Focused) "
    } else {
        " FileSystemTree "
    };
    let component_tree_title = if component_tree_focused {
        " TreeView (Focused) "
    } else {
        " TreeView "
    };

    let file_tree_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if file_tree_focused {
            focus_style
        } else {
            Style::default()
        })
        .title(file_tree_title);

    app.file_tree.render(frame, tree_chunks[0], file_tree_block);

    let tree_outer_area = tree_chunks[1];
    let tree_inner_area = Rect {
        x: tree_outer_area.x + 1,
        y: tree_outer_area.y + 1,
        width: tree_outer_area.width.saturating_sub(2),
        height: tree_outer_area.height.saturating_sub(2),
    };

    let tree_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if component_tree_focused {
            focus_style
        } else {
            Style::default()
        })
        .title(component_tree_title);
    frame.render_widget(tree_block, tree_outer_area);

    app.component_tree.render(frame, tree_inner_area);

    let mut info_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Combined Trees",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  Focus: [f] File tree  |  [c] Component tree"),
        Line::from("  Navigation: j/k, h/l, Enter"),
        Line::from("  Filter: / to search, Esc to clear"),
    ];

    let filter_info = match app.tree_focus {
        TreePaneFocus::FileTree if app.file_tree.filter_mode() => Some(vec![
            Line::from(""),
            Line::from(Span::styled(
                "  FileTree Filter Mode",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from("    Type to filter"),
            Line::from("    Esc   Clear & exit"),
            Line::from("    Enter Keep filter"),
        ]),
        TreePaneFocus::ComponentTree if app.component_tree.filter_mode() => Some(vec![
            Line::from(""),
            Line::from(Span::styled(
                "  TreeView Filter Mode",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from("    Type to filter"),
            Line::from("    Esc   Clear & exit"),
            Line::from("    Enter Keep filter"),
        ]),
        _ => None,
    };

    if let Some(extra_lines) = filter_info {
        info_lines.extend(extra_lines);
    }

    let info = Paragraph::new(info_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Controls "),
    );
    frame.render_widget(info, chunks[1]);
}
