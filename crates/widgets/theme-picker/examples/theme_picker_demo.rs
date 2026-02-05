use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};
use ratkit_theme_picker::{ThemePicker, ThemePickerEvent};

struct ThemePickerDemo {
    picker: ThemePicker,
    last_event: String,
}

impl ThemePickerDemo {
    fn new() -> Self {
        let mut picker = ThemePicker::new();
        picker.show();
        Self {
            picker,
            last_event: "Previewing themes".to_string(),
        }
    }
}

impl App for ThemePickerDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
            RunnerEvent::Crossterm(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                if key.code == KeyCode::Char('q') {
                    return Ok(RunnerAction::Quit);
                }

                if key.code == KeyCode::Char('t') {
                    if self.picker.is_visible() {
                        self.picker.hide();
                    } else {
                        self.picker.show();
                    }
                    return Ok(RunnerAction::Redraw);
                }

                if let Some(event) = self.picker.handle_key(&key.code) {
                    self.last_event = match event {
                        ThemePickerEvent::Selected(name) => format!("Selected: {}", name),
                        ThemePickerEvent::Cancelled => "Cancelled".to_string(),
                        ThemePickerEvent::PreviewChanged(name) => {
                            format!("Preview: {}", name)
                        }
                    };
                }
                Ok(RunnerAction::Redraw)
            }
            _ => Ok(RunnerAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let body = Paragraph::new(vec![
            Line::from("t: toggle picker"),
            Line::from("q: quit"),
            Line::from(self.last_event.clone()),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Theme Picker "),
        );
        frame.render_widget(body, area);
        self.picker.render(frame, area);
    }
}

fn main() -> io::Result<()> {
    let mut app = ThemePickerDemo::new();
    run(&mut app, RunConfig::default())
}
