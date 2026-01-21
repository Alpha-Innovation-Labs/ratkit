use crate::primitives::split_layout::PaneId;
use crate::primitives::split_layout::SplitAxis;

/// Node within the split layout tree.
#[derive(Debug, Clone)]
pub(crate) enum LayoutNode {
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
