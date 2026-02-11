use std::io;

use crossterm::event::{MouseButton, MouseEventKind};
use ratatui::{
    layout::{Alignment, Rect},
    text::Line,
    widgets::{Clear, Paragraph},
    Frame,
};
use ratkit::prelude::{
    run, CoordinatorAction, CoordinatorApp, CoordinatorEvent, LayoutResult, RunnerConfig,
};

struct MouseOnlyDemo {
    x: u16,
    y: u16,
}

impl MouseOnlyDemo {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl CoordinatorApp for MouseOnlyDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Mouse(mouse) => {
                self.x = mouse.x();
                self.y = mouse.y();

                if mouse.kind == MouseEventKind::Down(MouseButton::Right) {
                    return Ok(CoordinatorAction::Quit);
                }

                Ok(CoordinatorAction::Redraw)
            }
            _ => Ok(CoordinatorAction::Continue),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        frame.render_widget(Clear, area);

        let lines = vec![
            Line::from("mouse only example"),
            Line::from(""),
            Line::from(format!("x: {}", self.x)),
            Line::from(format!("y: {}", self.y)),
            Line::from(""),
            Line::from("Move the mouse to update coordinates"),
            Line::from("Right click to quit"),
        ];

        let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
        frame.render_widget(paragraph, centered_box(area, 42, 7));
    }
}

fn centered_box(area: Rect, width: u16, height: u16) -> Rect {
    let clamped_width = width.min(area.width.max(1));
    let clamped_height = height.min(area.height.max(1));

    Rect {
        x: area.x + area.width.saturating_sub(clamped_width) / 2,
        y: area.y + area.height.saturating_sub(clamped_height) / 2,
        width: clamped_width,
        height: clamped_height,
    }
}

fn main() -> io::Result<()> {
    let app = MouseOnlyDemo::new();
    run(app, RunnerConfig::default())
}
