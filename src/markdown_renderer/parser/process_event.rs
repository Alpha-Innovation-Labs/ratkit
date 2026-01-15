use pulldown_cmark::Event;

use super::parser_state::MarkdownParser;

impl MarkdownParser {
    pub fn process_event(&mut self, event: Event) {
        match event {
            Event::Start(tag) => self.process_start_tag(tag),
            Event::End(tag) => self.process_end_tag(tag),
            Event::Text(text) => self.process_text(&text),
            Event::Code(code) => self.process_code(&code),
            Event::SoftBreak | Event::HardBreak => self.process_break(),
            _ => {}
        }
    }
}
