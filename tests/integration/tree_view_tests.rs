use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui_toolkit::tree_view::{
    get_visible_paths, TreeKeyBindings, TreeNavigator, TreeNode, TreeViewState,
};

// ========================================
// Helper Functions for Test Setup
// ========================================

/// Create a simple flat tree with 3 nodes for basic tests
fn create_simple_tree() -> Vec<TreeNode<String>> {
    vec![
        TreeNode::new("Node 1".to_string()),
        TreeNode::new("Node 2".to_string()),
        TreeNode::new("Node 3".to_string()),
    ]
}

/// Create a tree with nested children for testing expansion
/// Structure:
/// - Root 1
///   - Child 1.1
///   - Child 1.2
/// - Root 2
///   - Child 2.1
///     - Grandchild 2.1.1
///     - Grandchild 2.1.2
fn create_nested_tree() -> Vec<TreeNode<String>> {
    vec![
        TreeNode::with_children(
            "Root 1".to_string(),
            vec![
                TreeNode::new("Child 1.1".to_string()),
                TreeNode::new("Child 1.2".to_string()),
            ],
        ),
        TreeNode::with_children(
            "Root 2".to_string(),
            vec![TreeNode::with_children(
                "Child 2.1".to_string(),
                vec![
                    TreeNode::new("Grandchild 2.1.1".to_string()),
                    TreeNode::new("Grandchild 2.1.2".to_string()),
                ],
            )],
        ),
    ]
}

/// Create a deeply nested tree for stress testing
/// Structure:
/// - Level 0
///   - Level 1
///     - Level 2
///       - Level 3
///         - Level 4
fn create_deep_tree() -> Vec<TreeNode<String>> {
    vec![TreeNode::with_children(
        "Level 0".to_string(),
        vec![TreeNode::with_children(
            "Level 1".to_string(),
            vec![TreeNode::with_children(
                "Level 2".to_string(),
                vec![TreeNode::with_children(
                    "Level 3".to_string(),
                    vec![TreeNode::new("Level 4".to_string())],
                )],
            )],
        )],
    )]
}

/// Create a key event for testing
fn key_event(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::empty(),
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    }
}

// ========================================
// TreeNode Tests
// ========================================

#[test]
fn test_tree_node_new() {
    let node = TreeNode::new("Test".to_string());
    assert_eq!(node.data, "Test");
    assert!(node.children.is_empty());
    assert!(!node.expandable);
}

#[test]
fn test_tree_node_with_children_empty() {
    // When creating a node with empty children vector, expandable should be false
    let node = TreeNode::with_children("Test".to_string(), vec![]);
    assert_eq!(node.data, "Test");
    assert!(node.children.is_empty());
    assert!(!node.expandable);
}

#[test]
fn test_tree_node_with_children_populated() {
    let children = vec![
        TreeNode::new("Child 1".to_string()),
        TreeNode::new("Child 2".to_string()),
    ];
    let node = TreeNode::with_children("Parent".to_string(), children);

    assert_eq!(node.data, "Parent");
    assert_eq!(node.children.len(), 2);
    assert!(node.expandable);
    assert_eq!(node.children[0].data, "Child 1");
    assert_eq!(node.children[1].data, "Child 2");
}

#[test]
fn test_tree_node_nested_structure() {
    let grandchild = TreeNode::new("Grandchild".to_string());
    let child = TreeNode::with_children("Child".to_string(), vec![grandchild]);
    let root = TreeNode::with_children("Root".to_string(), vec![child]);

    assert_eq!(root.data, "Root");
    assert!(root.expandable);
    assert_eq!(root.children.len(), 1);
    assert_eq!(root.children[0].data, "Child");
    assert!(root.children[0].expandable);
    assert_eq!(root.children[0].children[0].data, "Grandchild");
    assert!(!root.children[0].children[0].expandable);
}

// ========================================
// TreeViewState Tests
// ========================================

#[test]
fn test_tree_view_state_new() {
    let state = TreeViewState::new();
    assert!(state.selected_path.is_none());
    assert!(state.expanded.is_empty());
    assert_eq!(state.offset, 0);
}

