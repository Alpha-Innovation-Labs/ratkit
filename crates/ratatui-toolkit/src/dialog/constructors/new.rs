use crate::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn new(title: &'a str, message: &'a str) -> Self {
        Self {
            title,
            message,
            dialog_type: crate::dialog::DialogType::Info,
            buttons: Vec::new(),
            selected_button: 0,
            width_percent: 0.6,
            height_percent: 0.4,
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
