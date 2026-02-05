use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};
use ratkit_widget_event::WidgetEvent;

struct WidgetEventDemo {
    last: WidgetEvent,
}

impl WidgetEventDemo {
    fn new() -> Self {
        Self {
            last: WidgetEvent::None,
        }
    }
}

impl App for WidgetEventDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
            RunnerEvent::Crossterm(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Char('q') => return Ok(RunnerAction::Quit),
                    KeyCode::Char('s') => {
                        self.last = WidgetEvent::Selected { path: vec![0, 1] };
                    }
                    KeyCode::Char('t') => {
                        self.last = WidgetEvent::Toggled {
                            path: vec![0, 2],
                            expanded: true,
                        };
                    }
                    KeyCode::Char('f') => {
                        self.last = WidgetEvent::FilterModeChanged {
                            active: true,
                            filter: "name".to_string(),
                        };
                    }
                    _ => {
                        self.last = WidgetEvent::None;
                    }
                }
                Ok(RunnerAction::Redraw)
            }
            _ => Ok(RunnerAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let body = Paragraph::new(vec![
            Line::from("s: Selected"),
            Line::from("t: Toggled"),
            Line::from("f: FilterModeChanged"),
            Line::from(format!("Last: {:?}", self.last)),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" WidgetEvent "),
        );
        frame.render_widget(body, area);
    }
}

fn main() -> io::Result<()> {
    let mut app = WidgetEventDemo::new();
    run(&mut app, RunConfig::default())
}
