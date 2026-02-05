use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};
use ratkit_hotkey_service::{Hotkey, HotkeyRegistry, HotkeyScope};

struct HotkeyServiceDemo {
    registry: HotkeyRegistry,
    scope: HotkeyScope,
}

impl HotkeyServiceDemo {
    fn new() -> Self {
        let mut registry = HotkeyRegistry::new();
        registry.register(Hotkey::new("q", "Quit application").scope(HotkeyScope::Global));
        registry.register(Hotkey::new("j", "Move down").scope(HotkeyScope::Tab("Demo")));
        registry.register(Hotkey::new("k", "Move up").scope(HotkeyScope::Tab("Demo")));

        Self {
            registry,
            scope: HotkeyScope::Tab("Demo"),
        }
    }
}

impl App for HotkeyServiceDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
            RunnerEvent::Crossterm(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                if key.code == KeyCode::Char('q') {
                    return Ok(RunnerAction::Quit);
                }
                Ok(RunnerAction::Redraw)
            }
            _ => Ok(RunnerAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let mut lines = vec![Line::from("Hotkeys for Demo scope:"), Line::from("")];

        for hotkey in self.registry.get_hotkeys() {
            if hotkey.scope != self.scope && hotkey.scope != HotkeyScope::Global {
                continue;
            }
            lines.push(Line::from(format!(
                "{}  - {}",
                hotkey.key, hotkey.description
            )));
        }

        let body = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" HotkeyRegistry "),
        );
        frame.render_widget(body, area);
    }
}

fn main() -> io::Result<()> {
    let mut app = HotkeyServiceDemo::new();
    run(&mut app, RunConfig::default())
}
