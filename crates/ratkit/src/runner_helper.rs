//! Terminal runner for ratkit core runtime applications.

use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    cursor::Show,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::Paragraph,
    Frame, Terminal,
};

use crate::core::{
    CoordinatorApp, KeyboardEvent, MouseEvent, ResizeEvent, Runner, RunnerAction, RunnerConfig,
    RunnerEvent, TickEvent,
};

/// Run a coordinator application with the ratkit core runtime.
///
/// This function sets up the terminal, creates a Runner, and runs the event loop
/// until the application requests to quit.
///
/// # Example
///
/// ```rust,no_run
/// use ratkit::prelude::*;
/// use ratatui::Frame;
///
/// struct MyApp;
///
/// impl CoordinatorApp for MyApp {
///     fn on_event(&mut self, event: CoordinatorEvent) -> LayoutResult<CoordinatorAction> {
///         Ok(CoordinatorAction::Continue)
///     }
///
///     fn on_draw(&mut self, frame: &mut Frame) {
///         // Draw your UI here
///     }
/// }
///
/// fn main() -> std::io::Result<()> {
///     let app = MyApp;
///     run(app, RunnerConfig::default())
/// }
/// ```
pub fn run<A: CoordinatorApp>(app: A, config: RunnerConfig) -> io::Result<()> {
    install_panic_hook();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        Print("\x1b[?1006h\x1b[?1003h")
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_loop(&mut terminal, app, config, false);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        Print("\x1b[?1003l\x1b[?1006l")
    )?;
    terminal.show_cursor()?;

    result
}

/// Run a coordinator application with diagnostics overlay enabled.
pub fn run_with_diagnostics<A: CoordinatorApp>(app: A, config: RunnerConfig) -> io::Result<()> {
    install_panic_hook();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        Print("\x1b[?1006h\x1b[?1003h")
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_loop(&mut terminal, app, config, true);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        Print("\x1b[?1003l\x1b[?1006l")
    )?;
    terminal.show_cursor()?;

    result
}

fn run_loop<A: CoordinatorApp>(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: A,
    config: RunnerConfig,
    draw_diagnostics: bool,
) -> io::Result<()> {
    let mut runner = Runner::new(app).with_config(config);
    let size = terminal.size()?;
    runner
        .handle_event(RunnerEvent::Resize(ResizeEvent::new(
            size.width,
            size.height,
        )))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let mut last_tick = Instant::now();
    let tick_rate = config.tick_rate;
    let mut tick_count: u64 = 0;
    let mut last_fps = Instant::now();
    let mut frames = 0u32;
    let mut redraws = 0u64;
    let mut fps = 0u16;
    let mut last_mouse = (0u16, 0u16);

    // Initial draw
    terminal.draw(|frame| {
        let _ = runner.render(frame);
        if draw_diagnostics {
            draw_fps(frame, fps, redraws, last_mouse);
        }
    })?;
    redraws = redraws.saturating_add(1);

    loop {
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        if event::poll(timeout)? {
            let crossterm_event = event::read()?;

            if let Event::Mouse(mouse) = crossterm_event {
                last_mouse = (mouse.column, mouse.row);
            }

            let runner_event = convert_event(crossterm_event);

            let action = runner
                .handle_event(runner_event)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

            match action {
                RunnerAction::Quit => return Ok(()),
                RunnerAction::Redraw => {
                    terminal.draw(|frame| {
                        let _ = runner.render(frame);
                        if draw_diagnostics {
                            draw_fps(frame, fps, redraws, last_mouse);
                        }
                    })?;
                    redraws = redraws.saturating_add(1);
                    frames += 1;
                }
                RunnerAction::Continue => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            tick_count += 1;
            let tick_event = RunnerEvent::Tick(TickEvent::new(tick_count));
            let action = runner
                .handle_event(tick_event)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

            match action {
                RunnerAction::Quit => return Ok(()),
                RunnerAction::Redraw => {
                    terminal.draw(|frame| {
                        let _ = runner.render(frame);
                        if draw_diagnostics {
                            draw_fps(frame, fps, redraws, last_mouse);
                        }
                    })?;
                    redraws = redraws.saturating_add(1);
                    frames += 1;
                }
                RunnerAction::Continue => {}
            }

            last_tick = Instant::now();
        }

        // Update FPS counter
        let fps_elapsed = last_fps.elapsed();
        if fps_elapsed >= Duration::from_secs(1) {
            let elapsed_ms = fps_elapsed.as_millis().max(1) as u32;
            fps = ((frames.saturating_mul(1000)) / elapsed_ms) as u16;
            frames = 0;
            last_fps = Instant::now();
        }
    }
}

fn convert_event(event: Event) -> RunnerEvent {
    match event {
        Event::Key(key) => RunnerEvent::Keyboard(KeyboardEvent::from_crossterm(key)),
        Event::Mouse(mouse) => RunnerEvent::Mouse(MouseEvent::from_crossterm(mouse)),
        Event::Resize(width, height) => RunnerEvent::Resize(ResizeEvent::new(width, height)),
        _ => RunnerEvent::Tick(TickEvent::new(0)), // Fallback, should not happen often
    }
}

fn draw_fps(frame: &mut Frame, fps: u16, redraws: u64, mouse: (u16, u16)) {
    let area = frame.area();
    let text = format!(
        "FPS {:>3} | Redraws {} | Mouse {},{}",
        fps, redraws, mouse.0, mouse.1
    );
    let width = text.len() as u16 + 2;
    let x = area.x + area.width.saturating_sub(width);
    let rect = Rect {
        x,
        y: area.y,
        width,
        height: 1,
    };
    let line = Line::from(format!(" {} ", text));
    let style = Style::default().fg(Color::DarkGray);
    frame.render_widget(Paragraph::new(line).style(style), rect);
}

fn install_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            Print("\x1b[?1003l\x1b[?1006l")
        );
        let _ = execute!(io::stdout(), Show);
        original_hook(panic_info);
    }));
}
