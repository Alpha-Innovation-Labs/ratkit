use anyhow::Result;
use ratatui::style::{Color, Style};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use ratatui_toolkit::file_system_tree::{FileSystemEntry, FileSystemTree, FileSystemTreeConfig};
use ratatui_toolkit::tree_view::TreeViewState;

// ============================================================================
// Test Utilities
// ============================================================================

/// Create a test directory structure with files and subdirectories
fn create_test_directory_structure() -> Result<TempDir> {
    let temp_dir = TempDir::new()?;
    let root = temp_dir.path();

    // Create files in root
    fs::write(root.join("file1.txt"), "content1")?;
    fs::write(root.join("file2.rs"), "fn main() {}")?;
    fs::write(root.join(".hidden_file"), "hidden content")?;

    // Create subdirectories
    fs::create_dir(root.join("dir1"))?;
    fs::create_dir(root.join("dir2"))?;
    fs::create_dir(root.join(".hidden_dir"))?;

    // Create nested files
    fs::write(root.join("dir1").join("nested1.txt"), "nested content")?;
    fs::write(root.join("dir1").join("nested2.md"), "# Markdown")?;
    fs::write(root.join("dir1").join(".hidden_nested"), "hidden")?;

    // Create deeper nesting
    fs::create_dir(root.join("dir1").join("subdir1"))?;
    fs::write(
        root.join("dir1").join("subdir1").join("deep.json"),
        r#"{"key": "value"}"#,
    )?;

    // Empty directory
    fs::create_dir(root.join("empty_dir"))?;

    Ok(temp_dir)
}

/// Create a simple directory structure for basic tests
fn create_simple_directory() -> Result<TempDir> {
    let temp_dir = TempDir::new()?;
    let root = temp_dir.path();

    fs::write(root.join("test.txt"), "test")?;
    fs::create_dir(root.join("subdir"))?;
    fs::write(root.join("subdir").join("nested.txt"), "nested")?;

    Ok(temp_dir)
}

// ============================================================================
// FileSystemEntry Tests
// ============================================================================

#[test]
fn test_file_system_entry_new_file() -> Result<()> {
    let temp_dir = create_simple_directory()?;
    let file_path = temp_dir.path().join("test.txt");

    let entry = FileSystemEntry::new(file_path.clone())?;

    assert_eq!(entry.name, "test.txt");
    assert_eq!(entry.path, file_path);
    assert!(!entry.is_dir);
    assert!(!entry.is_hidden);

    Ok(())
}

#[test]
fn test_file_system_entry_new_directory() -> Result<()> {
    let temp_dir = create_simple_directory()?;
    let dir_path = temp_dir.path().join("subdir");

    let entry = FileSystemEntry::new(dir_path.clone())?;

    assert_eq!(entry.name, "subdir");
    assert_eq!(entry.path, dir_path);
    assert!(entry.is_dir);
    assert!(!entry.is_hidden);

    Ok(())
}

#[test]
fn test_file_system_entry_hidden_file() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let hidden_path = temp_dir.path().join(".hidden_file");

    let entry = FileSystemEntry::new(hidden_path.clone())?;

    assert_eq!(entry.name, ".hidden_file");
    assert_eq!(entry.path, hidden_path);
    assert!(!entry.is_dir);
    assert!(entry.is_hidden);

    Ok(())
}

#[test]
fn test_file_system_entry_hidden_directory() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let hidden_dir_path = temp_dir.path().join(".hidden_dir");

    let entry = FileSystemEntry::new(hidden_dir_path.clone())?;

    assert_eq!(entry.name, ".hidden_dir");
    assert_eq!(entry.path, hidden_dir_path);
    assert!(entry.is_dir);
    assert!(entry.is_hidden);

    Ok(())
}

// ============================================================================
// FileSystemTreeConfig Tests
// ============================================================================

#[test]
fn test_config_default() {
    let config = FileSystemTreeConfig::default();

    assert!(!config.show_hidden);
    assert!(config.use_dark_theme);
    assert_eq!(config.dir_style.fg, Some(Color::Cyan));
    assert_eq!(config.file_style.fg, Some(Color::White));
    assert_eq!(config.selected_style.bg, Some(Color::Blue));
}

