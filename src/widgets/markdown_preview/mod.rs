//! Markdown rendering widget for ratatui applications.
//!
//! Provides a feature-rich markdown viewer with TOC, selection, themes,
//! syntax highlighting, and more.

pub mod primitives;
pub mod services;
pub mod widgets;

pub use widgets::markdown_widget::extensions::{
    get_effective_theme_variant, handle_click, handle_mouse_event,
    handle_mouse_event_with_double_click, load_theme_from_json, palettes, should_render_line,
    ColorMapping, ColorPalette, CustomScrollbar, MarkdownStyle, MarkdownTheme, ScrollbarConfig,
    SyntaxHighlighter, SyntaxThemeVariant, ThemeVariant, Toc, TocConfig,
};
pub use widgets::markdown_widget::{
    render_element, render_element_with_options, render_markdown, render_markdown_to_elements,
    render_markdown_with_style, CacheState, CheckboxState, CodeBlockBorderKind, CodeBlockColors,
    CodeBlockTheme, CollapseState, ColumnAlignment, DisplaySettings, DoubleClickState, ElementKind,
    ExpandableEntry, ExpandableState, GitStats, GitStatsState, MarkdownDoubleClickEvent,
    MarkdownElement, MarkdownEvent, MarkdownSource, MarkdownState, MarkdownWidget,
    MarkdownWidgetMode, ParsedCache, RenderCache, RenderOptions, ScrollState, SelectionPos,
    SelectionState, SourceState, TableBorderKind, TextSegment, TocEntry, TocState, VimState,
    BLOCKQUOTE_MARKER, BULLET_MARKERS, CHECKBOX_CHECKED, CHECKBOX_TODO, CHECKBOX_UNCHECKED,
    HEADING_ICONS, HORIZONTAL_RULE_CHAR,
};
