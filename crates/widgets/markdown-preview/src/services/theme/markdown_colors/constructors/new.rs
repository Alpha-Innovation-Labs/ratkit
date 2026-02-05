//! New constructor for [`MarkdownColors`].

use ratatui::style::Color;

use crate::services::theme::MarkdownColors;

impl MarkdownColors {
    /// Creates a new [`MarkdownColors`] instance with the specified colors.
    ///
    /// # Arguments
    ///
    /// * `text` - Color for regular paragraph text
    /// * `heading` - Color for heading text (h1-h6)
    /// * `link` - Color for link URLs
    /// * `link_text` - Color for link display text
    /// * `code` - Color for inline code
    /// * `block_quote` - Color for block quote text
    /// * `emph` - Color for emphasized (italic) text
    /// * `strong` - Color for strong (bold) text
    /// * `horizontal_rule` - Color for horizontal rules
    /// * `list_item` - Color for unordered list bullets
    /// * `list_enumeration` - Color for ordered list numbers
    /// * `image` - Color for image markers
    /// * `image_text` - Color for image alt text
    /// * `code_block` - Color for code block text
    ///
    /// # Returns
    ///
    /// A new `MarkdownColors` instance with all colors configured.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::Color;
    /// use ratatui_toolkit::services::theme::MarkdownColors;
    ///
    /// let colors = MarkdownColors::new(
    ///     Color::White,
    ///     Color::Blue,
    ///     Color::Cyan,
    ///     Color::Green,
    ///     Color::Yellow,
    ///     Color::Gray,
    ///     Color::Magenta,
    ///     Color::LightRed,
    ///     Color::DarkGray,
    ///     Color::Blue,
    ///     Color::Cyan,
    ///     Color::Cyan,
    ///     Color::Green,
    ///     Color::White,
    /// );
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        text: Color,
        heading: Color,
        link: Color,
        link_text: Color,
        code: Color,
        block_quote: Color,
        emph: Color,
        strong: Color,
        horizontal_rule: Color,
        list_item: Color,
        list_enumeration: Color,
        image: Color,
        image_text: Color,
        code_block: Color,
    ) -> Self {
        Self {
            text,
            heading,
            link,
            link_text,
            code,
            block_quote,
            emph,
            strong,
            horizontal_rule,
            list_item,
            list_enumeration,
            image,
            image_text,
            code_block,
        }
    }
}
