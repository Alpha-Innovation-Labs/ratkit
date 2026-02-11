use std::io;

use crossterm::event::KeyCode;
use ratatui::{
    style::{Color, Style},
    text::Line,
    widgets::Paragraph,
    Frame,
};
use ratkit::prelude::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};
use ratkit::widgets::{Button, Pane};

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

impl CoordinatorApp for RatkitDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(keyboard) if keyboard.key_code == KeyCode::Char('q') => {
                Ok(CoordinatorAction::Quit)
            }
            _ => Ok(CoordinatorAction::Redraw),
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
            Line::from("(Runner-first + master widgets namespace demo)"),
        ];

        let inner = pane.render_block(frame, area).0;
        frame.render_widget(Paragraph::new(content), inner);
    }
}

fn main() -> io::Result<()> {
    let app = RatkitDemo::new();
    run_with_diagnostics(app, RunnerConfig::default())
}
