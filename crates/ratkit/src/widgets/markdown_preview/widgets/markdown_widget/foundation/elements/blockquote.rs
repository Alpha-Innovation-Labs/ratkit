//! Render blockquote.

use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::constants::BLOCKQUOTE_MARKER;
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::enums::TextSegment;
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::text::{
    render_text_segment, wrap_text,
};
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::MarkdownElement;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

/// Blockquote marker color (matching render-markdown.nvim - consistent blue for all levels)
const BLOCKQUOTE_COLOR: Color = Color::Rgb(100, 149, 237); // Cornflower blue

pub fn render(
    _element: &MarkdownElement,
    segments: &[TextSegment],
    depth: usize,
    width: usize,
    app_theme: Option<&crate::widgets::markdown_preview::services::theme::AppTheme>,
) -> Vec<Line<'static>> {
    let actual_depth = depth.max(1);

    // Each level adds "\u{258b} " (marker + space), so 2 chars per level
    let prefix_char_width = actual_depth * 2;
    let content_width = width.saturating_sub(prefix_char_width);

    // Use theme color for blockquote text if available
    let quote_text_color = app_theme
        .map(|t| t.markdown.block_quote)
        .unwrap_or(Color::Rgb(180, 180, 200));

    // Build content from segments - all blockquote text is italic
    let mut content_spans: Vec<Span<'static>> = Vec::new();
    let quote_style = Style::default()
        .fg(quote_text_color)
        .add_modifier(ratatui::style::Modifier::ITALIC);

    for segment in segments {
        content_spans.push(render_text_segment(segment, quote_style));
    }

    // Get plain text for wrapping calculation
    let plain_text: String = content_spans
        .iter()
        .map(|s| s.content.to_string())
        .collect();
    let wrapped = wrap_text(&plain_text, content_width);

    let marker_style = Style::default().fg(BLOCKQUOTE_COLOR);

    // Handle empty content - still render the markers
    if wrapped.is_empty() {
        let mut spans = Vec::new();
        for _ in 1..=actual_depth {
            spans.push(Span::styled(BLOCKQUOTE_MARKER.to_string(), marker_style));
            spans.push(Span::raw(" "));
        }
        return vec![Line::from(spans)];
    }

    wrapped
        .into_iter()
        .map(|line_text| {
            let mut spans = Vec::new();

            // Add marker for each depth level (all same color)
            for _ in 1..=actual_depth {
                spans.push(Span::styled(BLOCKQUOTE_MARKER.to_string(), marker_style));
                spans.push(Span::raw(" "));
            }

            // Add content
            spans.push(Span::styled(line_text, quote_style));

            Line::from(spans)
        })
        .collect()
}

/// Create blockquote prefix spans for use in other renderers (e.g., code blocks inside quotes)
pub fn create_blockquote_prefix(depth: usize) -> Vec<Span<'static>> {
    let marker_style = Style::default().fg(BLOCKQUOTE_COLOR);
    let mut spans = Vec::new();
    for _ in 1..=depth {
        spans.push(Span::styled(BLOCKQUOTE_MARKER.to_string(), marker_style));
        spans.push(Span::raw(" "));
    }
    spans
}

/// Get the character width of the blockquote prefix for a given depth
pub fn blockquote_prefix_width(depth: usize) -> usize {
    depth * 2 // Each level adds "\u{258b} " which is 2 chars
}