#[test]
fn test_config_builder_pattern() {
    let config = FileSystemTreeConfig::new()
        .with_show_hidden(true)
        .with_dark_theme(false)
        .with_dir_style(Style::default().fg(Color::Green))
        .with_file_style(Style::default().fg(Color::Yellow))
        .with_selected_style(Style::default().bg(Color::Red));

    assert!(config.show_hidden);
    assert!(!config.use_dark_theme);
    assert_eq!(config.dir_style.fg, Some(Color::Green));
    assert_eq!(config.file_style.fg, Some(Color::Yellow));
    assert_eq!(config.selected_style.bg, Some(Color::Red));
}

// ============================================================================
// FileSystemTree Initialization Tests
// ============================================================================

#[test]
fn test_new_with_valid_directory() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;

    assert_eq!(tree.root_path, temp_dir.path());
    assert!(!tree.nodes.is_empty());

    // Default config should hide hidden files
    let has_hidden = tree.nodes.iter().any(|n| n.data.is_hidden);
    assert!(!has_hidden);

    Ok(())
}

#[test]
fn test_new_with_nonexistent_directory() {
    let result = FileSystemTree::new(PathBuf::from("/nonexistent/path/that/does/not/exist"));
    assert!(result.is_err());
}

#[test]
fn test_with_config_show_hidden() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let config = FileSystemTreeConfig::new().with_show_hidden(true);
    let tree = FileSystemTree::with_config(temp_dir.path().to_path_buf(), config)?;

    // Should include hidden files
    let has_hidden = tree.nodes.iter().any(|n| n.data.is_hidden);
    assert!(has_hidden);

    // Verify we have the expected hidden entries
    let hidden_file = tree.nodes.iter().find(|n| n.data.name == ".hidden_file");
    assert!(hidden_file.is_some());

    let hidden_dir = tree.nodes.iter().find(|n| n.data.name == ".hidden_dir");
    assert!(hidden_dir.is_some());

    Ok(())
}

#[test]
fn test_with_config_hide_hidden() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let config = FileSystemTreeConfig::new().with_show_hidden(false);
    let tree = FileSystemTree::with_config(temp_dir.path().to_path_buf(), config)?;

    // Should not include hidden files
    let has_hidden = tree.nodes.iter().any(|n| n.data.is_hidden);
    assert!(!has_hidden);

    Ok(())
}

// ============================================================================
// Directory Sorting Tests
// ============================================================================

#[test]
fn test_directories_sorted_before_files() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;

    // Find the index of the first file
    let first_file_idx = tree.nodes.iter().position(|n| !n.data.is_dir);

    if let Some(first_file_idx) = first_file_idx {
        // All nodes before the first file should be directories
        for node in &tree.nodes[..first_file_idx] {
            assert!(
                node.data.is_dir,
                "Expected directory but found file: {}",
                node.data.name
            );
        }

        // All nodes from the first file onward should be files
        for node in &tree.nodes[first_file_idx..] {
            assert!(
                !node.data.is_dir,
                "Expected file but found directory: {}",
                node.data.name
            );
        }
    }

    Ok(())
}

#[test]
fn test_alphabetical_sorting_within_type() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;

    // Check that directories are sorted alphabetically
    let dirs: Vec<_> = tree.nodes.iter().filter(|n| n.data.is_dir).collect();
    for i in 0..dirs.len().saturating_sub(1) {
        assert!(
            dirs[i].data.name.to_lowercase() <= dirs[i + 1].data.name.to_lowercase(),
            "Directories not sorted: {} should come before {}",
            dirs[i].data.name,
            dirs[i + 1].data.name
        );
    }

    // Check that files are sorted alphabetically
    let files: Vec<_> = tree.nodes.iter().filter(|n| !n.data.is_dir).collect();
    for i in 0..files.len().saturating_sub(1) {
        assert!(
            files[i].data.name.to_lowercase() <= files[i + 1].data.name.to_lowercase(),
            "Files not sorted: {} should come before {}",
            files[i].data.name,
            files[i + 1].data.name
        );
    }

    Ok(())
}

