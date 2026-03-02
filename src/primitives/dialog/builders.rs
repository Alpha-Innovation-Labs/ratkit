use crate::primitives::dialog::types::{
    Dialog, DialogActionsLayout, DialogBodyRenderer, DialogFooter, DialogKeymap, DialogModalMode,
    DialogPadding, DialogShadow, DialogType, DialogWrap,
};
use crossterm::event::KeyCode;
use ratatui::layout::Alignment;
use ratatui::style::{Color, Modifier, Style};

impl<'a> Dialog<'a> {
    pub fn new(title: &'a str, message: &'a str) -> Self {
        Self {
            title,
            message,
            dialog_type: DialogType::Info,
            buttons: Vec::new(),
            selected_button: 0,
            width_percent: 0.6,
            height_percent: 0.4,
            footer: DialogFooter::Hidden,
            footer_style: Style::default().fg(Color::DarkGray),
            footer_alignment: Alignment::Center,
            title_inside: false,
            backdrop_style: None,
            shadow: DialogShadow::None,
            modal_mode: DialogModalMode::Blocking,
            border_color: None,
            style: Style::default(),
            button_selected_style: Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            button_style: Style::default(),
            actions_layout: DialogActionsLayout::Horizontal,
            actions_alignment: Alignment::Center,
            message_alignment: Alignment::Center,
            content_padding: DialogPadding::default(),
            wrap: DialogWrap::WordTrim,
            body_renderer: None,
            keymap: DialogKeymap::default(),
            wrap_button_navigation: false,
            button_areas: Vec::new(),
            theme_info_color: None,
            theme_success_color: None,
            theme_warning_color: None,
            theme_error_color: None,
            theme_confirm_color: None,
        }
    }

    pub fn info(title: &'a str, message: &'a str) -> Self {
        Self::new(title, message).dialog_type(DialogType::Info)
    }

    pub fn warning(title: &'a str, message: &'a str) -> Self {
        Self::new(title, message).dialog_type(DialogType::Warning)
    }

    pub fn error(title: &'a str, message: &'a str) -> Self {
        Self::new(title, message).dialog_type(DialogType::Error)
    }

    pub fn success(title: &'a str, message: &'a str) -> Self {
        Self::new(title, message).dialog_type(DialogType::Success)
    }

    pub fn confirm(title: &'a str, message: &'a str) -> Self {
        Self::new(title, message)
            .dialog_type(DialogType::Confirm)
            .buttons(vec!["Yes", "No"])
    }

    pub fn dialog_type(mut self, dialog_type: DialogType) -> Self {
        self.dialog_type = dialog_type;
        self
    }

    pub fn buttons(mut self, buttons: Vec<&'a str>) -> Self {
        self.buttons = buttons;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border_color = Some(border_color);
        self
    }

    pub fn width_percent(mut self, width_percent: f32) -> Self {
        self.width_percent = width_percent;
        self
    }

    pub fn height_percent(mut self, height_percent: f32) -> Self {
        self.height_percent = height_percent;
        self
    }

    pub fn footer(mut self, footer: &'a str) -> Self {
        self.footer = DialogFooter::Text(footer);
        self
    }

    pub fn hide_footer(mut self) -> Self {
        self.footer = DialogFooter::Hidden;
        self
    }

    pub fn footer_alignment(mut self, alignment: Alignment) -> Self {
        self.footer_alignment = alignment;
        self
    }

    pub fn footer_style(mut self, footer_style: Style) -> Self {
        self.footer_style = footer_style;
        self
    }

    pub fn title_inside(mut self, title_inside: bool) -> Self {
        self.title_inside = title_inside;
        self
    }

    pub fn overlay(mut self, overlay: bool) -> Self {
        self.backdrop_style = if overlay {
            Some(Style::default().bg(Color::Rgb(0, 0, 0)))
        } else {
            None
        };
        self
    }

    pub fn overlay_style(mut self, overlay_style: Style) -> Self {
        self.backdrop_style = Some(overlay_style);
        self
    }

    pub fn backdrop_style(mut self, backdrop_style: Style) -> Self {
        self.backdrop_style = Some(backdrop_style);
        self
    }

    pub fn no_backdrop(mut self) -> Self {
        self.backdrop_style = None;
        self
    }

    pub fn shadow(mut self, shadow: DialogShadow) -> Self {
        self.shadow = shadow;
        self
    }

    pub fn modal_mode(mut self, modal_mode: DialogModalMode) -> Self {
        self.modal_mode = modal_mode;
        self
    }

    pub fn button_selected_style(mut self, button_selected_style: Style) -> Self {
        self.button_selected_style = button_selected_style;
        self
    }

    pub fn button_style(mut self, button_style: Style) -> Self {
        self.button_style = button_style;
        self
    }

    pub fn actions_layout(mut self, layout: DialogActionsLayout) -> Self {
        self.actions_layout = layout;
        self
    }

    pub fn actions_alignment(mut self, alignment: Alignment) -> Self {
        self.actions_alignment = alignment;
        self
    }

    pub fn message_alignment(mut self, alignment: Alignment) -> Self {
        self.message_alignment = alignment;
        self
    }

    pub fn content_padding(mut self, horizontal: u16, vertical: u16) -> Self {
        self.content_padding = DialogPadding {
            horizontal,
            vertical,
        };
        self
    }

    pub fn wrap_mode(mut self, wrap: DialogWrap) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn body_renderer(mut self, body_renderer: Box<dyn DialogBodyRenderer + 'a>) -> Self {
        self.body_renderer = Some(body_renderer);
        self
    }

    pub fn keymap(mut self, keymap: DialogKeymap) -> Self {
        self.keymap = keymap;
        self
    }

    pub fn wrap_button_navigation(mut self, wrap: bool) -> Self {
        self.wrap_button_navigation = wrap;
        self
    }

    pub fn default_selection(mut self, index: usize) -> Self {
        if !self.buttons.is_empty() {
            self.selected_button = index.min(self.buttons.len() - 1);
        }
        self
    }

    pub fn next_keys(mut self, keys: Vec<KeyCode>) -> Self {
        self.keymap.next = keys;
        self
    }

    pub fn previous_keys(mut self, keys: Vec<KeyCode>) -> Self {
        self.keymap.previous = keys;
        self
    }

    pub fn confirm_keys(mut self, keys: Vec<KeyCode>) -> Self {
        self.keymap.confirm = keys;
        self
    }

    pub fn cancel_keys(mut self, keys: Vec<KeyCode>) -> Self {
        self.keymap.cancel = keys;
        self
    }

    #[cfg(feature = "theme")]
    pub fn with_theme(mut self, theme: &ratkit_theme::AppTheme) -> Self {
        self.theme_info_color = Some(theme.info);
        self.theme_success_color = Some(theme.success);
        self.theme_warning_color = Some(theme.warning);
        self.theme_error_color = Some(theme.error);
        self.theme_confirm_color = Some(theme.primary);

        self.style = Style::default().bg(theme.background_panel).fg(theme.text);

        self.button_selected_style = Style::default()
            .fg(theme.selected_text)
            .bg(theme.primary)
            .add_modifier(Modifier::BOLD);
        self.button_style = Style::default().fg(theme.text);

        self
    }
}
