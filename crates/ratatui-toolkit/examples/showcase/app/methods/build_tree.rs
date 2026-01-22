//! Build tree method for demo data.

use ratatui_toolkit::TreeNode;

use super::super::App;

impl App {
    /// Build the demo tree structure.
    pub fn build_tree(&self) -> Vec<TreeNode<String>> {
        vec![
            TreeNode::with_children(
                " Components".to_string(),
                vec![
                    TreeNode::new(" Button".to_string()),
                    TreeNode::new("󰍉 Dialog".to_string()),
                    TreeNode::new(" Toast".to_string()),
                    TreeNode::new("󱒅 Pane".to_string()),
                ],
            ),
            TreeNode::with_children(
                " Layout".to_string(),
                vec![TreeNode::new("󰯋 ResizableGrid".to_string())],
            ),
            TreeNode::with_children(
                " Widgets".to_string(),
                vec![
                    TreeNode::new(" TreeView".to_string()),
                    TreeNode::new(" MenuBar".to_string()),
                    TreeNode::new("󌌌 HotkeyFooter".to_string()),
                ],
            ),
            TreeNode::with_children(
                " Rendering".to_string(),
                vec![TreeNode::new(" MarkdownRenderer".to_string())],
            ),
            TreeNode::with_children(
                " Terminal".to_string(),
                vec![TreeNode::new(" TermTui".to_string())],
            ),
        ]
    }
}