#[test]
fn test_tree_view_state_default() {
    let state = TreeViewState::default();
    assert!(state.selected_path.is_none());
    assert!(state.expanded.is_empty());
    assert_eq!(state.offset, 0);
}

#[test]
fn test_tree_view_state_select() {
    let mut state = TreeViewState::new();
    let path = vec![0, 1];

    state.select(path.clone());
    assert_eq!(state.selected_path, Some(path));
}

#[test]
fn test_tree_view_state_clear_selection() {
    let mut state = TreeViewState::new();
    state.select(vec![0]);
    assert!(state.selected_path.is_some());

    state.clear_selection();
    assert!(state.selected_path.is_none());
}

#[test]
fn test_tree_view_state_expand() {
    let mut state = TreeViewState::new();
    let path = vec![0];

    state.expand(path.clone());
    assert!(state.expanded.contains(&path));
}

#[test]
fn test_tree_view_state_collapse() {
    let mut state = TreeViewState::new();
    let path = vec![0];

    state.expand(path.clone());
    assert!(state.expanded.contains(&path));

    state.collapse(path.clone());
    assert!(!state.expanded.contains(&path));
}

#[test]
fn test_tree_view_state_is_expanded() {
    let mut state = TreeViewState::new();
    let path = vec![0];

    assert!(!state.is_expanded(&path));

    state.expand(path.clone());
    assert!(state.is_expanded(&path));
}

#[test]
fn test_tree_view_state_toggle_expansion() {
    let mut state = TreeViewState::new();
    let path = vec![0];

    // Toggle to expand
    state.toggle_expansion(path.clone());
    assert!(state.expanded.contains(&path));

    // Toggle to collapse
    state.toggle_expansion(path.clone());
    assert!(!state.expanded.contains(&path));
}

#[test]
fn test_tree_view_state_expand_all_empty() {
    let mut state = TreeViewState::new();
    let nodes: Vec<TreeNode<String>> = vec![];

    state.expand_all(&nodes);
    assert!(state.expanded.is_empty());
}

#[test]
fn test_tree_view_state_expand_all_simple() {
    let mut state = TreeViewState::new();
    let nodes = create_simple_tree();

    // Simple tree has no expandable nodes
    state.expand_all(&nodes);
    assert!(state.expanded.is_empty());
}

#[test]
fn test_tree_view_state_expand_all_nested() {
    let mut state = TreeViewState::new();
    let nodes = create_nested_tree();

    state.expand_all(&nodes);

    // Should expand Root 1 [0], Root 2 [1], and Child 2.1 [1, 0]
    assert!(state.is_expanded(&[0]));
    assert!(state.is_expanded(&[1]));
    assert!(state.is_expanded(&[1, 0]));
    assert_eq!(state.expanded.len(), 3);
}

#[test]
fn test_tree_view_state_expand_all_deep() {
    let mut state = TreeViewState::new();
    let nodes = create_deep_tree();

    state.expand_all(&nodes);

    // Should expand all levels
    assert!(state.is_expanded(&[0]));
    assert!(state.is_expanded(&[0, 0]));
    assert!(state.is_expanded(&[0, 0, 0]));
    assert!(state.is_expanded(&[0, 0, 0, 0]));
    assert_eq!(state.expanded.len(), 4);
}

#[test]
fn test_tree_view_state_collapse_all() {
    let mut state = TreeViewState::new();
    let nodes = create_nested_tree();

    state.expand_all(&nodes);
    assert!(!state.expanded.is_empty());

    state.collapse_all();
    assert!(state.expanded.is_empty());
}

#[test]
fn test_tree_view_state_multiple_selections() {
    let mut state = TreeViewState::new();

    state.select(vec![0]);
    assert_eq!(state.selected_path, Some(vec![0]));

    // Selecting a new path should replace the old one
    state.select(vec![1, 2]);
    assert_eq!(state.selected_path, Some(vec![1, 2]));
}

// ========================================
// get_visible_paths Tests
// ========================================

#[test]
fn test_get_visible_paths_empty_tree() {
    let nodes: Vec<TreeNode<String>> = vec![];
    let state = TreeViewState::new();

    let paths = get_visible_paths(&nodes, &state);
    assert!(paths.is_empty());
}

