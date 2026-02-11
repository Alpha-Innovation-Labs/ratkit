/// Markdown colors module for MarkdownWidget theming.
///
/// This module provides [`MarkdownColors`] which contains all the colors needed
/// for rendering markdown content with proper syntax highlighting for headings,
/// links, code blocks, emphasis, and other markdown elements.
///
/// # Color Categories
///
/// The markdown color scheme includes:
/// - **Text colors**: Base text and heading colors
/// - **Link colors**: URL and link text colors
/// - **Code colors**: Inline code and code block colors
/// - **Emphasis colors**: Bold, italic, and quote colors
/// - **List colors**: Bullet and enumeration colors
///
/// # Example
///
/// ```rust,ignore
/// use ratatui::style::Color;
/// use ratatui_toolkit::services::theme::MarkdownColors;
///
/// let colors = MarkdownColors::default();
/// // Use colors.heading for heading text color
/// ```
use ratatui::style::Color;

/// Colors for rendering markdown content.
///
/// This struct contains all the colors needed for the [`MarkdownWidget`](crate::MarkdownWidget)
/// to render markdown with proper syntax highlighting for all markdown elements.
///
/// # Fields
///
/// The color scheme covers all common markdown elements:
///
/// - **Text**: `text`, `heading`
/// - **Links**: `link`, `link_text`
/// - **Code**: `code`, `code_block`
/// - **Emphasis**: `emph` (italic), `strong` (bold)
/// - **Structure**: `block_quote`, `horizontal_rule`
/// - **Lists**: `list_item`, `list_enumeration`
/// - **Images**: `image`, `image_text`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownColors {
    /// Color for regular paragraph text.
    ///
    /// The base color for all markdown text content.
    pub text: Color,

    /// Color for heading text (h1-h6).
    ///
    /// Typically a prominent color like the theme's primary color.
    pub heading: Color,

    /// Color for link URLs.
    ///
    /// Used for the actual URL portion of links.
    pub link: Color,

    /// Color for link display text.
    ///
    /// Used for the clickable text of a link `[text](url)`.
    pub link_text: Color,

    /// Color for inline code.
    ///
    /// Used for `code` spans within paragraphs.
    pub code: Color,

    /// Color for block quote text.
    ///
    /// Used for `> quoted text` blocks.
    pub block_quote: Color,

    /// Color for emphasized (italic) text.
    ///
    /// Used for `*italic*` or `_italic_` text.
    pub emph: Color,

    /// Color for strong (bold) text.
    ///
    /// Used for `**bold**` or `__bold__` text.
    pub strong: Color,

    /// Color for horizontal rules.
    ///
    /// Used for `---` or `***` separators.
    pub horizontal_rule: Color,

    /// Color for unordered list bullets.
    ///
    /// Used for `-`, `*`, or `+` list markers.
    pub list_item: Color,

    /// Color for ordered list numbers.
    ///
    /// Used for `1.`, `2.`, etc. list markers.
    pub list_enumeration: Color,

    /// Color for image markers.
    ///
    /// Used for the `!` prefix in `![alt](url)`.
    pub image: Color,

    /// Color for image alt text.
    ///
    /// Used for the alt text in `![alt text](url)`.
    pub image_text: Color,

    /// Color for code block text.
    ///
    /// Used for fenced code blocks.
    pub code_block: Color,
}

/// New constructor for [`MarkdownColors`].

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
    /// ```rust,ignore
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

/// Default trait implementation for [`MarkdownColors`].

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
