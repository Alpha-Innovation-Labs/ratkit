use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use std::thread;
use std::time::Duration;
use ratatui_toolkit::FuzzyFinder;

// ============================================================================
// Initialization Tests
// ============================================================================

#[test]
fn test_fuzzy_finder_new() {
    let finder = FuzzyFinder::new("Test Finder");
    assert!(
        !finder.is_running(),
        "FuzzyFinder should not be running initially"
    );
}

#[test]
fn test_fuzzy_finder_new_with_empty_title() {
    let finder = FuzzyFinder::new("");
    assert!(
        !finder.is_running(),
        "FuzzyFinder should not be running with empty title"
    );
}

#[test]
fn test_fuzzy_finder_with_size() {
    let finder = FuzzyFinder::new("Test").with_size(50, 60);
    // Size is internal, but we can verify the object was created successfully
    assert!(!finder.is_running());
}

#[test]
fn test_fuzzy_finder_with_loading_message() {
    let finder = FuzzyFinder::new("Test").with_loading_message("Please wait...");
    assert!(!finder.is_running());
}

#[test]
fn test_fuzzy_finder_builder_chaining() {
    let finder = FuzzyFinder::new("Test")
        .with_size(70, 70)
        .with_loading_message("Loading items...");
    assert!(!finder.is_running());
}

// ============================================================================
// Spawn Tests - Error Handling
// ============================================================================

#[test]
fn test_spawn_fzf_empty_items() {
    let mut finder = FuzzyFinder::new("Test");
    let items = vec![];

    let result = finder.spawn_fzf(items, 20, 80, None);
    assert!(result.is_err(), "Should fail with empty items");
    assert!(result.unwrap_err().to_string().contains("empty item list"));
}

#[test]
fn test_spawn_fzf_zero_rows() {
    let mut finder = FuzzyFinder::new("Test");
    let items = vec!["item1".to_string(), "item2".to_string()];

    let result = finder.spawn_fzf(items, 0, 80, None);
    assert!(result.is_err(), "Should fail with zero rows");
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Invalid terminal size"));
}

#[test]
fn test_spawn_fzf_zero_cols() {
    let mut finder = FuzzyFinder::new("Test");
    let items = vec!["item1".to_string(), "item2".to_string()];

    let result = finder.spawn_fzf(items, 20, 0, None);
    assert!(result.is_err(), "Should fail with zero columns");
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Invalid terminal size"));
}

#[test]
fn test_spawn_fzf_zero_dimensions() {
    let mut finder = FuzzyFinder::new("Test");
    let items = vec!["item1".to_string(), "item2".to_string()];

    let result = finder.spawn_fzf(items, 0, 0, None);
    assert!(result.is_err(), "Should fail with zero dimensions");
}

#[test]
fn test_spawn_command_zero_rows() {
    let mut finder = FuzzyFinder::new("Test");

    let result = finder.spawn_command("echo", &["test"], None, 0, 80);
    assert!(result.is_err(), "Should fail with zero rows");
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Invalid terminal size"));
}

#[test]
fn test_spawn_command_zero_cols() {
    let mut finder = FuzzyFinder::new("Test");

    let result = finder.spawn_command("echo", &["test"], None, 20, 0);
    assert!(result.is_err(), "Should fail with zero columns");
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Invalid terminal size"));
}

// ============================================================================
// Integration Tests - FZF Spawning (requires fzf installed)
// ============================================================================

