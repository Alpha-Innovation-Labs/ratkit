//! Markdown tab handler.

use super::TabHandler;
use crate::app::App;
use crossterm::event::KeyEvent;
use ratatui_toolkit::{MarkdownEvent, Toast, ToastLevel};

pub struct MarkdownHandler;

impl TabHandler for MarkdownHandler {
    fn handle_key(&mut self, app: &mut App, key: KeyEvent) {
        let event = app.markdown_widget.handle_key(key);

        match event {
            MarkdownEvent::Copied { text } => {
                let display = if text.len() > 30 {
                    format!("{}...", &text[..30])
                } else {
                    text
                };
                app.toast_manager.add(Toast::new(
                    &format!("Copied: {}", display),
                    ToastLevel::Success,
                    None,
                ));
            }
            MarkdownEvent::DoubleClick {
                line_number,
                line_kind,
                content,
            } => {
                let display_content = if content.len() > 40 {
                    format!("{}...", &content[..40])
                } else {
                    content
                };
                let msg = format!(
                    "Line {}: {} - \"{}\"",
                    line_number, line_kind, display_content
                );
                app.toast_manager
                    .add(Toast::new(&msg, ToastLevel::Info, None));
            }
            MarkdownEvent::HeadingToggled { text, .. } => {
                let display_text = if text.len() > 30 {
                    format!("{}...", &text[..30])
                } else {
                    text
                };
                app.toast_manager.add(Toast::new(
                    &format!("Toggled: {}", display_text),
                    ToastLevel::Info,
                    None,
                ));
            }
            _ => {}
        }
    }

    fn handle_mouse(&mut self, app: &mut App, mouse: crossterm::event::MouseEvent) {
        let inner_area = app.markdown_widget.inner_area.unwrap_or_default();

        let event = app.markdown_widget.handle_mouse(&mouse, inner_area);

        match event {
            MarkdownEvent::DoubleClick {
                line_number,
                line_kind,
                content,
            } => {
                let display_content = if content.len() > 40 {
                    format!("{}...", &content[..40])
                } else {
                    content
                };
                let msg = format!(
                    "Line {}: {} - \"{}\"",
                    line_number, line_kind, display_content
                );
                app.toast_manager
                    .add(Toast::new(&msg, ToastLevel::Info, None));
            }
            MarkdownEvent::Copied { text } => {
                let display = if text.len() > 30 {
                    format!("{}...", &text[..30])
                } else {
                    text
                };
                app.toast_manager.add(Toast::new(
                    &format!("Copied: {}", display),
                    ToastLevel::Success,
                    None,
                ));
            }
            MarkdownEvent::HeadingToggled { text, .. } => {
                let display_text = if text.len() > 30 {
                    format!("{}...", &text[..30])
                } else {
                    text
                };
                app.toast_manager.add(Toast::new(
                    &format!("Toggled: {}", display_text),
                    ToastLevel::Info,
                    None,
                ));
            }
            _ => {}
        }
    }

    fn needs_fast_refresh(&self, _app: &App) -> bool {
        false
    }
}
