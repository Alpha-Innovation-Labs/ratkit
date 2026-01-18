//! Tests for the TOC module.

use super::*;
use ratatui::layout::Rect;

/// Create a TocConfig with border disabled for position tests.
fn config_no_border() -> TocConfig {
    TocConfig::default().show_border(false)
}

const SAMPLE_MARKDOWN: &str = r#"# Main Title

Some intro text.

## Section 1

Content for section 1.

### Subsection 1.1

More content.

### Subsection 1.2

Even more content.

## Section 2

Content for section 2.

### Subsection 2.1

Final content.

## Section 3

Last section.
"#;

#[test]
fn test_toc_extracts_headings() {
    let toc = Toc::new(SAMPLE_MARKDOWN);

    // 7 headings: Main Title, Section 1, Subsection 1.1, Subsection 1.2, Section 2, Subsection 2.1, Section 3
    assert_eq!(toc.entry_count(), 7);

    // Check first entry
    let entries = toc.entries();
    assert_eq!(entries[0].text, "Main Title");
    assert_eq!(entries[0].level, 1);

    // Check a level 2 heading
    assert_eq!(entries[1].text, "Section 1");
    assert_eq!(entries[1].level, 2);

    // Check a level 3 heading
    assert_eq!(entries[2].text, "Subsection 1.1");
    assert_eq!(entries[2].level, 3);
}

#[test]
fn test_toc_scroll_offset_affects_entry_at_position() {
    let area = Rect::new(0, 0, 30, 5);

    // Without scroll offset, position 0 should be entry 0
    // Use config with no border to test position logic directly
    let toc = Toc::new(SAMPLE_MARKDOWN)
        .config(config_no_border())
        .expanded(true)
        .toc_scroll(0);

    assert_eq!(toc.entry_at_position(5, 0, area), Some(0));
    assert_eq!(toc.entry_at_position(5, 1, area), Some(1));
    assert_eq!(toc.entry_at_position(5, 2, area), Some(2));

    // With scroll offset of 2, position 0 should be entry 2
    let toc_scrolled = Toc::new(SAMPLE_MARKDOWN)
        .config(config_no_border())
        .expanded(true)
        .toc_scroll(2);

    assert_eq!(toc_scrolled.entry_at_position(5, 0, area), Some(2));
    assert_eq!(toc_scrolled.entry_at_position(5, 1, area), Some(3));
    assert_eq!(toc_scrolled.entry_at_position(5, 2, area), Some(4));
}

#[test]
fn test_toc_click_to_line() {
    let toc = Toc::new(SAMPLE_MARKDOWN);

    // Entry 0 (Main Title) should be at line 1
    assert_eq!(toc.click_to_line(0), Some(1));

    // Other entries should have their correct line numbers
    let entries = toc.entries();
    for (i, entry) in entries.iter().enumerate() {
        assert_eq!(toc.click_to_line(i), Some(entry.line_number));
    }

    // Out of bounds index should return None
    assert_eq!(toc.click_to_line(100), None);
}

#[test]
fn test_toc_entry_at_position_respects_scroll_offset_compact() {
    let area = Rect::new(0, 0, 12, 10);

    // Compact mode - 1 entry per row, with border (content starts at y=1)
    let toc = Toc::new(SAMPLE_MARKDOWN).expanded(false);

    // With border: y=0 is border, y=1 is first content row (entry 0)
    assert_eq!(toc.entry_at_position(5, 1, area), Some(0));
    assert_eq!(toc.entry_at_position(5, 2, area), Some(1));
}

#[test]
fn test_toc_entry_at_position_outside_area_returns_none() {
    let area = Rect::new(10, 10, 20, 5);

    let toc = Toc::new(SAMPLE_MARKDOWN)
        .config(config_no_border())
        .expanded(true);

    // Position outside area should return None
    assert_eq!(toc.entry_at_position(0, 0, area), None); // Before x
    assert_eq!(toc.entry_at_position(50, 10, area), None); // After x
    assert_eq!(toc.entry_at_position(15, 5, area), None); // Before y
    assert_eq!(toc.entry_at_position(15, 20, area), None); // After y
}

