//! JSON schema types for parsing opencode theme files.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Represents a color value in the theme JSON.
///
/// Color values can be:
/// - A direct hex color string (e.g., `"#ff0000"`)
/// - A reference to a definition (e.g., `"darkRed"`)
/// - A variant object with dark/light values
///
/// # Examples
///
/// ```json
/// // Direct hex
/// "primary": "#ff0000"
///
/// // Reference to def
/// "primary": "darkBlueBright"
///
/// // Variant object
/// "primary": { "dark": "darkBlueBright", "light": "lightBlue" }
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ColorValue {
    /// A direct color string (hex or reference name).
    Direct(String),

    /// A variant object with dark and light values.
    Variant {
        /// Color value for dark mode (hex or reference).
        dark: String,
        /// Color value for light mode (hex or reference).
        light: String,
    },
}

/// Root structure of an opencode theme JSON file.
///
/// # Structure
///
/// ```json
/// {
///   "$schema": "https://opencode.ai/theme.json",
///   "defs": {
///     "colorName": "#hexcode"
///   },
///   "theme": {
///     "semantic": { "dark": "colorName", "light": "colorName" }
///   }
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThemeJson {
    /// JSON schema URL (optional, used for validation).
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    /// Named color definitions.
    ///
    /// Maps color names to hex color values. These are used for
    /// references in the `theme` section.
    #[serde(default)]
    pub defs: HashMap<String, String>,

    /// Semantic color mappings.
    ///
    /// Maps semantic color names (e.g., "primary", "error") to
    /// color values, which can be direct hex codes, references
    /// to `defs`, or variant objects.
    #[serde(default)]
    pub theme: HashMap<String, ColorValue>,
}
