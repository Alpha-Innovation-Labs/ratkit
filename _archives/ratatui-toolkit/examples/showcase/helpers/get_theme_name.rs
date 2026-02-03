//! Get display name for code block themes.

use ratatui_toolkit::CodeBlockTheme;

/// Get the display name for a code block theme.
pub fn get_theme_name(theme: CodeBlockTheme) -> &'static str {
    match theme {
        CodeBlockTheme::AyuDark => "Ayu Dark",
        CodeBlockTheme::GitHubDark => "GitHub Dark",
        CodeBlockTheme::Dracula => "Dracula",
        CodeBlockTheme::Nord => "Nord",
        CodeBlockTheme::Monokai => "Monokai",
        CodeBlockTheme::OneDark => "One Dark",
        CodeBlockTheme::Gruvbox => "Gruvbox",
        CodeBlockTheme::TokyoNight => "Tokyo Night",
        CodeBlockTheme::Catppuccin => "Catppuccin",
    }
}
