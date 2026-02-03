//! Highlight with line numbers method for SyntaxHighlighter.

use ratatui::text::{Line, Text};

use crate::widgets::markdown_widget::extensions::theme::syntax_highlighter::SyntaxHighlighter;

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
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
}