// ============================================================================
// Lazy Loading Tests (expand_directory)
// ============================================================================

#[test]
fn test_directories_initially_not_loaded() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;

    // Find a directory node
    let dir_node = tree
        .nodes
        .iter()
        .find(|n| n.data.is_dir && n.data.name == "dir1");
    assert!(dir_node.is_some());

    let dir_node = dir_node.unwrap();
    // Should be marked as expandable
    assert!(dir_node.expandable);
    // But children should not be loaded yet
    assert!(dir_node.children.is_empty());

    Ok(())
}

#[test]
fn test_expand_directory_loads_children() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let mut tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;

    // Find the index of dir1
    let dir1_idx = tree
        .nodes
        .iter()
        .position(|n| n.data.name == "dir1")
        .expect("dir1 should exist");

    // Expand dir1
    tree.expand_directory(&[dir1_idx])?;

    // Now dir1 should have children
    let dir1_node = &tree.nodes[dir1_idx];
    assert!(!dir1_node.children.is_empty());

    // Verify expected children are present
    let child_names: Vec<_> = dir1_node.children.iter().map(|n| &n.data.name).collect();
    assert!(child_names.contains(&&"nested1.txt".to_string()));
    assert!(child_names.contains(&&"nested2.md".to_string()));
    assert!(child_names.contains(&&"subdir1".to_string()));

    Ok(())
}

#[test]
fn test_expand_nested_directory() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let mut tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;

    // Find dir1 and expand it
    let dir1_idx = tree
        .nodes
        .iter()
        .position(|n| n.data.name == "dir1")
        .unwrap();
    tree.expand_directory(&[dir1_idx])?;

    // Find subdir1 within dir1
    let subdir1_idx = tree.nodes[dir1_idx]
        .children
        .iter()
        .position(|n| n.data.name == "subdir1")
        .unwrap();

    // Expand subdir1
    tree.expand_directory(&[dir1_idx, subdir1_idx])?;

    // Verify subdir1 has children
    let subdir1_node = &tree.nodes[dir1_idx].children[subdir1_idx];
    assert!(!subdir1_node.children.is_empty());

    // Verify deep.json is present
    let has_deep_json = subdir1_node
        .children
        .iter()
        .any(|n| n.data.name == "deep.json");
    assert!(has_deep_json);

    Ok(())
}

#[test]
fn test_expand_empty_directory() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let mut tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;

    // Find empty_dir
    let empty_dir_idx = tree
        .nodes
        .iter()
        .position(|n| n.data.name == "empty_dir")
        .unwrap();

    // Expand empty directory
    tree.expand_directory(&[empty_dir_idx])?;

    // Should have no children
    let empty_dir_node = &tree.nodes[empty_dir_idx];
    assert!(empty_dir_node.children.is_empty());

    Ok(())
}

#[test]
fn test_expand_respects_show_hidden_config() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let config = FileSystemTreeConfig::new().with_show_hidden(false);
    let mut tree = FileSystemTree::with_config(temp_dir.path().to_path_buf(), config)?;

    // Find dir1 and expand it
    let dir1_idx = tree
        .nodes
        .iter()
        .position(|n| n.data.name == "dir1")
        .unwrap();
    tree.expand_directory(&[dir1_idx])?;

    // Should not include .hidden_nested
    let has_hidden = tree.nodes[dir1_idx]
        .children
        .iter()
        .any(|n| n.data.name == ".hidden_nested");
    assert!(!has_hidden);

    Ok(())
}

#[test]
fn test_expand_with_show_hidden_includes_hidden_files() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let config = FileSystemTreeConfig::new().with_show_hidden(true);
    let mut tree = FileSystemTree::with_config(temp_dir.path().to_path_buf(), config)?;

    // Find dir1 and expand it
    let dir1_idx = tree
        .nodes
        .iter()
        .position(|n| n.data.name == "dir1")
        .unwrap();
    tree.expand_directory(&[dir1_idx])?;

    // Should include .hidden_nested
    let has_hidden = tree.nodes[dir1_idx]
        .children
        .iter()
        .any(|n| n.data.name == ".hidden_nested");
    assert!(has_hidden);

    Ok(())
}

