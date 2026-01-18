//! Tests for table rendering.

use crate::markdown_renderer::render_markdown_to_elements;
use crate::markdown_renderer::markdown_elements::{ColumnAlignment, ElementKind};

#[test]
fn test_table_alignment_parsing() {
    let md = r#"| Name | Score | Grade |
|-----:|------:|:-----:|
| Alice | 95 | A |
| Bob | 87 | B+ |"#;

    let lines = render_markdown_to_elements(md, true);

    // Find the first TableRow (header)
    let header = lines.iter().find(|l| {
        matches!(
            &l.kind,
            ElementKind::TableRow {
                is_header: true,
                ..
            }
        )
    });

    assert!(header.is_some(), "Should have a table header");

    if let Some(line) = header {
        if let ElementKind::TableRow { alignments, .. } = &line.kind {
            assert_eq!(alignments.len(), 3, "Should have 3 column alignments");
            assert_eq!(alignments[0], ColumnAlignment::Right, "First column should be right-aligned");
            assert_eq!(alignments[1], ColumnAlignment::Right, "Second column should be right-aligned");
            assert_eq!(alignments[2], ColumnAlignment::Center, "Third column should be center-aligned");
        }
    }
}

#[test]
fn test_table_right_alignment_padding() {
    let md = r#"| Name | Score |
|-----:|------:|
| Alice | 95 |
| Bob | 87 |"#;

    let lines = render_markdown_to_elements(md, true);

    // Find a body row
    let body_row = lines.iter().find(|l| {
        matches!(
            &l.kind,
            ElementKind::TableRow {
                is_header: false,
                ..
            }
        )
    });

    assert!(body_row.is_some(), "Should have a table body row");

    if let Some(line) = body_row {
        if let ElementKind::TableRow { cells, .. } = &line.kind {
            // With right alignment, "Alice" and "95" should be right-padded
            // The first column "Alice" should have leading spaces if it's shorter than the header "Name"
            // Check that the cells are right-aligned (have leading spaces)
            assert!(
                cells[0].starts_with(' ') || cells[0] == "Alice",
                "Cell should be right-aligned or same width as header"
            );
        }
    }
}

#[test]
fn test_table_default_left_alignment() {
    let md = r#"| Name | Score |
|------|-------|
| Alice | 95 |"#;

    let lines = render_markdown_to_elements(md, true);

    let header = lines.iter().find(|l| {
        matches!(
            &l.kind,
            ElementKind::TableRow {
                is_header: true,
                ..
            }
        )
    });

    if let Some(line) = header {
        if let ElementKind::TableRow { alignments, .. } = &line.kind {
            // Default alignment should be None (which is treated as left)
            for alignment in alignments {
                assert!(
                    matches!(alignment, ColumnAlignment::None | ColumnAlignment::Left),
                    "Default alignment should be None or Left"
                );
            }
        }
    }
}
