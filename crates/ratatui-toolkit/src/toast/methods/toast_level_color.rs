use crate::toast::ToastLevel;

impl ToastLevel {
    pub fn color(&self) -> ratatui::style::Color {
        match self {
            ToastLevel::Success => ratatui::style::Color::Green,
            ToastLevel::Error => ratatui::style::Color::Red,
            ToastLevel::Info => ratatui::style::Color::Cyan,
            ToastLevel::Warning => ratatui::style::Color::Yellow,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ToastLevel::Success => "✓",
            ToastLevel::Error => "✗",
            ToastLevel::Info => "ℹ",
            ToastLevel::Warning => "⚠",
        }
    }
}
