//! Tests for markdown widget.

use crate::markdown_renderer::markdown_elements::MarkdownElement;
use crate::markdown_renderer::{
    markdown_elements::ElementKind, render_markdown_to_elements,
    scroll_manager::MarkdownScrollManager, TextSegment,
};
use ratatui::layout::Rect;

use super::helpers::{handle_click, is_in_area, should_render_line};

const TEST_WIDTH: usize = 80;

#[test]
fn test_should_render_heading_when_not_collapsed() {
    let scroll = MarkdownScrollManager::new();
    let element = MarkdownElement {
        kind: ElementKind::Heading {
            level: 1,
            text: vec![TextSegment::Plain("Test".to_string())],
            section_id: 1,
            collapsed: false,
        },
        section_id: None, // Headings don't have a parent section
        source_line: 1,
    };
    assert!(should_render_line(&element, 0, &scroll));
}

#[test]
fn test_should_not_render_content_when_section_collapsed() {
    let mut scroll = MarkdownScrollManager::new();
    scroll.collapse_section(1);

    // Content in section 1 should be hidden
    let element = MarkdownElement {
        kind: ElementKind::Paragraph(vec![TextSegment::Plain("Test".to_string())]),
        section_id: Some(1),
        source_line: 1,
    };
    assert!(!should_render_line(&element, 0, &scroll));
}

#[test]
fn test_heading_always_visible_even_when_collapsed() {
    let mut scroll = MarkdownScrollManager::new();
    scroll.collapse_section(1);

    // The heading itself should still be visible
    let element = MarkdownElement {
        kind: ElementKind::Heading {
            level: 1,
            text: vec![TextSegment::Plain("Test".to_string())],
            section_id: 1,
            collapsed: false,
        },
        section_id: None,
        source_line: 1,
    };
    assert!(should_render_line(&element, 0, &scroll));
}

#[test]
fn test_is_in_area() {
    let area = Rect::new(10, 5, 40, 20);
    assert!(is_in_area(15, 10, area));
    assert!(!is_in_area(5, 10, area));
    assert!(!is_in_area(55, 10, area));
}

// ==================== Header Click/Collapse Tests ====================

#[test]
fn test_clicking_heading_toggles_collapse_state() {
    let content = "# Heading 1\n\nSome paragraph content.";
    let mut scroll = MarkdownScrollManager::new();

    // Initially section 1 should not be collapsed
    assert!(!scroll.is_section_collapsed(1));

    // Click on the heading (y=0 is the first line)
    let handled = handle_click(0, 0, TEST_WIDTH, content, &mut scroll);

    assert!(handled, "Click on heading should be handled");
    assert!(
        scroll.is_section_collapsed(1),
        "Section should be collapsed after clicking heading"
    );

    // Click again to expand
    let handled = handle_click(0, 0, TEST_WIDTH, content, &mut scroll);

    assert!(handled, "Second click should also be handled");
    assert!(
        !scroll.is_section_collapsed(1),
        "Section should be expanded after second click"
    );
}

#[test]
fn test_content_hidden_when_section_collapsed() {
    let content =
        "# Heading 1\n\nParagraph under heading 1.\n\n## Heading 2\n\nParagraph under heading 2.";
    let mut scroll = MarkdownScrollManager::new();

    let elements = render_markdown_to_elements(content, true);

    // Find the paragraph under heading 1
    let paragraph_line = elements
        .iter()
        .find(|line| {
            matches!(&line.kind, ElementKind::Paragraph(segments)
                if segments.iter().any(|s| matches!(s, TextSegment::Plain(t) if t.contains("Paragraph under heading 1"))))
        })
        .expect("Should find paragraph under heading 1");

    // Should be visible initially
    assert!(
        should_render_line(paragraph_line, 0, &scroll),
        "Paragraph should be visible when section is expanded"
    );

    // Collapse section 1
    scroll.collapse_section(1);

    // Now paragraph should be hidden
    assert!(
        !should_render_line(paragraph_line, 0, &scroll),
        "Paragraph should be hidden when section is collapsed"
    );
}

