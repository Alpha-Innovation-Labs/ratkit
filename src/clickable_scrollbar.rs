//! Clickable scrollbar with mouse support adapted from rat-salsa's rat-scrolled
//!
//! This is a simplified version that provides:
//! - Click-to-jump functionality
//! - Drag scrollbar thumb
//! - Mouse wheel support
//!
//! Unlike ratatui's basic Scrollbar, this one handles mouse events.

use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::symbols;
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget};

/// Clickable scrollbar widget.
///
/// This wraps ratatui's Scrollbar but adds mouse interaction support.
#[derive(Debug, Default, Clone)]
pub struct ClickableScrollbar<'a> {
    orientation: ScrollbarOrientation,
    scrollbar: Scrollbar<'a>,
}

/// State for the clickable scrollbar.
///
/// This manages the scrolling position and handles mouse interactions.
#[derive(Debug, Clone)]
pub struct ClickableScrollbarState {
    /// Area where the scrollbar is rendered.
    /// Updated automatically during rendering.
    pub area: Rect,

    /// Orientation of the scrollbar.
    pub orientation: ScrollbarOrientation,

    /// Current scroll offset (position in content).
    pub offset: usize,

    /// Length of visible content area.
    pub page_len: usize,

    /// Maximum scroll offset (content_length - page_len).
    pub max_offset: usize,

    /// How many lines/columns to scroll per wheel event.
    /// Defaults to 1/10 of page_len.
    pub scroll_by: Option<usize>,

    /// Track if mouse drag is active.
    drag_active: bool,
}

/// Result of handling mouse events on the scrollbar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollbarEvent {
    /// No event or event not handled.
    None,
    /// Scroll up by N units.
    Up(usize),
    /// Scroll down by N units.
    Down(usize),
    /// Jump to absolute position.
    Position(usize),
}

impl<'a> ClickableScrollbar<'a> {
    pub fn new(orientation: ScrollbarOrientation) -> Self {
        Self {
            orientation: orientation.clone(),
            scrollbar: Scrollbar::new(orientation),
        }
    }

    /// Create a vertical scrollbar on the right side.
    pub fn vertical() -> Self {
        Self::new(ScrollbarOrientation::VerticalRight)
    }

    /// Create a horizontal scrollbar on the bottom.
    pub fn horizontal() -> Self {
        Self::new(ScrollbarOrientation::HorizontalBottom)
    }

    /// Set the scrollbar style.
    pub fn style(mut self, style: Style) -> Self {
        self.scrollbar = self.scrollbar.style(style);
        self
    }

    /// Set the thumb symbol.
    pub fn thumb_symbol(mut self, symbol: &'a str) -> Self {
        self.scrollbar = self.scrollbar.thumb_symbol(symbol);
        self
    }

    /// Set the thumb style.
    pub fn thumb_style(mut self, style: Style) -> Self {
        self.scrollbar = self.scrollbar.thumb_style(style);
        self
    }

    /// Set the track symbol.
    pub fn track_symbol(mut self, symbol: Option<&'a str>) -> Self {
        self.scrollbar = self.scrollbar.track_symbol(symbol);
        self
    }

    /// Set the track style.
    pub fn track_style(mut self, style: Style) -> Self {
        self.scrollbar = self.scrollbar.track_style(style);
        self
    }

    /// Set the begin symbol.
    pub fn begin_symbol(mut self, symbol: Option<&'a str>) -> Self {
        self.scrollbar = self.scrollbar.begin_symbol(symbol);
        self
    }

    /// Set the begin style.
    pub fn begin_style(mut self, style: Style) -> Self {
        self.scrollbar = self.scrollbar.begin_style(style);
        self
    }

    /// Set the end symbol.
    pub fn end_symbol(mut self, symbol: Option<&'a str>) -> Self {
        self.scrollbar = self.scrollbar.end_symbol(symbol);
        self
    }

    /// Set the end style.
    pub fn end_style(mut self, style: Style) -> Self {
        self.scrollbar = self.scrollbar.end_style(style);
        self
    }

    /// Set all symbols at once.
    pub fn symbols(mut self, symbols: symbols::scrollbar::Set) -> Self {
        self.scrollbar = self.scrollbar.symbols(symbols);
        self
    }
}

impl<'a> StatefulWidget for ClickableScrollbar<'a> {
    type State = ClickableScrollbarState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.area = area;
        state.orientation = self.orientation;