#[test]
fn test_get_visible_paths_simple_tree() {
    let nodes = create_simple_tree();
    let state = TreeViewState::new();

    let paths = get_visible_paths(&nodes, &state);
    assert_eq!(paths.len(), 3);
    assert_eq!(paths[0], vec![0]);
    assert_eq!(paths[1], vec![1]);
    assert_eq!(paths[2], vec![2]);
}

#[test]
fn test_get_visible_paths_nested_collapsed() {
    let nodes = create_nested_tree();
    let state = TreeViewState::new();

    let paths = get_visible_paths(&nodes, &state);
    // Only root nodes should be visible when collapsed
    assert_eq!(paths.len(), 2);
    assert_eq!(paths[0], vec![0]);
    assert_eq!(paths[1], vec![1]);
}

#[test]
fn test_get_visible_paths_nested_expanded() {
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();

    // Expand Root 1 [0]
    state.expand(vec![0]);

    let paths = get_visible_paths(&nodes, &state);
    // Should see: Root 1, Child 1.1, Child 1.2, Root 2
    assert_eq!(paths.len(), 4);
    assert_eq!(paths[0], vec![0]);
    assert_eq!(paths[1], vec![0, 0]);
    assert_eq!(paths[2], vec![0, 1]);
    assert_eq!(paths[3], vec![1]);
}

#[test]
fn test_get_visible_paths_nested_fully_expanded() {
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();

    state.expand_all(&nodes);

    let paths = get_visible_paths(&nodes, &state);
    // Should see all 7 nodes
    assert_eq!(paths.len(), 7);
    assert_eq!(paths[0], vec![0]);
    assert_eq!(paths[1], vec![0, 0]);
    assert_eq!(paths[2], vec![0, 1]);
    assert_eq!(paths[3], vec![1]);
    assert_eq!(paths[4], vec![1, 0]);
    assert_eq!(paths[5], vec![1, 0, 0]);
    assert_eq!(paths[6], vec![1, 0, 1]);
}

#[test]
fn test_get_visible_paths_partial_expansion() {
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();

    // Expand only Root 2 [1]
    state.expand(vec![1]);

    let paths = get_visible_paths(&nodes, &state);
    // Should see: Root 1, Root 2, Child 2.1 (but not grandchildren)
    assert_eq!(paths.len(), 3);
    assert_eq!(paths[0], vec![0]);
    assert_eq!(paths[1], vec![1]);
    assert_eq!(paths[2], vec![1, 0]);
}

#[test]
fn test_get_visible_paths_deep_tree_collapsed() {
    let nodes = create_deep_tree();
    let state = TreeViewState::new();

    let paths = get_visible_paths(&nodes, &state);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], vec![0]);
}

#[test]
fn test_get_visible_paths_deep_tree_expanded() {
    let nodes = create_deep_tree();
    let mut state = TreeViewState::new();

    state.expand_all(&nodes);

    let paths = get_visible_paths(&nodes, &state);
    assert_eq!(paths.len(), 5);
    assert_eq!(paths[0], vec![0]);
    assert_eq!(paths[1], vec![0, 0]);
    assert_eq!(paths[2], vec![0, 0, 0]);
    assert_eq!(paths[3], vec![0, 0, 0, 0]);
    assert_eq!(paths[4], vec![0, 0, 0, 0, 0]);
}

// ========================================
// TreeNavigator Tests - Selection
// ========================================

#[test]
fn test_navigator_select_next_empty_tree() {
    let navigator = TreeNavigator::new();
    let nodes: Vec<TreeNode<String>> = vec![];
    let mut state = TreeViewState::new();

    navigator.select_next(&nodes, &mut state);
    assert!(state.selected_path.is_none());
}

#[test]
fn test_navigator_select_next_no_selection() {
    let navigator = TreeNavigator::new();
    let nodes = create_simple_tree();
    let mut state = TreeViewState::new();

    navigator.select_next(&nodes, &mut state);
    // Should select first item
    assert_eq!(state.selected_path, Some(vec![0]));
}