#[test]
fn test_heading_remains_visible_when_collapsed() {
    let content = "# Heading 1\n\nSome content.";
    let mut scroll = MarkdownScrollManager::new();

    let elements = render_markdown_to_elements(content, true);

    // Find the heading
    let heading_line = elements
        .iter()
        .find(|line| matches!(&line.kind, ElementKind::Heading { level: 1, .. }))
        .expect("Should find H1 heading");

    // Collapse the section
    scroll.collapse_section(1);

    // Heading should still be visible
    assert!(
        should_render_line(heading_line, 0, &scroll),
        "Heading should remain visible even when its section is collapsed"
    );
}

#[test]
fn test_multiple_sections_collapse_independently() {
    let content =
        "# Section 1\n\nContent 1.\n\n# Section 2\n\nContent 2.\n\n# Section 3\n\nContent 3.";
    let mut scroll = MarkdownScrollManager::new();

    let elements = render_markdown_to_elements(content, true);

    // Find paragraphs for each section
    let find_paragraph_with_text = |text: &str| {
        elements
            .iter()
            .find(|line| {
                matches!(&line.kind, ElementKind::Paragraph(segments)
                    if segments.iter().any(|s| matches!(s, TextSegment::Plain(t) if t.contains(text))))
            })
            .expect(&format!("Should find paragraph containing '{}'", text))
    };

    let para1 = find_paragraph_with_text("Content 1");
    let para2 = find_paragraph_with_text("Content 2");
    let para3 = find_paragraph_with_text("Content 3");

    // Initially all visible
    assert!(should_render_line(para1, 0, &scroll));
    assert!(should_render_line(para2, 0, &scroll));
    assert!(should_render_line(para3, 0, &scroll));

    // Collapse only section 2
    scroll.collapse_section(2);

    // Only para2 should be hidden
    assert!(
        should_render_line(para1, 0, &scroll),
        "Content 1 should still be visible"
    );
    assert!(
        !should_render_line(para2, 0, &scroll),
        "Content 2 should be hidden"
    );
    assert!(
        should_render_line(para3, 0, &scroll),
        "Content 3 should still be visible"
    );

    // Collapse section 1 as well
    scroll.collapse_section(1);

    assert!(
        !should_render_line(para1, 0, &scroll),
        "Content 1 should now be hidden"
    );
    assert!(
        !should_render_line(para2, 0, &scroll),
        "Content 2 should still be hidden"
    );
    assert!(
        should_render_line(para3, 0, &scroll),
        "Content 3 should still be visible"
    );
}

#[test]
fn test_section_ids_assigned_correctly_to_content() {
    let content = "# Heading 1\n\nPara 1.\n\n## Heading 2\n\nPara 2.";
    let elements = render_markdown_to_elements(content, true);

    // H1 heading should have section_id: None (headings are always visible)
    let h1 = elements
        .iter()
        .find(|line| matches!(&line.kind, ElementKind::Heading { level: 1, .. }))
        .expect("Should find H1");
    assert_eq!(
        h1.section_id, None,
        "H1 heading should have section_id: None"
    );

    // Paragraph under H1 should have section_id: Some(1)
    let para1 = elements
        .iter()
        .find(|line| {
            matches!(&line.kind, ElementKind::Paragraph(segments)
                if segments.iter().any(|s| matches!(s, TextSegment::Plain(t) if t.contains("Para 1"))))
        })
        .expect("Should find Para 1");
    assert_eq!(
        para1.section_id,
        Some(1),
        "Paragraph under H1 should have section_id: Some(1)"
    );

    // H2 heading should have section_id: None
    let h2 = elements
        .iter()
        .find(|line| matches!(&line.kind, ElementKind::Heading { level: 2, .. }))
        .expect("Should find H2");
    assert_eq!(
        h2.section_id, None,
        "H2 heading should have section_id: None"
    );

    // Paragraph under H2 should have section_id: Some(2)
    let para2 = elements
        .iter()
        .find(|line| {
            matches!(&line.kind, ElementKind::Paragraph(segments)
                if segments.iter().any(|s| matches!(s, TextSegment::Plain(t) if t.contains("Para 2"))))
        })
        .expect("Should find Para 2");
    assert_eq!(
        para2.section_id,
        Some(2),
        "Paragraph under H2 should have section_id: Some(2)"
    );
}

