//! Shared terminal runner for ratkit examples.

use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::Show,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
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

/// Runner configuration.
#[derive(Debug, Clone, Copy)]
pub struct RunConfig {
    /// Duration between tick events.
    pub tick_rate: Duration,
}

impl Default for RunConfig {
    fn default() -> Self {
        Self {
            tick_rate: Duration::from_millis(50),
        }
    }
}

/// Events emitted by the runner.
#[derive(Debug, Clone)]
pub enum RunnerEvent {
    /// A raw crossterm event (excluding resize events).
    Crossterm(Event),
    /// A periodic tick event.
    Tick,
    /// A terminal resize event.
    Resize { width: u16, height: u16 },
}

/// Action requested by the application after handling an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunnerAction {
    /// Continue without redrawing.
    Continue,
    /// Redraw the frame.
    Redraw,
    /// Exit the loop.
    Quit,
}

/// Application hooks used by the runner loop.
pub trait App {
    /// Handle a runner event and return an action for the runner.
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction>;

    /// Draw the application frame.
    fn on_draw(&mut self, frame: &mut Frame);
}

/// Run an application inside a crossterm + ratatui terminal.
pub fn run<A: App>(app: &mut A, config: RunConfig) -> io::Result<()> {
    install_panic_hook();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_loop(&mut terminal, app, config);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_loop<A: App>(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut A,
    config: RunConfig,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut last_fps = Instant::now();
    let mut frames = 0u32;
    let mut redraws = 0u64;
    let mut fps = 0u16;
    let mut needs_redraw = true;

    loop {
        if needs_redraw {
            terminal.draw(|frame| {
                app.on_draw(frame);
                draw_fps(frame, fps, redraws);
            })?;

            frames += 1;
            redraws = redraws.saturating_add(1);
            let fps_elapsed = last_fps.elapsed();
            if fps_elapsed >= Duration::from_secs(1) {
                let elapsed_ms = fps_elapsed.as_millis().max(1) as u32;
                fps = ((frames.saturating_mul(1000)) / elapsed_ms) as u16;
                frames = 0;
                last_fps = Instant::now();
            }
            needs_redraw = false;
        }

        let timeout = config.tick_rate.saturating_sub(last_tick.elapsed());

        if event::poll(timeout)? {
            let event = event::read()?;
            let action = match event {
                Event::Resize(width, height) => {
                    app.on_event(RunnerEvent::Resize { width, height })?
                }
                _ => app.on_event(RunnerEvent::Crossterm(event))?,
            };

            match action {
                RunnerAction::Quit => return Ok(()),
                RunnerAction::Redraw => needs_redraw = true,
                RunnerAction::Continue => {}
            }
        }

        if last_tick.elapsed() >= config.tick_rate {
            match app.on_event(RunnerEvent::Tick)? {
                RunnerAction::Quit => return Ok(()),
                RunnerAction::Redraw => needs_redraw = true,
                RunnerAction::Continue => {}
            }
            last_tick = Instant::now();
        }
    }
}

fn draw_fps(frame: &mut Frame, fps: u16, redraws: u64) {
    let area = frame.area();
    let text = format!("FPS {:>3} | Redraws {}", fps, redraws);
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
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        let _ = execute!(io::stdout(), Show);
        original_hook(panic_info);
    }));
}
