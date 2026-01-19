//! Extension trait for scrolling operations.
//!
//! This module defines the trait that extends [`ClickableScrollbarState`] with
//! scrolling methods.

/// Extension trait providing scrolling methods for [`ClickableScrollbarState`].
pub trait ClickableScrollbarStateScrollExt {
    /// Sets the content length and visible page length.
    ///
    /// # Arguments
    ///
    /// * `content_len` - Total length of the scrollable content
    /// * `page_len` - Length of the visible content area
    ///
    /// # Returns
    ///
    /// Self for method chaining
    fn set_content(self, content_len: usize, page_len: usize) -> Self;

    /// Sets the current scroll position.
    ///
    /// # Arguments
    ///
    /// * `offset` - The scroll offset to set (will be clamped to max_offset)
    ///
    /// # Returns
    ///
    /// Self for method chaining
    fn position(self, offset: usize) -> Self;

    /// Gets the current scroll offset.
    ///
    /// # Returns
    ///
    /// The current scroll offset
    fn offset(&self) -> usize;

    /// Sets the scroll offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The new scroll offset (will be clamped to max_offset)
    ///
    /// # Returns
    ///
    /// true if the offset changed, false otherwise
    fn set_offset(&mut self, offset: usize) -> bool;

    /// Scrolls up by the specified number of units.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of units to scroll up
    ///
    /// # Returns
    ///
    /// true if the offset changed, false otherwise
    fn scroll_up(&mut self, n: usize) -> bool;

    /// Scrolls down by the specified number of units.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of units to scroll down
    ///
    /// # Returns
    ///
    /// true if the offset changed, false otherwise
    fn scroll_down(&mut self, n: usize) -> bool;

    /// Gets the scroll increment amount.
    ///
    /// # Returns
    ///
    /// The number of units to scroll per scroll event
    fn scroll_increment(&self) -> usize;
}
