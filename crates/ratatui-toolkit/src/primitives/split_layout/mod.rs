//! Multi-split layout primitive.
//!
//! Provides a tree of resizable splits that yields pane rectangles for rendering.
//! This type only calculates layout geometry. For interactive resizing with mouse drag,
//! use [`SplitLayoutWidget`](crate::widgets::split_layout::SplitLayoutWidget).
//!
//! # Setting initial split sizes
//!
//! Use `resize_divider` when you have a pane id and want to set that pane's
//! percentage of its split. Use `resize_split` when you already know the split node index.
//!
//! ```rust
//! use ratatui_toolkit::primitives::split_layout::SplitLayout;
//!
//! let mut layout = SplitLayout::new(0);
//! let right_pane = layout.split_pane_horizontally(0).unwrap();
//! let _ = layout.resize_divider(right_pane, 20);
//! ```
//!
//! # Example
//! ```rust
//! use ratatui_toolkit::primitives::split_layout::SplitLayout;
//!
//! let mut layout = SplitLayout::new(0);
//! let _ = layout.split_pane_vertically(0);
//! ```

pub mod constructors;
pub mod enums;
pub mod methods;
pub mod pane_layout;

mod constants;
mod pane_id;

pub use enums::split_axis::SplitAxis;
pub use pane_id::PaneId;
pub use pane_layout::PaneLayout;

use ratatui::layout::Rect;

use crate::primitives::split_layout::enums::layout_node::LayoutNode;

/// A layout tree for arranging multiple resizable panes.
///
/// Split layouts start with a single pane and can be subdivided into
/// horizontal or vertical splits. Each split stores a percentage for the
/// first pane, enabling resizing of the divider.
///
/// This type is non-interactive by itself. Use
/// [`SplitLayoutWidget`](crate::widgets::split_layout::SplitLayoutWidget) for
/// mouse-driven resizing.
///
/// # Example
/// ```rust
/// use ratatui_toolkit::primitives::split_layout::SplitLayout;
///
/// let mut layout = SplitLayout::new(1);
/// let _ = layout.split_pane_horizontally(1);
/// ```
#[derive(Debug, Clone)]
pub struct SplitLayout {
    root_index: usize,
    nodes: Vec<LayoutNode>,
    next_pane_id: PaneId,
}

/// Metadata describing a split divider within a layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplitDividerLayout {
    split_index: usize,
    axis: SplitAxis,
    area: Rect,
    ratio: u16,
}

impl SplitDividerLayout {
    /// Index of the split node within the layout tree.
    pub fn split_index(&self) -> usize {
        self.split_index
    }

    /// Axis for the split divider.
    pub fn axis(&self) -> SplitAxis {
        self.axis
    }

    /// Area covered by the split node.
    pub fn area(&self) -> Rect {
        self.area
    }

    /// Ratio (percentage) assigned to the first child of the split.
    pub fn ratio(&self) -> u16 {
        self.ratio
    }
}
