//! Markdown rendering module for NEXUS TUI.
//!
//! Provides styled rendering of markdown content using pulldown-cmark
//! for parsing and ratatui for terminal output.
//!
//! Styling is inspired by render-markdown.nvim with:
//! - Nerd Font icons for headings
//! - Full-width colored backgrounds for headings
//! - Box-drawing borders for code blocks and tables
//! - Styled bullet markers for lists
//! - Left border for blockquotes

mod file_watcher;
mod markdown_source;
mod markdown_style;
mod markdown_widget;
pub mod minimap;
mod render_markdown_to_lines;
mod scroll_manager;
mod markdown_elements;
mod syntax_highlighter;
mod theme;

#[cfg(test)]
mod tests;

pub use file_watcher::MarkdownFileWatcher;
pub use markdown_source::MarkdownSource;
pub use markdown_style::MarkdownStyle;
pub use markdown_widget::{
    copy_selection_to_clipboard, handle_mouse_event, handle_mouse_event_with_double_click,
    handle_mouse_event_with_selection, render_markdown_interactive,
    render_markdown_interactive_with_selection, render_markdown_statusline,
    render_markdown_statusline_from_scroll, render_markdown_with_minimap, DoubleClickState,
    GitStats, MarkdownDoubleClickEvent, MarkdownEvent, MarkdownRenderOptions, MarkdownWidget,
    MarkdownWidgetMode, SelectionMouseResult, SelectionState,
};
pub use render_markdown_to_lines::render_markdown_to_elements;
pub use minimap::{Minimap, MinimapConfig};
pub use scroll_manager::{ExpandableState, MarkdownScrollManager};
pub use markdown_elements::methods::render::render as render_element;
pub use markdown_elements::methods::render::render_with_options as render_element_with_options;
pub use markdown_elements::methods::render::RenderOptions;
pub use markdown_elements::{CodeBlockColors, CodeBlockTheme, ElementKind, MarkdownElement, TextSegment};
pub use syntax_highlighter::{SyntaxHighlighter, SyntaxThemeVariant};
pub use theme::{
    get_effective_theme_variant, load_theme_from_json, palettes, ColorMapping, ColorPalette,
    MarkdownTheme, ThemeVariant,
};

/// Render markdown string to ratatui Text with default styling
///
/// # Arguments
/// * `markdown` - The markdown string to render
/// * `max_width` - Optional maximum width for full-width backgrounds (defaults to 120)
pub fn render_markdown(markdown: &str, max_width: Option<usize>) -> ratatui::text::Text<'static> {
    let width = max_width.unwrap_or(120);
    let elements = render_markdown_to_elements(markdown, true);

    let mut lines = Vec::new();
    for element in elements {
        lines.extend(render_element(&element, width));
    }

    ratatui::text::Text::from(lines)
}

/// Render markdown string to ratatui Text with custom style configuration
///
/// # Arguments
/// * `markdown` - The markdown string to render
/// * `style` - Custom style configuration (currently unused, kept for API compatibility)
/// * `max_width` - Optional maximum width for full-width backgrounds (defaults to 120)
pub fn render_markdown_with_style(
    markdown: &str,
    _style: MarkdownStyle,
    max_width: Option<usize>,
) -> ratatui::text::Text<'static> {
    render_markdown(markdown, max_width)
}
