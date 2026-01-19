//! Terminal Pane Demo
//!
//! Demonstrates a simple embedded terminal pane using TermTui.
//!
//! Run with: cargo run --features terminal --example terminal_demo
//! Or: just demo-term

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, layout::Rect, Terminal};
use ratatui_toolkit::termtui::TermTui;
use std::io;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let mut term = TermTui::spawn_with_command("Terminal", &shell, &[])?;
    term.focused = true;

    let mut render_area = Rect::default();

    loop {
        terminal.draw(|frame| {
            render_area = frame.area();
            term.render(frame, render_area);
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('q')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        break;
                    }
                    term.handle_key(key);
                }
                Event::Mouse(mouse) => {
                    term.handle_mouse(mouse, render_area);
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
