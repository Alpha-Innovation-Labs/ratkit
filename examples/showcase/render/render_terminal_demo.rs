//! Render the terminal demo tab.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

/// Render the terminal demo.
pub fn render_terminal_demo(frame: &mut ratatui::Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Terminal - render directly in the area
    if let Some(ref mut term) = app.terminal {
        term.render(frame, chunks[0]);
    } else {
        // Fallback if terminal failed to spawn
        let fallback = Paragraph::new("Terminal failed to spawn").block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Terminal "),
        );
        frame.render_widget(fallback, chunks[0]);
    }

    // Copy mode indicator
    let copy_mode_info = if let Some(ref term) = app.terminal {
        if term.copy_mode.is_active() {
            "COPY MODE (hjkl/arrows to move, v to select, y to copy, Esc to exit)"
        } else {
            "Press Ctrl+X to enter copy mode"
        }
    } else {
        ""
    };

    // Info panel
    let info = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  TermTui Features",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  A terminal emulator using"),
        Line::from("  termwiz + mprocs architecture."),
        Line::from(""),
        Line::from("  Features:"),
        Line::from("    • VT100 escape sequences"),
        Line::from("    • Full color support (256/RGB)"),
        Line::from("    • VecDeque scrollback buffer"),
        Line::from("    • Copy mode (Ctrl+X)"),
        Line::from("    • Vim-style navigation (hjkl)"),
        Line::from("    • Visual selection (v + y)"),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", copy_mode_info),
            Style::default().fg(Color::Yellow),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Info "),
    );
    frame.render_widget(info, chunks[1]);
}
