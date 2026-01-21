use crate::primitives::split_layout::constants::DEFAULT_SPLIT_PERCENT;
use crate::primitives::split_layout::enums::layout_node::LayoutNode;
use crate::primitives::split_layout::PaneId;
use crate::primitives::split_layout::SplitAxis;
use crate::primitives::split_layout::SplitLayout;

impl SplitLayout {
    pub(super) fn split_pane(&mut self, pane_id: PaneId, axis: SplitAxis) -> Option<PaneId> {
        let pane_index = self.find_pane_node_index(pane_id)?;
        let new_pane_id = self.allocate_pane_id();
        let first_index = self.nodes.len();
        let second_index = self.nodes.len().saturating_add(1);

        self.nodes.push(LayoutNode::Pane { id: pane_id });
        self.nodes.push(LayoutNode::Pane { id: new_pane_id });
        self.nodes[pane_index] = LayoutNode::Split {
            axis,
            ratio: DEFAULT_SPLIT_PERCENT,
            first: first_index,
            second: second_index,
        };

        Some(new_pane_id)
    }
}
