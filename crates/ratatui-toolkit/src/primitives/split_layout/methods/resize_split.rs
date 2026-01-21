use crate::primitives::split_layout::constants::{MAX_SPLIT_PERCENT, MIN_SPLIT_PERCENT};
use crate::primitives::split_layout::enums::layout_node::LayoutNode;
use crate::primitives::split_layout::SplitLayout;

impl SplitLayout {
    /// Resizes a split node by index.
    pub fn resize_split(&mut self, split_index: usize, percent: u16) -> bool {
        let Some(LayoutNode::Split { ratio, .. }) = self.nodes.get_mut(split_index) else {
            return false;
        };

        *ratio = percent.clamp(MIN_SPLIT_PERCENT, MAX_SPLIT_PERCENT);
        true
    }
}
