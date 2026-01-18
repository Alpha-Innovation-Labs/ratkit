//! Render markdown with selection highlighting.

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
};

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

use super::render_markdown_interactive_with_options::render_markdown_interactive_with_options;
use super::selection_state::SelectionState;

/// Default selection highlight style (#26343F background).
const DEFAULT_SELECTION_BG: Color = Color::Rgb(38, 52, 63);

/// Render markdown with selection highlighting.
///
/// # Arguments
///
/// * `content` - The markdown content to render
/// * `scroll` - The scroll manager
/// * `area` - The area to render into
/// * `is_resizing` - Whether the widget is being resized
/// * `selection` - The selection state
///
/// # Returns
///
/// A tuple of (rendered text, all rendered lines for selection extraction).
pub fn render_markdown_interactive_with_selection(
    content: &str,
    scroll: &mut MarkdownScrollManager,
    area: Rect,
    is_resizing: bool,
    selection: &SelectionState,
) -> (Text<'static>, Vec<Line<'static>>) {
    render_markdown_interactive_with_selection_themed(
        content,
        scroll,
        area,
        is_resizing,
        selection,
        None,
    )
}

/// Render markdown with selection highlighting and optional theme support.
///
/// # Arguments
///
/// * `content` - The markdown content to render
/// * `scroll` - The scroll manager
/// * `area` - The area to render into
/// * `is_resizing` - Whether the widget is being resized
/// * `selection` - The selection state
/// * `app_theme` - Optional application theme for selection color
///
/// # Returns
///
/// A tuple of (rendered text, all rendered lines for selection extraction).
pub fn render_markdown_interactive_with_selection_themed(
    content: &str,
    scroll: &mut MarkdownScrollManager,
    area: Rect,
    is_resizing: bool,
    selection: &SelectionState,
    app_theme: Option<&crate::theme::AppTheme>,
) -> (Text<'static>, Vec<Line<'static>>) {
    // First render normally, passing the theme through for element styling
    let text = render_markdown_interactive_with_options(content, scroll, area, is_resizing, app_theme);

    // Get all rendered lines from cache for selection extraction
    let all_lines = scroll
        .render_cache
        .as_ref()
        .map(|c| c.lines.clone())
        .unwrap_or_default();

    // If no active selection, return as-is
    if !selection.is_active() || !selection.has_selection() {
        return (text, all_lines);
    }

    // Apply selection highlighting to visible lines
    let Some((start, end)) = selection.get_selection() else {
        return (text, all_lines);
    };

    // Get selection style from theme or use default
    let selection_style = Style::new().bg(
        app_theme
            .map(|t| t.background_element)
            .unwrap_or(DEFAULT_SELECTION_BG),
    );

    let highlighted_lines: Vec<Line<'static>> = text
        .lines
        .into_iter()
        .enumerate()
        .map(|(visible_idx, line)| {
            // Convert visible index to document index
            let doc_y = (scroll.scroll_offset + visible_idx) as i32;

            // Check if this line is in selection range
            if doc_y < start.y || doc_y > end.y {
                return line;
            }

            // This line is at least partially selected
            apply_selection_to_line(line, doc_y, &start, &end, selection_style)
        })
        .collect();

    (Text::from(highlighted_lines), all_lines)
}

/// Apply selection highlighting to a single line.
fn apply_selection_to_line(
    line: Line<'static>,
    doc_y: i32,
    start: &super::selection_state::SelectionPos,
    end: &super::selection_state::SelectionPos,
    selection_style: Style,
) -> Line<'static> {
    // Calculate the character range to highlight on this line
    let line_text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
    let line_len = line_text.chars().count() as i32;

    let (sel_start, sel_end) = if start.y == end.y {
        // Single line selection
        (start.x.max(0), end.x.min(line_len - 1))
    } else if doc_y == start.y {
        // First line of multi-line selection
        (start.x.max(0), line_len - 1)
    } else if doc_y == end.y {
        // Last line of multi-line selection
        (0, end.x.min(line_len - 1))
    } else {
        // Middle line - entire line selected
        (0, line_len - 1)
    };

    if sel_start > sel_end || sel_start >= line_len {
        return line;
    }

    // Rebuild spans with selection highlighting
    // Skip line bar (│) and blockquote markers (▋) from selection highlighting
    let mut new_spans = Vec::new();
    let mut current_pos = 0i32;

    for span in line.spans {
        let span_text = span.content.to_string();
        let span_len = span_text.chars().count() as i32;
        let span_end = current_pos + span_len;

        // Skip line numbers, line bar, and blockquote markers from selection
        // Line numbers are digits/spaces at the start, border is │, blockquote is ▋
        let is_line_number =
            current_pos == 0 && span_text.chars().all(|c| c.is_ascii_digit() || c == ' ');
        if is_line_number || span_text.contains('│') || span_text.contains('▋') {
            new_spans.push(span);
            current_pos = span_end;
            continue;
        }

        if span_end <= sel_start || current_pos > sel_end {
            // Span is entirely outside selection
            new_spans.push(span);
        } else if current_pos >= sel_start && span_end <= sel_end + 1 {
            // Span is entirely inside selection
            new_spans.push(Span::styled(span_text, selection_style));
        } else {
            // Span is partially selected - split it
            let chars: Vec<char> = span_text.chars().collect();

            // Before selection
            if current_pos < sel_start {
                let before_count = (sel_start - current_pos) as usize;
                let before: String = chars[..before_count].iter().collect();
                new_spans.push(Span::styled(before, span.style));
            }

            // Selected portion
            let sel_offset = (sel_start - current_pos).max(0) as usize;
            let sel_count =
                ((sel_end + 1 - current_pos).min(span_len) as usize).saturating_sub(sel_offset);
            if sel_count > 0 && sel_offset < chars.len() {
                let selected: String = chars
                    [sel_offset..sel_offset + sel_count.min(chars.len() - sel_offset)]
                    .iter()
                    .collect();
                new_spans.push(Span::styled(selected, selection_style));
            }

            // After selection
            let after_start = (sel_end + 1 - current_pos).max(0) as usize;
            if after_start < chars.len() {
                let after: String = chars[after_start..].iter().collect();
                new_spans.push(Span::styled(after, span.style));
            }
        }

        current_pos = span_end;
    }

    Line::from(new_spans)
}
