use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogType {
    Info,
    Success,
    Warning,
    Error,
    Confirm,
}

pub struct Dialog<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub dialog_type: DialogType,
    pub buttons: Vec<&'a str>,
    pub selected_button: usize,
    pub width_percent: f32,
    pub height_percent: f32,
    pub footer: Option<&'a str>,
    pub footer_style: Style,
    pub title_inside: bool,
    pub overlay: bool,
    pub overlay_style: Style,
    pub border_color: Option<Color>,
    pub style: Style,
    pub button_selected_style: Style,
    pub button_style: Style,
    pub button_areas: Vec<Rect>,
    pub theme_info_color: Option<Color>,
    pub theme_success_color: Option<Color>,
    pub theme_warning_color: Option<Color>,
    pub theme_error_color: Option<Color>,
    pub theme_confirm_color: Option<Color>,
}

pub struct DialogState {
    pub selected_button: usize,
}

impl DialogState {
    pub fn new() -> Self {
        Self { selected_button: 0 }
    }
}

impl Default for DialogState {
    fn default() -> Self {
        Self::new()
    }
}
