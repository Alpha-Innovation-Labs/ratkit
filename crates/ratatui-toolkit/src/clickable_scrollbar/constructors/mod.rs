//! Constructor functions for [`ClickableScrollbar`].
//!
//! This module provides builder-style constructors for creating scrollbar instances.
//! Use these to create and configure scrollbars with method chaining.

use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::symbols;
use ratatui::widgets::{Scrollbar, ScrollbarOrientation};

use crate::clickable_scrollbar::{ClickableScrollbar, ClickableScrollbarState};

impl<'a> ClickableScrollbar<'a> {
    /// Creates a new [`ClickableScrollbar`] with the specified orientation.
    pub fn new(orientation: ScrollbarOrientation) -> Self {
        Self {
            orientation: orientation.clone(),
            scrollbar: Scrollbar::new(orientation),
        }
    }

    /// Creates a vertical [`ClickableScrollbar`] positioned on the right side.
    pub fn vertical() -> Self {
        Self::new(ScrollbarOrientation::VerticalRight)
    }

    /// Creates a horizontal [`ClickableScrollbar`] positioned at the bottom.
    pub fn horizontal() -> Self {
        Self::new(ScrollbarOrientation::HorizontalBottom)
    }

    /// Sets the style for the scrollbar.
    pub fn style(mut self, style: Style) -> Self {
        self.scrollbar = self.scrollbar.style(style);
        self
    }

    /// Sets the thumb symbol (the draggable part).
    pub fn thumb_symbol(mut self, symbol: &'a str) -> Self {
        self.scrollbar = self.scrollbar.thumb_symbol(symbol);
        self
    }

    /// Sets the style for the thumb.
    pub fn thumb_style(mut self, style: Style) -> Self {
        self.scrollbar = self.scrollbar.thumb_style(style);
        self
    }

    /// Sets the track symbol.
    pub fn track_symbol(mut self, symbol: Option<&'a str>) -> Self {
        self.scrollbar = self.scrollbar.track_symbol(symbol);
        self
    }

    /// Sets the style for the track.
    pub fn track_style(mut self, style: Style) -> Self {
        self.scrollbar = self.scrollbar.track_style(style);
        self
    }

    /// Sets the begin symbol (up/left arrow).
    pub fn begin_symbol(mut self, symbol: Option<&'a str>) -> Self {
        self.scrollbar = self.scrollbar.begin_symbol(symbol);
        self
    }

    /// Sets the style for the begin symbol.
    pub fn begin_style(mut self, style: Style) -> Self {
        self.scrollbar = self.scrollbar.begin_style(style);
        self
    }

    /// Sets the end symbol (down/right arrow).
    pub fn end_symbol(mut self, symbol: Option<&'a str>) -> Self {
        self.scrollbar = self.scrollbar.end_symbol(symbol);
        self
    }

    /// Sets the style for the end symbol.
    pub fn end_style(mut self, style: Style) -> Self {
        self.scrollbar = self.scrollbar.end_style(style);
        self
    }

    /// Sets all scrollbar symbols at once.
    pub fn symbols(mut self, symbols: symbols::scrollbar::Set) -> Self {
        self.scrollbar = self.scrollbar.symbols(symbols);
        self
    }
}

impl ClickableScrollbarState {
    /// Creates a new [`ClickableScrollbarState`] with default values.
    pub fn new() -> Self {
        Self {
            area: Rect::default(),
            orientation: ScrollbarOrientation::VerticalRight,
            offset: 0,
            page_len: 0,
            max_offset: 0,
            scroll_by: None,
            drag_active: false,
        }
    }
}