#[test]
fn test_toc_entry_at_position_beyond_entries_returns_none() {
    let area = Rect::new(0, 0, 30, 20); // More rows than entries

    let toc = Toc::new(SAMPLE_MARKDOWN)
        .config(config_no_border())
        .expanded(true);
    let entry_count = toc.entry_count();

    // Position beyond entry count should return None
    assert_eq!(toc.entry_at_position(5, entry_count as u16, area), None);
    assert_eq!(
        toc.entry_at_position(5, (entry_count + 5) as u16, area),
        None
    );
}

#[test]
fn test_toc_config_default_values() {
    let config = TocConfig::default();

    assert_eq!(config.compact_width, 12);
    assert_eq!(config.expanded_width, 32);
    assert_eq!(config.height, 20);
}

#[test]
fn test_toc_builder_methods() {
    let toc = Toc::new(SAMPLE_MARKDOWN)
        .expanded(true)
        .viewport(10, 20, 100)
        .hovered(Some(2))
        .toc_scroll(5);

    assert!(toc.expanded);
    assert_eq!(toc.scroll_offset, 10);
    assert_eq!(toc.total_lines, 100);
    assert_eq!(toc.hovered_index, Some(2));
    assert_eq!(toc.toc_scroll_offset, 5);
}

#[test]
fn test_toc_ignores_headings_in_code_blocks() {
    let markdown_with_code = r#"# Real Heading

```
# This is not a heading
## Neither is this
```

## Another Real Heading
"#;

    let toc = Toc::new(markdown_with_code);
    assert_eq!(toc.entry_count(), 2);

    let entries = toc.entries();
    assert_eq!(entries[0].text, "Real Heading");
    assert_eq!(entries[1].text, "Another Real Heading");
}

#[test]
fn test_toc_handles_empty_content() {
    let toc = Toc::new("");
    assert_eq!(toc.entry_count(), 0);
    assert!(toc.entries().is_empty());
}

#[test]
fn test_toc_handles_no_headings() {
    let markdown = "Just some text.\n\nMore text here.\n";
    let toc = Toc::new(markdown);
    assert_eq!(toc.entry_count(), 0);
}

#[test]
fn test_toc_scroll_offset_clamping() {
    let area = Rect::new(0, 0, 30, 5);

    // With scroll offset larger than entries, should still work
    let toc = Toc::new(SAMPLE_MARKDOWN).expanded(true).toc_scroll(100); // Way beyond entries

    // All positions should return None since we've scrolled past all entries
    assert_eq!(toc.entry_at_position(5, 0, area), None);
}

#[test]
fn test_toc_entry_at_position_with_border() {
    let area = Rect::new(0, 0, 30, 10);

    // With border enabled (default), positions on the border should return None
    let toc_with_border = Toc::new(SAMPLE_MARKDOWN).expanded(true);

    // Position (5, 0) is on the top border - should return None
    assert_eq!(toc_with_border.entry_at_position(5, 0, area), None);

    // Position (0, 5) is on the left border - should return None
    assert_eq!(toc_with_border.entry_at_position(0, 5, area), None);

    // Position (5, 1) is inside content area - should return entry 0
    assert_eq!(toc_with_border.entry_at_position(5, 1, area), Some(0));

    // Position (5, 2) is inside content area - should return entry 1
    assert_eq!(toc_with_border.entry_at_position(5, 2, area), Some(1));
}

