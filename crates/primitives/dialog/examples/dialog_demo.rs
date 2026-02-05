use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::Frame;
use ratkit_dialog::{Dialog, DialogWidget};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};

struct DialogDemo {
    dialog: Dialog<'static>,
}

impl DialogDemo {
    fn new() -> Self {
        let dialog = Dialog::confirm("Delete file", "Remove README.md from disk?")
            .footer("Left/Right to change selection")
            .overlay(true)
            .title_inside(true);

        Self { dialog }
    }
}

impl App for DialogDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
            RunnerEvent::Crossterm(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Char('q') => return Ok(RunnerAction::Quit),
                    KeyCode::Left => {
                        if self.dialog.selected_button > 0 {
                            self.dialog.selected_button -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if self.dialog.selected_button + 1 < self.dialog.buttons.len() {
                            self.dialog.selected_button += 1;
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
        frame.render_widget(DialogWidget::new(&mut self.dialog), area);
    }
}

fn main() -> io::Result<()> {
    let mut app = DialogDemo::new();
    run(&mut app, RunConfig::default())
}
