use crate::markdown_renderer::render_markdown;

fn find_line_with_text<'a>(
    text: &'a ratatui::text::Text<'a>,
    search_text: &str,
) -> Option<&'a ratatui::text::Line<'a>> {
    text.lines.iter().find(|line| {
        line.spans
            .iter()
            .any(|span| span.content.contains(search_text))
    })
}

#[test]
fn test_h1_has_correct_icon() {
    let markdown = "# Test Heading";
    let text = render_markdown(markdown, Some(200));

    let heading_line =
        find_line_with_text(&text, "Test Heading").expect("Should find heading line");

    // Span 0: indent (empty for H1), Span 1: collapse indicator, Span 2: icon
    let collapse_indicator = &heading_line.spans[1].content;
    let icon = &heading_line.spans[2].content;

    assert!(
        collapse_indicator == "▼" || collapse_indicator == "▶",
        "H1 should have collapse indicator, got: {}",
        collapse_indicator
    );
    assert!(icon.contains("󰲡"), "H1 should have icon '󰲡', got: {}", icon);
}

#[test]
fn test_h2_has_correct_icon() {
    let markdown = "## Test Heading";
    let text = render_markdown(markdown, Some(200));

    let heading_line =
        find_line_with_text(&text, "Test Heading").expect("Should find heading line");

    // Span 0: indent, Span 1: collapse indicator, Span 2: icon
    let collapse_indicator = &heading_line.spans[1].content;
    let icon = &heading_line.spans[2].content;

    assert!(
        collapse_indicator == "▼" || collapse_indicator == "▶",
        "H2 should have collapse indicator, got: {}",
        collapse_indicator
    );
    assert!(icon.contains("󰲣"), "H2 should have icon '󰲣', got: {}", icon);
}

#[test]
fn test_h3_has_correct_icon() {
    let markdown = "### Test Heading";
    let text = render_markdown(markdown, Some(200));

    let heading_line =
        find_line_with_text(&text, "Test Heading").expect("Should find heading line");

    // Span 0: indent, Span 1: collapse indicator, Span 2: icon
    let collapse_indicator = &heading_line.spans[1].content;
    let icon = &heading_line.spans[2].content;

    assert!(
        collapse_indicator == "▼" || collapse_indicator == "▶",
        "H3 should have collapse indicator, got: {}",
        collapse_indicator
    );
    assert!(icon.contains("󰲥"), "H3 should have icon '󰲥', got: {}", icon);
}
