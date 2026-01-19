//! OpenCode dark theme palette.

use crate::markdown_widget::extensions::theme::ColorPalette;
use ratatui::style::Color;

/// OpenCode dark theme palette.
///
/// A custom dark theme palette with colors optimized for the OpenCode
/// application's visual style.
///
/// # Returns
///
/// A [`ColorPalette`] populated with OpenCode dark theme colors.
pub fn opencode_dark() -> ColorPalette {
    let mut palette = ColorPalette::new();
    palette.add_color("white", Color::Rgb(228, 231, 233));
    palette.add_color("black", Color::Rgb(26, 26, 29));
    palette.add_color("uiYellow", Color::Rgb(236, 200, 88));
    palette.add_color("hotlandOrange", Color::Rgb(224, 108, 85));
    palette.add_color("healGreen", Color::Rgb(152, 203, 115));
    palette.add_color("soulGreen", Color::Rgb(152, 203, 115));
    palette.add_color("textGray", Color::Rgb(155, 159, 168));
    palette.add_color("coreGray", Color::Rgb(155, 159, 168));
    palette.add_color("mttPink", Color::Rgb(197, 114, 219));
    palette.add_color("soulPurple", Color::Rgb(197, 114, 219));
    palette.add_color("soulRed", Color::Rgb(229, 93, 93));
    palette.add_color("determinationRed", Color::Rgb(229, 93, 93));
    palette.add_color("uiBlue", Color::Rgb(80, 163, 239));
    palette.add_color("oceanBlue", Color::Rgb(80, 163, 239));
    palette
}
