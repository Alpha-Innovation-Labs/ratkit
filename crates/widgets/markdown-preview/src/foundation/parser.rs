//! Parser module for markdown.

use super::elements::MarkdownElement;
use super::ElementKind;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

pub fn render_markdown_to_elements(content: &str) -> Vec<MarkdownElement> {
    let parser = Parser::new_ext(content, Options::all());
    let mut elements = Vec::new();

    for event in parser {
        match event {
            Event::Text(text) => {
                elements.push(MarkdownElement {
                    kind: ElementKind::Text,
                    content: text.to_string(),
                    style: None,
                });
            }
            Event::Code(code) => {
                elements.push(MarkdownElement {
                    kind: ElementKind::InlineCode,
                    content: code.to_string(),
                    style: None,
                });
            }
            Event::Html(html) => {
                elements.push(MarkdownElement {
                    kind: ElementKind::Text,
                    content: html.to_string(),
                    style: None,
                });
            }
            Event::SoftBreak => {
                elements.push(MarkdownElement {
                    kind: ElementKind::SoftBreak,
                    content: "\n".to_string(),
                    style: None,
                });
            }
            Event::Rule => {
                elements.push(MarkdownElement {
                    kind: ElementKind::HorizontalRule,
                    content: "---".to_string(),
                    style: None,
                });
            }
            _ => {}
        }
    }

    elements
}
