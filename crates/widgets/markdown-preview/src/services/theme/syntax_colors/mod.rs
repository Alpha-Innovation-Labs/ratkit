//! Syntax colors module for code syntax highlighting.
//!
//! This module provides [`SyntaxColors`] which contains all the colors needed
//! for rendering syntax-highlighted code in code blocks and diffs.
//!
//! # Color Categories
//!
//! The syntax color scheme includes standard categories used across
//! most programming languages:
//!
//! - **Comment**: Comments and documentation
//! - **Keyword**: Language keywords (if, else, fn, etc.)
//! - **Function**: Function and method names
//! - **Variable**: Variable and parameter names
//! - **String**: String literals
//! - **Number**: Numeric literals
//! - **Type**: Type names and annotations
//! - **Operator**: Operators (+, -, *, etc.)
//! - **Punctuation**: Brackets, commas, semicolons
//!
//! # Example
//!
//! ```rust
//! use ratatui::style::Color;
//! use ratatui_toolkit::services::theme::SyntaxColors;
//!
//! let colors = SyntaxColors::default();
//! // Use colors.keyword for language keywords
//! ```

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

mod constructors;
mod traits;
