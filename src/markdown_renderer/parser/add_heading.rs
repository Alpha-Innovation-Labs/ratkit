use pulldown_cmark::HeadingLevel;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use super::parser_state::MarkdownParser;

impl MarkdownParser {
    pub fn add_heading(&mut self, level: HeadingLevel, text: String) {
        let (icon, fg, bg) = match level {
            HeadingLevel::H1 => (self.style.h1_icon, self.style.h1_fg, self.style.h1_bg),
            HeadingLevel::H2 => (self.style.h2_icon, self.style.h2_fg, self.style.h2_bg),
            HeadingLevel::H3 => (self.style.h3_icon, self.style.h3_fg, self.style.h3_bg),
            HeadingLevel::H4 => (self.style.h4_icon, self.style.h3_fg, self.style.h3_bg),
            HeadingLevel::H5 => (self.style.h5_icon, self.style.h3_fg, self.style.h3_bg),
            HeadingLevel::H6 => (self.style.h6_icon, self.style.h3_fg, self.style.h3_bg),
        };

        // Add padding line before heading
        if !self.lines.is_empty() {
            self.lines.push(Line::from(""));
        }

        // Create heading text with icon, then pad to extend background full width
        let heading_text = format!("{}{}", icon, text);
        // Pad with spaces to make background extend to panel width
        let padded_text = format!("{:<width$}", heading_text, width = self.max_width);

        let heading_span = Span::styled(
            padded_text,
            Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD),
        );

        self.lines.push(Line::from(vec![heading_span]));

        // Add padding line after heading
        self.lines.push(Line::from(""));
    }
}
