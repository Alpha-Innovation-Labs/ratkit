//! Render frontmatter.

use super::super::MarkdownElement;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

pub fn render(
    _element: &MarkdownElement,
    fields: &[(String, String)],
    collapsed: bool,
    width: usize,
) -> Vec<Line<'static>> {
    let border_style = Style::default().fg(Color::DarkGray);
    let key_style = Style::default().fg(Color::Rgb(240, 113, 120)); // Coral/red for keys (#F07178)
    let value_style = Style::default().fg(Color::Rgb(170, 217, 76)); // Green for values (#AAD94C)
    let collapse_icon_style = Style::default().fg(Color::Yellow);

    // Border character for full-width lines
    let border_char = "─";

    if collapsed {
        let context_id = fields
            .iter()
            .find(|(k, _)| k == "context_id")
            .map(|(_, v)| v.as_str())
            .unwrap_or("frontmatter");

        // Build collapsed line: "▶ ─── context_id ───────────"
        let prefix = "▶ ─── ";
        let suffix = " ";
        let used_width =
            prefix.chars().count() + context_id.chars().count() + suffix.chars().count();
        let remaining = width.saturating_sub(used_width);
        let border_fill = border_char.repeat(remaining);

        vec![Line::from(vec![
            Span::styled("▶ ", collapse_icon_style),
            Span::styled("─── ", border_style),
            Span::styled(context_id.to_string(), key_style),
            Span::styled(" ", Style::default()),
            Span::styled(border_fill, border_style),
        ])]
    } else {
        let mut lines = Vec::new();

        // Top border: "▼ ────────────────────────"
        let prefix_len = 2; // "▼ "
        let top_border = border_char.repeat(width.saturating_sub(prefix_len));
        lines.push(Line::from(vec![
            Span::styled("▼ ", collapse_icon_style),
            Span::styled(top_border, border_style),
        ]));

        for (key, value) in fields {
            let key_prefix = format!("{}: ", key);
            let key_prefix_len = key_prefix.chars().count();

            // Calculate available width for value
            let value_width = width.saturating_sub(key_prefix_len);

            if value_width == 0 || value.chars().count() <= value_width {
                // Value fits on one line
                lines.push(Line::from(vec![
                    Span::styled(key_prefix, key_style),
                    Span::styled(value.clone(), value_style),
                ]));
            } else {
                // Wrap value across multiple lines
                let wrapped = wrap_text(value, value_width);
                for (i, line_text) in wrapped.iter().enumerate() {
                    if i == 0 {
                        // First line with key
                        lines.push(Line::from(vec![
                            Span::styled(key_prefix.clone(), key_style),
                            Span::styled(line_text.clone(), value_style),
                        ]));
                    } else {
                        // Continuation lines - indent to align with value
                        let continuation_indent = " ".repeat(key_prefix_len);
                        lines.push(Line::from(vec![
                            Span::styled(continuation_indent, Style::default()),
                            Span::styled(line_text.clone(), value_style),
                        ]));
                    }
                }
            }
        }

        // Bottom border: "────────────────────────"
        let bottom_border = border_char.repeat(width);
        lines.push(Line::from(vec![Span::styled(bottom_border, border_style)]));

        lines
    }
}

/// Render frontmatter start (opening border with collapse icon).
pub fn render_start(collapsed: bool, context_id: Option<&str>, width: usize) -> Line<'static> {
    let border_style = Style::default().fg(Color::DarkGray);
    let key_style = Style::default().fg(Color::Rgb(240, 113, 120)); // Coral/red (#F07178)
    let collapse_icon_style = Style::default().fg(Color::Yellow);
    let border_char = "─";

    if collapsed {
        // Collapsed: "▶ ─── context_id ───────────"
        let ctx = context_id.unwrap_or("frontmatter");
        let prefix = "▶ ─── ";
        let suffix = " ";
        let used_width = prefix.chars().count() + ctx.chars().count() + suffix.chars().count();
        let remaining = width.saturating_sub(used_width);
        let border_fill = border_char.repeat(remaining);

        Line::from(vec![
            Span::styled("▶ ", collapse_icon_style),
            Span::styled("─── ", border_style),
            Span::styled(ctx.to_string(), key_style),
            Span::styled(" ", Style::default()),
            Span::styled(border_fill, border_style),
        ])
    } else {
        // Expanded: "▼ ────────────────────────"
        let prefix_len = 2; // "▼ "
        let top_border = border_char.repeat(width.saturating_sub(prefix_len));
        Line::from(vec![
            Span::styled("▼ ", collapse_icon_style),
            Span::styled(top_border, border_style),
        ])
    }
}

/// Render a single frontmatter field.
pub fn render_field(key: &str, value: &str, width: usize) -> Vec<Line<'static>> {
    let key_style = Style::default().fg(Color::Rgb(240, 113, 120)); // Coral/red for keys (#F07178)
    let value_style = Style::default().fg(Color::Rgb(170, 217, 76)); // Green for values (#AAD94C)

    let key_prefix = format!("{}: ", key);
    let key_prefix_len = key_prefix.chars().count();
    let value_width = width.saturating_sub(key_prefix_len);

    if value_width == 0 || value.chars().count() <= value_width {
        // Value fits on one line
        vec![Line::from(vec![
            Span::styled(key_prefix, key_style),
            Span::styled(value.to_string(), value_style),
        ])]
    } else {
        // Wrap value across multiple lines
        let wrapped = wrap_text(value, value_width);
        let mut lines = Vec::new();
        for (i, line_text) in wrapped.iter().enumerate() {
            if i == 0 {
                lines.push(Line::from(vec![
                    Span::styled(key_prefix.clone(), key_style),
                    Span::styled(line_text.clone(), value_style),
                ]));
            } else {
                let continuation_indent = " ".repeat(key_prefix_len);
                lines.push(Line::from(vec![
                    Span::styled(continuation_indent, Style::default()),
                    Span::styled(line_text.clone(), value_style),
                ]));
            }
        }
        lines
    }
}

/// Render frontmatter end (closing border).
pub fn render_end(width: usize) -> Line<'static> {
    let border_style = Style::default().fg(Color::DarkGray);
    let border_char = "─";
    let bottom_border = border_char.repeat(width);
    Line::from(vec![Span::styled(bottom_border, border_style)])
}

/// Wrap text to fit within a given width.
fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 || text.is_empty() {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for word in text.split_whitespace() {
        let word_width = word.chars().count();

        if current_width == 0 {
            current_line = word.to_string();
            current_width = word_width;
        } else if current_width + 1 + word_width <= width {
            current_line.push(' ');
            current_line.push_str(word);
            current_width += 1 + word_width;
        } else {
            lines.push(current_line);
            current_line = word.to_string();
            current_width = word_width;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}
