//! Render the statusline demo tab.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui_toolkit::{AppTheme, StatusLineStacked, SLANT_BL_TR, SLANT_TL_BR};

use crate::app::App;
use crate::demo_mode::DemoMode;

/// Render the statusline demo.
///
/// # Arguments
///
/// * `frame` - The frame to render into.
/// * `area` - The area to render in.
/// * `app` - The application state.
/// * `theme` - The application theme.
pub fn render_statusline_demo(
    frame: &mut ratatui::Frame,
    area: Rect,
    app: &mut App,
    theme: &AppTheme,
) {
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
        Line::styled("    [n] Normal mode", Style::default().fg(theme.info)),
        Line::styled("    [i] Insert mode", Style::default().fg(theme.success)),
        Line::styled("    [v] Visual mode", Style::default().fg(theme.secondary)),
        Line::styled("    [c] Command mode", Style::default().fg(theme.warning)),
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
        DemoMode::Normal => (theme.info, " NORMAL "),
        DemoMode::Insert => (theme.success, " INSERT "),
        DemoMode::Visual => (theme.secondary, " VISUAL "),
        DemoMode::Command => (theme.warning, " COMMAND "),
    };

    let status1 = StatusLineStacked::new()
        .start(
            Span::from(mode_text).style(Style::new().fg(theme.background).bg(mode_color)),
            Span::from(SLANT_TL_BR).style(Style::new().fg(mode_color).bg(theme.background_panel)),
        )
        .start(
            Span::from(" main ").style(Style::new().fg(theme.text).bg(theme.background_panel)),
            Span::from(SLANT_TL_BR).style(Style::new().fg(theme.background_panel)),
        )
        .center("showcase.rs")
        .end(
            Span::from(" UTF-8 ").style(Style::new().fg(theme.background).bg(theme.primary)),
            Span::from(SLANT_BL_TR).style(Style::new().fg(theme.primary)),
        );
    frame.render_widget(status1, chunks[1]);

    // StatusLineStacked - Style 2
    let status2 = StatusLineStacked::new()
        .start(
            Span::from("  rust ").style(Style::new().fg(theme.background).bg(theme.error)),
            Span::from(SLANT_TL_BR).style(Style::new().fg(theme.error).bg(theme.text_muted)),
        )
        .start(
            Span::from(" src/lib.rs ")
                .style(Style::new().fg(theme.background).bg(theme.text_muted)),
            Span::from(SLANT_TL_BR).style(Style::new().fg(theme.text_muted)),
        )
        .center("ratatui-toolkit v0.1.0")
        .end(
            Span::from(" Ln 42 ").style(Style::new().fg(theme.background).bg(theme.success)),
            Span::from(SLANT_BL_TR).style(Style::new().fg(theme.success).bg(theme.text_muted)),
        )
        .end(
            Span::from(" Col 8 ").style(Style::new().fg(theme.background).bg(theme.text_muted)),
            Span::from(SLANT_BL_TR).style(Style::new().fg(theme.text_muted)),
        );
    frame.render_widget(status2, chunks[2]);

    // StatusLineStacked - Style 3 (minimal)
    let status3 = StatusLineStacked::new()
        .start(
            Span::from(" 󰈙 ").style(Style::new().fg(theme.primary)),
            Span::from("").style(Style::new()),
        )
        .center("Press ? for help")
        .end(
            Span::from(" 100% ").style(Style::new().fg(theme.success)),
            Span::from("").style(Style::new()),
        );
    frame.render_widget(status3, chunks[3]);
}
