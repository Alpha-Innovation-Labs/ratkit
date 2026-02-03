//! Highlight method for SyntaxHighlighter.

use ratatui::text::{Line, Text};

use crate::widgets::markdown_widget::extensions::theme::syntax_highlighter::SyntaxHighlighter;

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
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
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Highlight code content (always returns None when markdown feature is disabled).
    pub fn highlight(&self, _content: &str, _language: &str) -> Option<Text<'static>> {
        None
    }
}
