use pulldown_cmark::TagEnd;

use super::parser_state::MarkdownParser;

impl MarkdownParser {
    pub fn process_end_tag(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Heading(level) => {
                if self.in_heading {
                    let text = self
                        .current_spans
                        .iter()
                        .map(|s| s.content.to_string())
                        .collect::<String>();
                    self.current_spans.clear();
                    self.add_heading(level, text);
                    self.in_heading = false;
                    self.heading_level = None;
                }
            }
            TagEnd::CodeBlock => {
                if self.in_code_block {
                    self.add_code_block();
                    self.in_code_block = false;
                }
            }
            TagEnd::List(_) => {
                if self.list_depth > 0 {
                    self.list_depth -= 1;
                }
            }
            TagEnd::Item => {
                let text = self
                    .current_spans
                    .iter()
                    .map(|s| s.content.to_string())
                    .collect::<String>();
                self.current_spans.clear();
                if !text.is_empty() {
                    self.add_list_item(text);
                }
            }
            TagEnd::BlockQuote(_) => {
                self.in_quote = false;
            }
            TagEnd::Paragraph => {
                if !self.in_heading && !self.in_code_block {
                    self.flush_current_line();
                }
            }
            TagEnd::Emphasis => {
                self.in_emphasis = false;
            }
            TagEnd::Strong => {
                self.in_strong = false;
            }
            _ => {}
        }
    }
}
