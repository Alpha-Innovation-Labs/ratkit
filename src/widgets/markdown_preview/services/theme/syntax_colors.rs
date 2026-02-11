/// Syntax colors module for code syntax highlighting.
///
/// This module provides [`SyntaxColors`] which contains all the colors needed
/// for rendering syntax-highlighted code in code blocks and diffs.
///
/// # Color Categories
///
/// The syntax color scheme includes standard categories used across
/// most programming languages:
///
/// - **Comment**: Comments and documentation
/// - **Keyword**: Language keywords (if, else, fn, etc.)
/// - **Function**: Function and method names
/// - **Variable**: Variable and parameter names
/// - **String**: String literals
/// - **Number**: Numeric literals
/// - **Type**: Type names and annotations
/// - **Operator**: Operators (+, -, *, etc.)
/// - **Punctuation**: Brackets, commas, semicolons
///
/// # Example
///
/// ```rust,ignore
/// use ratatui::style::Color;
/// use ratatui_toolkit::services::theme::SyntaxColors;
///
/// let colors = SyntaxColors::default();
/// // Use colors.keyword for language keywords
/// ```
use ratatui::style::Color;

/// Colors for syntax highlighting in code blocks.
///
/// This struct contains colors for the standard syntax categories used
/// across most programming languages. These colors are used by code
/// rendering widgets to provide syntax highlighting.
///
/// # Fields
///
/// Each field corresponds to a syntax category:
///
/// - **comment**: Comments and documentation strings
/// - **keyword**: Language keywords and reserved words
/// - **function**: Function and method definitions/calls
/// - **variable**: Variables, parameters, and identifiers
/// - **string**: String and character literals
/// - **number**: Numeric literals (integers, floats)
/// - **type_**: Type names, generics, and type annotations
/// - **operator**: Mathematical and logical operators
/// - **punctuation**: Delimiters and punctuation marks
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxColors {
    /// Color for comments and documentation.
    ///
    /// Typically a muted color since comments are less important
    /// than actual code.
    pub comment: Color,

    /// Color for language keywords.
    ///
    /// Keywords like `fn`, `let`, `if`, `else`, `return`, etc.
    /// Usually a prominent color.
    pub keyword: Color,

    /// Color for function and method names.
    ///
    /// Used for function definitions and calls.
    pub function: Color,

    /// Color for variables and identifiers.
    ///
    /// Used for variable names, parameters, and general identifiers.
    pub variable: Color,

    /// Color for string literals.
    ///
    /// Used for `"strings"` and `'characters'`.
    pub string: Color,

    /// Color for numeric literals.
    ///
    /// Used for integers, floats, and other numeric values.
    pub number: Color,

    /// Color for type names and annotations.
    ///
    /// Used for type names, generics, and type annotations.
    /// Named `type_` to avoid Rust keyword conflict.
    pub type_: Color,

    /// Color for operators.
    ///
    /// Used for `+`, `-`, `*`, `/`, `=`, `==`, etc.
    pub operator: Color,

    /// Color for punctuation.
    ///
    /// Used for `{`, `}`, `(`, `)`, `,`, `;`, etc.
    pub punctuation: Color,
}

/// New constructor for [`SyntaxColors`].

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
    /// ```rust,ignore
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

/// Default trait implementation for [`SyntaxColors`].

impl Default for SyntaxColors {
    /// Creates a default syntax color scheme based on the Gruvbox dark theme.
    ///
    /// This provides a reasonable default that works well on dark terminal
    /// backgrounds with good contrast for code syntax highlighting.
    ///
    /// # Returns
    ///
    /// A `SyntaxColors` instance with Gruvbox-inspired colors.
    fn default() -> Self {
        Self {
            comment: Color::Rgb(146, 131, 116),     // gruvbox gray
            keyword: Color::Rgb(251, 73, 52),       // gruvbox bright red
            function: Color::Rgb(184, 187, 38),     // gruvbox bright green
            variable: Color::Rgb(131, 165, 152),    // gruvbox bright blue
            string: Color::Rgb(250, 189, 47),       // gruvbox bright yellow
            number: Color::Rgb(211, 134, 155),      // gruvbox bright purple
            type_: Color::Rgb(142, 192, 124),       // gruvbox bright aqua
            operator: Color::Rgb(254, 128, 25),     // gruvbox bright orange
            punctuation: Color::Rgb(235, 219, 178), // gruvbox fg1
        }
    }
}
