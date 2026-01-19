//! Terminal text attributes (colors and styles)

use ratatui::style::{Color as RatatuiColor, Modifier, Style};
use termwiz::color::ColorSpec;

/// Terminal color
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Color {
    /// Default terminal color
    #[default]
    Default,
    /// Indexed color (0-255)
    Indexed(u8),
    /// RGB true color
    Rgb(u8, u8, u8),
}

impl Color {
    /// Convert to ratatui color
    pub fn to_ratatui(self) -> Option<RatatuiColor> {
        match self {
            Color::Default => None,
            Color::Indexed(idx) => Some(match idx {
                0 => RatatuiColor::Black,
                1 => RatatuiColor::Red,
                2 => RatatuiColor::Green,
                3 => RatatuiColor::Yellow,
                4 => RatatuiColor::Blue,
                5 => RatatuiColor::Magenta,
                6 => RatatuiColor::Cyan,
                7 => RatatuiColor::Gray,
                8 => RatatuiColor::DarkGray,
                9 => RatatuiColor::LightRed,
                10 => RatatuiColor::LightGreen,
                11 => RatatuiColor::LightYellow,
                12 => RatatuiColor::LightBlue,
                13 => RatatuiColor::LightMagenta,
                14 => RatatuiColor::LightCyan,
                15 => RatatuiColor::White,
                _ => RatatuiColor::Indexed(idx),
            }),
            Color::Rgb(r, g, b) => Some(RatatuiColor::Rgb(r, g, b)),
        }
    }
}

impl From<ColorSpec> for Color {
    fn from(spec: ColorSpec) -> Self {
        match spec {
            ColorSpec::Default => Color::Default,
            ColorSpec::PaletteIndex(idx) => Color::Indexed(idx),
            ColorSpec::TrueColor(srgba) => Color::Rgb(
                (srgba.0 * 255.0) as u8,
                (srgba.1 * 255.0) as u8,
                (srgba.2 * 255.0) as u8,
            ),
        }
    }
}

// Text mode bit flags
const TEXT_MODE_BOLD: u8 = 1 << 0;
const TEXT_MODE_ITALIC: u8 = 1 << 1;
const TEXT_MODE_UNDERLINE: u8 = 1 << 2;
const TEXT_MODE_INVERSE: u8 = 1 << 3;
const TEXT_MODE_STRIKETHROUGH: u8 = 1 << 4;

/// Terminal cell attributes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Attrs {
    /// Foreground color
    pub fg: Color,
    /// Background color
    pub bg: Color,
    /// Text mode flags
    mode: u8,
}

impl Attrs {
    /// Create new attributes with default colors
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if bold
    pub fn bold(&self) -> bool {
        self.mode & TEXT_MODE_BOLD != 0
    }

    /// Set bold
    pub fn set_bold(&mut self, on: bool) {
        if on {
            self.mode |= TEXT_MODE_BOLD;
        } else {
            self.mode &= !TEXT_MODE_BOLD;
        }
    }

    /// Check if italic
    pub fn italic(&self) -> bool {
        self.mode & TEXT_MODE_ITALIC != 0
    }

    /// Set italic
    pub fn set_italic(&mut self, on: bool) {
        if on {
            self.mode |= TEXT_MODE_ITALIC;
        } else {
            self.mode &= !TEXT_MODE_ITALIC;
        }
    }

    /// Check if underline
    pub fn underline(&self) -> bool {
        self.mode & TEXT_MODE_UNDERLINE != 0
    }

    /// Set underline
    pub fn set_underline(&mut self, on: bool) {
        if on {
            self.mode |= TEXT_MODE_UNDERLINE;
        } else {
            self.mode &= !TEXT_MODE_UNDERLINE;
        }
    }

    /// Check if inverse (swap fg/bg)
    pub fn inverse(&self) -> bool {
        self.mode & TEXT_MODE_INVERSE != 0
    }

    /// Set inverse
    pub fn set_inverse(&mut self, on: bool) {
        if on {
            self.mode |= TEXT_MODE_INVERSE;
        } else {
            self.mode &= !TEXT_MODE_INVERSE;
        }
    }

    /// Check if strikethrough
    pub fn strikethrough(&self) -> bool {
        self.mode & TEXT_MODE_STRIKETHROUGH != 0
    }

    /// Set strikethrough
    pub fn set_strikethrough(&mut self, on: bool) {
        if on {
            self.mode |= TEXT_MODE_STRIKETHROUGH;
        } else {
            self.mode &= !TEXT_MODE_STRIKETHROUGH;
        }
    }

    /// Reset all attributes to default
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Convert to ratatui style
    pub fn to_ratatui(&self) -> Style {
        let mut style = Style::default();

        // Apply colors (handle inverse)
        let (fg, bg) = if self.inverse() {
            (self.bg, self.fg)
        } else {
            (self.fg, self.bg)
        };

        if let Some(color) = fg.to_ratatui() {
            style = style.fg(color);
        }
        if let Some(color) = bg.to_ratatui() {
            style = style.bg(color);
        }

        // Apply modifiers
        let mut modifiers = Modifier::empty();
        if self.bold() {
            modifiers |= Modifier::BOLD;
        }
        if self.italic() {
            modifiers |= Modifier::ITALIC;
        }
        if self.underline() {
            modifiers |= Modifier::UNDERLINED;
        }
        if self.strikethrough() {
            modifiers |= Modifier::CROSSED_OUT;
        }

        if !modifiers.is_empty() {
            style = style.add_modifier(modifiers);
        }

        style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_default() {
        let color = Color::Default;
        assert_eq!(color.to_ratatui(), None);
    }

    #[test]
    fn test_color_indexed() {
        assert_eq!(Color::Indexed(0).to_ratatui(), Some(RatatuiColor::Black));
        assert_eq!(Color::Indexed(1).to_ratatui(), Some(RatatuiColor::Red));
        assert_eq!(
            Color::Indexed(100).to_ratatui(),
            Some(RatatuiColor::Indexed(100))
        );
    }

    #[test]
    fn test_color_rgb() {
        assert_eq!(
            Color::Rgb(255, 128, 64).to_ratatui(),
            Some(RatatuiColor::Rgb(255, 128, 64))
        );
    }

    #[test]
    fn test_attrs_default() {
        let attrs = Attrs::default();
        assert!(!attrs.bold());
        assert!(!attrs.italic());
        assert!(!attrs.underline());
        assert!(!attrs.inverse());
    }

    #[test]
    fn test_attrs_set_modes() {
        let mut attrs = Attrs::new();

        attrs.set_bold(true);
        assert!(attrs.bold());

        attrs.set_italic(true);
        assert!(attrs.italic());

        attrs.set_underline(true);
        assert!(attrs.underline());

        attrs.set_inverse(true);
        assert!(attrs.inverse());

        attrs.reset();
        assert!(!attrs.bold());
        assert!(!attrs.italic());
    }
}
