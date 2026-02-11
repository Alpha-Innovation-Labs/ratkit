use crate::primitives::dialog::types::{Dialog, DialogType};
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
            footer: None,
            footer_style: Style::default().fg(Color::DarkGray),
            title_inside: false,
            overlay: false,
            overlay_style: Style::default()
                .bg(Color::Rgb(0, 0, 0))
                .fg(Color::Rgb(40, 40, 40)),
            border_color: None,
            style: Style::default(),
            button_selected_style: Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            button_style: Style::default(),
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
        self.footer = Some(footer);
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
        self.overlay = overlay;
        self
    }

    pub fn overlay_style(mut self, overlay_style: Style) -> Self {
        self.overlay_style = overlay_style;
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
