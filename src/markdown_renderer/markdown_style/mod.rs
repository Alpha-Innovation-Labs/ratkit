//! Configuration for markdown rendering styles.
//!
//! This module provides the `MarkdownStyle` struct which configures
//! the visual appearance of markdown elements including headings,
//! code blocks, lists, and more.

mod traits;

use ratatui::style::Color;

/// Configuration for markdown rendering styles.
///
/// This struct allows customization of all visual aspects of markdown
/// rendering, including heading icons and colors, bullet point styles,
/// code block appearance, and text colors.
///
/// # Example
///
/// ```rust
/// use ratatui_toolkit::markdown_renderer::MarkdownStyle;
///
/// // Use default styling
/// let style = MarkdownStyle::default();
/// ```
#[derive(Clone)]
pub struct MarkdownStyle {
    /// Icon displayed before H1 headings (e.g., "󰲡 ").
    pub h1_icon: &'static str,
    /// Icon displayed before H2 headings (e.g., "󰲣 ").
    pub h2_icon: &'static str,
    /// Icon displayed before H3 headings (e.g., "󰲥 ").
    pub h3_icon: &'static str,
    /// Icon displayed before H4 headings (e.g., "󰲧 ").
    pub h4_icon: &'static str,
    /// Icon displayed before H5 headings (e.g., "󰲩 ").
    pub h5_icon: &'static str,
    /// Icon displayed before H6 headings (e.g., "󰲫 ").
    pub h6_icon: &'static str,

    /// Foreground color for H1 headings.
    pub h1_fg: Color,
    /// Background color for H1 headings.
    pub h1_bg: Color,
    /// Foreground color for H2 headings.
    pub h2_fg: Color,
    /// Background color for H2 headings.
    pub h2_bg: Color,
    /// Foreground color for H3 headings.
    pub h3_fg: Color,
    /// Background color for H3 headings.
    pub h3_bg: Color,
    /// Foreground color for H4 headings.
    pub h4_fg: Color,
    /// Background color for H4 headings.
    pub h4_bg: Color,
    /// Foreground color for H5 headings.
    pub h5_fg: Color,
    /// Background color for H5 headings.
    pub h5_bg: Color,
    /// Foreground color for H6 headings.
    pub h6_fg: Color,
    /// Background color for H6 headings.
    pub h6_bg: Color,

    /// Bullet character for level 1 list items (e.g., "● ").
    pub bullet_l1: &'static str,
    /// Bullet character for level 2 list items (e.g., "○ ").
    pub bullet_l2: &'static str,
    /// Bullet character for level 3 list items (e.g., "◆ ").
    pub bullet_l3: &'static str,

    /// Whether to show a border around code blocks.
    pub code_block_border: bool,
    /// Background color for code blocks.
    pub code_block_bg: Color,
    /// Background color for inline code spans.
    pub inline_code_bg: Color,
    /// Foreground color for inline code spans.
    pub inline_code_fg: Color,

    /// Icon displayed at the start of blockquotes (e.g., "▐ ").
    pub quote_icon: &'static str,
    /// Foreground color for blockquote text.
    pub quote_fg: Color,
    /// Background color for blockquotes.
    pub quote_bg: Color,

    /// Icon for note callouts (e.g., "󰋽 ").
    pub callout_note_icon: &'static str,
    /// Icon for tip callouts (e.g., "󰌶 ").
    pub callout_tip_icon: &'static str,
    /// Icon for warning callouts (e.g., "󰀪 ").
    pub callout_warning_icon: &'static str,
    /// Icon for caution callouts (e.g., "󰳦 ").
    pub callout_caution_icon: &'static str,

    /// Default foreground color for body text.
    pub text_fg: Color,
    /// Default background color for body text.
    pub text_bg: Color,
    /// Foreground color for hyperlinks.
    pub link_fg: Color,
    /// Foreground color for emphasized (italic) text.
    pub emph_fg: Color,
    /// Foreground color for strong (bold) text.
    pub strong_fg: Color,
    /// Foreground color for horizontal rules.
    pub hr_fg: Color,
    /// Foreground color for table borders.
    pub table_border_fg: Color,
}
