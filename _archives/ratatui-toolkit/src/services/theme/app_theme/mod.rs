//! Application theme module for comprehensive TUI theming.
//!
//! This module provides [`AppTheme`] which is the main theme struct that
//! contains all colors needed for a complete TUI application, including
//! UI colors, text colors, background colors, border colors, and specialized
//! color sets for diffs, markdown, and syntax highlighting.
//!
//! # Color Categories
//!
//! The theme is organized into logical categories:
//!
//! - **UI Colors**: `primary`, `secondary`, `accent`, `error`, `warning`, `success`, `info`
//! - **Text Colors**: `text`, `text_muted`, `selected_text`
//! - **Background Colors**: `background`, `background_panel`, `background_element`, `background_menu`
//! - **Border Colors**: `border`, `border_active`, `border_subtle`
//! - **Specialized**: [`DiffColors`], [`MarkdownColors`], [`SyntaxColors`]
//!
//! # Loading Themes
//!
//! Themes can be loaded from JSON files in the opencode format using the
//! [`loader`](crate::services::theme::loader) module.
//!
//! # Example
//!
//! ```rust
//! use ratatui_toolkit::services::theme::{AppTheme, ThemeVariant};
//!
//! // Use default theme
//! let theme = AppTheme::default();
//!
//! // Access UI colors
//! let primary = theme.primary;
//! let error = theme.error;
//!
//! // Access specialized colors
//! let diff_added = theme.diff.added;
//! let heading_color = theme.markdown.heading;
//! ```

use ratatui::style::Color;

use crate::services::theme::diff_colors::DiffColors;
use crate::services::theme::markdown_colors::MarkdownColors;
use crate::services::theme::syntax_colors::SyntaxColors;

/// Comprehensive application theme with all widget colors.
///
/// This struct provides a complete color scheme for TUI applications,
/// covering all common UI elements and specialized widget colors.
///
/// # Theme Structure
///
/// The theme is organized into:
///
/// 1. **UI Colors** - Semantic colors for interactive elements
/// 2. **Text Colors** - Colors for text content
/// 3. **Background Colors** - Surface and container backgrounds
/// 4. **Border Colors** - Border and divider colors
/// 5. **Diff Colors** - Colors for diff rendering
/// 6. **Markdown Colors** - Colors for markdown content
/// 7. **Syntax Colors** - Colors for code syntax highlighting
///
/// # Loading from JSON
///
/// Use [`AppTheme::from_json`] or the loader module to load themes
/// from opencode-format JSON files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppTheme {
    // ========== UI Colors ==========
    /// Primary UI color for main interactive elements.
    ///
    /// Used for primary buttons, active selections, and key UI elements.
    pub primary: Color,

    /// Secondary UI color for supporting elements.
    ///
    /// Used for secondary buttons and less prominent interactive elements.
    pub secondary: Color,

    /// Accent color for highlighting and emphasis.
    ///
    /// Used to draw attention to specific UI elements.
    pub accent: Color,

    /// Error color for error states and messages.
    ///
    /// Used for error indicators, validation errors, and destructive actions.
    pub error: Color,

    /// Warning color for warning states and messages.
    ///
    /// Used for warnings and caution indicators.
    pub warning: Color,

    /// Success color for success states and messages.
    ///
    /// Used for success indicators and confirmation feedback.
    pub success: Color,

    /// Info color for informational elements.
    ///
    /// Used for help text, hints, and informational messages.
    pub info: Color,

    // ========== Text Colors ==========
    /// Primary text color for main content.
    ///
    /// The default color for body text and content.
    pub text: Color,

    /// Muted text color for secondary content.
    ///
    /// Used for less important text, placeholders, and hints.
    pub text_muted: Color,

    /// Text color for selected items.
    ///
    /// Used for text within selected or highlighted regions.
    pub selected_text: Color,

    // ========== Background Colors ==========
    /// Main background color.
    ///
    /// The primary application background.
    pub background: Color,

    /// Panel background color.
    ///
    /// Used for content panels and cards.
    pub background_panel: Color,

    /// Element background color.
    ///
    /// Used for interactive elements like buttons and inputs.
    pub background_element: Color,

    /// Menu background color.
    ///
    /// Used for dropdown menus and popover backgrounds.
    pub background_menu: Color,

    // ========== Border Colors ==========
    /// Default border color.
    ///
    /// Used for container borders and dividers.
    pub border: Color,

    /// Active border color.
    ///
    /// Used for focused or active element borders.
    pub border_active: Color,

    /// Subtle border color.
    ///
    /// Used for subtle dividers and less prominent borders.
    pub border_subtle: Color,

    // ========== Specialized Color Sets ==========
    /// Colors for diff rendering.
    ///
    /// Contains all colors needed for the CodeDiff widget.
    pub diff: DiffColors,

    /// Colors for markdown rendering.
    ///
    /// Contains all colors needed for the MarkdownWidget.
    pub markdown: MarkdownColors,

    /// Colors for syntax highlighting.
    ///
    /// Contains all colors needed for code syntax highlighting.
    pub syntax: SyntaxColors,
}

mod constructors;
mod methods;
mod traits;
