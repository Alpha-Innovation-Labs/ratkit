---
context_id: TPR_010
title: Tree View Primitive
project: toolkit-primitives
created: "2026-01-21"
---

# TPR_010: Tree View Primitive

## Desired Outcome

A TreeView primitive that displays hierarchical data in a tree structure with support for expansion, collapse, selection, and filtering. The primitive owns its data for maximum performance and provides a borrowing variant to avoid cloning. Keyboard navigation enables traversal through the tree with configurable keybindings.

## Next Actions

| Description | Test |
|-------------|------|
| TreeView renders hierarchical data with root and nested child nodes | `tree_renders_hierarchical_data` |
| TreeView takes ownership of TreeNode data structure | `tree_takes_ownership` |
| TreeViewRef borrows TreeNode data to avoid cloning | `tree_borrowing_variant_avoids_clone` |
| Expanding a parent node reveals its children in the display | `tree_expand_collapse_node` |
| Collapsing a parent node hides all descendant children | `tree_collapse_hides_children` |
| Selecting a node highlights it and tracks the selection state | `tree_select_node` |
| Keyboard navigation moves selection up, down, left, and right | `tree_keyboard_navigation` |
| Custom TreeKeyBindings map keys to navigation actions | `tree_custom_keybindings` |
| Enter key activates or toggles the currently selected node | `tree_enter_activates_node` |
| Filter input shows only nodes matching the search text | `tree_filter_matches_nodes` |
| Clearing filter restores all nodes to visible state | `tree_filter_clears_all` |
| Expanding a node with filter shows only matching descendants | `tree_filter_shows_matching_descendants` |
| TreeNavigator computes visible paths based on expansion state | `tree_visible_path_computed` |
| NodeState tracks rendering state for each node independently | `tree_node_state_tracked` |
| Empty tree displays placeholder or default visual state | `tree_empty_shows_placeholder` |
| Single root node with no children renders correctly | `tree_single_root_node` |
| Deeply nested hierarchy renders with proper indentation | `tree_deep_hierarchy` |
| Rapid expand/collapse operations respond without lag | `tree_responsive_expand_collapse` |
| Selection persists when filtering is applied | `tree_selection_persists_filter` |
| Collapsing root hides entire subtree except root | `tree_root_collapse_hides_all` |