// ============================================================================
// Selection and Navigation Tests
// ============================================================================

#[test]
fn test_get_selected_entry_none_selected() -> Result<()> {
    let temp_dir = create_simple_directory()?;
    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;
    let state = TreeViewState::new();

    let selected = tree.get_selected_entry(&state);
    assert!(selected.is_none());

    Ok(())
}

#[test]
fn test_get_selected_entry_valid_selection() -> Result<()> {
    let temp_dir = create_simple_directory()?;
    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;
    let mut state = TreeViewState::new();

    // Select first node
    state.select(vec![0]);

    let selected = tree.get_selected_entry(&state);
    assert!(selected.is_some());

    let entry = selected.unwrap();
    assert!(tree.nodes[0].data.name == entry.name);

    Ok(())
}

#[test]
fn test_get_selected_entry_nested_selection() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let mut tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;
    let mut state = TreeViewState::new();

    // Find and expand dir1
    let dir1_idx = tree
        .nodes
        .iter()
        .position(|n| n.data.name == "dir1")
        .unwrap();
    tree.expand_directory(&[dir1_idx])?;

    // Select first child of dir1
    state.select(vec![dir1_idx, 0]);

    let selected = tree.get_selected_entry(&state);
    assert!(selected.is_some());

    let entry = selected.unwrap();
    // Should match the first child of dir1
    assert_eq!(entry.name, tree.nodes[dir1_idx].children[0].data.name);

    Ok(())
}

#[test]
fn test_select_next_from_start() -> Result<()> {
    let temp_dir = create_simple_directory()?;
    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;
    let mut state = TreeViewState::new();

    // Initially no selection, select_next should select first item
    tree.select_next(&mut state);
    assert!(state.selected_path.is_some());
    assert_eq!(state.selected_path.as_ref().unwrap(), &vec![0]);

    Ok(())
}

#[test]
fn test_select_next_moves_down() -> Result<()> {
    let temp_dir = create_simple_directory()?;
    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;
    let mut state = TreeViewState::new();

    state.select(vec![0]);
    tree.select_next(&mut state);

    assert_eq!(state.selected_path.as_ref().unwrap(), &vec![1]);

    Ok(())
}

#[test]
fn test_select_previous_from_middle() -> Result<()> {
    let temp_dir = create_simple_directory()?;
    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;
    let mut state = TreeViewState::new();

    state.select(vec![1]);
    tree.select_previous(&mut state);

    assert_eq!(state.selected_path.as_ref().unwrap(), &vec![0]);

    Ok(())
}

#[test]
fn test_select_previous_at_start_stays_at_start() -> Result<()> {
    let temp_dir = create_simple_directory()?;
    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;
    let mut state = TreeViewState::new();

    state.select(vec![0]);
    tree.select_previous(&mut state);

    // Should stay at first item
    assert_eq!(state.selected_path.as_ref().unwrap(), &vec![0]);

    Ok(())
}

#[test]
fn test_select_next_at_end_stays_at_end() -> Result<()> {
    let temp_dir = create_simple_directory()?;
    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;
    let mut state = TreeViewState::new();

    let last_idx = tree.nodes.len() - 1;
    state.select(vec![last_idx]);
    tree.select_next(&mut state);

    // Should stay at last item
    assert_eq!(state.selected_path.as_ref().unwrap(), &vec![last_idx]);

    Ok(())
}

// ============================================================================
// Toggle Selection Tests
// ============================================================================