        if area.is_empty() {
            return;
        }

        // Create ratatui ScrollbarState for rendering
        let mut scrollbar_state = ScrollbarState::new(state.max_offset)
            .position(state.offset)
            .viewport_content_length(state.page_len);

        // Render using ratatui's scrollbar
        self.scrollbar.render(area, buf, &mut scrollbar_state);
    }
}

impl Default for ClickableScrollbarState {
    fn default() -> Self {
        Self::new()
    }
}

impl ClickableScrollbarState {
    /// Create a new scrollbar state.
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

    /// Set the content length and page length.
    /// This will calculate max_offset automatically.
    pub fn set_content(mut self, content_len: usize, page_len: usize) -> Self {
        self.page_len = page_len;
        self.max_offset = content_len.saturating_sub(page_len);
        self
    }

    /// Set the position.
    pub fn position(mut self, offset: usize) -> Self {
        self.offset = offset.min(self.max_offset);
        self
    }

    /// Get the current offset.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Set the offset.
    pub fn set_offset(&mut self, offset: usize) -> bool {
        let old = self.offset;
        self.offset = offset.min(self.max_offset);
        old != self.offset
    }

    /// Scroll up by N units.
    pub fn scroll_up(&mut self, n: usize) -> bool {
        let old = self.offset;
        self.offset = self.offset.saturating_sub(n);
        old != self.offset
    }

    /// Scroll down by N units.
    pub fn scroll_down(&mut self, n: usize) -> bool {
        let old = self.offset;
        self.offset = (self.offset + n).min(self.max_offset);
        old != self.offset
    }

    /// Get the scroll increment for wheel events.
    /// Defaults to 1/10 of page_len.
    pub fn scroll_increment(&self) -> usize {
        self.scroll_by
            .unwrap_or_else(|| (self.page_len / 10).max(1))
    }

    /// Handle a mouse event.
    /// Returns Some(ScrollbarEvent) if the event was handled.
    pub fn handle_mouse_event(&mut self, event: &MouseEvent) -> ScrollbarEvent {
        let (col, row) = (event.column, event.row);

        // Check if event is within scrollbar area
        if !self.area.contains((col, row).into()) {
            // Release drag if we're outside the area
            if self.drag_active {
                self.drag_active = false;
            }
            return ScrollbarEvent::None;
        }

        match event.kind {
            // Mouse wheel scrolling
            MouseEventKind::ScrollDown => {
                if self.is_vertical() {
                    ScrollbarEvent::Down(self.scroll_increment())
                } else {
                    ScrollbarEvent::None
                }
            }
            MouseEventKind::ScrollUp => {
                if self.is_vertical() {
                    ScrollbarEvent::Up(self.scroll_increment())
                } else {
                    ScrollbarEvent::None
                }
            }

            // Click to jump to position
            MouseEventKind::Down(MouseButton::Left) => {
                self.drag_active = true;
                let pos = self.map_position_to_offset(col, row);
                ScrollbarEvent::Position(pos)
            }

            // Drag scrollbar thumb
            MouseEventKind::Drag(MouseButton::Left) if self.drag_active => {
                let pos = self.map_position_to_offset(col, row);
                ScrollbarEvent::Position(pos)
            }

            // Release drag
            MouseEventKind::Up(MouseButton::Left) => {
                self.drag_active = false;
                ScrollbarEvent::None
            }

            _ => ScrollbarEvent::None,
        }
    }

    /// Map a mouse position to a scroll offset.
    fn map_position_to_offset(&self, col: u16, row: u16) -> usize {
        if self.is_vertical() {
            // Vertical scrollbar
            let pos = row.saturating_sub(self.area.y).saturating_sub(1) as usize;
            let span = self.area.height.saturating_sub(2) as usize;

            if span > 0 {
                (self.max_offset * pos) / span
            } else {
                0
            }
        } else {
            // Horizontal scrollbar
            let pos = col.saturating_sub(self.area.x).saturating_sub(1) as usize;
            let span = self.area.width.saturating_sub(2) as usize;

            if span > 0 {
                (self.max_offset * pos) / span
            } else {
                0
            }
        }
    }

    /// Is this a vertical scrollbar?
    fn is_vertical(&self) -> bool {
        matches!(
            self.orientation,
            ScrollbarOrientation::VerticalRight | ScrollbarOrientation::VerticalLeft
        )
    }
}
