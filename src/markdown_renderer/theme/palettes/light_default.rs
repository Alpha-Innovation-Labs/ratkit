//! Default light theme palette.

use crate::markdown_renderer::theme::ColorPalette;
use ratatui::style::Color;

/// Default light theme palette (based on GitHub Light).
///
/// This palette provides a comprehensive set of named colors suitable for
/// light terminal backgrounds. Colors are designed for good readability
/// and visual appeal in light mode.
///
/// # Returns
///
/// A [`ColorPalette`] populated with light theme colors.
///
/// # Available Colors
///
/// Same color names as [`dark_default`], but with values optimized for light backgrounds.
pub fn light_default() -> ColorPalette {
    let mut palette = ColorPalette::new();
    palette.add_color("white", Color::Rgb(36, 41, 47));
    palette.add_color("black", Color::Rgb(246, 248, 250));
    palette.add_color("uiYellow", Color::Rgb(227, 148, 46));
    palette.add_color("hotlandOrange", Color::Rgb(210, 77, 56));
    palette.add_color("healGreen", Color::Rgb(40, 167, 69));
    palette.add_color("soulGreen", Color::Rgb(40, 167, 69));
    palette.add_color("textGray", Color::Rgb(88, 96, 105));
    palette.add_color("coreGray", Color::Rgb(88, 96, 105));
    palette.add_color("mttPink", Color::Rgb(136, 46, 152));
    palette.add_color("soulPurple", Color::Rgb(136, 46, 152));
    palette.add_color("soulRed", Color::Rgb(200, 36, 36));
    palette.add_color("determinationRed", Color::Rgb(200, 36, 36));
    palette.add_color("uiBlue", Color::Rgb(3, 102, 214));
    palette.add_color("oceanBlue", Color::Rgb(3, 102, 214));
    palette.add_color("textGreen", Color::Rgb(34, 134, 58));
    palette.add_color("seaFoam", Color::Rgb(34, 134, 58));
    palette.add_color("cyan", Color::Rgb(5, 134, 151));
    palette.add_color("hotBlue", Color::Rgb(5, 134, 151));
    palette.add_color("magenta", Color::Rgb(136, 46, 152));
    palette.add_color("pink", Color::Rgb(136, 46, 152));
    palette.add_color("red", Color::Rgb(200, 36, 36));
    palette.add_color("orange", Color::Rgb(210, 77, 56));
    palette.add_color("yellow", Color::Rgb(227, 148, 46));
    palette.add_color("green", Color::Rgb(40, 167, 69));
    palette.add_color("blue", Color::Rgb(3, 102, 214));
    palette.add_color("purple", Color::Rgb(136, 46, 152));
    palette
}
