//! Apply selection highlighting to rendered lines.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::markdown_widget::foundation::types::SelectionPos;
use crate::markdown_widget::state::selection_state::SelectionState;

/// Selection highlight background color (dark blue for better visibility).
const SELECTION_BG: Color = Color::Rgb(55, 75, 120);

/// Apply selection highlighting to visible lines.
///
/// # Arguments
///
/// * `lines` - The visible lines to highlight.
/// * `selection` - The selection state.
/// * `scroll_offset` - The current scroll offset.
///
/// # Returns
///
/// Lines with selection highlighting applied.
pub fn apply_selection_highlighting(
    lines: Vec<Line<'static>>,
    selection: &SelectionState,
    scroll_offset: usize,
) -> Vec<Line<'static>> {
    // If no active selection, return as-is
    if !selection.active {
        return lines;
    }

    let Some((start, end)) = selection.get_selection() else {
        return lines;
    };

    let selection_style = Style::new().bg(SELECTION_BG);

    lines
        .into_iter()
        .enumerate()
        .map(|(visible_idx, line)| {
            // Convert visible index to document index
            let doc_y = (scroll_offset + visible_idx) as i32;

            // Check if this line is in selection range
            if doc_y < start.y || doc_y > end.y {
                return line;
            }

            // This line is at least partially selected
            apply_selection_to_line(line, doc_y, &start, &end, selection_style)
        })
        .collect()
}

/// Apply selection highlighting to a single line.
fn apply_selection_to_line(
    line: Line<'static>,
    doc_y: i32,
    start: &SelectionPos,
    end: &SelectionPos,
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
            new_spans.push(Span::styled(span_text, span.style.bg(SELECTION_BG)));
        } else {
            // Span is partially selected - split it
            let chars: Vec<char> = span_text.chars().collect();

            // Before selection
            if current_pos < sel_start {
                let before_count = (sel_start - current_pos) as usize;
                let before: String = chars[..before_count].iter().collect();
                new_spans.push(Span::styled(before, span.style));
            }

            // Selected part
            let sel_local_start = (sel_start - current_pos).max(0) as usize;
            let sel_local_end = ((sel_end - current_pos + 1) as usize).min(chars.len());
            if sel_local_start < sel_local_end {
                let selected: String = chars[sel_local_start..sel_local_end].iter().collect();
                new_spans.push(Span::styled(selected, span.style.bg(SELECTION_BG)));
            }

            // After selection
            let after_start = (sel_end - current_pos + 1) as usize;
            if after_start < chars.len() {
                let after: String = chars[after_start..].iter().collect();
                new_spans.push(Span::styled(after, span.style));
            }
        }

        current_pos = span_end;
    }

    Line::from(new_spans)
}
