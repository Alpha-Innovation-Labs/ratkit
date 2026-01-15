use ratatui::style::Color;

use crate::markdown_renderer::render_markdown;

#[test]
fn test_code_block_with_border() {
    let markdown = "```rust\nfn main() {}\n```";
    let text = render_markdown(markdown, Some(200));

    // Should have border lines
    let has_top_border = text
        .lines
        .iter()
        .any(|line| line.spans.iter().any(|span| span.content.contains("╭")));
    let has_bottom_border = text
        .lines
        .iter()
        .any(|line| line.spans.iter().any(|span| span.content.contains("╰")));

    assert!(has_top_border, "Code block should have top border");
    assert!(has_bottom_border, "Code block should have bottom border");
}

#[test]
fn test_inline_code_styling() {
    let markdown = "This is `inline code` in text";
    let text = render_markdown(markdown, Some(200));

    // Find the inline code span
    let has_inline_code = text.lines.iter().flat_map(|line| &line.spans).any(|span| {
        span.content.contains("inline code") && span.style.bg == Some(Color::Rgb(68, 71, 90))
    });

    assert!(has_inline_code, "Inline code should have background color");
}
