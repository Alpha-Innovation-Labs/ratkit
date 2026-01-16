//! TermTui Demo - Interactive terminal emulator
//!
//! Run with: cargo run --features terminal --example termtui_demo

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
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Spawn a shell
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let mut term = TermTui::spawn_with_command("TermTui Demo", &shell, &[])?;
    term.focused = true;

    // Track the render area for mouse coordinate translation
    let mut render_area = Rect::default();

    // Main loop
    loop {
        // Render
        terminal.draw(|frame| {
            render_area = frame.area();
            term.render(frame, render_area);
        })?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => {
                    // Ctrl+Q to quit
                    if key.code == KeyCode::Char('q')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        break;
                    }

                    // Pass to terminal
                    term.handle_key(key);
                }
                Event::Mouse(mouse) => {
                    // Use the new unified handle_mouse method
                    term.handle_mouse(mouse, render_area);
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
