use ratatui::style::{Color, Modifier, Style};

use crate::file_system_tree::FileSystemTreeConfig;

impl Default for FileSystemTreeConfig {
    fn default() -> Self {
        Self {
            show_hidden: false,
            use_dark_theme: true,
            dir_style: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            file_style: Style::default().fg(Color::White),
            selected_style: Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        }
    }
}
