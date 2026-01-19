//! Handle keyboard events for the markdown widget.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::markdown_widget::foundation::events::MarkdownEvent;
use crate::markdown_widget::widget::enums::MarkdownWidgetMode;
use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Handle a keyboard event for navigation and actions.
    ///
    /// This method handles:
    /// - `j` / `Down`: Move focused line down (scrolls when near edge)
    /// - `k` / `Up`: Move focused line up (scrolls when near edge)
    /// - `PageDown`: Scroll down by viewport height
    /// - `PageUp`: Scroll up by viewport height
    /// - `Home` / `gg`: Go to top
    /// - `End` / `G`: Go to bottom
    /// - `Esc`: Exit selection mode
    /// - `y`: Copy selection to clipboard (when selection active)
    /// - `Ctrl+Shift+C`: Copy selection to clipboard
    ///
    /// Returns a `MarkdownEvent` indicating what action was taken.
    pub fn handle_key_event(&mut self, key: KeyEvent) -> MarkdownEvent {
        // Handle selection-related keys first
        if key.code == KeyCode::Esc && self.selection.is_active() {
            self.selection.exit();
            self.mode = MarkdownWidgetMode::Normal;
            self.vim.clear_pending_g();
            return MarkdownEvent::SelectionEnded;
        }

        // Copy selection with 'y' (vim-style)
        if key.code == KeyCode::Char('y') && self.selection.has_selection() {
            if let Some(text) = self.selection.get_selected_text() {
                if !text.is_empty() {
                    if let Ok(mut clipboard) = arboard::Clipboard::new() {
                        if clipboard.set_text(&text).is_ok() {
                            self.selection.exit();
                            self.mode = MarkdownWidgetMode::Normal;
                            self.vim.clear_pending_g();
                            return MarkdownEvent::Copied { text };
                        }
                    }
                }
            }
        }

        // Copy selection with Ctrl+Shift+C
        if key.code == KeyCode::Char('C')
            && key.modifiers.contains(KeyModifiers::CONTROL)
            && key.modifiers.contains(KeyModifiers::SHIFT)
        {
            if let Some(text) = self.selection.get_selected_text() {
                if !text.is_empty() {
                    if let Ok(mut clipboard) = arboard::Clipboard::new() {
                        if clipboard.set_text(&text).is_ok() {
                            self.selection.exit();
                            self.mode = MarkdownWidgetMode::Normal;
                            self.vim.clear_pending_g();
                            return MarkdownEvent::Copied { text };
                        }
                    }
                }
            }
        }

        // Handle vim-style 'gg' for go to top
        if key.code == KeyCode::Char('g') {
            if self.vim.check_pending_gg() {
                // Second 'g' within timeout - go to top
                self.scroll.scroll_to_top();
                return MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                };
            }
            // First 'g' or timeout expired - set pending
            self.vim.set_pending_g();
            return MarkdownEvent::None;
        }

        // Any other key clears pending 'g'
        self.vim.clear_pending_g();

        // Handle navigation keys
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                // Move focused line down (scrolls when near edge)
                self.scroll.line_down();
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                // Move focused line up (scrolls when near edge)
                self.scroll.line_up();
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            KeyCode::PageDown => {
                let old_offset = self.scroll.scroll_offset;
                self.scroll.scroll_down(self.scroll.viewport_height);
                MarkdownEvent::Scrolled {
                    offset: self.scroll.scroll_offset,
                    direction: (self.scroll.scroll_offset.saturating_sub(old_offset) as i32),
                }
            }
            KeyCode::PageUp => {
                let old_offset = self.scroll.scroll_offset;
                self.scroll.scroll_up(self.scroll.viewport_height);
                MarkdownEvent::Scrolled {
                    offset: self.scroll.scroll_offset,
                    direction: -(old_offset.saturating_sub(self.scroll.scroll_offset) as i32),
                }
            }
            KeyCode::Home => {
                self.scroll.scroll_to_top();
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            KeyCode::End | KeyCode::Char('G') => {
                self.scroll.scroll_to_bottom();
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            _ => MarkdownEvent::None,
        }
    }
}
