//! Tab handlers for demo tabs.
//!
//! This module provides a TabHandler trait and implementations for each demo tab,
//! extracting tab-specific keyboard and mouse handling from main.rs.

pub mod markdown_handler;
pub mod code_diff_handler;
pub mod tree_handler;
pub mod terminal_handler;
pub mod split_grid_handler;
pub mod ai_chat_handler;
pub mod primitives_handler;
pub mod theme_picker_handler;
pub mod mouse_click_handler;
pub mod main_loop_handler;

use crate::app::App;
use crossterm::event::{KeyEvent, MouseEvent};

pub trait TabHandler {
    fn handle_key(&mut self, app: &mut App, key: KeyEvent);
    fn handle_mouse(&mut self, app: &mut App, mouse: MouseEvent);
    fn needs_fast_refresh(&self, app: &App) -> bool;
}
