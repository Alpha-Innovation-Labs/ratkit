//! Configuration for TOC appearance.

use ratatui::style::Style;

/// TOC visual style mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TocStyle {
    /// Standard style with border and text styling.
    #[default]
    Normal,
    /// Clerk-style with accent left border indicator.
    Clerk,
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
    /// Visual style mode for the TOC.
    pub style: TocStyle,
    /// Style for normal heading text.
    pub text_style: Style,
    /// Style for the current/active heading.
    pub active_style: Style,
    /// Style for hovered heading.
    pub hover_style: Style,
    /// Style for the accent bar in clerk mode (left border indicator).
    pub accent_style: Style,
    /// Style for the active accent bar in clerk mode.
    pub active_accent_style: Style,
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
