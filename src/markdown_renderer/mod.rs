mod markdown_style;
mod parser;
mod render;

#[cfg(test)]
mod tests;

pub use markdown_style::MarkdownStyle;
pub use render::{render_markdown, render_markdown_with_style};
