//! Syntax highlighting for code blocks using syntect.
//!
//! This module provides syntax highlighting functionality for code blocks
//! in markdown documents using the syntect library.

use ratatui::text::{Line, Text};

/// Theme variant for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SyntaxThemeVariant {
    #[default]
    Dark,
    Light,
}

/// Highlighter for code blocks using syntect.
pub struct SyntaxHighlighter {
    #[cfg(feature = "markdown")]
    syntax_set: syntect::parsing::SyntaxSet,
    #[cfg(feature = "markdown")]
    theme: syntect::highlighting::Theme,
    #[cfg(feature = "markdown")]
    theme_variant: SyntaxThemeVariant,
}

#[cfg(feature = "markdown")]
impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with default dark theme.
    pub fn new() -> Self {
        let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        let theme = theme_set.themes["base16-ocean.dark"].clone();

        Self {
            syntax_set,
            theme,
            theme_variant: SyntaxThemeVariant::Dark,
        }
    }

    /// Create a new syntax highlighter with dark theme.
    pub fn with_dark_theme() -> Self {
        let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        let theme = theme_set.themes["base16-ocean.dark"].clone();

        Self {
            syntax_set,
            theme,
            theme_variant: SyntaxThemeVariant::Dark,
        }
    }

    /// Create a new syntax highlighter with GitHub Light theme.
    pub fn with_light_theme() -> Self {
        let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        let theme = theme_set
            .themes
            .get("github-light")
            .cloned()
            .unwrap_or_else(|| theme_set.themes["base16-ocean.light"].clone());

        Self {
            syntax_set,
            theme,
            theme_variant: SyntaxThemeVariant::Light,
        }
    }

    /// Create a new syntax highlighter with a specific theme name.
    ///
    /// # Arguments
    ///
    /// * `theme_name` - Name of the theme (e.g., "base16-ocean.dark", "github-dark", "github-light")
    pub fn with_named_theme(theme_name: &str) -> Self {
        let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        let theme = theme_set
            .themes
            .get(theme_name)
            .cloned()
            .unwrap_or_else(|| {
                // Fallback to dark theme
                theme_set.themes["base16-ocean.dark"].clone()
            });

        let theme_variant = if theme_name.contains("light") {
            SyntaxThemeVariant::Light
        } else {
            SyntaxThemeVariant::Dark
        };

        Self {
            syntax_set,
            theme,
            theme_variant,
        }
    }

    /// Set the theme to dark mode.
    pub fn set_dark_theme(&mut self) {
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        self.theme = theme_set.themes["base16-ocean.dark"].clone();
        self.theme_variant = SyntaxThemeVariant::Dark;
    }

    /// Set the theme to light mode (GitHub Light).
    pub fn set_light_theme(&mut self) {
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        self.theme = theme_set
            .themes
            .get("github-light")
            .cloned()
            .unwrap_or_else(|| theme_set.themes["base16-ocean.light"].clone());
        self.theme_variant = SyntaxThemeVariant::Light;
    }

    /// Switch between light and dark themes.
    pub fn toggle_theme(&mut self) {
        match self.theme_variant {
            SyntaxThemeVariant::Dark => self.set_light_theme(),
            SyntaxThemeVariant::Light => self.set_dark_theme(),
        }
    }

    /// Get the current theme variant.
    pub fn theme_variant(&self) -> SyntaxThemeVariant {
        self.theme_variant
    }

    /// Create a new syntax highlighter with custom theme.
    pub fn with_custom_theme(theme: syntect::highlighting::Theme) -> Self {
        let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
        let theme_variant = if theme.name.as_deref().unwrap_or("").contains("light") {
            SyntaxThemeVariant::Light
        } else {
            SyntaxThemeVariant::Dark
        };

        Self {
            syntax_set,
            theme,
            theme_variant,
        }
    }

    /// Highlight code content for a given language.
    ///
    /// # Arguments
    ///
    /// * `content` - The code content to highlight
    /// * `language` - The language identifier (e.g., "rust", "python", "javascript")
    ///
    /// # Returns
    ///
    /// Syntax highlighted text, or `None` if language is not recognized
    pub fn highlight(&self, content: &str, language: &str) -> Option<Text<'static>> {
        let syntax = self.find_syntax(language)?;

        let mut highlighter = syntect::easy::HighlightLines::new(&syntax, &self.theme);

        let mut lines = Vec::new();

        for line in content.lines() {
            if let Ok(highlighted) = highlighter.highlight_line(line, &self.syntax_set) {
                let spans: Vec<ratatui::text::Span<'static>> = highlighted
                    .into_iter()
                    .map(|(style, text)| {
                        let ratatui_style = syntect_tui::translate_style(style)
                            .unwrap_or_else(|_| ratatui::style::Style::default());
                        ratatui::text::Span::styled(text.to_string(), ratatui_style)
                    })
                    .collect();

                lines.push(Line::from(spans));
            } else {
                lines.push(Line::from(line.to_string()));
            }
        }

        Some(Text::from(lines))
    }

    /// Highlight multiple lines of code with line numbers.
    ///
    /// # Arguments
    ///
    /// * `content` - The code content to highlight
    /// * `language` - The language identifier
    /// * `start_line` - Starting line number for display
    ///
    /// # Returns
    ///
    /// Syntax highlighted text with line numbers
    pub fn highlight_with_line_numbers(
        &self,
        content: &str,
        language: &str,
        start_line: usize,
    ) -> Option<Text<'static>> {
        let syntax = self.find_syntax(language)?;

        let mut highlighter = syntect::easy::HighlightLines::new(&syntax, &self.theme);

        let mut lines = Vec::new();

        for (i, line) in content.lines().enumerate() {
            let line_num = start_line + i;
            let line_num_str = format!("{:4} ", line_num);

            if let Ok(highlighted) = highlighter.highlight_line(line, &self.syntax_set) {
                let num_style =
                    ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(100, 100, 100));
                let num_span = ratatui::text::Span::styled(line_num_str, num_style);

                let content_spans: Vec<ratatui::text::Span<'static>> = highlighted
                    .into_iter()
                    .map(|(style, text)| {
                        let ratatui_style = syntect_tui::translate_style(style)
                            .unwrap_or_else(|_| ratatui::style::Style::default());
                        ratatui::text::Span::styled(text.to_string(), ratatui_style)
                    })
                    .collect();

                let mut all_spans = vec![num_span];
                all_spans.extend(content_spans);
                lines.push(Line::from(all_spans));
            } else {
                let num_style =
                    ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(100, 100, 100));
                let span =
                    ratatui::text::Span::styled(format!("{}{}", line_num_str, line), num_style);
                lines.push(Line::from(span));
            }
        }

        Some(Text::from(lines))
    }

    /// Find a syntax definition for given language identifier.
    fn find_syntax(&self, language: &str) -> Option<syntect::parsing::SyntaxReference> {
        if language.is_empty() {
            return None;
        }

        self.syntax_set
            .find_syntax_by_token(language)
            .or_else(|| self.syntax_set.find_syntax_by_name(language))
            .or_else(|| self.syntax_set.find_syntax_by_extension(language))
            .cloned()
    }
}

