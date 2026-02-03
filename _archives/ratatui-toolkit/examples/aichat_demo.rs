//! Standalone AI Chat Widget Demo
//!
//! Run with: cargo run --example aichat_demo --features full

use std::io;

use crossterm::{
    cursor::{SetCursorStyle, Show},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
    Terminal,
};
use ratatui_toolkit::{AIChat, Message};

struct AppState {
    chat: AIChat,
    selected_command_index: usize,
    is_loading: bool,
}

fn main() -> io::Result<()> {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        let _ = execute!(io::stdout(), Show);
        original_hook(panic_info);
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        SetCursorStyle::BlinkingBar
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.show_cursor()?;

    let mut chat = AIChat::new_ai_chat();
    chat.messages().add(Message::assistant(
        "Hello! I'm your AI assistant. Type a message and press Enter to send.\n\nCommands:\n  /clear - Clear all messages\n\nFile Attachments:\n  Type @ to attach files from the current directory\n  Use ↑/↓ to navigate the file list\n  Press Enter or Esc to close popup\n\nCommand Mode:\n  Type / to enter command mode\n  Use ↑/↓ to navigate commands\n  Press Enter to execute or Esc to cancel".to_string(),
    ));
    chat.input().load_files_from_cwd();

    let app = AppState {
        chat,
        selected_command_index: 0,
        is_loading: false,
    };

    let result = run_demo(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    result
}

fn run_demo(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    mut app: AppState,
) -> io::Result<()> {
    let mut render_area = Rect::default();

    loop {
        terminal.draw(|frame| {
            render_area = frame.area();
            let widget = app
                .chat
                .with_selected_command_index(app.selected_command_index)
                .with_user_message_style(
                    Style::default()
                        .fg(Color::LightCyan)
                        .add_modifier(Modifier::BOLD),
                )
                .with_ai_message_style(Style::default().fg(Color::White))
                .with_input_style(Style::default().fg(Color::White))
                .with_prompt("You: ".to_string());
            if app.is_loading {
                widget.set_loading(true);
            }
            widget.render(frame, render_area);
        })?;

        if app.is_loading {
            std::thread::sleep(std::time::Duration::from_millis(500));
            app.chat.messages().add(Message::assistant(
                "This is a simulated AI response. In a real implementation, this would connect to an LLM API.".to_string(),
            ));
            app.is_loading = false;
        }

        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                    if key.code == KeyCode::Enter {
                        if app.chat.input().is_command_mode() {
                            let filtered = app.chat.filtered_commands();
                            if let Some(cmd) = filtered.get(app.selected_command_index) {
                                if cmd == "/clear" {
                                    app.chat.messages().clear();
                                }
                                app.chat.input().handle_key(key);
                            }
                        } else if let Some(result) = app.chat.input().handle_key(key) {
                            app.chat.messages().add(Message::user(result.clone()));
                            app.is_loading = true;
                        }
                    } else if app.chat.input().is_command_mode() {
                        if key.code == KeyCode::Up || key.code == KeyCode::Char('k') {
                            let filtered = app.chat.filtered_commands();
                            if !filtered.is_empty() {
                                app.selected_command_index = if app.selected_command_index == 0 {
                                    filtered.len() - 1
                                } else {
                                    app.selected_command_index - 1
                                };
                            }
                        } else if key.code == KeyCode::Down || key.code == KeyCode::Char('j') {
                            let filtered = app.chat.filtered_commands();
                            if !filtered.is_empty() {
                                app.selected_command_index =
                                    (app.selected_command_index + 1) % filtered.len();
                            }
                        } else {
                            app.chat.input().handle_key(key);
                        }
                    } else {
                        app.chat.input().handle_key(key);
                    }
                }
                _ => {}
            }
        }
    }
}
