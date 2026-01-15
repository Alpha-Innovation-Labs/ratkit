use pulldown_cmark::HeadingLevel;
use ratatui::text::{Line, Span};

use super::super::markdown_style::MarkdownStyle;

/// State machine for markdown parsing
pub struct MarkdownParser {
    pub lines: Vec<Line<'static>>,
    pub current_spans: Vec<Span<'static>>,
    pub list_depth: usize,
    pub in_code_block: bool,
    pub code_block_lines: Vec<String>,
    pub code_block_lang: Option<String>,
    pub in_heading: bool,
    pub heading_level: Option<HeadingLevel>,
    pub in_quote: bool,
    pub in_strong: bool,
    pub in_emphasis: bool,
    pub style: MarkdownStyle,
    pub max_width: usize,
}

impl MarkdownParser {
    pub fn new(style: MarkdownStyle, max_width: usize) -> Self {
        Self {
            lines: Vec::new(),
            current_spans: Vec::new(),
            list_depth: 0,
            in_code_block: false,
            code_block_lines: Vec::new(),
            code_block_lang: None,
            in_heading: false,
            heading_level: None,
            in_quote: false,
            in_strong: false,
            in_emphasis: false,
            style,
            max_width,
        }
    }

    pub fn finalize(mut self) -> Vec<Line<'static>> {
        self.flush_current_line();
        self.lines
    }

    pub fn flush_current_line(&mut self) {
        if !self.current_spans.is_empty() {
            let spans = std::mem::take(&mut self.current_spans);
            self.lines.push(Line::from(spans));
        }
    }
}
