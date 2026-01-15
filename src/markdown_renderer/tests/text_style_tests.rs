use ratatui::style::Modifier;

use crate::markdown_renderer::render_markdown;

#[test]
fn test_bold_text_in_paragraph() {
    let markdown = "This is **bold** text";
    let text = render_markdown(markdown, Some(200));

    // Find the span with "bold" text
    let has_bold = text.lines.iter().flat_map(|line| &line.spans).any(|span| {
        span.content.contains("bold") && span.style.add_modifier.contains(Modifier::BOLD)
    });

    assert!(has_bold, "Bold text should have BOLD modifier");
}

#[test]
fn test_italic_text_in_paragraph() {
    let markdown = "This is *italic* text";
    let text = render_markdown(markdown, Some(200));

    // Find the span with "italic" text
    let has_italic = text.lines.iter().flat_map(|line| &line.spans).any(|span| {
        span.content.contains("italic") && span.style.add_modifier.contains(Modifier::ITALIC)
    });

    assert!(has_italic, "Italic text should have ITALIC modifier");
}

#[test]
fn test_bold_and_italic_together() {
    let markdown = "This is ***bold and italic*** text";
    let text = render_markdown(markdown, Some(200));

    // Find the span with both modifiers
    let has_both = text.lines.iter().flat_map(|line| &line.spans).any(|span| {
        span.content.contains("bold and italic")
            && span.style.add_modifier.contains(Modifier::BOLD)
            && span.style.add_modifier.contains(Modifier::ITALIC)
    });

    assert!(has_both, "Text should have both BOLD and ITALIC modifiers");
}
