use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use ratatui_interact::components::{
    SplitPaneState, SplitPane, SplitPaneStyle, Orientation,
    Progress, Spinner, SpinnerFrames, SpinnerState,
};
use std::time::Duration;
use std::io;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut split_state = SplitPaneState::new(50);
    let mut spinner_state = SpinnerState::new();
    let mut progress = 0.0;
    let tick_rate = Duration::from_millis(100);

    loop {
        terminal.draw(|f| {
            let size = f.area();

            let split_pane = SplitPane::new(&split_state)
                .orientation(Orientation::Horizontal)
                .style(SplitPaneStyle::default());

            let (left_area, _divider_area, right_area) = split_pane.calculate_areas(size);

            let instructions = vec![
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
                "  ratatui-interact Demo",
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
                "",
                "  Features:",
                "  ✓ Resizable split panes",
                "  ✓ Animated spinner (Braille)",
                "  ✓ Progress bar",
                "",
                "  Controls:",
                "  • Left/Right - Resize panes",
                "  • Q/Esc - Quit",
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
            ];
            let instructions = ratatui::widgets::Paragraph::new(instructions.join("\n"))
                .block(ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .title("Left Pane"))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(instructions, left_area);

            let right_chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .margin(1)
                .constraints([
                    ratatui::layout::Constraint::Length(3),
                    ratatui::layout::Constraint::Length(3),
                    ratatui::layout::Constraint::Length(3),
                    ratatui::layout::Constraint::Min(0),
                ])
                .split(right_area);

            let header = ratatui::widgets::Paragraph::new("🎨 Animated Widgets")
                .style(ratatui::style::Style::default()
                    .fg(ratatui::style::Color::Cyan)
                    .add_modifier(ratatui::style::Modifier::BOLD));
            f.render_widget(header, right_chunks[0]);

            let spinner = Spinner::new(&spinner_state)
                .label("Loading...")
                .frames(SpinnerFrames::Braille);
            f.render_widget(spinner, right_chunks[1]);

            let prog = Progress::new(progress).label("Progress:");
            f.render_widget(prog, right_chunks[2]);
        })?;

        if event::poll(tick_rate)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Left => {
                            split_state.split_percent = split_state.split_percent.saturating_sub(5);
                            split_state.split_percent = split_state.split_percent.max(10).min(90);
                        }
                        KeyCode::Right => {
                            split_state.split_percent += 5;
                            split_state.split_percent = split_state.split_percent.max(10).min(90);
                        }
                        _ => {}
                    }
                }
                Event::Mouse(mouse) => {
                    if let MouseEventKind::Drag(_) = mouse.kind {
                        if mouse.column > 40 {
                            split_state.split_percent += 1;
                        } else {
                            split_state.split_percent = split_state.split_percent.saturating_sub(1);
                        }
                        split_state.split_percent = split_state.split_percent.max(10).min(90);
                    }
                }
                _ => {}
            }
        }

        spinner_state.tick_with_frames(SpinnerFrames::Braille.frames().len());
        progress = (progress + 0.005) % 1.01;
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
