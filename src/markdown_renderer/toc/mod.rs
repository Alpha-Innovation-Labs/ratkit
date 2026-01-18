//! Table of Contents widget for markdown document navigation.
//!
//! Provides a compact or expanded view of document headings for quick navigation.
//! In compact mode, shows heading indicators as horizontal lines.
//! In expanded mode, shows full heading text with indentation.
//!
//! # Features
//!
//! - Compact mode: horizontal lines indicating heading positions and levels
//! - Expanded mode: full heading text with hierarchy indentation
//! - Current heading highlight (blue in expanded, bright in compact)
//! - Hover highlight for items
//! - Click-to-scroll navigation

mod constructors;
mod enums;
mod methods;
mod traits;

#[cfg(test)]
mod tests;

pub use constructors::*;

use ratatui::style::Style;

/// A single entry in the table of contents.
#[derive(Debug, Clone)]
pub struct TocEntry {
    /// The heading text.
    pub text: String,
    /// The heading level (1-6).
    pub level: u8,
    /// The line number in the document (for scrolling).
    pub line_number: usize,
    /// The section ID for collapse state tracking.
    pub section_id: usize,
}

/// Configuration for TOC appearance.
#[derive(Debug, Clone)]
pub struct TocConfig {
    /// Width of the TOC in compact mode.
    pub compact_width: u16,
    /// Width of the TOC in expanded mode.
    pub expanded_width: u16,
    /// Height of the TOC.
    pub height: u16,
    /// Style for normal heading text.
    pub text_style: Style,
    /// Style for the current/active heading.
    pub active_style: Style,
    /// Style for hovered heading.
    pub hover_style: Style,
    /// Background style for the TOC panel.
    pub background_style: Style,
    /// Style for the compact mode lines.
    pub line_style: Style,
    /// Style for the active line in compact mode.
    pub active_line_style: Style,
    /// Whether to show a border around the TOC (only in expanded mode).
    pub show_border: bool,
    /// Style for the border.
    pub border_style: Style,
    /// Style for the title text in the border.
    pub title_style: Style,
    /// Title text to show in the border header.
    pub title: String,
    /// Spacing between lines in compact mode (in 1/8 cell units).
    /// 1 = tightest (8 lines per row), 8 = one line per row.
    pub line_spacing: u8,
}

/// Table of Contents widget for markdown navigation.
///
/// Shows document headings in either compact (lines) or expanded (text) mode.
/// Supports hover interactions and click-to-scroll navigation.
#[derive(Debug)]
pub struct Toc<'a> {
    /// The markdown content to extract headings from.
    pub(crate) content: &'a str,
    /// Extracted heading entries.
    pub(crate) entries: Vec<TocEntry>,
    /// Index of the currently active heading (in viewport).
    pub(crate) active_index: Option<usize>,
    /// Index of the currently hovered heading.
    pub(crate) hovered_index: Option<usize>,
    /// Whether the TOC is in expanded mode.
    pub(crate) expanded: bool,
    /// Configuration for appearance.
    pub(crate) config: TocConfig,
    /// Current scroll offset of the document.
    pub(crate) scroll_offset: usize,
    /// Total lines in the document.
    pub(crate) total_lines: usize,
    /// Scroll offset for the TOC list itself (when expanded).
    pub(crate) toc_scroll_offset: usize,
}