#[test]
fn test_navigator_select_next_simple_tree() {
    let navigator = TreeNavigator::new();
    let nodes = create_simple_tree();
    let mut state = TreeViewState::new();
    state.select(vec![0]);

    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![1]));

    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![2]));
}

#[test]
fn test_navigator_select_next_at_end() {
    let navigator = TreeNavigator::new();
    let nodes = create_simple_tree();
    let mut state = TreeViewState::new();
    state.select(vec![2]); // Last item

    navigator.select_next(&nodes, &mut state);
    // Should stay at last item (no wrapping)
    assert_eq!(state.selected_path, Some(vec![2]));
}

#[test]
fn test_navigator_select_next_skips_collapsed() {
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();
    state.select(vec![0]);

    // With Root 1 collapsed, next should go to Root 2
    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![1]));
}

#[test]
fn test_navigator_select_next_with_expanded() {
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();
    state.expand(vec![0]);
    state.select(vec![0]);

    // With Root 1 expanded, next should go to Child 1.1
    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![0, 0]));

    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![0, 1]));

    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![1]));
}

#[test]
fn test_navigator_select_previous_empty_tree() {
    let navigator = TreeNavigator::new();
    let nodes: Vec<TreeNode<String>> = vec![];
    let mut state = TreeViewState::new();

    navigator.select_previous(&nodes, &mut state);
    assert!(state.selected_path.is_none());
}

#[test]
fn test_navigator_select_previous_no_selection() {
    let navigator = TreeNavigator::new();
    let nodes = create_simple_tree();
    let mut state = TreeViewState::new();

    navigator.select_previous(&nodes, &mut state);
    // Should select first item
    assert_eq!(state.selected_path, Some(vec![0]));
}

#[test]
fn test_navigator_select_previous_simple_tree() {
    let navigator = TreeNavigator::new();
    let nodes = create_simple_tree();
    let mut state = TreeViewState::new();
    state.select(vec![2]);

    navigator.select_previous(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![1]));

    navigator.select_previous(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![0]));
}

#[test]
fn test_navigator_select_previous_at_start() {
    let navigator = TreeNavigator::new();
    let nodes = create_simple_tree();
    let mut state = TreeViewState::new();
    state.select(vec![0]); // First item

    navigator.select_previous(&nodes, &mut state);
    // Should stay at first item (no wrapping)
    assert_eq!(state.selected_path, Some(vec![0]));
}

// ========================================
// TreeNavigator Tests - Goto
// ========================================

#[test]
fn test_navigator_goto_top_empty_tree() {
    let navigator = TreeNavigator::new();
    let nodes: Vec<TreeNode<String>> = vec![];
    let mut state = TreeViewState::new();

    navigator.goto_top(&nodes, &mut state);
    assert!(state.selected_path.is_none());
}

#[test]
fn test_navigator_goto_top() {
    let navigator = TreeNavigator::new();
    let nodes = create_simple_tree();
    let mut state = TreeViewState::new();
    state.select(vec![2]);

    navigator.goto_top(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![0]));
}

#[test]
fn test_navigator_goto_bottom_empty_tree() {
    let navigator = TreeNavigator::new();
    let nodes: Vec<TreeNode<String>> = vec![];
    let mut state = TreeViewState::new();

    navigator.goto_bottom(&nodes, &mut state);
    assert!(state.selected_path.is_none());
}

#[test]
fn test_navigator_goto_bottom() {
    let navigator = TreeNavigator::new();
    let nodes = create_simple_tree();
    let mut state = TreeViewState::new();
    state.select(vec![0]);

    navigator.goto_bottom(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![2]));
}

#[test]
fn test_navigator_goto_bottom_with_expansion() {
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();
    state.expand_all(&nodes);

    navigator.goto_bottom(&nodes, &mut state);
    // Should go to last visible item (Grandchild 2.1.2)
    assert_eq!(state.selected_path, Some(vec![1, 0, 1]));
}

// ========================================
// TreeNavigator Tests - Expansion
// ========================================

#[test]
fn test_navigator_expand_selected_no_selection() {
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();

    navigator.expand_selected(&nodes, &mut state);
    // No change if nothing selected
    assert!(state.expanded.is_empty());
}

