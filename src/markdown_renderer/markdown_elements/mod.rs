//! Markdown element types for markdown rendering.
//!
//! Represents parsed markdown elements with styling information
//! for render-markdown.nvim style rendering.

pub mod element;
pub mod enums;
pub mod methods;

pub use element::MarkdownElement;
pub use enums::{
    CheckboxState, CodeBlockBorderKind, ColumnAlignment, ElementKind, TableBorderKind, TextSegment,
};

pub mod constants;
pub use constants::{
    get_language_icon, get_link_icon, CodeBlockColors, CodeBlockTheme, BLOCKQUOTE_MARKER,
    BULLET_MARKERS, CHECKBOX_CHECKED, CHECKBOX_TODO, CHECKBOX_UNCHECKED, HEADING_ICONS,
    HORIZONTAL_RULE_CHAR,
};
