use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};
use ratkit_scroll::calculate_scroll_offset;

struct ScrollDemo {
    selected: usize,
    items: Vec<String>,
}

impl ScrollDemo {
    fn new() -> Self {
        let items = (1..=50).map(|i| format!("Item {}", i)).collect();
        Self { selected: 0, items }
    }
}

impl App for ScrollDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
            RunnerEvent::Crossterm(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Char('q') => return Ok(RunnerAction::Quit),
                    KeyCode::Up => {
                        if self.selected > 0 {
                            self.selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.selected + 1 < self.items.len() {
                            self.selected += 1;
                        }
                    }
                    _ => {}
                }
                Ok(RunnerAction::Redraw)
            }
            _ => Ok(RunnerAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        let header = Paragraph::new(Line::from("Scroll offset demo (Up/Down, q to quit)"))
            .block(Block::default().borders(Borders::ALL).title(" Header "));
        frame.render_widget(header, chunks[0]);

        let visible_count = chunks[1].height.saturating_sub(2) as usize;
        let offset = calculate_scroll_offset(self.selected, visible_count.max(1), self.items.len());

        let mut lines = Vec::new();
        for (idx, item) in self
            .items
            .iter()
            .enumerate()
            .skip(offset)
            .take(visible_count)
        {
            if idx == self.selected {
                lines.push(Line::from(format!("> {}", item)));
            } else {
                lines.push(Line::from(format!("  {}", item)));
            }
        }

        let body =
            Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" Items "));
        frame.render_widget(body, chunks[1]);
    }
}

fn main() -> io::Result<()> {
    let mut app = ScrollDemo::new();
    run(&mut app, RunConfig::default())
}
