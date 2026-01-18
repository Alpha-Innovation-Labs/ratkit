//! Default trait implementation for [`MarkdownColors`].

use ratatui::style::Color;

use crate::theme::MarkdownColors;

impl Default for MarkdownColors {
    /// Creates a default markdown color scheme based on the original hardcoded colors.
    ///
    /// This matches the original colors used in the markdown renderer before the
    /// theme system was introduced, providing a familiar and well-tested color scheme
    /// that works well on dark terminal backgrounds.
    ///
    /// # Returns
    ///
    /// A `MarkdownColors` instance with the original markdown renderer colors.
    fn default() -> Self {
        Self {
            text: Color::Rgb(191, 189, 182),             // Original text (Ayu fg)
            heading: Color::Rgb(255, 180, 255),          // Original H1 color (bright magenta)
            link: Color::Rgb(100, 200, 100),             // Original link color (bright green)
            link_text: Color::Rgb(100, 200, 100),        // Same as link
            code: Color::Rgb(230, 180, 100),             // Original inline code (warm amber)
            block_quote: Color::Rgb(180, 180, 200),      // Original blockquote text (light gray)
            emph: Color::Rgb(100, 150, 255),             // Original autolink/italic (bright blue)
            strong: Color::Rgb(255, 180, 84),            // Ayu func color (orange)
            horizontal_rule: Color::Rgb(100, 100, 100),  // Original HR (medium gray)
            list_item: Color::Rgb(100, 200, 100),        // Match link color (green)
            list_enumeration: Color::Rgb(100, 200, 100), // Match link color (green)
            image: Color::Rgb(100, 200, 100),            // Match link color (green)
            image_text: Color::Rgb(100, 200, 100),       // Match link color (green)
            code_block: Color::Rgb(191, 189, 182),       // Match text color
        }
    }
}
