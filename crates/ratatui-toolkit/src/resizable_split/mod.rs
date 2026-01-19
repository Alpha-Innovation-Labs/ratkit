//! Resizable split component
//!
//! Provides resizable split panels with mouse drag support.

pub mod constructors;
pub mod methods;
pub mod traits;

/// Direction of split
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    /// Vertical split (left/right panels) - divider is vertical, mouse drags horizontally
    Vertical,
    /// Horizontal split (top/bottom panels) - divider is horizontal, mouse drags vertically
    Horizontal,
}

/// Tracks state of a resizable split
#[derive(Debug, Clone)]
pub struct ResizableSplit {
    /// Current split position as percentage (0-100)
    pub split_percent: u16,
    /// Minimum percentage for first panel (left or top)
    pub min_percent: u16,
    /// Maximum percentage for first panel (left or top)
    pub max_percent: u16,
    /// Whether currently dragging divider
    pub is_dragging: bool,
    /// Whether mouse is hovering over divider
    pub is_hovering: bool,
    /// Direction of split
    pub direction: SplitDirection,
    /// The column or row position of divider (updated each frame)
    pub divider_pos: u16,
}
