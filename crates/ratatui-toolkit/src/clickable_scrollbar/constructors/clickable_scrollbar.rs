//! Constructors for [`ClickableScrollbar`].
//!
//! This module contains builder-style constructor functions for creating
//! [`ClickableScrollbar`] instances with method chaining.

use ratatui::style::Style;
use ratatui::symbols;
use ratatui::widgets::{Scrollbar, ScrollbarOrientation};

use crate::clickable_scrollbar::ClickableScrollbar;

impl<'a> ClickableScrollbar<'a> {
    /// Creates a new [`ClickableScrollbar`] with the specified orientation.
    ///
    /// # Arguments
    ///
    /// * `orientation` - The orientation of the scrollbar (vertical or horizontal)
    ///
    /// # Returns
    ///
    /// A new [`ClickableScrollbar`] instance with the specified orientation
    pub fn new(orientation: ScrollbarOrientation) -> Self {
        Self {
            orientation: orientation.clone(),
            scrollbar: Scrollbar::new(orientation),
        }
    }

    /// Creates a vertical [`ClickableScrollbar`] positioned on the right side.
    ///
    /// # Returns
    ///
    /// A new vertical scrollbar instance
    pub fn vertical() -> Self {
        Self::new(ScrollbarOrientation::VerticalRight)
    }

    /// Creates a horizontal [`ClickableScrollbar`] positioned at the bottom.
    ///
    /// # Returns
    ///
    /// A new horizontal scrollbar instance
    pub fn horizontal() -> Self {
        Self::new(ScrollbarOrientation::HorizontalBottom)
    }

    /// Sets the style for the scrollbar.
    ///
    /// # Arguments
    ///
    /// * `style` - The style to apply to the scrollbar
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn style(mut self, style: Style) -> Self {
        self.scrollbar = self.scrollbar.style(style);
        self
    }

    /// Sets the thumb symbol (the draggable part).
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to use for the thumb
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn thumb_symbol(mut self, symbol: &'a str) -> Self {
        self.scrollbar = self.scrollbar.thumb_symbol(symbol);
        self
    }

    /// Sets the style for the thumb.
    ///
    /// # Arguments
    ///
    /// * `style` - The style to apply to the thumb
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn thumb_style(mut self, style: Style) -> Self {
        self.scrollbar = self.scrollbar.thumb_style(style);
        self
    }

    /// Sets the track symbol.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to use for the track (or None for default)
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn track_symbol(mut self, symbol: Option<&'a str>) -> Self {
        self.scrollbar = self.scrollbar.track_symbol(symbol);
        self
    }

    /// Sets the style for the track.
    ///
    /// # Arguments
    ///
    /// * `style` - The style to apply to the track
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn track_style(mut self, style: Style) -> Self {
        self.scrollbar = self.scrollbar.track_style(style);
        self
    }

    /// Sets the begin symbol (up/left arrow).
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to use for the begin arrow (or None for default)
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn begin_symbol(mut self, symbol: Option<&'a str>) -> Self {
        self.scrollbar = self.scrollbar.begin_symbol(symbol);
        self
    }

    /// Sets the style for the begin symbol.
    ///
    /// # Arguments
    ///
    /// * `style` - The style to apply to the begin symbol
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn begin_style(mut self, style: Style) -> Self {
        self.scrollbar = self.scrollbar.begin_style(style);
        self
    }

    /// Sets the end symbol (down/right arrow).
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to use for the end arrow (or None for default)
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn end_symbol(mut self, symbol: Option<&'a str>) -> Self {
        self.scrollbar = self.scrollbar.end_symbol(symbol);
        self
    }

    /// Sets the style for the end symbol.
    ///
    /// # Arguments
    ///
    /// * `style` - The style to apply to the end symbol
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn end_style(mut self, style: Style) -> Self {
        self.scrollbar = self.scrollbar.end_style(style);
        self
    }

    /// Sets all scrollbar symbols at once.
    ///
    /// # Arguments
    ///
    /// * `symbols` - A set of scrollbar symbols
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn symbols(mut self, symbols: symbols::scrollbar::Set) -> Self {
        self.scrollbar = self.scrollbar.symbols(symbols);
        self
    }
}
