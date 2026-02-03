//! Terminal tab handler.

use super::TabHandler;
use crate::app::App;
use crossterm::event::{KeyEvent, MouseEvent};

pub struct TerminalHandler;

impl TabHandler for TerminalHandler {
    fn handle_key(&mut self, app: &mut App, key: KeyEvent) {
        if let Some(ref mut term) = app.terminal {
            term.handle_key(key);
        }
    }

    fn handle_mouse(&mut self, app: &mut App, mouse: MouseEvent) {
        if let Some(content_area) = app.terminal_content_area {
            app.terminal_split.handle_mouse(mouse, content_area);
        }
    }

    fn needs_fast_refresh(&self, app: &App) -> bool {
        app.terminal_split.needs_fast_refresh()
    }
}
