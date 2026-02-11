use std::fmt::Write as FmtWrite;

use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::widgets::theme_picker::builtin_themes::BUILTIN_THEMES;
use crate::widgets::theme_picker::state::ThemePickerState;
use crate::widgets::theme_picker::theme_colors::ThemeColors;

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

fn load_theme_preview(theme_name: &str) -> ThemeColors {
    match theme_name {
        "ayu" => ThemeColors {
            primary: Color::Rgb(57, 194, 91),
            secondary: Color::Rgb(68, 189, 50),
            accent: Color::Rgb(66, 206, 244),
            background: Color::Rgb(9, 15, 18),
            background_menu: Color::Rgb(30, 34, 40),
            background_panel: Color::Rgb(39, 43, 52),
            text: Color::Rgb(198, 202, 210),
            text_muted: Color::Rgb(92, 99, 112),
            border: Color::Rgb(39, 43, 52),
            border_active: Color::Rgb(57, 194, 91),
            success: Color::Rgb(68, 189, 50),
            warning: Color::Rgb(233, 185, 74),
            error: Color::Rgb(232, 93, 117),
            info: Color::Rgb(66, 206, 244),
        },
        "dracula" => ThemeColors {
            primary: Color::Rgb(139, 92, 246),
            secondary: Color::Rgb(189, 147, 249),
            accent: Color::Rgb(255, 121, 198),
            background: Color::Rgb(40, 42, 54),
            background_menu: Color::Rgb(68, 71, 90),
            background_panel: Color::Rgb(68, 71, 90),
            text: Color::Rgb(248, 248, 242),
            text_muted: Color::Rgb(68, 71, 90),
            border: Color::Rgb(68, 71, 90),
            border_active: Color::Rgb(139, 92, 246),
            success: Color::Rgb(80, 250, 123),
            warning: Color::Rgb(241, 250, 140),
            error: Color::Rgb(255, 85, 85),
            info: Color::Rgb(139, 92, 246),
        },
        "nord" => ThemeColors {
            primary: Color::Rgb(136, 192, 208),
            secondary: Color::Rgb(129, 161, 193),
            accent: Color::Rgb(191, 97, 106),
            background: Color::Rgb(46, 52, 64),
            background_menu: Color::Rgb(59, 66, 82),
            background_panel: Color::Rgb(67, 76, 94),
            text: Color::Rgb(216, 222, 233),
            text_muted: Color::Rgb(136, 192, 208),
            border: Color::Rgb(67, 76, 94),
            border_active: Color::Rgb(136, 192, 208),
            success: Color::Rgb(163, 190, 140),
            warning: Color::Rgb(235, 203, 139),
            error: Color::Rgb(191, 97, 106),
            info: Color::Rgb(136, 192, 208),
        },
        "gruvbox" => ThemeColors {
            primary: Color::Rgb(184, 187, 38),
            secondary: Color::Rgb(250, 189, 47),
            accent: Color::Rgb(214, 93, 14),
            background: Color::Rgb(40, 30, 28),
            background_menu: Color::Rgb(60, 50, 48),
            background_panel: Color::Rgb(80, 70, 68),
            text: Color::Rgb(235, 219, 178),
            text_muted: Color::Rgb(168, 153, 132),
            border: Color::Rgb(80, 70, 68),
            border_active: Color::Rgb(184, 187, 38),
            success: Color::Rgb(152, 195, 121),
            warning: Color::Rgb(250, 189, 47),
            error: Color::Rgb(204, 36, 29),
            info: Color::Rgb(69, 133, 136),
        },
        _ => ThemeColors::default(),
    }
}

use ratatui::style::Color;

pub struct ThemePicker {
    state: ThemePickerState,
    width: u16,
    title: String,
    show_footer: bool,
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
                Some(ThemePickerEvent::Cancelled)
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if !filtered.is_empty() {
                    let new_index = (self.state.index() + 1) % filtered.len();
                    self.state.set_index(new_index);
                    if let Some((_, theme_name)) = filtered.get(new_index) {
                        let theme = load_theme_preview(theme_name);
                        self.state.set_current_preview(theme);
                        return Some(ThemePickerEvent::PreviewChanged(theme_name.to_string()));
                    }
                }
                None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if !filtered.is_empty() {
                    let new_index = if self.state.index() == 0 {
                        filtered.len() - 1
                    } else {
                        self.state.index() - 1
                    };
                    self.state.set_index(new_index);
                    if let Some((_, theme_name)) = filtered.get(new_index) {
                        let theme = load_theme_preview(theme_name);
                        self.state.set_current_preview(theme);
                        return Some(ThemePickerEvent::PreviewChanged(theme_name.to_string()));
                    }
                }
                None
            }
            KeyCode::Enter => {
                if let Some((_, theme_name)) = filtered.get(self.state.index()) {
                    self.state.hide();
                    self.state.clear_filter();
                    self.state.set_index(0);
                    return Some(ThemePickerEvent::Selected(theme_name.to_string()));
                }
                None
            }
            KeyCode::Backspace => {
                self.state.pop_filter();
                self.state.set_index(0);
                None
            }
            KeyCode::Char(c) => {
                if c.is_alphanumeric() || *c == ' ' || *c == '-' {
                    self.state.push_filter(*c);
                }
                None
            }
            _ => None,
        }
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
                        .bg(current_theme.background)
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

    pub fn set_current_theme(&mut self, theme: &ThemeColors) {
        self.state.set_current_preview(theme.clone());
    }

    pub fn state(&self) -> &ThemePickerState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut ThemePickerState {
        &mut self.state
    }
}
