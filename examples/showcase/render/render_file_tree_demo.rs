//! Render the file system tree demo tab.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, StatefulWidget},
};
use ratatui_toolkit::AppTheme;

use crate::app::App;

/// Render the file system tree demo.
pub fn render_file_tree_demo(
    frame: &mut ratatui::Frame,
    area: Rect,
    app: &mut App,
    _theme: &AppTheme,
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

    // Render the FileSystemTree with a block
    // FileSystemTree handles its own filter line rendering internally
    let tree_widget = file_tree
        .clone()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" FileSystemTree - Current Directory "),
        );
    frame.render_stateful_widget(tree_widget, chunks[0], &mut app.file_tree_state);

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
