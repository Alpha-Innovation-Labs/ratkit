//! Main render implementation for MarkdownElement.

use crate::markdown_widget::foundation::elements::constants::CodeBlockTheme;
use crate::markdown_widget::foundation::elements::enums::ElementKind;
use crate::markdown_widget::foundation::elements::methods::render_blockquote;
use crate::markdown_widget::foundation::elements::methods::render_code_block;
use crate::markdown_widget::foundation::elements::methods::render_expandable;
use crate::markdown_widget::foundation::elements::methods::render_frontmatter;
use crate::markdown_widget::foundation::elements::methods::render_heading;
use crate::markdown_widget::foundation::elements::methods::render_horizontal_rule;
use crate::markdown_widget::foundation::elements::methods::render_list_item;
use crate::markdown_widget::foundation::elements::methods::render_paragraph;
use crate::markdown_widget::foundation::elements::methods::render_table_border;
use crate::markdown_widget::foundation::elements::methods::render_table_row;
use crate::markdown_widget::foundation::elements::MarkdownElement;
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
        } => render_heading::render(element, *level, text, *collapsed, width, options.app_theme),
        ElementKind::HeadingBorder { level } => {
            vec![render_heading::render_border(element, *level, width, options.app_theme)]
        }
        ElementKind::CodeBlockHeader {
            language,
            blockquote_depth,
        } => {
            vec![render_code_block::render_header(
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
            vec![render_code_block::render_content(
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
            vec![render_code_block::render_border(
                element,
                kind,
                width,
                options.theme,
                *blockquote_depth,
            )]
        }
        ElementKind::Paragraph(segments) => render_paragraph::render(element, segments, width, options.app_theme),
        ElementKind::ListItem {
            depth,
            ordered,
            number,
            content,
        } => render_list_item::render(element, *depth, *ordered, *number, content, width, options.app_theme),
        ElementKind::Blockquote { content, depth } => {
            render_blockquote::render(element, content, *depth, width, options.app_theme)
        }
        ElementKind::TableRow {
            cells, is_header, ..
        } => {
            vec![render_table_row::render(element, cells, *is_header)]
        }
        ElementKind::TableBorder(kind) => {
            vec![render_table_border::render(element, kind)]
        }
        ElementKind::HorizontalRule => {
            vec![render_horizontal_rule::render(element, width, options.app_theme)]
        }
        ElementKind::Empty => {
            // Use a space so the line can receive highlight styling
            vec![Line::from(" ")]
        }
        ElementKind::Frontmatter { fields, collapsed } => {
            render_frontmatter::render(element, fields, *collapsed, width)
        }
        ElementKind::FrontmatterStart {
            collapsed,
            context_id,
        } => {
            vec![render_frontmatter::render_start(
                *collapsed,
                context_id.as_deref(),
                width,
            )]
        }
        ElementKind::FrontmatterField { key, value } => {
            render_frontmatter::render_field(key, value, width)
        }
        ElementKind::FrontmatterEnd => {
            vec![render_frontmatter::render_end(width)]
        }
        ElementKind::Expandable {
            content_id,
            lines,
            max_lines,
            collapsed,
            total_lines,
        } => render_expandable::render_expandable(
            element,
            content_id,
            lines,
            *max_lines,
            *collapsed,
            *total_lines,
            width,
            options.app_theme,
        ),
        ElementKind::ExpandToggle {
            content_id,
            expanded,
            hidden_count,
        } => render_expandable::render_expand_toggle(
            element,
            content_id,
            *expanded,
            *hidden_count,
            width,
            options.app_theme,
        ),
    }
}
