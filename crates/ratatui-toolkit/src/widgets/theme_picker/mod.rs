//! Theme picker widget for selecting application themes.
//!
//! A centered modal dialog with search filtering and live preview.
//!
//! # Features
//!
//! - Search/filter themes by name
//! - Keyboard navigation (j/k/Up/Down)
//! - Live theme preview as you navigate
//! - Enter to confirm, Esc to cancel
//! - Builder pattern for configuration
//!
//! # Example
//!
//! ```rust,no_run
//! use ratatui_toolkit::{ThemePicker, ThemePickerEvent};
//!
//! let mut picker = ThemePicker::new();
//! picker.show();
//!
//! // Handle events
//! while let Some(event) = picker.handle_key(key) {
//!     match event {
//!         ThemePickerEvent::Selected(theme_name) => {
//!             println!("Selected: {}", theme_name);
//!         }
//!         ThemePickerEvent::Cancelled => {
//!             println!("Cancelled");
//!         }
//!         ThemePickerEvent::PreviewChanged(theme_name) => {
//!             println!("Previewing: {}", theme_name);
//!         }
//!     }
//! }
//! ```

mod state;

pub use state::{ThemePickerState, ThemePickerStateSnapshot};

use crate::services::theme::loader::{load_builtin_theme, BUILTIN_THEMES};
use crate::services::theme::AppTheme;
use crate::ThemeVariant;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use std::fmt::Write as FmtWrite;

const MAX_VISIBLE_THEMES: usize = 20;
const POPUP_WIDTH: u16 = 44;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemePickerEvent {
    Selected(String),
    Cancelled,
    PreviewChanged(String),
}

