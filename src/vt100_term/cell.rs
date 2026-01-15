//! Terminal cell representation

use ratatui::style::{Color, Modifier, Style};
use termwiz::cell::Intensity;
use termwiz::color::ColorAttribute;

/// Text attributes for a cell
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Attrs {
    pub fgcolor: ColorAttribute,
    pub bgcolor: ColorAttribute,
    pub intensity: Intensity,
    pub underline: bool,
    pub blink: bool,
    pub reverse: bool,
    pub strikethrough: bool,
    pub italic: bool,
}

impl Default for Attrs {
    fn default() -> Self {
        Self {
            fgcolor: ColorAttribute::Default,
            bgcolor: ColorAttribute::Default,
            intensity: Intensity::Normal,
            underline: false,
            blink: false,
            reverse: false,
            strikethrough: false,
            italic: false,
        }
    }
}

impl Attrs {
    /// Convert termwiz color to ratatui color
    fn termwiz_to_ratatui_color(color: ColorAttribute) -> Color {
        match color {
            ColorAttribute::Default => Color::Reset,
            ColorAttribute::PaletteIndex(idx) => {
                // Standard 256 color palette
                match idx {
                    0 => Color::Black,
                    1 => Color::Red,
                    2 => Color::Green,
                    3 => Color::Yellow,
                    4 => Color::Blue,
                    5 => Color::Magenta,
                    6 => Color::Cyan,
                    7 => Color::Gray,
                    8 => Color::DarkGray,
                    9 => Color::LightRed,
                    10 => Color::LightGreen,
                    11 => Color::LightYellow,
                    12 => Color::LightBlue,
                    13 => Color::LightMagenta,
                    14 => Color::LightCyan,
                    15 => Color::White,
                    _ => Color::Indexed(idx),
                }
            }
            ColorAttribute::TrueColorWithPaletteFallback(rgb, _)
            | ColorAttribute::TrueColorWithDefaultFallback(rgb) => Color::Rgb(
                (rgb.0 * 255.0) as u8,
                (rgb.1 * 255.0) as u8,
                (rgb.2 * 255.0) as u8,
            ),
        }
    }

    /// Convert attributes to ratatui modifiers
    pub fn to_ratatui_modifier(&self) -> Modifier {
        let mut modifier = Modifier::empty();

        if self.intensity == Intensity::Bold {
            modifier |= Modifier::BOLD;
        }
        if self.intensity == Intensity::Half {
            modifier |= Modifier::DIM;
        }
        if self.underline {
            modifier |= Modifier::UNDERLINED;
        }
        if self.blink {
            modifier |= Modifier::SLOW_BLINK;
        }
        if self.reverse {
            modifier |= Modifier::REVERSED;
        }
        if self.strikethrough {
            modifier |= Modifier::CROSSED_OUT;
        }
        if self.italic {
            modifier |= Modifier::ITALIC;
        }

        modifier
    }

    /// Convert to ratatui style
    pub fn to_ratatui_style(&self) -> Style {
        let fg = Self::termwiz_to_ratatui_color(self.fgcolor);
        let bg = Self::termwiz_to_ratatui_color(self.bgcolor);

        Style::default()
            .fg(fg)
            .bg(bg)
            .add_modifier(self.to_ratatui_modifier())
    }
}

/// A single terminal cell
#[derive(Debug, Clone)]
pub struct Cell {
    /// The character in this cell
    pub text: String,

    /// Text attributes
    pub attrs: Attrs,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            text: " ".to_string(),
            attrs: Attrs::default(),
        }
    }
}

impl Cell {
    /// Create a new cell with a character
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            attrs: Attrs::default(),
        }
    }

    /// Create a new cell with character and attributes
    pub fn with_attrs(text: impl Into<String>, attrs: Attrs) -> Self {
        Self {
            text: text.into(),
            attrs,
        }
    }

    /// Check if cell has content (not just space)
    pub fn has_contents(&self) -> bool {
        !self.text.trim().is_empty()
    }

    /// Convert to ratatui cell
    pub fn to_ratatui(&self) -> ratatui::buffer::Cell {
        let mut cell = ratatui::buffer::Cell::default();
        cell.set_symbol(&self.text);
        cell.set_style(self.attrs.to_ratatui_style());
        cell
    }
}
