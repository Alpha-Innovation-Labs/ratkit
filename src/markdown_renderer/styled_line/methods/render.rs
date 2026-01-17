//! Main render implementation for StyledLine.

use super::super::{CodeBlockTheme, StyledLine, StyledLineKind};
use super::render_blockquote;
use super::render_code_block;
use super::render_expandable;
use super::render_frontmatter;
use super::render_heading;
use super::render_horizontal_rule;
use super::render_list_item;
use super::render_paragraph;
use super::render_table_border;
use super::render_table_row;
use ratatui::text::Line;

/// Render options for styled lines
#[derive(Debug, Clone, Copy, Default)]
pub struct RenderOptions {
    /// Whether to show line numbers in code blocks
    pub show_line_numbers: bool,
    /// Color theme for code blocks
    pub theme: CodeBlockTheme,
}

/// Render a styled line to ratatui Line with given width.
pub fn render(styled_line: &StyledLine, width: usize) -> Vec<Line<'static>> {
    render_with_options(styled_line, width, RenderOptions::default())
}

/// Render a styled line with options.
pub fn render_with_options(
    styled_line: &StyledLine,
    width: usize,
    options: RenderOptions,
) -> Vec<Line<'static>> {
    match &styled_line.kind {
        StyledLineKind::Heading {
            level,
            text,
            collapsed,
            ..
        } => render_heading::render(styled_line, *level, text, *collapsed, width),
        StyledLineKind::HeadingBorder { level } => {
            vec![render_heading::render_border(styled_line, *level, width)]
        }
        StyledLineKind::CodeBlockHeader {
            language,
            blockquote_depth,
        } => {
            vec![render_code_block::render_header(
                styled_line,
                language,
                width,
                options.theme,
                *blockquote_depth,
            )]
        }
        StyledLineKind::CodeBlockContent {
            content,
            highlighted,
            line_number,
            blockquote_depth,
        } => {
            vec![render_code_block::render_content(
                styled_line,
                content,
                highlighted.as_ref(),
                width,
                if options.show_line_numbers {
                    Some(*line_number)
                } else {
                    None
                },
                options.theme,
                *blockquote_depth,
            )]
        }
        StyledLineKind::CodeBlockBorder {
            kind,
            blockquote_depth,
        } => {
            vec![render_code_block::render_border(
                styled_line,
                kind,
                width,
                options.theme,
                *blockquote_depth,
            )]
        }
        StyledLineKind::Paragraph(segments) => {
            render_paragraph::render(styled_line, segments, width)
        }
        StyledLineKind::ListItem {
            depth,
            ordered,
            number,
            content,
        } => render_list_item::render(styled_line, *depth, *ordered, *number, content, width),
        StyledLineKind::Blockquote { content, depth } => {
            render_blockquote::render(styled_line, content, *depth, width)
        }
        StyledLineKind::TableRow {
            cells, is_header, ..
        } => {
            vec![render_table_row::render(styled_line, cells, *is_header)]
        }
        StyledLineKind::TableBorder(kind) => {
            vec![render_table_border::render(styled_line, kind)]
        }
        StyledLineKind::HorizontalRule => {
            vec![render_horizontal_rule::render(styled_line, width)]
        }
        StyledLineKind::Empty => {
            // Use a space so the line can receive highlight styling
            vec![Line::from(" ")]
        }
        StyledLineKind::Frontmatter { fields, collapsed } => {
            render_frontmatter::render(styled_line, fields, *collapsed, width)
        }
        StyledLineKind::FrontmatterStart {
            collapsed,
            context_id,
        } => {
            vec![render_frontmatter::render_start(
                *collapsed,
                context_id.as_deref(),
                width,
            )]
        }
        StyledLineKind::FrontmatterField { key, value } => {
            render_frontmatter::render_field(key, value, width)
        }
        StyledLineKind::FrontmatterEnd => {
            vec![render_frontmatter::render_end(width)]
        }
        StyledLineKind::Expandable {
            content_id,
            lines,
            max_lines,
            collapsed,
            total_lines,
        } => render_expandable::render_expandable(
            styled_line,
            content_id,
            lines,
            *max_lines,
            *collapsed,
            *total_lines,
            width,
        ),
        StyledLineKind::ExpandToggle {
            content_id,
            expanded,
            hidden_count,
        } => render_expandable::render_expand_toggle(
            styled_line,
            content_id,
            *expanded,
            *hidden_count,
            width,
        ),
    }
}
