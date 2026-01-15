use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use super::parser_state::MarkdownParser;

impl MarkdownParser {
    pub fn add_quote_line(&mut self, text: String) {
        let spans = vec![
            Span::styled(
                self.style.quote_icon,
                Style::default().fg(self.style.quote_fg),
            ),
            Span::styled(
                text,
                Style::default()
                    .fg(self.style.quote_fg)
                    .bg(self.style.quote_bg)
                    .add_modifier(Modifier::ITALIC),
            ),
        ];
        self.lines.push(Line::from(spans));
    }
}