#[test]
fn test_toggle_selected_expands_directory() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let mut tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;
    let mut state = TreeViewState::new();

    // Find and select a directory
    let dir_idx = tree.nodes.iter().position(|n| n.data.is_dir).unwrap();
    state.select(vec![dir_idx]);

    // Directory should not be expanded initially
    assert!(!state.is_expanded(&vec![dir_idx]));

    // Toggle should expand it
    tree.toggle_selected(&mut state)?;
    assert!(state.is_expanded(&vec![dir_idx]));

    // Children should be loaded
    assert!(!tree.nodes[dir_idx].children.is_empty());

    Ok(())
}

#[test]
fn test_toggle_selected_collapses_expanded_directory() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let mut tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;
    let mut state = TreeViewState::new();

    // Find and select a directory
    let dir_idx = tree.nodes.iter().position(|n| n.data.is_dir).unwrap();
    state.select(vec![dir_idx]);

    // Expand it
    tree.toggle_selected(&mut state)?;
    assert!(state.is_expanded(&vec![dir_idx]));

    // Toggle again should collapse it
    tree.toggle_selected(&mut state)?;
    assert!(!state.is_expanded(&vec![dir_idx]));

    Ok(())
}

#[test]
fn test_toggle_selected_on_file_does_nothing() -> Result<()> {
    let temp_dir = create_simple_directory()?;
    let mut tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;
    let mut state = TreeViewState::new();

    // Find and select a file
    let file_idx = tree.nodes.iter().position(|n| !n.data.is_dir).unwrap();
    state.select(vec![file_idx]);

    // Toggle should do nothing for files
    tree.toggle_selected(&mut state)?;
    assert!(!state.is_expanded(&vec![file_idx]));

    Ok(())
}

// ============================================================================
// Path Resolution Tests
// ============================================================================

#[test]
fn test_file_path_resolution() -> Result<()> {
    let temp_dir = create_simple_directory()?;
    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;
    let mut state = TreeViewState::new();

    // Select the file
    let file_idx = tree
        .nodes
        .iter()
        .position(|n| n.data.name == "test.txt")
        .unwrap();
    state.select(vec![file_idx]);

    let entry = tree.get_selected_entry(&state).unwrap();
    assert_eq!(entry.path, temp_dir.path().join("test.txt"));
    assert!(entry.path.exists());

    Ok(())
}

#[test]
fn test_nested_file_path_resolution() -> Result<()> {
    let temp_dir = create_simple_directory()?;
    let mut tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;

    // Expand subdir
    let subdir_idx = tree
        .nodes
        .iter()
        .position(|n| n.data.name == "subdir")
        .unwrap();
    tree.expand_directory(&[subdir_idx])?;

    // Get the nested file
    let nested_file = &tree.nodes[subdir_idx].children[0];
    assert_eq!(nested_file.data.name, "nested.txt");
    assert_eq!(
        nested_file.data.path,
        temp_dir.path().join("subdir").join("nested.txt")
    );
    assert!(nested_file.data.path.exists());

    Ok(())
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_expand_invalid_path_gracefully() -> Result<()> {
    let temp_dir = create_simple_directory()?;
    let mut tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;

    // Try to expand a non-existent path
    let result = tree.expand_directory(&[999]);
    // Should not panic, and operation should complete (even if it's a no-op)
    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_get_entry_at_invalid_path() -> Result<()> {
    let temp_dir = create_simple_directory()?;
    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;
    let mut state = TreeViewState::new();

    // Select an invalid path
    state.select(vec![999]);

    let entry = tree.get_selected_entry(&state);
    assert!(entry.is_none());

    Ok(())
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_empty_directory_tree() -> Result<()> {
    let temp_dir = TempDir::new()?;
    // Don't create any files or subdirectories

    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;
    assert!(tree.nodes.is_empty());

    Ok(())
}

#[test]
fn test_deeply_nested_structure() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let mut current = temp_dir.path().to_path_buf();

    // Create a deep nesting: level1/level2/level3/level4/file.txt
    for i in 1..=4 {
        current = current.join(format!("level{}", i));
        fs::create_dir(&current)?;
    }
    fs::write(current.join("deep_file.txt"), "deep content")?;

    let mut tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;

    // Expand level by level
    tree.expand_directory(&[0])?; // level1
    tree.expand_directory(&[0, 0])?; // level2
    tree.expand_directory(&[0, 0, 0])?; // level3
    tree.expand_directory(&[0, 0, 0, 0])?; // level4

    // Verify we can reach the deep file
    let level4_node = &tree.nodes[0].children[0].children[0].children[0];
    let deep_file = &level4_node.children[0];
    assert_eq!(deep_file.data.name, "deep_file.txt");

    Ok(())
}

#[test]
fn test_unicode_filenames() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let root = temp_dir.path();

    // Create files with unicode characters
    fs::write(root.join("日本語.txt"), "content")?;
    fs::write(root.join("émojis😀.md"), "content")?;
    fs::write(root.join("Ελληνικά.rs"), "content")?;

    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;

    let names: Vec<_> = tree.nodes.iter().map(|n| &n.data.name).collect();
    assert!(names.contains(&&"日本語.txt".to_string()));
    assert!(names.contains(&&"émojis😀.md".to_string()));
    assert!(names.contains(&&"Ελληνικά.rs".to_string()));

    Ok(())
}

#[test]
fn test_special_characters_in_filenames() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let root = temp_dir.path();

    // Create files with special characters (avoiding platform-restricted ones)
    fs::write(root.join("file with spaces.txt"), "content")?;
    fs::write(root.join("file-with-dashes.txt"), "content")?;
    fs::write(root.join("file_with_underscores.txt"), "content")?;

    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;

    let names: Vec<_> = tree.nodes.iter().map(|n| &n.data.name).collect();
    assert!(names.contains(&&"file with spaces.txt".to_string()));
    assert!(names.contains(&&"file-with-dashes.txt".to_string()));
    assert!(names.contains(&&"file_with_underscores.txt".to_string()));

    Ok(())
}

