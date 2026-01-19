//! Render the file system tree demo tab.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, StatefulWidget},
};
use ratatui_toolkit::AppTheme;

use crate::app::App;

/// Renders the filter input line at the bottom of the tree.
fn render_filter_line(
    filter_text: Option<&str>,
    filter_mode: bool,
    area: Rect,
    buf: &mut ratatui::buffer::Buffer,
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

/// Render the file system tree demo.
pub fn render_file_tree_demo(
    frame: &mut ratatui::Frame,
    area: Rect,
    app: &mut App,
    theme: &AppTheme,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Check if we have a file tree
    let Some(ref file_tree) = app.file_tree else {
        let error = Paragraph::new("Failed to load file system tree")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(" File Tree - Error "),
            )
            .style(Style::default().fg(Color::Red));
        frame.render_widget(error, area);
        return;
    };

    let filter_mode = app.file_tree_state.filter_mode;
    let has_filter = app
        .file_tree_state
        .filter
        .as_ref()
        .is_some_and(|f| !f.is_empty());
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
        .title(" FileSystemTree - Current Directory ");
    frame.render_widget(block, tree_outer_area);

    // Clone the tree for rendering (since we need to consume it)
    let tree_widget = file_tree.clone();
    frame.render_stateful_widget(tree_widget, tree_content_area, &mut app.file_tree_state);

    // Render filter line if needed
    if show_filter_line && tree_inner_area.height > 1 {
        let buf = frame.buffer_mut();
        render_filter_line(
            app.file_tree_state.filter.as_deref(),
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
            "  FileSystemTree Features",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  Navigation:"),
        Line::from("    j/Down  Move down"),
        Line::from("    k/Up    Move up"),
        Line::from("    l/Right Expand directory"),
        Line::from("    h/Left  Collapse directory"),
        Line::from("    Enter   Toggle expand"),
        Line::from("    /       Filter files"),
        Line::from(""),
        Line::from("  Features:"),
        Line::from("    - Auto-reads directories"),
        Line::from("    - Lazy-loads subdirectories"),
        Line::from("    - Dev icons for file types"),
        Line::from("    - Hidden file support"),
        Line::from("    - Vim-style navigation"),
        Line::from("    - File filtering"),
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
