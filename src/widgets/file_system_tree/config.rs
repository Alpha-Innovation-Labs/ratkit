use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Copy)]
pub struct FileSystemTreeConfig {
    pub show_hidden: bool,
    pub use_dark_theme: bool,
    pub dir_style: Style,
    pub file_style: Style,
    pub selected_style: Style,
}

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

impl FileSystemTreeConfig {
    pub fn show_hidden(mut self, show: bool) -> Self {
        self.show_hidden = show;
        self
    }

    pub fn use_dark_theme(mut self, dark: bool) -> Self {
        self.use_dark_theme = dark;
        self
    }

    pub fn dir_style(mut self, style: Style) -> Self {
        self.dir_style = style;
        self
    }

    pub fn file_style(mut self, style: Style) -> Self {
        self.file_style = style;
        self
    }

    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }
}
