use std::io;
use std::sync::{Arc, Mutex};

use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};
use ratatui::{buffer::Buffer, Frame};
use ratkit::primitives::dialog::{
    Dialog, DialogAction, DialogActionsLayout, DialogBodyRenderer, DialogModalMode, DialogShadow,
    DialogWidget, DialogWrap,
};
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

struct MenuState {
    items: Vec<&'static str>,
    selected: usize,
}

impl MenuState {
    fn move_up(&mut self) {
        if self.items.is_empty() {
            self.selected = 0;
            return;
        }
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.items.is_empty() {
            self.selected = 0;
            return;
        }
        if self.selected + 1 < self.items.len() {
            self.selected += 1;
        }
    }
}

struct SelectMenuBody {
    state: Arc<Mutex<MenuState>>,
}

impl SelectMenuBody {
    fn new(state: Arc<Mutex<MenuState>>) -> Self {
        Self { state }
    }
}

impl DialogBodyRenderer for SelectMenuBody {
    fn render_body(&mut self, area: Rect, buf: &mut Buffer) {
        if area.width < 4 || area.height < 4 {
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5), Constraint::Min(4)])
            .split(area);

        let intro = Paragraph::new(
            "You are about to permanently remove README.md from disk. This action cannot be undone.\nChoose a reason below, then confirm with Yes/No.",
        )
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });
        intro.render(chunks[0], buf);

        let menu_block = Block::default()
            .title(" Reason ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let menu_inner = menu_block.inner(chunks[1]);
        menu_block.render(chunks[1], buf);

        let state = self.state.lock().expect("menu state poisoned");
        for (idx, item) in state.items.iter().enumerate() {
            let y = menu_inner.y.saturating_add(idx as u16);
            if y >= menu_inner.y + menu_inner.height {
                break;
            }

            let is_selected = idx == state.selected;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };

            let prefix = if is_selected { ">" } else { " " };
            let line = Line::from(vec![Span::styled(format!("{} {}", prefix, item), style)]);
            buf.set_line(menu_inner.x, y, &line, menu_inner.width);
        }
    }
}

struct DialogDemo {
    dialog: Dialog<'static>,
    menu_state: Arc<Mutex<MenuState>>,
}

impl DialogDemo {
    fn new() -> Self {
        let menu_state = Arc::new(Mutex::new(MenuState {
            items: vec![
                "Outdated documentation",
                "File moved to docs/",
                "Replaced by generated docs",
                "Accidental duplicate file",
            ],
            selected: 0,
        }));

        let dialog = Dialog::confirm("Delete file", "")
            .buttons(vec!["Yes", "No"])
            .default_selection(1)
            .actions_layout(DialogActionsLayout::Horizontal)
            .message_alignment(ratatui::layout::Alignment::Left)
            .content_padding(2, 1)
            .wrap_mode(DialogWrap::WordTrim)
            .body_renderer(Box::new(SelectMenuBody::new(menu_state.clone())))
            .next_keys(vec![KeyCode::Tab, KeyCode::Right, KeyCode::Char('l')])
            .previous_keys(vec![KeyCode::BackTab, KeyCode::Left, KeyCode::Char('h')])
            .hide_footer()
            .shadow(DialogShadow::Medium)
            .overlay(true)
            .modal_mode(DialogModalMode::Blocking)
            .title_inside(true)
            .wrap_button_navigation(true);

        Self { dialog, menu_state }
    }
}

impl CoordinatorApp for DialogDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(keyboard) => {
                if keyboard.key_code == KeyCode::Char('q') {
                    return Ok(CoordinatorAction::Quit);
                }

                match keyboard.key_code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        if let Ok(mut state) = self.menu_state.lock() {
                            state.move_up();
                        }
                        return Ok(CoordinatorAction::Redraw);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if let Ok(mut state) = self.menu_state.lock() {
                            state.move_down();
                        }
                        return Ok(CoordinatorAction::Redraw);
                    }
                    _ => {}
                }

                let result = self.dialog.handle_key_event(keyboard.key_code);
                if let Some(action) = result.action {
                    match action {
                        DialogAction::Confirm(index) => {
                            if self.dialog.buttons.get(index) == Some(&"Yes") {
                                return Ok(CoordinatorAction::Quit);
                            }
                        }
                        DialogAction::Cancel | DialogAction::Close => {
                            return Ok(CoordinatorAction::Quit);
                        }
                        DialogAction::Select(_) => {}
                    }
                }

                Ok(CoordinatorAction::Redraw)
            }
            _ => Ok(CoordinatorAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        frame.render_widget(DialogWidget::new(&mut self.dialog), area);
    }
}

fn main() -> io::Result<()> {
    let app = DialogDemo::new();
    run_with_diagnostics(app, RunnerConfig::default())
}
