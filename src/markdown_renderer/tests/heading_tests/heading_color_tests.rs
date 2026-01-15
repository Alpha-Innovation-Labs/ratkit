use ratatui::style::Color;

use crate::markdown_renderer::render_markdown;

#[test]
fn test_h1_has_correct_colors() {
    let markdown = "# Test";
    let text = render_markdown(markdown, Some(200));

    let heading_line = text
        .lines
        .iter()
        .find(|line| !line.spans.is_empty() && line.spans[0].content.contains("Test"))
        .expect("Should find heading line");

    let style = heading_line.spans[0].style;
    assert_eq!(
        style.fg,
        Some(Color::Rgb(255, 255, 255)),
        "H1 foreground should be white"
    );
    assert_eq!(
        style.bg,
        Some(Color::Rgb(30, 58, 138)),
        "H1 background should be deep blue"
    );
}

#[test]
fn test_h2_has_correct_colors() {
    let markdown = "## Test";
    let text = render_markdown(markdown, Some(200));

    let heading_line = text
        .lines
        .iter()
        .find(|line| !line.spans.is_empty() && line.spans[0].content.contains("Test"))
        .expect("Should find heading line");

    let style = heading_line.spans[0].style;
    assert_eq!(
        style.fg,
        Some(Color::Rgb(255, 255, 255)),
        "H2 foreground should be white"
    );
    assert_eq!(
        style.bg,
        Some(Color::Rgb(8, 145, 178)),
        "H2 background should be cyan/teal"
    );
}

#[test]
fn test_h3_has_correct_colors() {
    let markdown = "### Test";
    let text = render_markdown(markdown, Some(200));

    let heading_line = text
        .lines
        .iter()
        .find(|line| !line.spans.is_empty() && line.spans[0].content.contains("Test"))
        .expect("Should find heading line");

    let style = heading_line.spans[0].style;
    assert_eq!(
        style.fg,
        Some(Color::Rgb(255, 255, 255)),
        "H3 foreground should be white"
    );
    assert_eq!(
        style.bg,
        Some(Color::Rgb(168, 85, 247)),
        "H3 background should be purple/magenta"
    );
}
