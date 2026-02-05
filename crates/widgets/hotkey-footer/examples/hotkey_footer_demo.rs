use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};
use ratkit_hotkey_footer::{HotkeyFooter, HotkeyItem};

struct HotkeyFooterDemo;

impl App for HotkeyFooterDemo {
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
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        let body = Paragraph::new(Line::from("Footer on the bottom. Press q to quit.")).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Hotkey Footer "),
        );
        frame.render_widget(body, chunks[0]);

        let footer = HotkeyFooter::new(vec![
            HotkeyItem::new("q", "quit"),
            HotkeyItem::new("?", "help"),
            HotkeyItem::new("/", "search"),
        ]);
        footer.render(frame, chunks[1]);
    }
}

fn main() -> io::Result<()> {
    let mut app = HotkeyFooterDemo;
    run(&mut app, RunConfig::default())
}