#[test]
fn test_navigator_expand_selected_leaf_node() {
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();
    state.expand(vec![0]);
    state.select(vec![0, 0]); // Child 1.1 (leaf node)

    navigator.expand_selected(&nodes, &mut state);
    // Should not expand leaf nodes
    assert!(!state.is_expanded(&[0, 0]));
}

#[test]
fn test_navigator_expand_selected_parent_node() {
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();
    state.select(vec![0]); // Root 1 (has children)

    navigator.expand_selected(&nodes, &mut state);
    assert!(state.is_expanded(&[0]));
}

#[test]
fn test_navigator_expand_selected_already_expanded() {
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();
    state.select(vec![0]);
    state.expand(vec![0]);

    navigator.expand_selected(&nodes, &mut state);
    // Should remain expanded
    assert!(state.is_expanded(&[0]));
}

#[test]
fn test_navigator_collapse_selected_no_selection() {
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();

    navigator.collapse_selected(&nodes, &mut state);
    // No change if nothing selected
    assert!(state.selected_path.is_none());
}

#[test]
fn test_navigator_collapse_selected_expanded_node() {
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();
    state.select(vec![0]);
    state.expand(vec![0]);

    navigator.collapse_selected(&nodes, &mut state);
    // Should collapse
    assert!(!state.is_expanded(&[0]));
    // Selection should remain on the same node
    assert_eq!(state.selected_path, Some(vec![0]));
}

#[test]
fn test_navigator_collapse_selected_collapsed_child_moves_to_parent() {
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();
    state.expand(vec![0]);
    state.select(vec![0, 0]); // Child 1.1

    navigator.collapse_selected(&nodes, &mut state);
    // Should move to parent since child is not expanded
    assert_eq!(state.selected_path, Some(vec![0]));
}

#[test]
fn test_navigator_collapse_selected_root_level() {
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();
    state.select(vec![0]); // Root 1 (not expanded)

    navigator.collapse_selected(&nodes, &mut state);
    // Should stay at root level (can't move to parent)
    assert_eq!(state.selected_path, Some(vec![0]));
}

#[test]
fn test_navigator_toggle_selected_no_selection() {
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();

    navigator.toggle_selected(&nodes, &mut state);
    // No change if nothing selected
    assert!(state.expanded.is_empty());
}

#[test]
fn test_navigator_toggle_selected_leaf_node() {
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();
    state.expand(vec![0]);
    state.select(vec![0, 0]); // Child 1.1 (leaf node)

    navigator.toggle_selected(&nodes, &mut state);
    // Should not toggle leaf nodes
    assert!(!state.is_expanded(&[0, 0]));
}

#[test]
fn test_navigator_toggle_selected_parent_node() {
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();
    state.select(vec![0]); // Root 1 (has children)

    // Toggle to expand
    navigator.toggle_selected(&nodes, &mut state);
    assert!(state.is_expanded(&[0]));

    // Toggle to collapse
    navigator.toggle_selected(&nodes, &mut state);
    assert!(!state.is_expanded(&[0]));
}

// ========================================
// TreeNavigator Tests - Key Handling
// ========================================

#[test]
fn test_navigator_handle_key_default_bindings() {
    let navigator = TreeNavigator::new();
    let nodes = create_simple_tree();
    let mut state = TreeViewState::new();
    state.select(vec![0]);

    // Test 'j' for next
    let handled = navigator.handle_key(key_event(KeyCode::Char('j')), &nodes, &mut state);
    assert!(handled);
    assert_eq!(state.selected_path, Some(vec![1]));

    // Test 'k' for previous
    let handled = navigator.handle_key(key_event(KeyCode::Char('k')), &nodes, &mut state);
    assert!(handled);
    assert_eq!(state.selected_path, Some(vec![0]));
}

#[test]
fn test_navigator_handle_key_arrow_bindings() {
    let navigator = TreeNavigator::new();
    let nodes = create_simple_tree();
    let mut state = TreeViewState::new();
    state.select(vec![0]);

    // Test Down arrow for next
    let handled = navigator.handle_key(key_event(KeyCode::Down), &nodes, &mut state);
    assert!(handled);
    assert_eq!(state.selected_path, Some(vec![1]));

    // Test Up arrow for previous
    let handled = navigator.handle_key(key_event(KeyCode::Up), &nodes, &mut state);
    assert!(handled);
    assert_eq!(state.selected_path, Some(vec![0]));
}

