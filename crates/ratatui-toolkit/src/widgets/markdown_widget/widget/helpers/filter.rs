//! Filter navigation helpers for MarkdownWidget.

use crate::widgets::markdown_widget::foundation::elements::{render, ElementKind, TextSegment};
use crate::widgets::markdown_widget::foundation::parser::render_markdown_to_elements;
use crate::widgets::markdown_widget::state::CollapseState;

use super::should_render_collapsed_line;

pub fn get_filtered_visual_lines(
    content: &str,
    filter_text: &str,
    collapse: &CollapseState,
    width: usize,
) -> Vec<usize> {
    let filter_lower = filter_text.to_lowercase();
    let elements = render_markdown_to_elements(content, true);
    let mut filtered_visual_lines: Vec<usize> = Vec::new();
    let mut visual_line_idx = 0;

    for (idx, element) in elements.iter().enumerate() {
        if !should_render_collapsed_line(element, idx, collapse) {
            continue;
        }

        let rendered = render(element, width);
        let line_count = rendered.len();

        let text = element_to_plain_text_for_filter(&element.kind);
        let text_lower = text.to_lowercase();

        if text_lower.contains(&filter_lower) || filter_lower.is_empty() {
            for offset in 0..line_count {
                filtered_visual_lines.push(visual_line_idx + offset + 1);
            }
        }

        visual_line_idx += line_count;
    }

    filtered_visual_lines
}

pub fn find_next_filtered_line(
    content: &str,
    filter_text: &str,
    collapse: &CollapseState,
    current_visual_line: usize,
    width: usize,
) -> Option<usize> {
    let filtered = get_filtered_visual_lines(content, filter_text, collapse, width);
    if filtered.is_empty() {
        return None;
    }

    let mut search_idx = 0;
    for (i, &line) in filtered.iter().enumerate() {
        if line >= current_visual_line {
            search_idx = i;
            break;
        }
        search_idx = i + 1;
    }

    filtered.get(search_idx).copied()
}

pub fn find_prev_filtered_line(
    content: &str,
    filter_text: &str,
    collapse: &CollapseState,
    current_visual_line: usize,
    width: usize,
) -> Option<usize> {
    let filtered = get_filtered_visual_lines(content, filter_text, collapse, width);
    if filtered.is_empty() {
        return None;
    }

    for (_i, &line) in filtered.iter().enumerate().rev() {
        if line < current_visual_line {
            return Some(line);
        }
    }

    filtered.last().copied()
}

fn text_segment_to_string(segment: &TextSegment) -> String {
    match segment {
        TextSegment::Plain(s) => s.clone(),
        TextSegment::Bold(s) => s.clone(),
        TextSegment::Italic(s) => s.clone(),
        TextSegment::BoldItalic(s) => s.clone(),
        TextSegment::InlineCode(s) => s.clone(),
        TextSegment::Link { text, .. } => text.clone(),
        TextSegment::Strikethrough(s) => s.clone(),
        TextSegment::Html(s) => s.clone(),
        TextSegment::Checkbox(_) => String::new(),
    }
}

pub fn element_to_plain_text_for_filter(kind: &ElementKind) -> String {
    match kind {
        ElementKind::Heading { text, .. } => text
            .iter()
            .map(text_segment_to_string)
            .collect::<Vec<_>>()
            .join(""),
        ElementKind::Paragraph(segments) => segments
            .iter()
            .map(text_segment_to_string)
            .collect::<Vec<_>>()
            .join(""),
        ElementKind::ListItem { content, .. } => content
            .iter()
            .map(text_segment_to_string)
            .collect::<Vec<_>>()
            .join(""),
        ElementKind::Blockquote { content, .. } => content
            .iter()
            .map(text_segment_to_string)
            .collect::<Vec<_>>()
            .join(""),
        ElementKind::CodeBlockContent { content, .. } => content.clone(),
        ElementKind::TableRow { cells, .. } => cells.join(" | "),
        ElementKind::FrontmatterField { key, value, .. } => format!("{}: {}", key, value),
        ElementKind::Expandable { .. } => String::new(),
        _ => String::new(),
    }
}
