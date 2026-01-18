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
