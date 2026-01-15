use crate::markdown_renderer::render_markdown;

#[test]
fn test_list_with_custom_bullets() {
    let markdown = "- Item 1\n- Item 2\n- Item 3";
    let text = render_markdown(markdown, Some(200));

    // Lists use custom bullets (○ for first level due to depth tracking)
    let has_custom_bullet = text
        .lines
        .iter()
        .any(|line| line.spans.iter().any(|span| span.content.contains("○")));

    assert!(has_custom_bullet, "List should use custom bullet '○'");

    // Verify all items are present
    assert!(text.lines.iter().any(|line| line
        .spans
        .iter()
        .any(|span| span.content.contains("Item 1"))));
    assert!(text.lines.iter().any(|line| line
        .spans
        .iter()
        .any(|span| span.content.contains("Item 2"))));
    assert!(text.lines.iter().any(|line| line
        .spans
        .iter()
        .any(|span| span.content.contains("Item 3"))));
}
