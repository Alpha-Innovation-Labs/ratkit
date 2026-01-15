use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use super::parser_state::MarkdownParser;

impl MarkdownParser {
    pub fn add_code_block(&mut self) {
        if self.code_block_lines.is_empty() {
            return;
        }

        // Add top border if enabled
        if self.style.code_block_border {
            let border = Span::styled(
                "╭─────────────────────────────────────────────────────",
                Style::default().fg(Color::DarkGray),
            );
            self.lines.push(Line::from(border));
        }

        // Add each line of code with background
        for line in &self.code_block_lines {
            let code_span = if self.style.code_block_border {
                vec![
                    Span::styled("│ ", Style::default().fg(Color::DarkGray)),
                    Span::styled(line.clone(), Style::default().bg(self.style.code_block_bg)),
                ]
            } else {
                vec![Span::styled(
                    format!("  {}", line),
                    Style::default().bg(self.style.code_block_bg),
                )]
            };
            self.lines.push(Line::from(code_span));
        }

        // Add bottom border if enabled
        if self.style.code_block_border {
            let border = Span::styled(
                "╰─────────────────────────────────────────────────────",
                Style::default().fg(Color::DarkGray),
            );
            self.lines.push(Line::from(border));
        }

        self.code_block_lines.clear();
        self.code_block_lang = None;
    }
}
