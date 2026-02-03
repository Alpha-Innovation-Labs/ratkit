//! Elements module.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CodeBlockBorderKind {
    #[default]
    Full,
    BottomOnly,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub struct CodeBlockColors {
    pub line_number: Color,
    pub line_number_gutter: Color,
    pub content_even: Color,
    pub content_odd: Color,
    pub border: Color,
    pub border_highlight: Color,
    pub highlight: Color,
}

impl Default for CodeBlockColors {
    fn default() -> Self {
        Self {
            line_number: Color::DarkGray,
            line_number_gutter: Color::Reset,
            content_even: Color::Reset,
            content_odd: Color::Reset,
            border: Color::DarkGray,
            border_highlight: Color::Gray,
            highlight: Color::LightYellow,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CodeBlockTheme {
    #[default]
    OneDark,
    GitHub,
    Monokai,
    SolarizedLight,
    SolarizedDark,
}

#[derive(Debug, Clone)]
pub struct MarkdownElement {
    pub kind: ElementKind,
    pub content: String,
    pub style: Option<Style>,
}

#[derive(Debug, Clone, Default)]
pub struct TextSegment {
    pub text: String,
    pub style: Style,
    pub is_code: bool,
}
