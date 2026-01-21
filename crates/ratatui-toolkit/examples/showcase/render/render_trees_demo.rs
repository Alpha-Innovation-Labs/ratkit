//! Render the combined trees demo tab.

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui_toolkit::{matches_filter, AppTheme, TreeNode, TreeViewRef};

use crate::app::{App, TreePaneFocus};

/// Filter function for String nodes.
///
/// Returns true if the string matches the filter (case-insensitive contains).
fn string_matches_filter(data: &String, filter: &Option<String>) -> bool {
    matches_filter(data, filter)
}

/// Renders the filter input line at the bottom of the tree.
fn render_filter_line(
    filter_text: Option<&str>,
    filter_mode: bool,
    area: Rect,
    buf: &mut Buffer,
    theme: &AppTheme,
) {
    if area.height == 0 {
        return;
    }

    let y = area.y + area.height - 1;
    let filter_str = filter_text.unwrap_or("");
    let cursor = if filter_mode { "_" } else { "" };

    let line = Line::from(vec![
        Span::styled(
            "/ ",
            Style::default()
                .fg(theme.warning)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(filter_str, Style::default().fg(theme.text)),
        Span::styled(
            cursor,
            Style::default()
                .fg(theme.warning)
                .add_modifier(Modifier::SLOW_BLINK),
        ),
    ]);

    let bg_style = Style::default().bg(theme.background_panel);
    for x in area.x..(area.x + area.width) {
        buf[(x, y)].set_style(bg_style);
    }

    buf.set_line(area.x, y, &line, area.width);
}

/// Render the combined trees demo.
pub fn render_trees_demo(
    frame: &mut ratatui::Frame,
    area: Rect,
    app: &mut App,
    tree_nodes: &[TreeNode<String>],
    theme: &AppTheme,
) {
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

    let Some(ref file_tree) = app.file_tree else {
        let error = Paragraph::new("Failed to load file system tree")
            .block(file_tree_block)
            .style(Style::default().fg(Color::Red));
        frame.render_widget(error, tree_chunks[0]);
        return;
    };

    let file_tree_widget = file_tree.clone().block(file_tree_block);
    frame.render_stateful_widget(file_tree_widget, tree_chunks[0], &mut app.file_tree_state);

    let tree_outer_area = tree_chunks[1];
    let tree_inner_area = Rect {
        x: tree_outer_area.x + 1,
        y: tree_outer_area.y + 1,
        width: tree_outer_area.width.saturating_sub(2),
        height: tree_outer_area.height.saturating_sub(2),
    };

    let filter_mode = app.tree_state.filter_mode;
    let has_filter = app
        .tree_state
        .filter
        .as_ref()
        .is_some_and(|f| !f.is_empty());
    let show_filter_line = filter_mode || has_filter;

    let tree_content_area = if show_filter_line && tree_inner_area.height > 1 {
        Rect {
            height: tree_inner_area.height - 1,
            ..tree_inner_area
        }
    } else {
        tree_inner_area
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

    let tree = TreeViewRef::new(tree_nodes)
        .highlight_style(Style::default().bg(theme.background_panel))
        .render_fn(move |data: &String, state| {
            let style = if state.is_selected {
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            Line::styled(data.clone(), style)
        })
        .filter_fn(string_matches_filter);

    frame.render_stateful_widget(tree, tree_content_area, &mut app.tree_state);

    if show_filter_line && tree_inner_area.height > 1 {
        let buf = frame.buffer_mut();
        render_filter_line(
            app.tree_state.filter.as_deref(),
            filter_mode,
            tree_inner_area,
            buf,
            theme,
        );
    }

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
        TreePaneFocus::FileTree if app.file_tree_state.filter_mode => Some(vec![
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
        TreePaneFocus::ComponentTree if app.tree_state.filter_mode => Some(vec![
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
