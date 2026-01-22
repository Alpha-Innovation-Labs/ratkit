use ratatui::layout::Rect;

/// Axis for splitting panes.
///
/// Use vertical splits for left/right panes and horizontal splits for
/// top/bottom panes.
///
/// # Example
/// ```rust
/// use ratatui_toolkit::primitives::resizable_grid::SplitAxis;
///
/// let axis = SplitAxis::Vertical;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitAxis {
    /// Vertical split (left/right panes).
    Vertical,
    /// Horizontal split (top/bottom panes).
    Horizontal,
}

/// Deprecated alias for [`SplitAxis`].
///
/// This type was used in the legacy `resizable_split` module. New code
/// should use [`SplitAxis`] directly.
#[deprecated(
    since = "0.1.0",
    note = "Use `SplitAxis` instead. This type will be removed in a future version."
)]
pub use SplitAxis as SplitDirection;

/// Identifier for panes managed by a `SplitLayout` or `ResizableGrid`.
///
/// Pane IDs are stable identifiers that persist even when the layout tree
/// is modified. Each new pane is assigned a unique ID automatically.
pub type PaneId = u32;

/// Default percentage for new splits.
pub const DEFAULT_SPLIT_PERCENT: u16 = 50;

/// Minimum percentage for a split pane.
pub const MIN_SPLIT_PERCENT: u16 = 10;

/// Maximum percentage for a split pane.
pub const MAX_SPLIT_PERCENT: u16 = 90;

/// Node within the split layout tree.
///
/// The tree is composed of leaf nodes (panes) and internal split nodes.
/// Split nodes reference their children by index in the nodes vector.
#[derive(Debug, Clone)]
pub enum LayoutNode {
    /// Leaf pane with a stable identifier.
    Pane {
        /// Identifier for the pane.
        id: PaneId,
    },
    /// Internal split with child nodes and sizing ratio.
    Split {
        /// Axis used for the split.
        axis: SplitAxis,
        /// Percentage allocated to the first child.
        ratio: u16,
        /// Index of the first child node.
        first: usize,
        /// Index of the second child node.
        second: usize,
    },
}

/// Metadata describing a split divider within a layout.
///
/// This struct provides read-only access to divider geometry and properties,
/// useful for rendering dividers or hit-testing during resize operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplitDividerLayout {
    pub(crate) split_index: usize,
    pub(crate) axis: SplitAxis,
    pub(crate) area: Rect,
    pub(crate) ratio: u16,
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

/// A grid-based layout for arranging multiple resizable panes.
///
/// Grid layouts support hierarchical splits in both horizontal and vertical
/// directions, enabling complex multi-pane arrangements.
///
/// # Example
/// ```rust
/// use ratatui_toolkit::primitives::resizable_grid::ResizableGrid;
///
/// let mut grid = ResizableGrid::new(0);
/// ```
#[derive(Debug, Clone)]
pub struct ResizableGrid {
    pub root_index: usize,
    pub nodes: Vec<LayoutNode>,
    pub next_pane_id: PaneId,
    pub hovered_split: Option<usize>,
    pub dragging_split: Option<usize>,
    pub hit_threshold: u16,
}
