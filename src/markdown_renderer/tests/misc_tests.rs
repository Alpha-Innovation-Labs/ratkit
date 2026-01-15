use crate::markdown_renderer::render_markdown;

#[test]
fn test_markdown_with_frontmatter_like_content() {
    // Test that markdown content that looks like frontmatter is rendered correctly
    let markdown = "# Heading\n\nSome content about version: 1.0";
    let text = render_markdown(markdown, Some(200));

    // Should have heading with icon
    let has_heading = text
        .lines
        .iter()
        .any(|line| line.spans.iter().any(|span| span.content.contains("󰲡")));

    assert!(has_heading, "Should render heading with icon");
}
