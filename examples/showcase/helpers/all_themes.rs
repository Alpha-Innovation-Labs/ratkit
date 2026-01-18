//! Get all available code block themes.

use ratatui_toolkit::markdown_renderer::CodeBlockTheme;

/// Get all available code block themes.
pub fn all_themes() -> Vec<CodeBlockTheme> {
    vec![
        CodeBlockTheme::AyuDark,
        CodeBlockTheme::GitHubDark,
        CodeBlockTheme::Dracula,
        CodeBlockTheme::Nord,
        CodeBlockTheme::Monokai,
        CodeBlockTheme::OneDark,
        CodeBlockTheme::Gruvbox,
        CodeBlockTheme::TokyoNight,
        CodeBlockTheme::Catppuccin,
    ]
}
