use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind};
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit_button::Button;
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};

struct ButtonDemo {
    button: Button,
    clicks: u32,
}

impl ButtonDemo {
    fn new() -> Self {
        Self {
            button: Button::new("Action"),
            clicks: 0,
        }
    }
}

impl App for ButtonDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
            RunnerEvent::Crossterm(Event::Key(key))
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') =>
            {
                Ok(RunnerAction::Quit)
            }
            RunnerEvent::Crossterm(Event::Mouse(mouse)) => {
                match mouse.kind {
                    MouseEventKind::Moved => {
                        self.button.update_hover(mouse.column, mouse.row);
                    }
                    MouseEventKind::Down(MouseButton::Left) => {
                        self.button.update_hover(mouse.column, mouse.row);
                        if self.button.is_clicked(mouse.column, mouse.row) {
                            self.clicks += 1;
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
        let block = Block::default().borders(Borders::ALL).title(" Button ");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let button_area = button_demo_area(inner, &self.button);
        self.button.set_area(button_area);
        let button_text = format!(" [{}] ", self.button.text());
        let button_style = if self.button.hovered() {
            self.button.hover()
        } else {
            self.button.normal()
        };
        let button = Paragraph::new(Line::from(Span::styled(button_text, button_style)));
        frame.render_widget(button, button_area);

        let body_area = Rect {
            x: inner.x,
            y: inner.y + 2,
            width: inner.width,
            height: inner.height.saturating_sub(2),
        };
        let body = Paragraph::new(Line::from(format!(
            "Click the button (mouse) or press q to quit. Clicks: {}",
            self.clicks
        )));
        frame.render_widget(body, body_area);
    }
}

fn button_demo_area(inner: Rect, button: &Button) -> Rect {
    let button_width = format!(" [{}] ", button.text()).len() as u16;
    let x = inner.x + 2;
    let y = inner.y + 1;

    Rect {
        x,
        y,
        width: button_width,
        height: 1,
    }
}

fn main() -> io::Result<()> {
    let mut app = ButtonDemo::new();
    run(&mut app, RunConfig::default())
}
