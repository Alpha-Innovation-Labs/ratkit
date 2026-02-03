use crate::primitives::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn new(title: &'a str, message: &'a str) -> Self {
        Self {
            title,
            message,
            dialog_type: crate::primitives::dialog::DialogType::Info,
            buttons: Vec::new(),
            selected_button: 0,
            width_percent: 0.6,
            height_percent: 0.4,
            footer: None,
            footer_style: ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray),
            title_inside: false,
            overlay: false,
            overlay_style: ratatui::style::Style::default()
                .bg(ratatui::style::Color::Rgb(0, 0, 0))
                .fg(ratatui::style::Color::Rgb(40, 40, 40)),
            border_color: None,
            style: ratatui::style::Style::default(),
            button_selected_style: ratatui::style::Style::default()
                .fg(ratatui::style::Color::Black)
                .bg(ratatui::style::Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
            button_style: ratatui::style::Style::default(),
            button_areas: Vec::new(),
            theme_info_color: None,
            theme_success_color: None,
            theme_warning_color: None,
            theme_error_color: None,
            theme_confirm_color: None,
        }
    }
}