#[cfg(not(feature = "markdown"))]
impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter (no-op when markdown feature is disabled).
    pub fn new() -> Self {
        Self
    }

    /// Create a new syntax highlighter with dark theme (no-op when markdown feature is disabled).
    pub fn with_dark_theme() -> Self {
        Self
    }

    /// Create a new syntax highlighter with light theme (no-op when markdown feature is disabled).
    pub fn with_light_theme() -> Self {
        Self
    }

    /// Create a new syntax highlighter with a named theme (no-op when markdown feature is disabled).
    pub fn with_named_theme(_theme_name: &str) -> Self {
        Self
    }

    /// Set dark theme (no-op when markdown feature is disabled).
    pub fn set_dark_theme(&mut self) {}

    /// Set light theme (no-op when markdown feature is disabled).
    pub fn set_light_theme(&mut self) {}

    /// Toggle theme (no-op when markdown feature is disabled).
    pub fn toggle_theme(&mut self) {}

    /// Get current theme variant (always returns Dark when markdown feature is disabled).
    pub fn theme_variant(&self) -> SyntaxThemeVariant {
        SyntaxThemeVariant::Dark
    }

    /// Highlight code content (always returns None when markdown feature is disabled).
    pub fn highlight(&self, _content: &str, _language: &str) -> Option<Text<'static>> {
        None
    }
}
