use crate::markdown_renderer::render_markdown;
use crate::markdown_renderer::render_markdown_to_elements;

#[test]
fn test_max_source_line_matches_file() {
    let content = include_str!("../../../examples/markdown_demo_full.md");
    let source_lines = content.lines().count();
    let elements = render_markdown_to_elements(content, true);

    // Get the maximum source line from styled lines
    let max_source_line = elements.iter().map(|l| l.source_line).max().unwrap_or(0);

    // The max source line should match the source file line count
    // (or be very close - within 1-2 lines due to trailing newlines or empty line handling)
    assert!(
        max_source_line >= source_lines - 2 && max_source_line <= source_lines + 1,
        "Expected max source line ({}) to match source lines ({})",
        max_source_line,
        source_lines
    );
}

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
