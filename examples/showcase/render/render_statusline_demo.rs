//! Render the statusline demo tab.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui_toolkit::{StatusLineStacked, SLANT_BL_TR, SLANT_TL_BR};

use crate::app::App;
use crate::demo_mode::DemoMode;

/// Render the statusline demo.
pub fn render_statusline_demo(frame: &mut ratatui::Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Instructions
            Constraint::Length(1), // StatusLineStacked demo
            Constraint::Length(1), // Another style
            Constraint::Length(1), // Yet another style
        ])
        .split(area);

    // Instructions
    let mode_name = match app.status_mode {
        DemoMode::Normal => "NORMAL",
        DemoMode::Insert => "INSERT",
        DemoMode::Visual => "VISUAL",
        DemoMode::Command => "COMMAND",
    };

    let content = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  StatusLineStacked - Neovim-style Powerline",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!("  Current mode: {}", mode_name)),
        Line::from(""),
        Line::from("  Press a key to change mode:"),
        Line::from(""),
        Line::styled("    [n] Normal mode", Style::default().fg(Color::Blue)),
        Line::styled("    [i] Insert mode", Style::default().fg(Color::Green)),
        Line::styled("    [v] Visual mode", Style::default().fg(Color::Magenta)),
        Line::styled("    [c] Command mode", Style::default().fg(Color::Yellow)),
        Line::from(""),
        Line::from("  Features:"),
        Line::from("    • Powerline-style diagonal separators"),
        Line::from("    • Stacked indicators left & right"),
        Line::from("    • Requires Nerd Font for glyphs"),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" StatusLineStacked Demo "),
    );
    frame.render_widget(content, chunks[0]);

    // StatusLineStacked - Style 1 (mode indicator)
    let (mode_color, mode_text) = match app.status_mode {
        DemoMode::Normal => (Color::Blue, " NORMAL "),
        DemoMode::Insert => (Color::Green, " INSERT "),
        DemoMode::Visual => (Color::Magenta, " VISUAL "),
        DemoMode::Command => (Color::Yellow, " COMMAND "),
    };

    let status1 = StatusLineStacked::new()
        .start(
            Span::from(mode_text).style(Style::new().fg(Color::Black).bg(mode_color)),
            Span::from(SLANT_TL_BR).style(Style::new().fg(mode_color).bg(Color::DarkGray)),
        )
        .start(
            Span::from(" main ").style(Style::new().fg(Color::White).bg(Color::DarkGray)),
            Span::from(SLANT_TL_BR).style(Style::new().fg(Color::DarkGray)),
        )
        .center("showcase.rs")
        .end(
            Span::from(" UTF-8 ").style(Style::new().fg(Color::Black).bg(Color::Cyan)),
            Span::from(SLANT_BL_TR).style(Style::new().fg(Color::Cyan)),
        );
    frame.render_widget(status1, chunks[1]);

    // StatusLineStacked - Style 2
    let status2 = StatusLineStacked::new()
        .start(
            Span::from("  rust ").style(Style::new().fg(Color::Black).bg(Color::Red)),
            Span::from(SLANT_TL_BR).style(Style::new().fg(Color::Red).bg(Color::Gray)),
        )
        .start(
            Span::from(" src/lib.rs ").style(Style::new().fg(Color::Black).bg(Color::Gray)),
            Span::from(SLANT_TL_BR).style(Style::new().fg(Color::Gray)),
        )
        .center("ratatui-toolkit v0.1.0")
        .end(
            Span::from(" Ln 42 ").style(Style::new().fg(Color::Black).bg(Color::Green)),
            Span::from(SLANT_BL_TR).style(Style::new().fg(Color::Green).bg(Color::Gray)),
        )
        .end(
            Span::from(" Col 8 ").style(Style::new().fg(Color::Black).bg(Color::Gray)),
            Span::from(SLANT_BL_TR).style(Style::new().fg(Color::Gray)),
        );
    frame.render_widget(status2, chunks[2]);

    // StatusLineStacked - Style 3 (minimal)
    let status3 = StatusLineStacked::new()
        .start(
            Span::from(" 󰈙 ").style(Style::new().fg(Color::Cyan)),
            Span::from("").style(Style::new()),
        )
        .center("Press ? for help")
        .end(
            Span::from(" 100% ").style(Style::new().fg(Color::Green)),
            Span::from("").style(Style::new()),
        );
    frame.render_widget(status3, chunks[3]);
}
