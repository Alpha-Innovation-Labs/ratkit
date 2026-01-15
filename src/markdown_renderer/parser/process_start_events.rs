use pulldown_cmark::{CodeBlockKind, Tag};

use super::parser_state::MarkdownParser;

impl MarkdownParser {
    pub fn process_start_tag(&mut self, tag: Tag) {
        match tag {
            Tag::Heading { level, .. } => {
                self.in_heading = true;
                self.heading_level = Some(level);
            }
            Tag::CodeBlock(kind) => {
                self.in_code_block = true;
                if let CodeBlockKind::Fenced(lang) = kind {
                    self.code_block_lang = Some(lang.to_string());
                }
            }
            Tag::List(_) => {
                self.list_depth += 1;
            }
            Tag::Item => {
                // Item content will be collected in current_spans
            }
            Tag::BlockQuote(_) => {
                self.in_quote = true;
            }
            Tag::Emphasis => {
                self.in_emphasis = true;
            }
            Tag::Strong => {
                self.in_strong = true;
            }
            _ => {}
        }
    }
}
