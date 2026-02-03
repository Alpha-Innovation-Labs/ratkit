//! Foundation module for markdown widget.
//!
//! Contains core elements, types, events, helpers, and rendering functions.

use pulldown_cmark::{HeadingLevel, Options, Parser};
use ratatui::style::{Color, Style};

pub mod elements;
pub mod events;
pub mod functions;
pub mod helpers;
pub mod parser;
pub mod source;
pub mod types;

pub use elements::CodeBlockTheme;
pub use events::{MarkdownDoubleClickEvent, MarkdownEvent};
pub use functions::{render_markdown, render_markdown_with_style};
pub use types::GitStats;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementKind {
    Text,
    CodeBlock {
        language: Option<String>,
        theme: CodeBlockTheme,
    },
    BlockQuote,
    List(u64),
    ListItem,
    Heading(HeadingLevel),
    Bold,
    Italic,
    Strikethrough,
    Link {
        destination: String,
        title: Option<String>,
    },
    Image {
        destination: String,
        title: Option<String>,
    },
    Table(Vec<String>),
    TableRow,
    TableCell,
    HorizontalRule,
    SoftBreak,
    InlineCode,
    FootnoteReference(String),
}

#[derive(Debug, Clone)]
pub struct MarkdownElement {
    pub kind: ElementKind,
    pub content: String,
    pub style: Option<Style>,
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Debug, Clone)]
pub struct TextSegment {
    pub text: String,
    pub style: Style,
    pub is_code: bool,
}

pub fn create_markdown_parser(content: &str) -> Parser<'_> {
    Parser::new_ext(content, Options::all())
}
