//! Render the tree view demo tab.

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui_toolkit::{matches_filter, AppTheme, TreeNode, TreeViewRef};

use crate::app::App;

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

    // Build the filter line
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

    // Fill background for the filter line
    let bg_style = Style::default().bg(theme.background_panel);
    for x in area.x..(area.x + area.width) {
        buf[(x, y)].set_style(bg_style);
    }

    buf.set_line(area.x, y, &line, area.width);
}

/// Render the tree view demo.
///
/// # Arguments
///
/// * `frame` - The frame to render into.
/// * `area` - The area to render in.
/// * `app` - The application state.
/// * `tree_nodes` - The tree nodes to display.
/// * `theme` - The application theme.
pub fn render_tree_demo(
    frame: &mut ratatui::Frame,
    area: Rect,
    app: &mut App,
    tree_nodes: &[TreeNode<String>],
    theme: &AppTheme,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let primary = theme.primary;
    let background_panel = theme.background_panel;
    let filter_mode = app.tree_state.filter_mode;
    let has_filter = app.tree_state.filter.as_ref().is_some_and(|f| !f.is_empty());
    let show_filter_line = filter_mode || has_filter;

    // Calculate inner area for the tree (accounting for block border)
    let tree_outer_area = chunks[0];
    let tree_inner_area = Rect {
        x: tree_outer_area.x + 1,
        y: tree_outer_area.y + 1,
        width: tree_outer_area.width.saturating_sub(2),
        height: tree_outer_area.height.saturating_sub(2),
    };

    // Calculate tree content area (leave room for filter line if needed)
    let tree_content_area = if show_filter_line && tree_inner_area.height > 1 {
        Rect {
            height: tree_inner_area.height - 1,
            ..tree_inner_area
        }
    } else {
        tree_inner_area
    };

    // Render the block first
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" TreeView - Component Browser ");
    frame.render_widget(block, tree_outer_area);

    // Create and render the tree view
    let tree = TreeViewRef::new(tree_nodes)
        .highlight_style(Style::default().bg(background_panel))
        .render_fn(move |data: &String, state| {
            let style = if state.is_selected {
                Style::default().fg(primary).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            Line::styled(data.clone(), style)
        })
        .filter_fn(string_matches_filter);

    frame.render_stateful_widget(tree, tree_content_area, &mut app.tree_state);

    // Render filter line if needed
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

    // Show filter mode indicator in info panel
    let filter_info = if filter_mode {
        vec![
            Line::from(""),
            Line::from(Span::styled(
                "  Filter Mode Active",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from("    Type to filter"),
            Line::from("    Esc   Clear & exit"),
            Line::from("    Enter Keep filter"),
        ]
    } else {
        vec![]
    };

    let mut info_lines = vec![
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
        Line::from("    /    Filter nodes"),
        Line::from(""),
        Line::from("  Features:"),
        Line::from("    • Generic data type"),
        Line::from("    • Custom render function"),
        Line::from("    • Configurable keybindings"),
        Line::from("    • Full-row selection"),
        Line::from("    • Node filtering"),
    ];
    info_lines.extend(filter_info);

    let info = Paragraph::new(info_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Controls "),
    );

    frame.render_widget(info, chunks[1]);
}
