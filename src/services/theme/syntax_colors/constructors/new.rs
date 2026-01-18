//! New constructor for [`SyntaxColors`].

use ratatui::style::Color;

use crate::services::theme::SyntaxColors;

impl SyntaxColors {
    /// Creates a new [`SyntaxColors`] instance with the specified colors.
    ///
    /// # Arguments
    ///
    /// * `comment` - Color for comments and documentation
    /// * `keyword` - Color for language keywords
    /// * `function` - Color for function and method names
    /// * `variable` - Color for variables and identifiers
    /// * `string` - Color for string literals
    /// * `number` - Color for numeric literals
    /// * `type_` - Color for type names and annotations
    /// * `operator` - Color for operators
    /// * `punctuation` - Color for punctuation
    ///
    /// # Returns
    ///
    /// A new `SyntaxColors` instance with all colors configured.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::Color;
    /// use ratatui_toolkit::services::theme::SyntaxColors;
    ///
    /// let colors = SyntaxColors::new(
    ///     Color::Gray,
    ///     Color::Red,
    ///     Color::Green,
    ///     Color::Blue,
    ///     Color::Yellow,
    ///     Color::Magenta,
    ///     Color::Cyan,
    ///     Color::LightRed,
    ///     Color::White,
    /// );
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        comment: Color,
        keyword: Color,
        function: Color,
        variable: Color,
        string: Color,
        number: Color,
        type_: Color,
        operator: Color,
        punctuation: Color,
    ) -> Self {
        Self {
            comment,
            keyword,
            function,
            variable,
            string,
            number,
            type_,
            operator,
            punctuation,
        }
    }
}
