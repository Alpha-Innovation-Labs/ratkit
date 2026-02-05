//! Main render implementation for MarkdownElement.

use crate::widgets::markdown_widget::foundation::elements::blockquote;
use crate::widgets::markdown_widget::foundation::elements::code_block;
use crate::widgets::markdown_widget::foundation::elements::constants::CodeBlockTheme;
use crate::widgets::markdown_widget::foundation::elements::enums::ElementKind;
use crate::widgets::markdown_widget::foundation::elements::expandable;
use crate::widgets::markdown_widget::foundation::elements::frontmatter;
use crate::widgets::markdown_widget::foundation::elements::heading;
use crate::widgets::markdown_widget::foundation::elements::horizontal_rule;
use crate::widgets::markdown_widget::foundation::elements::list_item;
use crate::widgets::markdown_widget::foundation::elements::paragraph;
use crate::widgets::markdown_widget::foundation::elements::table;
use crate::widgets::markdown_widget::foundation::elements::MarkdownElement;
use ratatui::text::Line;

/// Render options for markdown elements
#[derive(Debug, Clone, Copy, Default)]
pub struct RenderOptions<'a> {
    /// Whether to show line numbers in code blocks
    pub show_line_numbers: bool,
    /// Color theme for code blocks
    pub theme: CodeBlockTheme,
    /// Optional application theme for consistent styling
    pub app_theme: Option<&'a crate::services::theme::AppTheme>,
    /// Whether to show collapse indicators on headings (default: false)
    pub show_heading_collapse: bool,
}

/// Render a markdown element to ratatui Line with given width.
pub fn render(element: &MarkdownElement, width: usize) -> Vec<Line<'static>> {
    render_with_options(element, width, RenderOptions::default())
}

/// Render a markdown element with options.
pub fn render_with_options(
    element: &MarkdownElement,
    width: usize,
    options: RenderOptions<'_>,
) -> Vec<Line<'static>> {
    match &element.kind {
        ElementKind::Heading {
            level,
            text,
            collapsed,
            ..
        } => heading::render(
            element,
            *level,
            text,
            *collapsed,
            width,
            options.app_theme,
            options.show_heading_collapse,
        ),
        ElementKind::HeadingBorder { level } => {
            vec![heading::render_border(
                element,
                *level,
                width,
                options.app_theme,
            )]
        }
        ElementKind::CodeBlockHeader {
            language,
            blockquote_depth,
        } => {
            vec![code_block::render_header(
                element,
                language,
                width,
                options.theme,
                *blockquote_depth,
            )]
        }
        ElementKind::CodeBlockContent {
            content,
            highlighted,
            line_number,
            blockquote_depth,
        } => {
            vec![code_block::render_content(
                element,
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
        ElementKind::CodeBlockBorder {
            kind,
            blockquote_depth,
        } => {
            vec![code_block::render_border(
                element,
                kind,
                width,
                options.theme,
                *blockquote_depth,
            )]
        }
        ElementKind::Paragraph(segments) => {
            paragraph::render(element, segments, width, options.app_theme)
        }
        ElementKind::ListItem {
            depth,
            ordered,
            number,
            content,
        } => list_item::render(
            element,
            *depth,
            *ordered,
            *number,
            content,
            width,
            options.app_theme,
        ),
        ElementKind::Blockquote { content, depth } => {
            blockquote::render(element, content, *depth, width, options.app_theme)
        }
        ElementKind::TableRow {
            cells, is_header, ..
        } => {
            vec![table::render_table_row(element, cells, *is_header)]
        }
        ElementKind::TableBorder(kind) => {
            vec![table::render_table_border(element, kind)]
        }
        ElementKind::HorizontalRule => {
            vec![horizontal_rule::render(element, width, options.app_theme)]
        }
        ElementKind::Empty => {
            // Use a space so the line can receive highlight styling
            vec![Line::from(" ")]
        }
        ElementKind::Frontmatter { fields, collapsed } => {
            frontmatter::render(element, fields, *collapsed, width)
        }
        ElementKind::FrontmatterStart {
            collapsed,
            context_id,
        } => {
            vec![frontmatter::render_start(
                *collapsed,
                context_id.as_deref(),
                width,
            )]
        }
        ElementKind::FrontmatterField { key, value } => {
            frontmatter::render_field(key, value, width)
        }
        ElementKind::FrontmatterEnd => {
            vec![frontmatter::render_end(width)]
        }
        ElementKind::Expandable {
            content_id,
            lines,
            max_lines,
            collapsed,
            total_lines: _,
        } => expandable::render_expandable(
            element,
            content_id,
            lines,
            *max_lines,
            *collapsed,
            width,
            options.app_theme,
        ),
        ElementKind::ExpandToggle {
            content_id,
            expanded,
            hidden_count,
        } => expandable::render_expand_toggle(
            element,
            content_id,
            *expanded,
            *hidden_count,
            width,
            options.app_theme,
        ),
    }
}
