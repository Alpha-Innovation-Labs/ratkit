//! Hex color parsing utilities.

use ratatui::style::Color;

/// Parses a hex color string into a ratatui Color.
///
/// Supports the following formats:
/// - `#RRGGBB` - Standard 6-digit hex with hash
/// - `RRGGBB` - 6-digit hex without hash
/// - `#RGB` - Short 3-digit hex with hash (expanded to 6-digit)
/// - `RGB` - Short 3-digit hex without hash
///
/// # Arguments
///
/// * `hex` - The hex color string to parse
///
/// # Returns
///
/// `Some(Color::Rgb(r, g, b))` if parsing succeeds, `None` otherwise.
///
/// # Example
///
/// ```rust
/// use ratatui::style::Color;
/// use ratatui_toolkit::services::theme::loader::parse_hex_color;
///
/// let color = parse_hex_color("#ff0000");
/// assert_eq!(color, Some(Color::Rgb(255, 0, 0)));
///
/// let color = parse_hex_color("00ff00");
/// assert_eq!(color, Some(Color::Rgb(0, 255, 0)));
///
/// let color = parse_hex_color("#0f0");
/// assert_eq!(color, Some(Color::Rgb(0, 255, 0)));
/// ```
pub fn parse_hex_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');

    match hex.len() {
        // Short format: RGB -> RRGGBB
        3 => {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
            // Expand: F -> FF (multiply by 17 or use bit shift)
            Some(Color::Rgb(r * 17, g * 17, b * 17))
        }
        // Standard format: RRGGBB
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Color::Rgb(r, g, b))
        }
        // Extended format: RRGGBBAA (ignore alpha)
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Color::Rgb(r, g, b))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color_with_hash() {
        assert_eq!(parse_hex_color("#ff0000"), Some(Color::Rgb(255, 0, 0)));
        assert_eq!(parse_hex_color("#00ff00"), Some(Color::Rgb(0, 255, 0)));
        assert_eq!(parse_hex_color("#0000ff"), Some(Color::Rgb(0, 0, 255)));
    }

    #[test]
    fn test_parse_hex_color_without_hash() {
        assert_eq!(parse_hex_color("ff0000"), Some(Color::Rgb(255, 0, 0)));
        assert_eq!(parse_hex_color("282828"), Some(Color::Rgb(40, 40, 40)));
    }

    #[test]
    fn test_parse_hex_color_short_format() {
        assert_eq!(parse_hex_color("#f00"), Some(Color::Rgb(255, 0, 0)));
        assert_eq!(parse_hex_color("0f0"), Some(Color::Rgb(0, 255, 0)));
    }

    #[test]
    fn test_parse_hex_color_with_alpha() {
        assert_eq!(parse_hex_color("#ff0000ff"), Some(Color::Rgb(255, 0, 0)));
    }

    #[test]
    fn test_parse_hex_color_invalid() {
        assert_eq!(parse_hex_color("invalid"), None);
        assert_eq!(parse_hex_color("gg0000"), None);
    }
}
