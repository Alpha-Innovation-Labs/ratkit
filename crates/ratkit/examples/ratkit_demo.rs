use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit::{Button, Pane};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};

struct RatkitDemo {
    button: Button,
}

impl RatkitDemo {
    fn new() -> Self {
        Self {
            button: Button::new("Run"),
        }
    }
}

impl App for RatkitDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
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
        let pane = Pane::new("ratkit")
            .with_icon("◎")
            .with_uniform_padding(1)
            .border_style(Style::default().fg(Color::Cyan));

        let button_line = self.button.render_with_title(area, "ratkit demo");
        let content = vec![
            button_line,
            Line::from("Press q to quit"),
            Line::from("(Meta-crate re-export demo)"),
        ];

        let inner = pane.render_block(frame, area).0;
        frame.render_widget(Paragraph::new(content), inner);
    }
}

fn main() -> io::Result<()> {
    let mut app = RatkitDemo::new();
    run(&mut app, RunConfig::default())
}
