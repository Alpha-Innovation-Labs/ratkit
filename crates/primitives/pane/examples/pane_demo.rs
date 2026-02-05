use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    style::{Color, Style},
    text::Line,
    Frame,
};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};
use ratkit_pane::Pane;

struct PaneDemo {
    ticks: u64,
}

impl App for PaneDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
            RunnerEvent::Tick => {
                self.ticks += 1;
                Ok(RunnerAction::Redraw)
            }
            RunnerEvent::Crossterm(Event::Key(key))
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') =>
            {
                Ok(RunnerAction::Quit)
            }
            _ => Ok(RunnerAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let pane = Pane::new("Pane")
            .with_icon("■")
            .with_uniform_padding(1)
            .border_style(Style::default().fg(Color::Cyan));

        pane.render_paragraph(
            frame,
            area,
            vec![
                Line::from("Minimal pane demo"),
                Line::from(format!("Ticks: {}", self.ticks)),
                Line::from("Press q to quit"),
            ],
        );
    }
}

fn main() -> io::Result<()> {
    let mut app = PaneDemo { ticks: 0 };
    run(&mut app, RunConfig::default())
}
