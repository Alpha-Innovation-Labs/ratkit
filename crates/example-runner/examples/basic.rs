use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::Alignment,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};

struct RunnerDemo {
    ticks: u64,
}

impl App for RunnerDemo {
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
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Example Runner ");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let content = Paragraph::new(Line::from(format!(
            "Ticks: {}  |  Press q to quit",
            self.ticks
        )))
        .alignment(Alignment::Center);
        frame.render_widget(content, inner);
    }
}

fn main() -> io::Result<()> {
    let mut app = RunnerDemo { ticks: 0 };
    run(&mut app, RunConfig::default())
}
