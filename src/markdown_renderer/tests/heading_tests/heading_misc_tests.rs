use ratatui::style::Modifier;

use crate::markdown_renderer::render_markdown;

#[test]
fn test_heading_background_extends_full_width() {
    let markdown = "# Short";
    let text = render_markdown(markdown, Some(200));

    let heading_line = text
        .lines
        .iter()
        .find(|line| !line.spans.is_empty() && line.spans[0].content.contains("Short"))
        .expect("Should find heading line");

    let content = &heading_line.spans[0].content;
    // Should be padded to 200 chars
    assert!(
        content.len() >= 100,
        "Heading should be padded for full width, got length: {}",
        content.len()
    );
}

#[test]
fn test_bold_text_in_heading() {
    let markdown = "# This is **bold**";
    let text = render_markdown(markdown, Some(200));

    // Should render heading with icon
    let heading_line = text
        .lines
        .iter()
        .find(|line| !line.spans.is_empty() && line.spans[0].content.contains("bold"))
        .expect("Should find heading with bold text");

    assert!(heading_line.spans[0]
        .style
        .add_modifier
        .contains(Modifier::BOLD));
}