/// Markdown content with many headings to test clicking on later entries.
const MANY_HEADINGS_MARKDOWN: &str = r#"# Heading 1
## Heading 2
### Heading 3
### Heading 4
### Heading 5
## Heading 6
### Heading 7
### Heading 8
### Heading 9
### Heading 10
## Heading 11
### Heading 12
### Heading 13
### Heading 14
### Heading 15
## Heading 16
### Heading 17
### Heading 18
### Heading 19
### Heading 20
## Heading 21
### Heading 22
### Heading 23
### Heading 24
### Heading 25
## Heading 26
### Heading 27
### Heading 28
### Heading 29
### Heading 30
## Heading 31
### Heading 32
### Heading 33
"#;

#[test]
fn test_toc_extracts_many_headings() {
    let toc = Toc::new(MANY_HEADINGS_MARKDOWN);

    // Should have 33 headings
    assert_eq!(toc.entry_count(), 33);

    // Check first and last entries
    let entries = toc.entries();
    assert_eq!(entries[0].text, "Heading 1");
    assert_eq!(entries[32].text, "Heading 33");
}

#[test]
fn test_toc_click_last_entry_with_many_headings() {
    // Create a TOC area smaller than entry count to simulate capped height
    // Area height is 20, but we have 33 entries
    let area = Rect::new(0, 0, 40, 20);

    let toc = Toc::new(MANY_HEADINGS_MARKDOWN)
        .config(config_no_border())
        .expanded(true)
        .toc_scroll(0);

    // Verify entry count
    assert_eq!(toc.entry_count(), 33);

    // Click at y=0 should return entry 0
    assert_eq!(toc.entry_at_position(5, 0, area), Some(0));

    // Click at y=32 should return entry 32 (last entry)
    // This is beyond area.height (20) but should still work
    assert_eq!(toc.entry_at_position(5, 32, area), Some(32));

    // Click at y=33 should return None (beyond entries)
    assert_eq!(toc.entry_at_position(5, 33, area), None);
}

#[test]
fn test_toc_click_last_entry_with_border() {
    // Area height is 20, but we have 33 entries (35 with border)
    let area = Rect::new(0, 0, 40, 20);

    let toc = Toc::new(MANY_HEADINGS_MARKDOWN).expanded(true).toc_scroll(0);

    // Verify entry count
    assert_eq!(toc.entry_count(), 33);

    // With border: y=0 is border, y=1 is first entry
    // Click at y=1 should return entry 0
    assert_eq!(toc.entry_at_position(5, 1, area), Some(0));

    // Click at y=33 should return entry 32 (last entry)
    // y=33 with border offset 1 = relative_y 32
    assert_eq!(toc.entry_at_position(5, 33, area), Some(32));

    // Click at y=34 should return None (beyond entries)
    assert_eq!(toc.entry_at_position(5, 34, area), None);
}

#[test]
fn test_toc_click_with_scroll_offset_and_many_entries() {
    let area = Rect::new(0, 0, 40, 20);

    // Scroll to show later entries
    let toc = Toc::new(MANY_HEADINGS_MARKDOWN)
        .config(config_no_border())
        .expanded(true)
        .toc_scroll(20); // Scroll down 20 entries

    // Click at y=0 should return entry 20 (scroll_offset + relative_y)
    assert_eq!(toc.entry_at_position(5, 0, area), Some(20));

    // Click at y=12 should return entry 32 (20 + 12)
    assert_eq!(toc.entry_at_position(5, 12, area), Some(32));

    // Click at y=13 should return None (20 + 13 = 33, beyond entries)
    assert_eq!(toc.entry_at_position(5, 13, area), None);
}

