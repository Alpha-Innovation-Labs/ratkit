use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogType {
    Info,
    Success,
    Warning,
    Error,
    Confirm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogActionsLayout {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogWrap {
    WordTrim,
    WordNoTrim,
    NoWrap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogModalMode {
    Blocking,
    Passthrough,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogShadow {
    None,
    Soft,
    Medium,
    Strong,
    Custom {
        offset_x: u16,
        offset_y: u16,
        style: Style,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DialogPadding {
    pub horizontal: u16,
    pub vertical: u16,
}

impl Default for DialogPadding {
    fn default() -> Self {
        Self {
            horizontal: 1,
            vertical: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogFooter<'a> {
    Hidden,
    Text(&'a str),
}

#[derive(Debug, Clone)]
pub struct DialogKeymap {
    pub next: Vec<KeyCode>,
    pub previous: Vec<KeyCode>,
    pub confirm: Vec<KeyCode>,
    pub cancel: Vec<KeyCode>,
    pub close: Vec<KeyCode>,
}

impl Default for DialogKeymap {
    fn default() -> Self {
        Self {
            next: vec![
                KeyCode::Tab,
                KeyCode::Right,
                KeyCode::Down,
                KeyCode::Char('j'),
                KeyCode::Char('l'),
            ],
            previous: vec![
                KeyCode::BackTab,
                KeyCode::Left,
                KeyCode::Up,
                KeyCode::Char('k'),
                KeyCode::Char('h'),
            ],
            confirm: vec![KeyCode::Enter, KeyCode::Char(' ')],
            cancel: vec![KeyCode::Esc],
            close: vec![KeyCode::Char('q')],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogAction {
    Select(usize),
    Confirm(usize),
    Cancel,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DialogEventResult {
    pub consumed: bool,
    pub action: Option<DialogAction>,
}

impl DialogEventResult {
    pub fn ignored() -> Self {
        Self {
            consumed: false,
            action: None,
        }
    }

    pub fn consumed(action: Option<DialogAction>) -> Self {
        Self {
            consumed: true,
            action,
        }
    }
}

pub trait DialogBodyRenderer {
    fn render_body(&mut self, area: Rect, buf: &mut Buffer);
}

pub struct Dialog<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub dialog_type: DialogType,
    pub buttons: Vec<&'a str>,
    pub selected_button: usize,
    pub width_percent: f32,
    pub height_percent: f32,
    pub footer: DialogFooter<'a>,
    pub footer_style: Style,
    pub footer_alignment: Alignment,
    pub title_inside: bool,
    pub backdrop_style: Option<Style>,
    pub shadow: DialogShadow,
    pub modal_mode: DialogModalMode,
    pub border_color: Option<Color>,
    pub style: Style,
    pub button_selected_style: Style,
    pub button_style: Style,
    pub actions_layout: DialogActionsLayout,
    pub actions_alignment: Alignment,
    pub message_alignment: Alignment,
    pub content_padding: DialogPadding,
    pub wrap: DialogWrap,
    pub body_renderer: Option<Box<dyn DialogBodyRenderer + 'a>>,
    pub keymap: DialogKeymap,
    pub wrap_button_navigation: bool,
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
