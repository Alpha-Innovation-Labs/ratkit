//! Color value resolution from theme definitions.

use std::collections::HashMap;

use ratatui::style::Color;

use crate::services::theme::loader::parse_color::parse_hex_color;
use crate::services::theme::loader::theme_json::ColorValue;
use crate::services::theme::ThemeVariant;

/// Resolves a color value from the theme JSON, handling defs references.
///
/// This function takes a color value and resolves it to a ratatui Color by:
/// 1. If it's a variant object, selecting the dark or light value based on variant
/// 2. If the value is a hex color, parsing it directly
/// 3. If the value is a reference name, looking it up in defs
///
/// # Arguments
///
/// * `value` - The color value from the theme JSON
/// * `defs` - The definitions map from the theme JSON
/// * `variant` - Which variant (dark/light) to use
///
/// # Returns
///
/// `Some(Color)` if resolution succeeds, `None` otherwise.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
/// use ratatui::style::Color;
/// use ratatui_toolkit::services::theme::{ThemeVariant, loader::{ColorValue, resolve_color_value}};
///
/// let mut defs = HashMap::new();
/// defs.insert("myBlue".to_string(), "#0000ff".to_string());
///
/// // Direct hex
/// let direct = ColorValue::Direct("#ff0000".to_string());
/// let color = resolve_color_value(&direct, &defs, ThemeVariant::Dark);
/// assert_eq!(color, Some(Color::Rgb(255, 0, 0)));
///
/// // Reference to def
/// let reference = ColorValue::Direct("myBlue".to_string());
/// let color = resolve_color_value(&reference, &defs, ThemeVariant::Dark);
/// assert_eq!(color, Some(Color::Rgb(0, 0, 255)));
/// ```
pub fn resolve_color_value(
    value: &ColorValue,
    defs: &HashMap<String, String>,
    variant: ThemeVariant,
) -> Option<Color> {
    match value {
        ColorValue::Direct(s) => resolve_string(s, defs),
        ColorValue::Variant { dark, light } => {
            let s = match variant {
                ThemeVariant::Dark => dark,
                ThemeVariant::Light => light,
            };
            resolve_string(s, defs)
        }
    }
}

/// Resolves a string value to a color.
///
/// The string can be either a hex color or a reference to a definition.
fn resolve_string(s: &str, defs: &HashMap<String, String>) -> Option<Color> {
    // Try parsing as hex first
    if s.starts_with('#') {
        return parse_hex_color(s);
    }

    // Try looking up in defs
    if let Some(hex) = defs.get(s) {
        return parse_hex_color(hex);
    }

    // Maybe it's a hex without the # prefix
    if s.chars().all(|c| c.is_ascii_hexdigit()) && (s.len() == 6 || s.len() == 3) {
        return parse_hex_color(s);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_direct_hex() {
        let defs = HashMap::new();
        let value = ColorValue::Direct("#ff0000".to_string());
        let result = resolve_color_value(&value, &defs, ThemeVariant::Dark);
        assert_eq!(result, Some(Color::Rgb(255, 0, 0)));
    }

    #[test]
    fn test_resolve_reference() {
        let mut defs = HashMap::new();
        defs.insert("darkRed".to_string(), "#cc241d".to_string());

        let value = ColorValue::Direct("darkRed".to_string());
        let result = resolve_color_value(&value, &defs, ThemeVariant::Dark);
        assert_eq!(result, Some(Color::Rgb(204, 36, 29)));
    }

    #[test]
    fn test_resolve_variant_dark() {
        let mut defs = HashMap::new();
        defs.insert("darkBlue".to_string(), "#458588".to_string());
        defs.insert("lightBlue".to_string(), "#076678".to_string());

        let value = ColorValue::Variant {
            dark: "darkBlue".to_string(),
            light: "lightBlue".to_string(),
        };

        let dark_result = resolve_color_value(&value, &defs, ThemeVariant::Dark);
        assert_eq!(dark_result, Some(Color::Rgb(69, 133, 136)));

        let light_result = resolve_color_value(&value, &defs, ThemeVariant::Light);
        assert_eq!(light_result, Some(Color::Rgb(7, 102, 120)));
    }
}
