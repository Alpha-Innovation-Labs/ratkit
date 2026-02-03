//! Split grid tab handler.

use super::TabHandler;
use crate::app::App;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::widgets::{Block, BorderType, Borders};

pub struct SplitGridHandler;

impl TabHandler for SplitGridHandler {
    fn handle_key(&mut self, _app: &mut App, _key: KeyEvent) {}

    fn handle_mouse(&mut self, app: &mut App, mouse: MouseEvent) {
        if let Some(content_area) = app.grid_content_area {
            let grid_area = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .inner(content_area);

            app.grid_split_widget.handle_mouse(mouse, grid_area);
        }
    }

    fn needs_fast_refresh(&self, _app: &App) -> bool {
        _app.grid_split_widget.needs_fast_refresh()
    }
}
