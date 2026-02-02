//! Code diff tab handler.

use super::TabHandler;
use crate::app::App;
use crossterm::event::{KeyEvent, MouseEvent};

pub struct CodeDiffHandler;

impl TabHandler for CodeDiffHandler {
    fn handle_key(&mut self, app: &mut App, key: KeyEvent) {
        app.code_diff.handle_key(key.code);
    }

    fn handle_mouse(&mut self, app: &mut App, mouse: MouseEvent) {
        if let Some(area) = app.code_diff.area {
            app.code_diff.handle_mouse(mouse, area);
        }
    }

    fn needs_fast_refresh(&self, app: &App) -> bool {
        app.code_diff.is_sidebar_dragging()
    }
}