#[test]
fn test_navigator_handle_key_expand_collapse() {
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();
    state.select(vec![0]);

    // Test 'l' for expand
    let handled = navigator.handle_key(key_event(KeyCode::Char('l')), &nodes, &mut state);
    assert!(handled);
    assert!(state.is_expanded(&[0]));

    // Test 'h' for collapse
    let handled = navigator.handle_key(key_event(KeyCode::Char('h')), &nodes, &mut state);
    assert!(handled);
    assert!(!state.is_expanded(&[0]));
}

#[test]
fn test_navigator_handle_key_toggle() {
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();
    state.select(vec![0]);

    // Test Enter for toggle
    let handled = navigator.handle_key(key_event(KeyCode::Enter), &nodes, &mut state);
    assert!(handled);
    assert!(state.is_expanded(&[0]));

    let handled = navigator.handle_key(key_event(KeyCode::Enter), &nodes, &mut state);
    assert!(handled);
    assert!(!state.is_expanded(&[0]));
}

#[test]
fn test_navigator_handle_key_goto() {
    let navigator = TreeNavigator::new();
    let nodes = create_simple_tree();
    let mut state = TreeViewState::new();
    state.select(vec![1]);

    // Test 'g' for goto top
    let handled = navigator.handle_key(key_event(KeyCode::Char('g')), &nodes, &mut state);
    assert!(handled);
    assert_eq!(state.selected_path, Some(vec![0]));

    // Test 'G' for goto bottom
    let handled = navigator.handle_key(key_event(KeyCode::Char('G')), &nodes, &mut state);
    assert!(handled);
    assert_eq!(state.selected_path, Some(vec![2]));
}

#[test]
fn test_navigator_handle_key_unhandled() {
    let navigator = TreeNavigator::new();
    let nodes = create_simple_tree();
    let mut state = TreeViewState::new();

    // Test unhandled key
    let handled = navigator.handle_key(key_event(KeyCode::Char('x')), &nodes, &mut state);
    assert!(!handled);
}

#[test]
fn test_navigator_handle_key_release_ignored() {
    let navigator = TreeNavigator::new();
    let nodes = create_simple_tree();
    let mut state = TreeViewState::new();

    // Create a key release event
    let release_event = KeyEvent {
        code: KeyCode::Char('j'),
        modifiers: KeyModifiers::empty(),
        kind: KeyEventKind::Release,
        state: KeyEventState::empty(),
    };

    let handled = navigator.handle_key(release_event, &nodes, &mut state);
    assert!(!handled);
    assert!(state.selected_path.is_none());
}

// ========================================
// TreeNavigator Tests - Custom Keybindings
// ========================================

#[test]
fn test_custom_keybindings() {
    let custom_bindings = TreeKeyBindings::new()
        .with_next(vec![KeyCode::Char('n')])
        .with_previous(vec![KeyCode::Char('p')])
        .with_expand(vec![KeyCode::Char('e')])
        .with_collapse(vec![KeyCode::Char('c')])
        .with_toggle(vec![KeyCode::Char('t')])
        .with_goto_top(vec![KeyCode::Char('1')])
        .with_goto_bottom(vec![KeyCode::Char('9')]);

    let navigator = TreeNavigator::with_keybindings(custom_bindings);
    let nodes = create_simple_tree();
    let mut state = TreeViewState::new();
    state.select(vec![0]);

    // Test custom 'n' for next
    let handled = navigator.handle_key(key_event(KeyCode::Char('n')), &nodes, &mut state);
    assert!(handled);
    assert_eq!(state.selected_path, Some(vec![1]));

    // Test custom 'p' for previous
    let handled = navigator.handle_key(key_event(KeyCode::Char('p')), &nodes, &mut state);
    assert!(handled);
    assert_eq!(state.selected_path, Some(vec![0]));

    // Old default bindings should not work
    let handled = navigator.handle_key(key_event(KeyCode::Char('j')), &nodes, &mut state);
    assert!(!handled);
}

