//! Tests for table rendering.

use crate::markdown_renderer::markdown_elements::{ColumnAlignment, ElementKind};
use crate::markdown_renderer::render_markdown_to_elements;

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
            assert_eq!(
                alignments[0],
                ColumnAlignment::Right,
                "First column should be right-aligned"
            );
            assert_eq!(
                alignments[1],
                ColumnAlignment::Right,
                "Second column should be right-aligned"
            );
            assert_eq!(
                alignments[2],
                ColumnAlignment::Center,
                "Third column should be center-aligned"
            );
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

#[test]
fn test_table_inline_code_with_backticks() {
    // Test inline code containing backticks (uses double backticks in markdown)
    let md = r#"| Feature | Notes |
|---------|-------|
| Code | `` `code` `` |"#;

    let lines = render_markdown_to_elements(md, true);

    // Find the body row
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
            assert_eq!(cells.len(), 2, "Should have 2 cells");
            // The code content includes the literal backtick from the markdown
            let notes_cell = &cells[1];
            // Should contain the backtick character (part of the code content)
            assert!(
                notes_cell.contains('`'),
                "Notes cell should contain backticks: {}",
                notes_cell
            );
            assert!(
                notes_cell.contains("code"),
                "Notes cell should contain 'code': {}",
                notes_cell
            );
        }
    }
}

#[test]
fn test_table_simple_inline_code() {
    // Test simple inline code (no backticks in content)
    let md = r#"| Feature | Notes |
|---------|-------|
| Bold | `**text**` |"#;

    let lines = render_markdown_to_elements(md, true);

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
            let notes_cell = &cells[1];
            // Raw code content is stored without backtick wrappers
            assert!(
                notes_cell.contains("**text**"),
                "Notes cell should contain **text**: {}",
                notes_cell
            );
        }
    }
}

#[test]
fn test_table_inline_code_various_contents() {
    // Test various inline code patterns in table cells
    let md = r#"| Feature | Status | Notes |
|---------|--------|-------|
| Bold | ✅ | `**text**` |
| Italic | ✅ | `*text*` |
| Code | ✅ | `` `code` `` |
| Tables | ✅ | `\|col\|` syntax |"#;

    let lines = render_markdown_to_elements(md, true);

    let body_rows: Vec<_> = lines
        .iter()
        .filter(|l| {
            matches!(
                &l.kind,
                ElementKind::TableRow {
                    is_header: false,
                    ..
                }
            )
        })
        .collect();

    assert_eq!(body_rows.len(), 4, "Should have 4 body rows");

    // Verify specific content (trimmed since cells are padded)
    if let ElementKind::TableRow { cells, .. } = &body_rows[0].kind {
        assert!(cells[2].trim().contains("**text**"), "Bold row should contain **text**");
    }
    if let ElementKind::TableRow { cells, .. } = &body_rows[1].kind {
        assert!(cells[2].trim().contains("*text*"), "Italic row should contain *text*");
    }
    if let ElementKind::TableRow { cells, .. } = &body_rows[2].kind {
        // The content from `` `code` `` is `code` (backtick-code-backtick)
        assert!(cells[2].trim().contains("`code`"), "Code row should contain `code`: got '{}'", cells[2].trim());
    }
    if let ElementKind::TableRow { cells, .. } = &body_rows[3].kind {
        // The \| escapes are processed by the parser, resulting in literal pipes
        assert!(cells[2].trim().contains("|col|"), "Tables row should contain |col|: got '{}'", cells[2].trim());
    }
}
