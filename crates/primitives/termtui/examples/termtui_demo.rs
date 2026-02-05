use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, MouseEvent};
use ratatui::{widgets::Block, Frame};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};
use ratkit_termtui::TermTui;

struct TermTuiDemo {
    term: TermTui,
    last_area: ratatui::layout::Rect,
}

impl TermTuiDemo {
    fn new() -> Self {
        let mut term = TermTui::new("TermTui");
        term.focused = true;
        Self {
            term,
            last_area: ratatui::layout::Rect::default(),
        }
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) {
        let _ = self.term.handle_mouse(mouse, self.last_area);
    }
}

impl App for TermTuiDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
            RunnerEvent::Resize { width, height } => {
                self.term.resize(height, width);
                Ok(RunnerAction::Redraw)
            }
            RunnerEvent::Crossterm(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                if key.code == KeyCode::Char('q') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    return Ok(RunnerAction::Quit);
                }
                self.term.handle_key(key);
                Ok(RunnerAction::Redraw)
            }
            RunnerEvent::Crossterm(Event::Mouse(mouse)) => {
                self.handle_mouse(mouse);
                Ok(RunnerAction::Redraw)
            }
            _ => Ok(RunnerAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let block = Block::default().title(" Ctrl+Q to quit ");
        let inner = block.inner(area);
        self.last_area = inner;
        frame.render_widget(block, area);
        self.term.render(frame, inner);
    }
}

fn main() -> io::Result<()> {
    let mut app = TermTuiDemo::new();
    run(&mut app, RunConfig::default())
}