#[test]
fn test_multiple_keys_per_action() {
    let custom_bindings = TreeKeyBindings::new().with_next(vec![
        KeyCode::Char('j'),
        KeyCode::Down,
        KeyCode::Char('n'),
    ]);

    let navigator = TreeNavigator::with_keybindings(custom_bindings);
    let nodes = create_simple_tree();

    // All three keys should work
    let mut state = TreeViewState::new();
    state.select(vec![0]);
    navigator.handle_key(key_event(KeyCode::Char('j')), &nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![1]));

    let mut state = TreeViewState::new();
    state.select(vec![0]);
    navigator.handle_key(key_event(KeyCode::Down), &nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![1]));

    let mut state = TreeViewState::new();
    state.select(vec![0]);
    navigator.handle_key(key_event(KeyCode::Char('n')), &nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![1]));
}

// ========================================
// Integration Tests - Complex Scenarios
// ========================================

#[test]
fn test_complex_navigation_scenario() {
    // Test a realistic navigation scenario with expansion and navigation
    let navigator = TreeNavigator::new();
    let nodes = create_nested_tree();
    let mut state = TreeViewState::new();

    // Start at top
    navigator.goto_top(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![0]));

    // Expand Root 1
    navigator.expand_selected(&nodes, &mut state);
    assert!(state.is_expanded(&[0]));

    // Navigate to Child 1.1
    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![0, 0]));

    // Try to collapse (should move to parent since child is not expanded)
    navigator.collapse_selected(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![0]));

    // Collapse Root 1 (since it's still expanded)
    navigator.collapse_selected(&nodes, &mut state);
    assert!(!state.is_expanded(&[0]));

    // Navigate to Root 2
    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![1]));

    // Expand Root 2
    navigator.expand_selected(&nodes, &mut state);
    assert!(state.is_expanded(&[1]));

    // Navigate to Child 2.1
    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![1, 0]));

    // Expand Child 2.1
    navigator.expand_selected(&nodes, &mut state);
    assert!(state.is_expanded(&[1, 0]));

    // Navigate to Grandchild 2.1.1
    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![1, 0, 0]));

    // Go to bottom (should be Grandchild 2.1.2)
    navigator.goto_bottom(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![1, 0, 1]));
}

#[test]
fn test_single_node_tree() {
    let nodes = vec![TreeNode::new("Only".to_string())];
    let navigator = TreeNavigator::new();
    let mut state = TreeViewState::new();

    // Select first
    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![0]));

    // Try to go next (should stay)
    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![0]));

    // Try to go previous (should stay)
    navigator.select_previous(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![0]));

    // Try to expand (should not expand leaf)
    navigator.expand_selected(&nodes, &mut state);
    assert!(!state.is_expanded(&[0]));
}

#[test]
fn test_deep_navigation_all_expanded() {
    let nodes = create_deep_tree();
    let mut state = TreeViewState::new();
    state.expand_all(&nodes);

    let navigator = TreeNavigator::new();

    // Start at top
    navigator.goto_top(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![0]));

    // Navigate down through all levels
    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![0, 0]));

    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![0, 0, 0]));

    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![0, 0, 0, 0]));

    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![0, 0, 0, 0, 0]));

    // Should be at bottom
    navigator.select_next(&nodes, &mut state);
    assert_eq!(state.selected_path, Some(vec![0, 0, 0, 0, 0]));
}

#[test]
fn test_expansion_state_persistence() {
    // Test that expansion state is maintained correctly
    let _nodes = create_nested_tree();
    let mut state = TreeViewState::new();

    // Expand multiple nodes
    state.expand(vec![0]);
    state.expand(vec![1]);
    state.expand(vec![1, 0]);

    // Check all are expanded
    assert!(state.is_expanded(&[0]));
    assert!(state.is_expanded(&[1]));
    assert!(state.is_expanded(&[1, 0]));

    // Collapse one
    state.collapse(vec![1]);

    // Check state
    assert!(state.is_expanded(&[0]));
    assert!(!state.is_expanded(&[1]));
    assert!(state.is_expanded(&[1, 0]));

    // Collapse all
    state.collapse_all();
    assert!(state.expanded.is_empty());
}