// ============================================================================
// Block Wrapper Tests
// ============================================================================

#[test]
fn test_tree_with_block() -> Result<()> {
    use ratatui::widgets::Block;

    let temp_dir = create_simple_directory()?;
    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;

    let block = Block::default().title("File Browser");
    let tree_with_block = tree.block(block);

    // Should still have the same nodes
    assert!(!tree_with_block.nodes.is_empty());

    Ok(())
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_full_navigation_workflow() -> Result<()> {
    let temp_dir = create_test_directory_structure()?;
    let mut tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;
    let mut state = TreeViewState::new();

    // Start navigation
    tree.select_next(&mut state); // Select first item
    assert!(state.selected_path.is_some());

    // Find a directory
    while let Some(entry) = tree.get_selected_entry(&state) {
        if entry.is_dir {
            break;
        }
        tree.select_next(&mut state);
    }

    // Expand the directory
    let current_path = state.selected_path.clone().unwrap();
    tree.toggle_selected(&mut state)?;

    // Verify it's expanded
    assert!(state.is_expanded(&current_path));

    // Navigate into the expanded directory
    tree.select_next(&mut state);

    // Current selection should be in the children
    let new_path = state.selected_path.as_ref().unwrap();
    assert!(new_path.len() > current_path.len());

    // Collapse the directory
    state.select(current_path.clone());
    tree.toggle_selected(&mut state)?;
    assert!(!state.is_expanded(&current_path));

    Ok(())
}

#[test]
fn test_mixed_file_types() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let root = temp_dir.path();

    // Create various file types
    fs::write(root.join("document.txt"), "text")?;
    fs::write(root.join("code.rs"), "code")?;
    fs::write(root.join("config.toml"), "config")?;
    fs::write(root.join("data.json"), "data")?;
    fs::write(root.join("README.md"), "readme")?;
    fs::create_dir(root.join("src"))?;
    fs::create_dir(root.join("tests"))?;

    let tree = FileSystemTree::new(temp_dir.path().to_path_buf())?;

    // Verify all items are present
    assert_eq!(tree.nodes.len(), 7);

    // Verify directories come first
    assert!(tree.nodes[0].data.is_dir);
    assert!(tree.nodes[1].data.is_dir);
    assert!(!tree.nodes[2].data.is_dir);

    Ok(())
}
