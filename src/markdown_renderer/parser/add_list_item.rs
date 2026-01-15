use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use super::parser_state::MarkdownParser;

impl MarkdownParser {
    pub fn add_list_item(&mut self, text: String) {
        let bullet = match self.list_depth {
            0 => self.style.bullet_l1,
            1 => self.style.bullet_l2,
            _ => self.style.bullet_l3,
        };

        let indent = "  ".repeat(self.list_depth);
        let spans = vec![
            Span::raw(indent),
            Span::styled(bullet, Style::default().fg(Color::Cyan)),
            Span::raw(text),
        ];

        self.lines.push(Line::from(spans));
    }
}