#[test]
fn test_toc_click_with_nonzero_area_position() {
    // Simulate a real scenario where TOC area is positioned on the right side of screen
    // TOC starts at (60, 5) with width 30 and height 25
    let area = Rect::new(60, 5, 30, 25);

    let toc = Toc::new(MANY_HEADINGS_MARKDOWN)
        .config(config_no_border())
        .expanded(true)
        .toc_scroll(0);

    assert_eq!(toc.entry_count(), 33);

    // Click at (65, 5) should return entry 0
    // relative_y = 5 - 5 = 0
    assert_eq!(toc.entry_at_position(65, 5, area), Some(0));

    // Click at (65, 10) should return entry 5
    // relative_y = 10 - 5 = 5
    assert_eq!(toc.entry_at_position(65, 10, area), Some(5));

    // Click at (65, 37) should return entry 32 (last entry)
    // relative_y = 37 - 5 = 32
    assert_eq!(toc.entry_at_position(65, 37, area), Some(32));

    // Click at (65, 38) should return None
    // relative_y = 38 - 5 = 33, which is >= entries.len()
    assert_eq!(toc.entry_at_position(65, 38, area), None);
}

#[test]
fn test_toc_click_with_nonzero_area_and_border() {
    // Simulate a real scenario with border
    let area = Rect::new(60, 5, 30, 25);

    let toc = Toc::new(MANY_HEADINGS_MARKDOWN).expanded(true).toc_scroll(0);

    assert_eq!(toc.entry_count(), 33);

    // With border: content starts at y = area.y + 1 = 6
    // Click at (65, 5) is on border - should return None
    assert_eq!(toc.entry_at_position(65, 5, area), None);

    // Click at (65, 6) should return entry 0
    // relative_y = 6 - 6 = 0
    assert_eq!(toc.entry_at_position(65, 6, area), Some(0));

    // Click at (65, 11) should return entry 5
    // relative_y = 11 - 6 = 5
    assert_eq!(toc.entry_at_position(65, 11, area), Some(5));

    // Click at (65, 38) should return entry 32 (last entry)
    // relative_y = 38 - 6 = 32
    assert_eq!(toc.entry_at_position(65, 38, area), Some(32));

    // Click at (65, 39) should return None
    // relative_y = 39 - 6 = 33, which is >= entries.len()
    assert_eq!(toc.entry_at_position(65, 39, area), None);
}

/// This is the exact markdown content from markdown_demo_full.md
/// Used to test the actual showcase scenario.
const SHOWCASE_MARKDOWN: &str = include_str!("../../../examples/markdown_demo_full.md");

#[test]
fn test_toc_extracts_showcase_headings_correctly() {
    let toc = Toc::new(SHOWCASE_MARKDOWN);
    let entries = toc.entries();

    // Count headings excluding those in code blocks
    // Expected: 33 headings (# Example usage at line 66 is inside Python code block)
    assert_eq!(toc.entry_count(), 33, "Unexpected heading count");

    // Verify key entries
    assert_eq!(entries[0].text, "asd"); // First heading
    assert_eq!(entries[3].text, "Markdown Renderer Showcase");

    // Verify "Nested Blockquotes" is at index 23
    assert_eq!(entries[23].text, "Nested Blockquotes");

    // Verify "Heading Level 6" is at index 32 (last entry)
    assert_eq!(entries[32].text, "Heading Level 6");
}

#[test]
fn test_toc_click_showcase_last_entries() {
    // Test clicking on entries after "Nested Blockquotes" in the showcase
    let area = Rect::new(60, 2, 40, 20);

    let toc = Toc::new(SHOWCASE_MARKDOWN).expanded(true).toc_scroll(0);

    assert_eq!(toc.entry_count(), 33);

    // With border, content starts at y = area.y + 1 = 3
    // Click at y=26 (relative_y = 23) should return "Nested Blockquotes" (index 23)
    assert_eq!(toc.entry_at_position(65, 26, area), Some(23));

    // Click at y=27 (relative_y = 24) should return "Code in Quote" (index 24)
    assert_eq!(toc.entry_at_position(65, 27, area), Some(24));

    // Click at y=35 (relative_y = 32) should return "Heading Level 6" (index 32)
    assert_eq!(toc.entry_at_position(65, 35, area), Some(32));

    // Verify the entry at 32 is "Heading Level 6"
    let entries = toc.entries();
    assert_eq!(entries[32].text, "Heading Level 6");
}
