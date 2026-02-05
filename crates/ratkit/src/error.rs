//! Error types for the layout manager.

use thiserror::Error;

/// Result type alias for layout operations.
pub type LayoutResult<T> = Result<T, LayoutError>;

/// Errors that can occur in the layout manager.
#[derive(Debug, Error)]
pub enum LayoutError {
    /// Element with the given ID is not registered.
    #[error("Element not registered: {0}")]
    ElementNotFound(ElementId),

    /// Element is already registered.
    #[error("Element already registered: {0}")]
    ElementAlreadyRegistered(ElementId),

    /// Invalid layout region specification.
    #[error("Invalid layout region: {0}")]
    InvalidRegion(String),

    /// Layout computation failed due to terminal size constraints.
    #[error("Layout computation failed: {0}")]
    LayoutComputation(String),

    /// Focus operation failed.
    #[error("Focus error: {0}")]
    Focus(String),

    /// Event routing failed.
    #[error("Event routing error: {0}")]
    EventRouting(String),

    /// Mouse capture error.
    #[error("Mouse capture error: {0}")]
    MouseCapture(String),

    /// Terminal size is too small for layout.
    #[error("Terminal too small: minimum {0}x{1}, got {2}x{3}")]
    TerminalTooSmall(u16, u16, u16, u16),
}

impl LayoutError {
    pub fn element_not_found(id: ElementId) -> Self {
        Self::ElementNotFound(id)
    }

    pub fn element_already_registered(id: ElementId) -> Self {
        Self::ElementAlreadyRegistered(id)
    }

    pub fn invalid_region(msg: impl Into<String>) -> Self {
        Self::InvalidRegion(msg.into())
    }

    pub fn layout_computation(msg: impl Into<String>) -> Self {
        Self::LayoutComputation(msg.into())
    }

    pub fn focus(msg: impl Into<String>) -> Self {
        Self::Focus(msg.into())
    }

    pub fn event_routing(msg: impl Into<String>) -> Self {
        Self::EventRouting(msg.into())
    }

    pub fn mouse_capture(msg: impl Into<String>) -> Self {
        Self::MouseCapture(msg.into())
    }

    pub fn terminal_too_small(min_width: u16, min_height: u16, width: u16, height: u16) -> Self {
        Self::TerminalTooSmall(min_width, min_height, width, height)
    }
}

use crate::types::ElementId;
