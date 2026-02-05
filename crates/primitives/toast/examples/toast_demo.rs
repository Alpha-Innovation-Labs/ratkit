use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};
use ratkit_toast::{render_toasts, ToastManager};

struct ToastDemo {
    toasts: ToastManager,
}

impl ToastDemo {
    fn new() -> Self {
        Self {
            toasts: ToastManager::new(),
        }
    }
}

impl App for ToastDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
            RunnerEvent::Tick => {
                self.toasts.remove_expired();
                Ok(RunnerAction::Redraw)
            }
            RunnerEvent::Crossterm(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Char('q') => return Ok(RunnerAction::Quit),
                    KeyCode::Char('t') => self.toasts.info("Background task finished"),
                    KeyCode::Char('e') => self.toasts.error("Something went wrong"),
                    KeyCode::Char('c') => self.toasts.clear(),
                    _ => {}
                }
                Ok(RunnerAction::Redraw)
            }
            _ => Ok(RunnerAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let body = Paragraph::new(vec![
            Line::from("t: info toast"),
            Line::from("e: error toast"),
            Line::from("c: clear"),
            Line::from("q: quit"),
        ])
        .block(Block::default().borders(Borders::ALL).title(" Toasts "));
        frame.render_widget(body, area);

        render_toasts(frame, &self.toasts);
    }
}

fn main() -> io::Result<()> {
    let mut app = ToastDemo::new();
    run(&mut app, RunConfig::default())
}
