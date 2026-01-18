//! Render code block header, content, and border.

use super::super::{get_language_icon, CodeBlockBorderKind, CodeBlockTheme, MarkdownElement};
use super::render_blockquote::{blockquote_prefix_width, create_blockquote_prefix};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

pub fn render_header(
    _element: &MarkdownElement,
    language: &str,
    width: usize,
    theme: CodeBlockTheme,
    blockquote_depth: usize,
) -> Line<'static> {
    let colors = theme.colors();
    let icon = get_language_icon(language);
    let lang_display = if language.is_empty() {
        "text"
    } else {
        language
    };

    // Account for blockquote prefix in width
    let bq_width = blockquote_prefix_width(blockquote_depth);
    let effective_width = width.saturating_sub(bq_width);

    // Format: ╭─ icon language ─────────────────╮
    let header_text = format!(" {} ", lang_display);
    let header_len = icon.chars().count() + header_text.chars().count();
    let remaining = effective_width.saturating_sub(header_len + 4); // 4 for ╭─ and ─╮

    let border_style = Style::default().fg(colors.border);
    let header_style = Style::default()
        .fg(colors.header_text)
        .bg(colors.header_bg);
    let icon_style = Style::default()
        .fg(colors.icon)
        .bg(colors.header_bg)
        .add_modifier(Modifier::BOLD);

    // Create header with background only on icon and text, not on border dashes
    let dashes = "─".repeat(remaining);

    let mut spans = create_blockquote_prefix(blockquote_depth);
    spans.extend(vec![
        Span::styled("╭─ ", border_style),
        Span::styled(icon.to_string(), icon_style),
        Span::styled(header_text, header_style),
        Span::styled(dashes, border_style),
        Span::styled("╮", border_style),
    ]);

    Line::from(spans)
}

pub fn render_content(
    _element: &MarkdownElement,
    content: &str,
    highlighted: Option<&ratatui::text::Text<'static>>,
    width: usize,
    line_number: Option<usize>,
    theme: CodeBlockTheme,
    blockquote_depth: usize,
) -> Line<'static> {
    let colors = theme.colors();
    let border_style = Style::default().fg(colors.border);
    let line_num_style = Style::default().fg(colors.line_number).bg(colors.background);
    let bg_style = Style::default().bg(colors.background);

    // Account for blockquote prefix in width
    let bq_width = blockquote_prefix_width(blockquote_depth);
    let effective_width = width.saturating_sub(bq_width);

    // Calculate line number width and format (minimal: " 1 ")
    let (line_num_str, line_num_width) = if let Some(num) = line_number {
        let s = format!("{:2} ", num);
        let w = s.chars().count();
        (s, w)
    } else {
        (" ".to_string(), 1)
    };

    let inner_width = effective_width.saturating_sub(3 + line_num_width); // 1 for "│" left, 2 for " │" right

    let mut all_spans = create_blockquote_prefix(blockquote_depth);

    if let Some(highlighted_text) = highlighted {
        let spans: Vec<Span<'static>> = highlighted_text
            .lines
            .iter()
            .flat_map(|line| line.spans.clone())
            .collect();

        let total_width: usize = spans.iter().map(|s| s.content.chars().count()).sum();
        let padding = inner_width.saturating_sub(total_width);

        let content_spans: Vec<Span<'static>> = spans
            .into_iter()
            .map(|mut span| {
                span.style = span.style.bg(colors.background);
                span
            })
            .collect();

        all_spans.push(Span::styled("│", border_style));
        all_spans.push(Span::styled(line_num_str, line_num_style));
        all_spans.extend(content_spans);

        if padding > 0 {
            all_spans.push(Span::styled(" ".repeat(padding), bg_style));
        }

        all_spans.push(Span::styled(" │", border_style));

        Line::from(all_spans)
    } else {
        let padded = if content.chars().count() < inner_width {
            format!(
                "{}{}",
                content,
                " ".repeat(inner_width.saturating_sub(content.chars().count()))
            )
        } else {
            content.chars().take(inner_width).collect()
        };

        // Use a light green for unhighlighted code
        let code_style = Style::default()
            .fg(colors.header_text)
            .bg(colors.background);

        all_spans.extend(vec![
            Span::styled("│", border_style),
            Span::styled(line_num_str, line_num_style),
            Span::styled(padded, code_style),
            Span::styled(" │", border_style),
        ]);

        Line::from(all_spans)
    }
}

pub fn render_border(
    _element: &MarkdownElement,
    kind: &CodeBlockBorderKind,
    width: usize,
    theme: CodeBlockTheme,
    blockquote_depth: usize,
) -> Line<'static> {
    let colors = theme.colors();

    // Account for blockquote prefix in width
    let bq_width = blockquote_prefix_width(blockquote_depth);
    let effective_width = width.saturating_sub(bq_width);
    let inner_width = effective_width.saturating_sub(2);

    let border_style = Style::default().fg(colors.border);

    let content = match kind {
        CodeBlockBorderKind::Top => {
            // This is now handled by render_header, so just return empty or minimal
            format!("╭{}╮", "─".repeat(inner_width))
        }
        CodeBlockBorderKind::HeaderSeparator => {
            // No separator needed in GitHub style - header flows into content
            format!("│{}│", " ".repeat(inner_width))
        }
        CodeBlockBorderKind::Bottom => {
            format!("╰{}╯", "─".repeat(inner_width))
        }
    };

    let mut spans = create_blockquote_prefix(blockquote_depth);
    spans.push(Span::styled(content, border_style));

    Line::from(spans)
}
