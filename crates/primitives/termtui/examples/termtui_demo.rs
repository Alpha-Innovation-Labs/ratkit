use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers, MouseEvent as CrosstermMouseEvent};
use ratatui::{widgets::Block, Frame};
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, MouseEvent, ResizeEvent, RunnerConfig,
};
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

    fn handle_mouse(&mut self, mouse: CrosstermMouseEvent) {
        let _ = self.term.handle_mouse(mouse, self.last_area);
    }
}

impl CoordinatorApp for TermTuiDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Resize(ResizeEvent { width, height }) => {
                self.term.resize(height, width);
                Ok(CoordinatorAction::Redraw)
            }
            CoordinatorEvent::Keyboard(keyboard) => {
                if keyboard.key_code == KeyCode::Char('q')
                    && keyboard.modifiers.contains(KeyModifiers::CONTROL)
                {
                    return Ok(CoordinatorAction::Quit);
                }
                let key_event = crossterm::event::KeyEvent {
                    code: keyboard.key_code,
                    modifiers: keyboard.modifiers,
                    kind: KeyEventKind::Press,
                    state: crossterm::event::KeyEventState::empty(),
                };
                self.term.handle_key(key_event);
                Ok(CoordinatorAction::Redraw)
            }
            CoordinatorEvent::Mouse(mouse) => {
                let crossterm_mouse = crossterm::event::MouseEvent {
                    kind: mouse.kind,
                    column: mouse.column,
                    row: mouse.row,
                    modifiers: mouse.modifiers,
                };
                self.handle_mouse(crossterm_mouse);
                Ok(CoordinatorAction::Redraw)
            }
            _ => Ok(CoordinatorAction::Redraw),
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

fn main() -> std::io::Result<()> {
    let app = TermTuiDemo::new();
    run_with_diagnostics(app, RunnerConfig::default())
}
