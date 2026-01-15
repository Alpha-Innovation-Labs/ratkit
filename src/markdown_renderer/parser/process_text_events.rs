use ratatui::style::{Modifier, Style};
use ratatui::text::Span;

use super::parser_state::MarkdownParser;

impl MarkdownParser {
    pub fn process_text(&mut self, text: &str) {
        if self.in_code_block {
            self.code_block_lines.push(text.to_string());
        } else if self.in_heading {
            self.current_spans.push(Span::raw(text.to_string()));
        } else if self.in_quote {
            self.add_quote_line(text.to_string());
        } else {
            // Apply bold and/or italic styling based on current state
            let mut style = Style::default();
            if self.in_strong {
                style = style.add_modifier(Modifier::BOLD);
            }
            if self.in_emphasis {
                style = style.add_modifier(Modifier::ITALIC);
            }

            let span = if self.in_strong || self.in_emphasis {
                Span::styled(text.to_string(), style)
            } else {
                Span::raw(text.to_string())
            };
            self.current_spans.push(span);
        }
    }

    pub fn process_code(&mut self, code: &str) {
        // Inline code
        let code_span = Span::styled(
            format!(" {} ", code),
            Style::default()
                .fg(self.style.inline_code_fg)
                .bg(self.style.inline_code_bg),
        );
        self.current_spans.push(code_span);
    }

    pub fn process_break(&mut self) {
        if !self.in_code_block && !self.in_heading {
            self.flush_current_line();
        }
    }
}
