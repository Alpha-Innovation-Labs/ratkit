//! Handle keyboard events for the markdown widget.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::widgets::markdown_widget::foundation::events::MarkdownEvent;
use crate::widgets::markdown_widget::widget::enums::MarkdownWidgetMode;
use crate::widgets::markdown_widget::widget::MarkdownWidget;

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
    /// - `/`: Enter filter mode
    /// - `Esc`: Exit selection mode or filter mode
    /// - `y`: Copy selection to clipboard (when selection active)
    /// - `Ctrl+Shift+C`: Copy selection to clipboard
    ///
    /// Returns a `MarkdownEvent` indicating what action was taken.
    pub fn handle_key_event(&mut self, key: KeyEvent) -> MarkdownEvent {
        // Handle filter mode first
        if self.filter_mode {
            return self.handle_filter_key(key);
        }

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
            KeyCode::Char('/') => {
                self.filter_mode = true;
                self.filter = Some(String::new());
                self.mode = MarkdownWidgetMode::Filter;
                MarkdownEvent::FilterModeChanged {
                    active: true,
                    filter: String::new(),
                }
            }
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

    /// Handle a keyboard event in filter mode.
    ///
    /// This method handles:
    /// - `Esc`: Exit filter mode and keep filter text
    /// - `Enter`: Exit filter mode, clear filter, and jump to line
    /// - `Backspace`: Remove last character from filter
    /// - `Char(c)`: Add character to filter
    /// - `j` / `Down` / `Ctrl+n`: Move to next filtered line
    /// - `k` / `Up` / `Ctrl+p`: Move to previous filtered line
    fn handle_filter_key(&mut self, key: KeyEvent) -> MarkdownEvent {
        match key.code {
            KeyCode::Esc => {
                let focused_line = self.scroll.current_line;
                // Clear filter and exit filter mode
                self.filter_mode = false;
                self.filter = None;
                self.mode = MarkdownWidgetMode::Normal;
                // Sync to ScrollState
                self.scroll.filter_mode = false;
                self.scroll.filter = None;
                // Clear render cache so all content is shown again
                self.cache.render = None;
                MarkdownEvent::FilterModeExited { line: focused_line }
            }
            KeyCode::Enter => {
                let focused_line = self.scroll.current_line;
                // Clear filter and exit filter mode
                self.filter_mode = false;
                self.filter = None;
                self.mode = MarkdownWidgetMode::Normal;
                // Sync to ScrollState
                self.scroll.filter_mode = false;
                self.scroll.filter = None;
                // Clear render cache so all content is shown again
                self.cache.render = None;
                MarkdownEvent::FilterModeExited { line: focused_line }
            }
            KeyCode::Backspace => {
                if let Some(filter) = &mut self.filter {
                    filter.pop();
                    return MarkdownEvent::FilterModeChanged {
                        active: true,
                        filter: filter.clone(),
                    };
                }
                MarkdownEvent::None
            }
            KeyCode::Char('j') | KeyCode::Down => {
                let filter = self.filter.clone().unwrap_or_default();
                let next_line = self.find_next_filter_match(filter);
                if let Some(line) = next_line {
                    self.scroll.current_line = line;
                }
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                let filter = self.filter.clone().unwrap_or_default();
                let prev_line = self.find_prev_filter_match(filter);
                if let Some(line) = prev_line {
                    self.scroll.current_line = line;
                }
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let filter = self.filter.clone().unwrap_or_default();
                let next_line = self.find_next_filter_match(filter);
                if let Some(line) = next_line {
                    self.scroll.current_line = line;
                }
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let filter = self.filter.clone().unwrap_or_default();
                let prev_line = self.find_prev_filter_match(filter);
                if let Some(line) = prev_line {
                    self.scroll.current_line = line;
                }
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            KeyCode::Char(c) => {
                if let Some(filter) = &mut self.filter {
                    filter.push(c);
                    return MarkdownEvent::FilterModeChanged {
                        active: true,
                        filter: filter.clone(),
                    };
                }
                MarkdownEvent::None
            }
            _ => MarkdownEvent::None,
        }
    }

    /// Find the next line that matches the filter text (by original line number).
    fn find_next_filter_match(&self, filter: String) -> Option<usize> {
        if filter.is_empty() {
            return None;
        }
        let filter_lower = filter.to_lowercase();
        let elements =
            crate::widgets::markdown_widget::foundation::parser::render_markdown_to_elements(
                self.content,
                true,
            );
        let current = self.scroll.current_line;

        for (idx, element) in elements.iter().enumerate() {
            let line_num = idx + 1;
            if line_num <= current {
                continue;
            }
            if !crate::widgets::markdown_widget::extensions::selection::should_render_line(
                element,
                idx,
                self.collapse,
            ) {
                continue;
            }
            let text = super::super::helpers::element_to_plain_text_for_filter(&element.kind)
                .to_lowercase();
            if text.contains(&filter_lower) {
                return Some(line_num);
            }
        }
        None
    }

    /// Find the previous line that matches the filter text (by original line number).
    fn find_prev_filter_match(&self, filter: String) -> Option<usize> {
        if filter.is_empty() {
            return None;
        }
        let filter_lower = filter.to_lowercase();
        let elements =
            crate::widgets::markdown_widget::foundation::parser::render_markdown_to_elements(
                self.content,
                true,
            );
        let current = self.scroll.current_line;

        for (idx, element) in elements.iter().enumerate().rev() {
            let line_num = idx + 1;
            if line_num >= current {
                continue;
            }
            if !crate::widgets::markdown_widget::extensions::selection::should_render_line(
                element,
                idx,
                self.collapse,
            ) {
                continue;
            }
            let text = super::super::helpers::element_to_plain_text_for_filter(&element.kind)
                .to_lowercase();
            if text.contains(&filter_lower) {
                return Some(line_num);
            }
        }
        None
    }
}
