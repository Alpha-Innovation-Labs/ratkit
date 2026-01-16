//! Theme definitions for markdown rendering.
//!
//! Supports loading themes from JSON files with named colors and light/dark mode.

use ratatui::style::Color;
use std::collections::HashMap;

#[cfg(feature = "markdown")]
use serde::Deserialize;
#[cfg(feature = "markdown")]
use serde_json;

/// Color palette mapping named colors to RGB values
#[derive(Debug, Clone, Default)]
pub struct ColorPalette(HashMap<String, Color>);

impl ColorPalette {
    /// Create a new empty palette
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Add a color to the palette
    pub fn add_color(&mut self, name: &str, color: Color) {
        self.0.insert(name.to_string(), color);
    }

    /// Get a color by name, with fallback
    pub fn get(&self, name: &str) -> Option<Color> {
        self.0.get(name).copied()
    }

    /// Get a color by name with default fallback
    pub fn get_or_default(&self, name: &str) -> Color {
        self.get(name).unwrap_or(Color::White)
    }
}

/// Markdown theme configuration
#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarkdownTheme {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub markdown_text: Option<ColorMapping>,
    #[serde(default)]
    pub markdown_heading: Option<ColorMapping>,
    #[serde(default)]
    pub markdown_code: Option<ColorMapping>,
    #[serde(default)]
    pub markdown_block_quote: Option<ColorMapping>,
    #[serde(default)]
    pub markdown_emph: Option<ColorMapping>,
    #[serde(default)]
    pub markdown_strong: Option<ColorMapping>,
    #[serde(default)]
    pub markdown_link: Option<ColorMapping>,
    #[serde(default)]
    pub markdown_hr: Option<ColorMapping>,
    #[serde(default)]
    pub markdown_table: Option<ColorMapping>,
}

/// Color mapping for light/dark modes
#[derive(Debug, Clone, Deserialize)]
pub struct ColorMapping {
    #[serde(default)]
    pub dark: Option<String>,
    #[serde(default)]
    pub light: Option<String>,
}

impl ColorMapping {
    /// Get color for the specified color scheme
    pub fn get_color(&self, palette: &ColorPalette, is_dark: bool) -> Color {
        let key = if is_dark {
            self.dark.as_ref().or(self.light.as_ref())
        } else {
            self.light.as_ref().or(self.dark.as_ref())
        };
        key.map(|s| palette.get_or_default(s))
            .unwrap_or(Color::White)
    }
}

/// Predefined color palettes for common themes
pub mod palettes {
    use super::ColorPalette;
    use ratatui::style::Color;

    /// Default dark theme palette (based on One Dark Pro / base16-ocean.dark)
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

    /// Default light theme palette (based on GitHub Light)
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

    /// OpenCode dark theme palette
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
}

/// Theme variant selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeVariant {
    #[default]
    Dark,
    Light,
    Auto, // Detect from terminal
}

/// Load a markdown theme from JSON string
pub fn load_theme_from_json(json: &str) -> Result<MarkdownTheme, serde_json::Error> {
    serde_json::from_str(json)
}

/// Get the effective color scheme based on variant and terminal detection
#[allow(unexpected_cfgs)]
pub fn get_effective_theme_variant(variant: ThemeVariant) -> ThemeVariant {
    match variant {
        ThemeVariant::Auto => {
            // Simple terminal detection: check for dark terminal indicators
            // This is a basic implementation that can be enhanced
            #[cfg(feature = "termenv")]
            {
                use termenv::Config;
                let config = Config::default();
                if config.profile() == Some(termenv::Profile::Dark) {
                    ThemeVariant::Dark
                } else {
                    ThemeVariant::Light
                }
            }
            #[cfg(not(feature = "termenv"))]
            {
                // Default to dark mode if termenv is not available
                ThemeVariant::Dark
            }
        }
        _ => variant,
    }
}