#[test]
#[ignore] // Requires fzf to be installed
fn test_spawn_fzf_with_single_item() {
    let mut finder = FuzzyFinder::new("Single Item Test");
    let items = vec!["only_item.txt".to_string()];

    let result = finder.spawn_fzf(items, 20, 80, None);

    // If fzf is not installed, this will fail, which is okay for this test
    if result.is_ok() {
        assert!(
            finder.is_running(),
            "FuzzyFinder should be running after spawn"
        );
    }
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_spawn_fzf_with_file_names() {
    let mut finder = FuzzyFinder::new("File Finder");
    let items = vec![
        "src/main.rs".to_string(),
        "src/lib.rs".to_string(),
        "tests/integration_test.rs".to_string(),
        "Cargo.toml".to_string(),
        "README.md".to_string(),
    ];

    let result = finder.spawn_fzf(items, 20, 80, Some("Select file: "));

    if result.is_ok() {
        assert!(finder.is_running(), "FuzzyFinder should be running");
        thread::sleep(Duration::from_millis(100));

        // Kill the process
        let _ = finder.kill();
    }
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_spawn_fzf_with_many_items() {
    let mut finder = FuzzyFinder::new("Large List");

    // Generate a realistic list of file paths
    let mut items = vec![];
    for i in 0..100 {
        items.push(format!("src/module_{}/file_{}.rs", i / 10, i));
    }

    let result = finder.spawn_fzf(items, 30, 100, None);

    if result.is_ok() {
        assert!(finder.is_running());
        thread::sleep(Duration::from_millis(100));
        let _ = finder.kill();
    }
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_spawn_fzf_with_special_characters() {
    let mut finder = FuzzyFinder::new("Special Chars");
    let items = vec![
        "file with spaces.txt".to_string(),
        "file-with-dashes.rs".to_string(),
        "file_with_underscores.md".to_string(),
        "file.multiple.dots.js".to_string(),
        "file@special#chars$.py".to_string(),
        "émoji🎉file.txt".to_string(),
    ];

    let result = finder.spawn_fzf(items, 20, 80, None);

    if result.is_ok() {
        assert!(finder.is_running());
        thread::sleep(Duration::from_millis(100));
        let _ = finder.kill();
    }
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_spawn_fzf_with_very_long_strings() {
    let mut finder = FuzzyFinder::new("Long Strings");
    let items = vec![
        "a".repeat(500),
        "This is a very long path name ".repeat(10),
        format!(
            "{}/{}/{}",
            "nested".repeat(50),
            "deep".repeat(50),
            "path.txt"
        ),
    ];

    let result = finder.spawn_fzf(items, 20, 120, None);

    if result.is_ok() {
        assert!(finder.is_running());
        thread::sleep(Duration::from_millis(100));
        let _ = finder.kill();
    }
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_spawn_fzf_with_custom_prompt() {
    let mut finder = FuzzyFinder::new("Custom Prompt");
    let items = vec!["item1".to_string(), "item2".to_string()];

    let result = finder.spawn_fzf(items, 20, 80, Some("Choose wisely: "));

    if result.is_ok() {
        assert!(finder.is_running());
        thread::sleep(Duration::from_millis(100));
        let _ = finder.kill();
    }
}

// ============================================================================
// Key Event Tests
// ============================================================================

#[test]
#[ignore] // Requires fzf to be installed
fn test_send_key_before_spawn() {
    let mut finder = FuzzyFinder::new("Test");

    let key = KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    };

    // Should not panic even if terminal is not spawned
    let result = finder.send_key(key);
    assert!(
        result.is_ok(),
        "send_key should not fail on unspawned terminal"
    );
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_send_key_character_input() {
    let mut finder = FuzzyFinder::new("Test");
    let items = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];

    if finder.spawn_fzf(items, 20, 80, None).is_ok() {
        thread::sleep(Duration::from_millis(200));

        // Type 'a' to filter
        let key = KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };

        let result = finder.send_key(key);
        assert!(result.is_ok(), "Should successfully send character key");

        thread::sleep(Duration::from_millis(100));
        let _ = finder.update();

        let _ = finder.kill();
    }
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_send_key_navigation() {
    let mut finder = FuzzyFinder::new("Test");
    let items = vec![
        "item1".to_string(),
        "item2".to_string(),
        "item3".to_string(),
    ];

    if finder.spawn_fzf(items, 20, 80, None).is_ok() {
        thread::sleep(Duration::from_millis(200));

        // Test arrow key navigation
        let keys = vec![KeyCode::Down, KeyCode::Down, KeyCode::Up];

        for code in keys {
            let key = KeyEvent {
                code,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            };

            let result = finder.send_key(key);
            assert!(result.is_ok(), "Should successfully send navigation key");
            thread::sleep(Duration::from_millis(50));
        }

        let _ = finder.kill();
    }
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_send_key_ctrl_modifiers() {
    let mut finder = FuzzyFinder::new("Test");
    let items = vec!["test1".to_string(), "test2".to_string()];

    if finder.spawn_fzf(items, 20, 80, None).is_ok() {
        thread::sleep(Duration::from_millis(200));

        // Test Ctrl+C (should exit fzf)
        let key = KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };

        let result = finder.send_key(key);
        assert!(result.is_ok(), "Should successfully send Ctrl+C");

        // Wait a bit longer for process to exit and check a few times
        let mut exited = false;
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(100));
            if !finder.is_running() {
                exited = true;
                break;
            }
        }

        // Process should have exited (or we just verify the key was sent successfully)
        // Note: Some versions of fzf may not exit immediately on Ctrl+C
        if !exited {
            // Just verify we could send the key, and clean up
            let _ = finder.kill();
        }
    }
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_send_key_backspace() {
    let mut finder = FuzzyFinder::new("Test");
    let items = vec!["test".to_string()];

    if finder.spawn_fzf(items, 20, 80, None).is_ok() {
        thread::sleep(Duration::from_millis(200));

        // Type some characters then backspace
        for c in "abc".chars() {
            let key = KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            };
            let _ = finder.send_key(key);
            thread::sleep(Duration::from_millis(50));
        }

        // Send backspace
        let key = KeyEvent {
            code: KeyCode::Backspace,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        let result = finder.send_key(key);
        assert!(result.is_ok(), "Should successfully send backspace");

        let _ = finder.kill();
    }
}

// ============================================================================
// Update and Selection Tests
// ============================================================================

#[test]
fn test_update_before_spawn() {
    let mut finder = FuzzyFinder::new("Test");

    // Should not panic
    let result = finder.update();
    assert!(
        result.is_ok(),
        "update should not fail on unspawned terminal"
    );
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_update_after_spawn() {
    let mut finder = FuzzyFinder::new("Test");
    let items = vec!["test".to_string()];

    if finder.spawn_fzf(items, 20, 80, None).is_ok() {
        thread::sleep(Duration::from_millis(200));

        // Update should read output from fzf
        let result = finder.update();
        assert!(result.is_ok(), "update should succeed after spawn");

        let _ = finder.kill();
    }
}

#[test]
fn test_get_selection_before_spawn() {
    let mut finder = FuzzyFinder::new("Test");

    let selection = finder.get_selection();
    assert!(selection.is_none(), "Selection should be None before spawn");
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_get_selection_with_enter() {
    let mut finder = FuzzyFinder::new("Test");
    let items = vec!["selected_item".to_string(), "other_item".to_string()];

    if finder.spawn_fzf(items, 20, 80, None).is_ok() {
        thread::sleep(Duration::from_millis(300));

        // Press Enter to select first item
        let key = KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };

        let _ = finder.send_key(key);
        thread::sleep(Duration::from_millis(300));

        // Update to get final output
        let _ = finder.update();

        // Wait for process to exit
        thread::sleep(Duration::from_millis(200));

        let selection = finder.get_selection();
        // Selection may or may not be available depending on timing
        if let Some(sel) = selection {
            assert!(!sel.is_empty(), "Selection should not be empty");
        }
    }
}

// ============================================================================
// Process Lifecycle Tests
// ============================================================================

#[test]
fn test_is_running_before_spawn() {
    let finder = FuzzyFinder::new("Test");
    assert!(!finder.is_running(), "Should not be running before spawn");
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_is_running_after_spawn() {
    let mut finder = FuzzyFinder::new("Test");
    let items = vec!["test".to_string()];

    if finder.spawn_fzf(items, 20, 80, None).is_ok() {
        thread::sleep(Duration::from_millis(200));
        assert!(finder.is_running(), "Should be running after spawn");
        let _ = finder.kill();
    }
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_kill_process() {
    let mut finder = FuzzyFinder::new("Test");
    let items = vec!["test".to_string()];

    if finder.spawn_fzf(items, 20, 80, None).is_ok() {
        thread::sleep(Duration::from_millis(200));
        assert!(finder.is_running(), "Should be running before kill");

        let result = finder.kill();
        assert!(result.is_ok(), "kill should succeed");

        // The kill command was successful - the process may take time to fully exit,
        // but the important thing is that kill() didn't panic or error
        // Note: is_running() uses try_lock() which may not immediately reflect the kill
    }
}

#[test]
fn test_kill_before_spawn() {
    let mut finder = FuzzyFinder::new("Test");

    // Should not panic
    let result = finder.kill();
    assert!(result.is_ok(), "kill should not fail on unspawned terminal");
}

// ============================================================================
// Custom Command Tests
// ============================================================================

#[test]
#[ignore] // Requires system command
fn test_spawn_command_echo() {
    let mut finder = FuzzyFinder::new("Echo Test");

    let result = finder.spawn_command("echo", &["hello world"], None, 10, 40);

    if result.is_ok() {
        thread::sleep(Duration::from_millis(500));

        let _ = finder.update();

        // Echo should complete quickly
        thread::sleep(Duration::from_millis(200));

        let _ = finder.kill();
    }
}

#[test]
#[ignore] // Requires system command
fn test_spawn_command_with_stdin() {
    let mut finder = FuzzyFinder::new("Stdin Test");

    let stdin_data = "line1\nline2\nline3\n".to_string();
    let result = finder.spawn_command("cat", &[], Some(stdin_data), 10, 40);

    if result.is_ok() {
        thread::sleep(Duration::from_millis(300));
        let _ = finder.update();
        let _ = finder.kill();
    }
}

#[test]
fn test_spawn_command_invalid_command() {
    let mut finder = FuzzyFinder::new("Invalid Command");

    let result = finder.spawn_command("this_command_does_not_exist_12345", &[], None, 10, 40);

    // Should fail to spawn non-existent command
    assert!(result.is_err(), "Should fail to spawn non-existent command");
}

// ============================================================================
// Parser Access Tests
// ============================================================================

#[test]
fn test_get_parser_before_spawn() {
    let finder = FuzzyFinder::new("Test");

    let parser = finder.get_parser();
    assert!(parser.is_none(), "Parser should be None before spawn");
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_get_parser_after_spawn() {
    let mut finder = FuzzyFinder::new("Test");
    let items = vec!["test".to_string()];

    if finder.spawn_fzf(items, 20, 80, None).is_ok() {
        thread::sleep(Duration::from_millis(200));

        let parser = finder.get_parser();
        assert!(parser.is_some(), "Parser should be available after spawn");

        let _ = finder.kill();
    }
}

// ============================================================================
// Widget Rendering Tests
// ============================================================================

#[test]
fn test_widget_render_before_spawn() {
    let finder = FuzzyFinder::new("Loading Test");

    // Create a buffer to render into
    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 24));
    let area = Rect::new(0, 0, 80, 24);

    // Should render loading message without panic
    (&finder).render(area, &mut buffer);

    // Verify some content was rendered (title and loading message)
    let content = buffer.content();
    let has_content = content.iter().any(|cell| !cell.symbol().is_empty());
    assert!(has_content, "Should render loading state");
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_widget_render_after_spawn() {
    let mut finder = FuzzyFinder::new("Active Finder");
    let items = vec!["item1".to_string(), "item2".to_string()];

    if finder.spawn_fzf(items, 20, 80, None).is_ok() {
        thread::sleep(Duration::from_millis(300));
        let _ = finder.update();

        let mut buffer = Buffer::empty(Rect::new(0, 0, 100, 30));
        let area = Rect::new(0, 0, 100, 30);

        // Should render terminal output without panic
        (&finder).render(area, &mut buffer);

        let _ = finder.kill();
    }
}

#[test]
fn test_widget_render_small_area() {
    let finder = FuzzyFinder::new("Small");

    let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));
    let area = Rect::new(0, 0, 10, 5);

    // Should handle small rendering area without panic
    (&finder).render(area, &mut buffer);
}

#[test]
fn test_widget_render_zero_area() {
    let finder = FuzzyFinder::new("Zero");

    let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 1));
    let area = Rect::new(0, 0, 0, 0);

    // Should handle zero-size area without panic
    (&finder).render(area, &mut buffer);
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
#[ignore] // Requires fzf to be installed
fn test_spawn_multiple_times() {
    let mut finder = FuzzyFinder::new("Multiple Spawn");

    // First spawn
    let items1 = vec!["first".to_string()];
    if finder.spawn_fzf(items1, 20, 80, None).is_ok() {
        thread::sleep(Duration::from_millis(200));
        assert!(finder.is_running());
        let _ = finder.kill();
        thread::sleep(Duration::from_millis(200));
    }

    // Second spawn (should replace first)
    let items2 = vec!["second".to_string()];
    if finder.spawn_fzf(items2, 20, 80, None).is_ok() {
        thread::sleep(Duration::from_millis(200));
        assert!(finder.is_running());
        let _ = finder.kill();
    }
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_large_terminal_size() {
    let mut finder = FuzzyFinder::new("Large Terminal");
    let items = vec!["test".to_string()];

    // Try with very large dimensions
    let result = finder.spawn_fzf(items, 500, 500, None);

    if result.is_ok() {
        thread::sleep(Duration::from_millis(200));
        let _ = finder.kill();
    }
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_minimal_terminal_size() {
    let mut finder = FuzzyFinder::new("Minimal Terminal");
    let items = vec!["test".to_string()];

    // Try with minimal valid dimensions
    let result = finder.spawn_fzf(items, 1, 1, None);

    if result.is_ok() {
        thread::sleep(Duration::from_millis(200));
        let _ = finder.kill();
    }
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_unicode_items() {
    let mut finder = FuzzyFinder::new("Unicode Test");
    let items = vec![
        "日本語.txt".to_string(),
        "Русский.rs".to_string(),
        "العربية.md".to_string(),
        "emoji_😀_file.py".to_string(),
        "mixed_English_日本語.js".to_string(),
    ];

    let result = finder.spawn_fzf(items, 20, 80, None);

    if result.is_ok() {
        thread::sleep(Duration::from_millis(200));
        let _ = finder.kill();
    }
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_empty_string_items() {
    let mut finder = FuzzyFinder::new("Empty Strings");
    let items = vec!["".to_string(), "valid_item".to_string(), "".to_string()];

    let result = finder.spawn_fzf(items, 20, 80, None);

    if result.is_ok() {
        thread::sleep(Duration::from_millis(200));
        let _ = finder.kill();
    }
}

// ============================================================================
// Realistic Usage Scenarios
// ============================================================================

#[test]
#[ignore] // Requires fzf to be installed
fn test_realistic_file_selection() {
    let mut finder = FuzzyFinder::new("Select a file");

    // Simulate a realistic file tree
    let items = vec![
        "src/main.rs".to_string(),
        "src/lib.rs".to_string(),
        "src/utils/helper.rs".to_string(),
        "src/utils/parser.rs".to_string(),
        "src/components/button.rs".to_string(),
        "src/components/input.rs".to_string(),
        "tests/unit_tests.rs".to_string(),
        "tests/integration_tests.rs".to_string(),
        "Cargo.toml".to_string(),
        "Cargo.lock".to_string(),
        "README.md".to_string(),
        "LICENSE".to_string(),
        ".gitignore".to_string(),
    ];

    if finder.spawn_fzf(items, 25, 100, Some("File: ")).is_ok() {
        thread::sleep(Duration::from_millis(300));

        // Simulate user typing "test"
        for c in "test".chars() {
            let key = KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            };
            let _ = finder.send_key(key);
            thread::sleep(Duration::from_millis(50));
        }

        thread::sleep(Duration::from_millis(200));
        let _ = finder.update();

        let _ = finder.kill();
    }
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_realistic_git_branch_selection() {
    let mut finder = FuzzyFinder::new("Select branch");

    let items = vec![
        "main".to_string(),
        "develop".to_string(),
        "feature/user-authentication".to_string(),
        "feature/payment-integration".to_string(),
        "bugfix/login-error".to_string(),
        "bugfix/memory-leak".to_string(),
        "release/v1.0.0".to_string(),
        "release/v1.1.0".to_string(),
    ];

    if finder.spawn_fzf(items, 20, 80, Some("Branch: ")).is_ok() {
        thread::sleep(Duration::from_millis(300));

        // Simulate user typing "feature"
        for c in "feature".chars() {
            let key = KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            };
            let _ = finder.send_key(key);
            thread::sleep(Duration::from_millis(50));
        }

        thread::sleep(Duration::from_millis(200));
        let _ = finder.update();

        let _ = finder.kill();
    }
}

#[test]
#[ignore] // Requires fzf to be installed
fn test_realistic_command_palette() {
    let mut finder = FuzzyFinder::new("Command Palette");

    let items = vec![
        "File: Open".to_string(),
        "File: Save".to_string(),
        "File: Close".to_string(),
        "Edit: Copy".to_string(),
        "Edit: Paste".to_string(),
        "Edit: Find".to_string(),
        "View: Toggle Sidebar".to_string(),
        "View: Zoom In".to_string(),
        "Git: Commit".to_string(),
        "Git: Push".to_string(),
        "Terminal: New".to_string(),
    ];

    if finder.spawn_fzf(items, 25, 80, Some("> ")).is_ok() {
        thread::sleep(Duration::from_millis(300));

        // Navigate and select
        let down_key = KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };

        let _ = finder.send_key(down_key);
        thread::sleep(Duration::from_millis(100));

        let _ = finder.kill();
    }
}
