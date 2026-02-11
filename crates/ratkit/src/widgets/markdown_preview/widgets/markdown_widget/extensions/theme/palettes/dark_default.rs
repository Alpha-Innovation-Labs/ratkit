//! Default dark theme palette.

use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::theme::ColorPalette;
use ratatui::style::Color;

/// Default dark theme palette (based on One Dark Pro / base16-ocean.dark).
///
/// This palette provides a comprehensive set of named colors suitable for
/// dark terminal backgrounds. Colors are designed for good readability
/// and visual appeal in dark mode.
///
/// # Returns
///
/// A [`ColorPalette`] populated with dark theme colors.
///
/// # Available Colors
///
/// Basic colors: `white`, `black`, `red`, `orange`, `yellow`, `green`, `blue`, `purple`, `cyan`, `magenta`, `pink`
///
/// Semantic colors: `uiYellow`, `hotlandOrange`, `healGreen`, `soulGreen`, `textGray`, `coreGray`,
/// `mttPink`, `soulPurple`, `soulRed`, `determinationRed`, `uiBlue`, `oceanBlue`, `textGreen`,
/// `seaFoam`, `hotBlue`
pub fn dark_default() -> ColorPalette {
    let mut palette = ColorPalette::new();
    palette.add_color("white", Color::Rgb(220, 220, 220));
    palette.add_color("black", Color::Rgb(40, 44, 52));
    palette.add_color("uiYellow", Color::Rgb(230, 192, 123));
    palette.add_color("hotlandOrange", Color::Rgb(191, 97, 106));
    palette.add_color("healGreen", Color::Rgb(163, 190, 140));
    palette.add_color("soulGreen", Color::Rgb(163, 190, 140));
    palette.add_color("textGray", Color::Rgb(144, 145, 156));
    palette.add_color("coreGray", Color::Rgb(144, 145, 156));
    palette.add_color("mttPink", Color::Rgb(198, 120, 221));
    palette.add_color("soulPurple", Color::Rgb(198, 120, 221));
    palette.add_color("soulRed", Color::Rgb(191, 97, 106));
    palette.add_color("determinationRed", Color::Rgb(191, 97, 106));
    palette.add_color("uiBlue", Color::Rgb(97, 175, 239));
    palette.add_color("oceanBlue", Color::Rgb(97, 175, 239));
    palette.add_color("textGreen", Color::Rgb(152, 195, 121));
    palette.add_color("seaFoam", Color::Rgb(152, 195, 121));
    palette.add_color("cyan", Color::Rgb(58, 159, 156));
    palette.add_color("hotBlue", Color::Rgb(58, 159, 156));
    palette.add_color("magenta", Color::Rgb(198, 120, 221));
    palette.add_color("pink", Color::Rgb(198, 120, 221));
    palette.add_color("red", Color::Rgb(191, 97, 106));
    palette.add_color("orange", Color::Rgb(208, 135, 112));
    palette.add_color("yellow", Color::Rgb(230, 192, 123));
    palette.add_color("green", Color::Rgb(163, 190, 140));
    palette.add_color("blue", Color::Rgb(97, 175, 239));
    palette.add_color("purple", Color::Rgb(198, 120, 221));
    palette
}
