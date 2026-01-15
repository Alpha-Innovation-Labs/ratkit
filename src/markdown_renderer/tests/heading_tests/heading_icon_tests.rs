use crate::markdown_renderer::render_markdown;

#[test]
fn test_h1_has_correct_icon() {
    let markdown = "# Test Heading";
    let text = render_markdown(markdown, Some(200));

    // Find the heading line (skip empty line)
    let heading_line = text
        .lines
        .iter()
        .find(|line| !line.spans.is_empty() && line.spans[0].content.contains("Test Heading"))
        .expect("Should find heading line");

    let content = &heading_line.spans[0].content;
    assert!(
        content.starts_with("󰲡 "),
        "H1 should start with icon '󰲡 ', got: {}",
        content
    );
}

#[test]
fn test_h2_has_correct_icon() {
    let markdown = "## Test Heading";
    let text = render_markdown(markdown, Some(200));

    let heading_line = text
        .lines
        .iter()
        .find(|line| !line.spans.is_empty() && line.spans[0].content.contains("Test Heading"))
        .expect("Should find heading line");

    let content = &heading_line.spans[0].content;
    assert!(
        content.starts_with("󰲣 "),
        "H2 should start with icon '󰲣 ', got: {}",
        content
    );
}

#[test]
fn test_h3_has_correct_icon() {
    let markdown = "### Test Heading";
    let text = render_markdown(markdown, Some(200));

    let heading_line = text
        .lines
        .iter()
        .find(|line| !line.spans.is_empty() && line.spans[0].content.contains("Test Heading"))
        .expect("Should find heading line");

    let content = &heading_line.spans[0].content;
    assert!(
        content.starts_with("󰲥 "),
        "H3 should start with icon '󰲥 ', got: {}",
        content
    );
}
