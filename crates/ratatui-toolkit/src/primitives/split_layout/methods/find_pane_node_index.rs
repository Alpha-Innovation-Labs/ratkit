use crate::primitives::split_layout::enums::layout_node::LayoutNode;
use crate::primitives::split_layout::PaneId;
use crate::primitives::split_layout::SplitLayout;

impl SplitLayout {
    pub(super) fn find_pane_node_index(&self, pane_id: PaneId) -> Option<usize> {
        let mut stack = vec![self.root_index];

        while let Some(node_index) = stack.pop() {
            let Some(node) = self.nodes.get(node_index) else {
                continue;
            };

            match node {
                LayoutNode::Pane { id } => {
                    if *id == pane_id {
                        return Some(node_index);
                    }
                }
                LayoutNode::Split { first, second, .. } => {
                    stack.push(*second);
                    stack.push(*first);
                }
            }
        }

        None
    }
}
