//! AI chat tab handler.

use super::TabHandler;
use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use ratatui_toolkit::widgets::ai_chat::AIChatEvent;

pub struct AiChatHandler;

impl TabHandler for AiChatHandler {
    fn handle_key(&mut self, app: &mut App, key: KeyEvent) {
        let event = app.ai_chat.handle_key(key.code);
        match event {
            AIChatEvent::MessageSubmitted(_) => {
                app.toast_manager.info("Generating response...");
            }
            AIChatEvent::FileAttached(file) => {
                app.toast_manager.info(format!("Attached file: {}", file));
            }
            AIChatEvent::Command(cmd) => {
                app.toast_manager.info(format!("Command: {}", cmd));
            }
            AIChatEvent::None => {}
        }
    }

    fn handle_mouse(&mut self, _app: &mut App, _mouse: MouseEvent) {}

    fn needs_fast_refresh(&self, _app: &App) -> bool {
        false
    }
}