fn format_display_name(name: &str) -> String {
    name.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn filter_themes(filter: &str) -> Vec<(usize, &'static str)> {
    BUILTIN_THEMES
        .iter()
        .enumerate()
        .filter(|(_, name)| {
            if filter.is_empty() {
                true
            } else {
                let filter_lower = filter.to_lowercase();
                name.to_lowercase().contains(&filter_lower)
                    || format_display_name(name)
                        .to_lowercase()
                        .contains(&filter_lower)
            }
        })
        .map(|(i, name)| (i, *name))
        .collect()
}

fn calculate_scroll_offset(
    selected_index: usize,
    visible_count: usize,
    total_count: usize,
) -> usize {
    if total_count <= visible_count {
        return 0;
    }
    let half_visible = visible_count / 2;
    if selected_index <= half_visible {
        0
    } else if selected_index >= total_count - half_visible {
        total_count - visible_count
    } else {
        selected_index - half_visible
    }
}

pub struct ThemePicker {
    state: ThemePickerState,
    width: u16,
    title: String,
    show_footer: bool,
    on_preview: Option<Box<dyn Fn(&AppTheme) + Send>>,
    on_select: Option<Box<dyn Fn(&str) + Send>>,
    on_cancel: Option<Box<dyn Fn() + Send>>,
}

impl Default for ThemePicker {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemePicker {
    pub fn new() -> Self {
        Self {
            state: ThemePickerState::new(),
            width: POPUP_WIDTH,
            title: "Select Theme".to_string(),
            show_footer: true,
            on_preview: None,
            on_select: None,
            on_cancel: None,
        }
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn show_footer(mut self, show: bool) -> Self {
        self.show_footer = show;
        self
    }

    pub fn on_preview(mut self, callback: impl Fn(&AppTheme) + Send + 'static) -> Self {
        self.on_preview = Some(Box::new(callback));
        self
    }

    pub fn on_select(mut self, callback: impl Fn(&str) + Send + 'static) -> Self {
        self.on_select = Some(Box::new(callback));
        self
    }

    pub fn on_cancel(mut self, callback: impl Fn() + Send + 'static) -> Self {
        self.on_cancel = Some(Box::new(callback));
        self
    }

    pub fn show(&mut self) {
        self.state.show();
    }

    pub fn hide(&mut self) {
        self.state.hide();
    }

    pub fn is_visible(&self) -> bool {
        self.state.is_visible()
    }

    pub fn is_shown(&self) -> bool {
        self.state.is_visible()
    }

    pub fn handle_key(&mut self, key: &crossterm::event::KeyCode) -> Option<ThemePickerEvent> {
        if !self.state.is_visible() {
            return None;
        }

        use crossterm::event::KeyCode;

        let filtered = filter_themes(&self.state.filter());

        match key {
            KeyCode::Esc => {
                self.state.hide();
                self.state.clear_filter();
                self.state.set_index(0);
                if let Some(ref cb) = self.on_cancel {
                    cb();
                }
                return Some(ThemePickerEvent::Cancelled);
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if !filtered.is_empty() {
                    let new_index = (self.state.index() + 1) % filtered.len();
                    self.state.set_index(new_index);
                    if let Some((original_idx, _)) = filtered.get(new_index) {
                        if let Some(theme_name) = BUILTIN_THEMES.get(*original_idx) {
                            if let Ok(theme) = load_builtin_theme(theme_name, ThemeVariant::Dark) {
                                self.state.set_current_preview(theme.clone());
                                if let Some(ref cb) = self.on_preview {
                                    cb(&theme);
                                }
                                return Some(ThemePickerEvent::PreviewChanged(
                                    theme_name.to_string(),
                                ));
                            }
                        }
                    }
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if !filtered.is_empty() {
                    let new_index = if self.state.index() == 0 {
                        filtered.len() - 1
                    } else {
                        self.state.index() - 1
                    };
                    self.state.set_index(new_index);
                    if let Some((original_idx, _)) = filtered.get(new_index) {
                        if let Some(theme_name) = BUILTIN_THEMES.get(*original_idx) {
                            if let Ok(theme) = load_builtin_theme(theme_name, ThemeVariant::Dark) {
                                self.state.set_current_preview(theme.clone());
                                if let Some(ref cb) = self.on_preview {
                                    cb(&theme);
                                }
                                return Some(ThemePickerEvent::PreviewChanged(
                                    theme_name.to_string(),
                                ));
                            }
                        }
                    }
                }
            }
            KeyCode::Enter => {
                if let Some((_, theme_name)) = filtered.get(self.state.index()) {
                    self.state.hide();
                    self.state.clear_filter();
                    self.state.set_index(0);
                    if let Some(ref cb) = self.on_select {
                        cb(theme_name);
                    }
                    return Some(ThemePickerEvent::Selected(theme_name.to_string()));
                }
            }
            KeyCode::Backspace => {
                self.state.pop_filter();
                self.state.set_index(0);
                let new_filtered = filter_themes(&self.state.filter());
                if let Some((original_idx, _)) = new_filtered.first() {
                    if let Some(theme_name) = BUILTIN_THEMES.get(*original_idx) {
                        if let Ok(theme) = load_builtin_theme(theme_name, ThemeVariant::Dark) {
                            self.state.set_current_preview(theme.clone());
                            if let Some(ref cb) = self.on_preview {
                                cb(&theme);
                            }
                        }
                    }
                }
            }
            KeyCode::Char(c) => {
                if c.is_alphanumeric() || *c == ' ' || *c == '-' {
                    self.state.push_filter(*c);
                    self.state.set_index(0);
                    let new_filtered = filter_themes(&self.state.filter());
                    if let Some((original_idx, _)) = new_filtered.first() {
                        if let Some(theme_name) = BUILTIN_THEMES.get(*original_idx) {
                            if let Ok(theme) = load_builtin_theme(theme_name, ThemeVariant::Dark) {
                                self.state.set_current_preview(theme.clone());
                                if let Some(ref cb) = self.on_preview {
                                    cb(&theme);
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        None
    }

    pub fn handle_mouse(&mut self, _mouse: crossterm::event::MouseEvent) {
        // Theme picker doesn't currently support mouse interaction
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        if !self.state.is_visible() {
            return;
        }

        let current_theme = self.state.current_preview();
        let filtered = filter_themes(&self.state.filter());
        let visible_count = filtered.len().min(MAX_VISIBLE_THEMES);
        let popup_height = (visible_count + if self.show_footer { 7 } else { 5 }) as u16;

        let popup_area = Rect {
            x: area.width.saturating_sub(self.width) / 2,
            y: area.height.saturating_sub(popup_height) / 2,
            width: self.width.min(area.width),
            height: popup_height.min(area.height),
        };

        frame.render_widget(Clear, popup_area);

        let scroll_offset =
            calculate_scroll_offset(self.state.index(), visible_count, filtered.len());

        let mut items: Vec<Line> = Vec::new();

        let search_style = Style::default().fg(current_theme.text);
        let cursor = if self.state.filter().is_empty() {
            "_"
        } else {
            ""
        };
        let mut filter_str = String::new();
        let _ = write!(filter_str, "{}{}", self.state.filter(), cursor);
        items.push(Line::from(vec![
            Span::styled(" / ", Style::default().fg(current_theme.text_muted)),
            Span::styled(filter_str, search_style.add_modifier(Modifier::BOLD)),
        ]));

        let separator = "─".repeat(self.width.saturating_sub(2) as usize);
        items.push(Line::from(Span::styled(
            separator,
            Style::default().fg(current_theme.border),
        )));

        if filtered.is_empty() {
            items.push(Line::from(Span::styled(
                "   No matching themes",
                Style::default().fg(current_theme.text_muted),
            )));
        } else {
            for (filtered_idx, (original_idx, theme_name)) in filtered
                .iter()
                .enumerate()
                .skip(scroll_offset)
                .take(visible_count)
            {
                let display_name = format_display_name(theme_name);
                let is_selected = filtered_idx == self.state.index();

                let prefix = if is_selected { " > " } else { "   " };
                let suffix = if *original_idx == self.state.saved_index() {
                    " *"
                } else {
                    ""
                };

                let style = if is_selected {
                    Style::default()
                        .fg(current_theme.primary)
                        .bg(current_theme.background_panel)
                        .add_modifier(Modifier::BOLD)
                } else if *original_idx == self.state.saved_index() {
                    Style::default()
                        .fg(current_theme.success)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(current_theme.text)
                };

                items.push(Line::from(Span::styled(
                    format!("{}{}{}", prefix, display_name, suffix),
                    style,
                )));
            }
        }

        if self.show_footer {
            items.push(Line::from(""));
            items.push(Line::from(vec![
                Span::styled(" [", Style::default().fg(current_theme.text_muted)),
                Span::styled("j/k", Style::default().fg(current_theme.accent)),
                Span::styled("] scroll  [", Style::default().fg(current_theme.text_muted)),
                Span::styled("Enter", Style::default().fg(current_theme.success)),
                Span::styled("] select  [", Style::default().fg(current_theme.text_muted)),
                Span::styled("Esc", Style::default().fg(current_theme.error)),
                Span::styled("] cancel", Style::default().fg(current_theme.text_muted)),
            ]));
        }

        let title = if !self.state.filter().is_empty() {
            format!(
                " {} ({}/{}) ",
                self.title,
                filtered.len(),
                BUILTIN_THEMES.len()
            )
        } else if filtered.len() > visible_count {
            format!(
                " {} ({}/{}) ",
                self.title,
                self.state.index() + 1,
                filtered.len()
            )
        } else {
            format!(" {} ", self.title)
        };

        let popup = Paragraph::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(current_theme.border_active))
                .style(Style::default().bg(current_theme.background_menu))
                .title(Span::styled(
                    title,
                    Style::default()
                        .fg(current_theme.primary)
                        .add_modifier(Modifier::BOLD),
                )),
        );

        frame.render_widget(popup, popup_area);
    }

    pub fn set_saved_index(&mut self, index: usize) {
        self.state.set_saved_index(index);
    }

    pub fn saved_index(&self) -> usize {
        self.state.saved_index()
    }

    pub fn set_current_theme(&mut self, theme: &AppTheme) {
        self.state.set_current_preview(theme.clone());
    }

    pub fn state(&self) -> &ThemePickerState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut ThemePickerState {
        &mut self.state
    }
}
