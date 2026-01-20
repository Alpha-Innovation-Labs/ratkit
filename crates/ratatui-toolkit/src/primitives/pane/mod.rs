//! Pane component
//!
//! Provides styled panel components with title, icon, padding, and optional footer.

pub mod constructors;
pub mod methods;
pub mod traits;

use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::BorderType;

/// A styled panel component with title, icon, padding, and optional footer
#[derive(Clone, Debug)]
pub struct Pane<'a> {
    /// Title text for pane
    pub title: String,

    /// Icon to display before the title (optional)
    pub icon: Option<String>,

    /// Padding around the content (top, right, bottom, left)
    pub padding: (u16, u16, u16, u16),

    /// Simple text footer (optional) - displayed in border
    pub text_footer: Option<Line<'a>>,

    /// Height of the footer area when using widget footers
    pub footer_height: u16,

    /// Styling
    pub border_style: Style,
    pub border_type: BorderType,
    pub title_style: Style,
    pub footer_style: Style,
}