#[test]
fn test_code_block_collapses_with_section() {
    let content = "# Code Section\n\n```rust\nfn main() {}\n```";
    let mut scroll = MarkdownScrollManager::new();

    let elements = render_markdown_to_elements(content, true);

    // Find the code block content
    let code_line = elements
        .iter()
        .find(|line| matches!(&line.kind, ElementKind::CodeBlockContent { .. }))
        .expect("Should find code block content");

    // Should be visible initially
    assert!(
        should_render_line(code_line, 0, &scroll),
        "Code block should be visible when section is expanded"
    );

    // Collapse section 1
    scroll.collapse_section(1);

    // Code block should be hidden
    assert!(
        !should_render_line(code_line, 0, &scroll),
        "Code block should be hidden when section is collapsed"
    );
}

#[test]
fn test_list_collapses_with_section() {
    let content = "# List Section\n\n- Item 1\n- Item 2\n- Item 3";
    let mut scroll = MarkdownScrollManager::new();

    let elements = render_markdown_to_elements(content, true);

    // Find a list item
    let list_item = elements
        .iter()
        .find(|line| matches!(&line.kind, ElementKind::ListItem { .. }))
        .expect("Should find list item");

    // Should be visible initially
    assert!(
        should_render_line(list_item, 0, &scroll),
        "List item should be visible when section is expanded"
    );

    // Collapse section 1
    scroll.collapse_section(1);

    // List item should be hidden
    assert!(
        !should_render_line(list_item, 0, &scroll),
        "List item should be hidden when section is collapsed"
    );
}

#[test]
fn test_expand_all_and_collapse_all() {
    let _content = "# Section 1\n\nContent 1.\n\n# Section 2\n\nContent 2.";
    let mut scroll = MarkdownScrollManager::new();

    // Collapse individual sections
    scroll.collapse_section(1);
    scroll.collapse_section(2);

    assert!(scroll.is_section_collapsed(1));
    assert!(scroll.is_section_collapsed(2));

    // Expand all
    scroll.expand_all_sections();

    assert!(
        !scroll.is_section_collapsed(1),
        "Section 1 should be expanded"
    );
    assert!(
        !scroll.is_section_collapsed(2),
        "Section 2 should be expanded"
    );

    // Collapse all
    scroll.collapse_all_sections();

    assert!(
        scroll.is_section_collapsed(1),
        "Section 1 should be collapsed"
    );
    assert!(
        scroll.is_section_collapsed(2),
        "Section 2 should be collapsed"
    );
}

#[test]
fn test_click_on_non_heading_returns_false() {
    let content = "# Heading\n\nSome paragraph text here.";
    let mut scroll = MarkdownScrollManager::new();

    // Click on a paragraph line (not the heading)
    // The heading takes line 0, empty line 1, paragraph starts at line 2
    let handled = handle_click(0, 2, TEST_WIDTH, content, &mut scroll);

    assert!(
        !handled,
        "Click on paragraph should not be handled as a collapse toggle"
    );
    assert!(
        !scroll.is_section_collapsed(1),
        "Section should not be collapsed when clicking on paragraph"
    );
}

#[test]
fn test_cache_invalidated_after_collapse_toggle() {
    let content = "# Heading\n\nContent.";
    let mut scroll = MarkdownScrollManager::new();

    // Simulate having a cache
    scroll.render_cache = Some(crate::markdown_renderer::scroll_manager::RenderCache {
        content_hash: 12345,
        width: 80,
        show_line_numbers: false,
        theme: scroll.code_block_theme,
        app_theme_hash: 0,
        lines: vec![],
        line_boundaries: vec![],
    });

    assert!(scroll.render_cache.is_some(), "Cache should exist");

    // Click on heading to toggle collapse
    handle_click(0, 0, TEST_WIDTH, content, &mut scroll);

    assert!(
        scroll.render_cache.is_none(),
        "Cache should be invalidated after toggling collapse"
    );
}
