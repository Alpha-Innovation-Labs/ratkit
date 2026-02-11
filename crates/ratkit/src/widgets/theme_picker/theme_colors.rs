use ratatui::style::Color;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeColors {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
    pub background_menu: Color,
    pub background_panel: Color,
    pub text: Color,
    pub text_muted: Color,
    pub border: Color,
    pub border_active: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Magenta,
            accent: Color::Yellow,
            background: Color::Black,
            background_menu: Color::Rgb(30, 30, 30),
            background_panel: Color::Rgb(40, 40, 40),
            text: Color::White,
            text_muted: Color::DarkGray,
            border: Color::DarkGray,
            border_active: Color::Cyan,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Blue,
        }
    }
}

impl ThemeColors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn primary(mut self, color: Color) -> Self {
        self.primary = color;
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    pub fn text(mut self, color: Color) -> Self {
        self.text = color;
        self
    }
}
