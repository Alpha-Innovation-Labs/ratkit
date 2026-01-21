//! End-to-end tests for TermTui terminal emulator
//!
//! These tests spawn actual terminal sessions and verify:
//! - Terminal creation and shell spawning
//! - Command execution (ls, echo, pwd)
//! - Output rendering and display
//! - Copy mode functionality
//! - Keyboard input handling

#![cfg(feature = "terminal")]

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui_toolkit::termtui::TermTui;
use std::time::Duration;

/// Helper function to get text from terminal screen
fn get_screen_text(term: &TermTui) -> Vec<String> {
    let parser = term.parser.lock().unwrap();
    let screen = parser.screen();
    let grid = screen.primary_grid();
    let size = grid.size();

    let mut lines = Vec::new();
    for row_idx in 0..size.rows {
        if let Some(row) = grid.visible_row(row_idx) {
            let line_text: String = (0..size.cols)
                .filter_map(|col| {
                    row.get(col).and_then(|cell| {
                        let text = cell.text();
                        if text.is_empty() || cell.is_wide_continuation() {
                            None
                        } else {
                            Some(text)
                        }
                    })
                })
                .collect();
            if !line_text.trim().is_empty() {
                lines.push(line_text.trim().to_string());
            }
        }
    }
    lines
}

/// Wait for a specific string to appear in the terminal output
fn wait_for_output(term: &TermTui, expected: &str, timeout_ms: u64) -> bool {
    let start = std::time::Instant::now();
    while start.elapsed() < Duration::from_millis(timeout_ms) {
        let lines = get_screen_text(term);
        for line in &lines {
            if line.contains(expected) {
                return true;
            }
        }
        tokio::time::sleep(Duration::from_millis($1)).await;
    }
    false
}

/// Test 1: Terminal creation and shell spawning
#[tokio::test]
async fn test_terminal_spawn_and_type() {
    // Spawn a shell
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let mut term = TermTui::spawn_with_command("Test Terminal", &shell, &[])
        .expect("Failed to spawn terminal");

    // Wait for shell prompt to appear
    let prompt_found = wait_for_output(&term, "$", 2000);
    assert!(prompt_found, "Shell prompt should appear");

    // Type 'echo' command
    let command = "echo";
    for ch in command.chars() {
        let key = KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE);
        term.handle_key(key);
        tokio::time::sleep(Duration::from_millis($1)).await;
    }

    // Press Enter
    let enter = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
    term.handle_key(enter);

    // Wait for echo output
    tokio::time::sleep(Duration::from_millis($1)).await;
}

/// Test 2: Execute 'echo' command and verify output
#[tokio::test]
async fn test_echo_command() {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let mut term =
        TermTui::spawn_with_command("Echo Test", &shell, &[]).expect("Failed to spawn terminal");

    // Wait for prompt
    tokio::time::sleep(Duration::from_millis($1)).await;

    // Type 'echo hello world'
    let command = "echo hello world";
    for ch in command.chars() {
        let key = KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE);
        term.handle_key(key);
        tokio::time::sleep(Duration::from_millis($1)).await;
    }

    term.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    // Wait for output
    let found = wait_for_output(&term, "hello world", 1000);
    assert!(found, "Expected 'hello world' in terminal output");

    // Verify output is present
    let lines = get_screen_text(&term);
    assert!(
        lines.iter().any(|l| l.contains("hello world")),
        "Output should contain 'hello world'"
    );
}

/// Test 3: Execute 'pwd' command
#[tokio::test]
async fn test_pwd_command() {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let mut term =
        TermTui::spawn_with_command("Pwd Test", &shell, &[]).expect("Failed to spawn terminal");

    tokio::time::sleep(Duration::from_millis($1)).await;

    // Type 'pwd'
    let command = "pwd";
    for ch in command.chars() {
        let key = KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE);
        term.handle_key(key);
        tokio::time::sleep(Duration::from_millis($1)).await;
    }

    term.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    // Wait for output (any non-empty line)
    tokio::time::sleep(Duration::from_millis($1)).await;

    let lines = get_screen_text(&term);
    assert!(!lines.is_empty(), "pwd command should produce output");

    // Should see current directory path
    let has_path = lines.iter().any(|l| l.starts_with('/') || l.contains(':'));
    assert!(has_path, "pwd should show directory path, got: {:?}", lines);
}

/// Test 4: Execute 'ls' command
#[tokio::test]
async fn test_ls_command() {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let mut term = TermTui::spawn_with_command("Ls Test", &shell, &[])
        .expect("Failed to spawn terminal");

    tokio::time::sleep(Duration::from_millis(500)).await;

    // Type 'ls'
    let command = "ls";
    for ch in command.chars() {
        let key = KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE);
        term.handle_key(key);
        tokio::time::sleep(Duration::from_millis($1)).await;
    }

    term.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    // Wait for output
    tokio::time::sleep(Duration::from_millis($1)).await;

    let lines = get_screen_text(&term);
    assert!(!lines.is_empty(), "ls command should produce output");

    // Common files/directories that should exist
    let expected_items = vec!["Cargo.toml", "src", "tests", "crates"];
    let has_any = expected_items
        .iter()
        .any(|item| lines.iter().any(|line| line.contains(item)));
    assert!(
        has_any,
        "ls should show some known files/dirs, got: {:?}",
        lines
    );
}

/// Test 5: Multiple commands in sequence
#[tokio::test]\nasync fn test_$1() {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let mut term = TermTui::spawn_with_command("Multi Command", &shell, &[])
        .expect("Failed to spawn terminal");

    // Wait for prompt
    tokio::time::sleep(Duration::from_millis($1)).await;

    // Command 1: echo test1
    let cmd1 = "echo test1";
    for ch in cmd1.chars() {
        term.handle_key(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE));
        tokio::time::sleep(Duration::from_millis($1)).await;
    }
    term.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    tokio::time::sleep(Duration::from_millis($1)).await;

    // Command 2: echo test2
    let cmd2 = "echo test2";
    for ch in cmd2.chars() {
        term.handle_key(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE));
        tokio::time::sleep(Duration::from_millis($1)).await;
    }
    term.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    // Wait for both outputs
    let found1 = wait_for_output(&term, "test1", 1000);
    let found2 = wait_for_output(&term, "test2", 1000);

    assert!(found1, "Should find 'test1' in output");
    assert!(found2, "Should find 'test2' in output");
}

/// Test 6: Copy mode activation
#[tokio::test]\nasync fn test_$1() {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let mut term =
        TermTui::spawn_with_command("Copy Mode", &shell, &[]).expect("Failed to spawn terminal");

    tokio::time::sleep(Duration::from_millis($1)).await;

    // Verify not in copy mode initially
    assert!(
        !term.copy_mode.is_active(),
        "Should not be in copy mode initially"
    );

    // Enter copy mode with Ctrl+X
    term.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::CONTROL));

    // Should now be in copy mode
    assert!(term.copy_mode.is_active(), "Should enter copy mode");

    // Exit copy mode with Esc
    term.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

    // Should no longer be in copy mode
    assert!(!term.copy_mode.is_active(), "Should exit copy mode");
}

/// Test 7: Copy mode navigation
#[tokio::test]\nasync fn test_$1() {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let mut term =
        TermTui::spawn_with_command("Copy Nav", &shell, &[]).expect("Failed to spawn terminal");

    tokio::time::sleep(Duration::from_millis($1)).await;

    // Enter copy mode
    term.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::CONTROL));
    assert!(term.copy_mode.is_active());

    // Navigate down
    term.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

    // Navigate up
    term.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));

    // Navigate right
    term.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));

    // Navigate left
    term.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));

    // Should still be in copy mode
    assert!(
        term.copy_mode.is_active(),
        "Should remain in copy mode after navigation"
    );

    // Exit
    term.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
}

/// Test 8: Test with different commands
#[tokio::test]\nasync fn test_$1() {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let mut term =
        TermTui::spawn_with_command("Various Cmds", &shell, &[]).expect("Failed to spawn terminal");

    tokio::time::sleep(Duration::from_millis($1)).await;

    // Test date command
    let command = "date";
    for ch in command.chars() {
        term.handle_key(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE));
        tokio::time::sleep(Duration::from_millis($1)).await;
    }
    term.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    // Wait for date output (should contain numbers or weekday)
    let found = wait_for_output(&term, "202", 2000)
        || wait_for_output(&term, "Mon", 2000)
        || wait_for_output(&term, "Tue", 2000)
        || wait_for_output(&term, "Wed", 2000)
        || wait_for_output(&term, "Thu", 2000)
        || wait_for_output(&term, "Fri", 2000)
        || wait_for_output(&term, "Sat", 2000)
        || wait_for_output(&term, "Sun", 2000);

    assert!(found, "date command should produce recognizable output");
}

/// Test 9: Terminal focus state
#[tokio::test]\nasync fn test_$1() {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let mut term =
        TermTui::spawn_with_command("Focus Test", &shell, &[]).expect("Failed to spawn terminal");

    // Initially not focused
    assert!(!term.focused, "Terminal should not be focused initially");

    // Set focused
    term.focused = true;
    assert!(term.focused, "Terminal should be focused");

    // Type while focused
    term.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    term.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    term.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    tokio::time::sleep(Duration::from_millis($1)).await;

    // Verify something was typed
    let lines = get_screen_text(&term);
    // There should be at least some content now
    assert!(
        !lines.is_empty(),
        "Terminal should have content after typing"
    );
}

/// Test 10: Error handling - invalid command
#[tokio::test]\nasync fn test_$1() {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let mut term =
        TermTui::spawn_with_command("Invalid Cmd", &shell, &[]).expect("Failed to spawn terminal");

    tokio::time::sleep(Duration::from_millis($1)).await;

    // Type an invalid command
    let command = "nonexistentcommand12345";
    for ch in command.chars() {
        term.handle_key(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE));
        tokio::time::sleep(Duration::from_millis($1)).await;
    }
    term.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    // Wait for error message
    tokio::time::sleep(Duration::from_millis($1)).await;

    // Different shells show errors differently, but there should be some response
    let lines = get_screen_text(&term);
    assert!(
        !lines.is_empty(),
        "Should have some response to invalid command"
    );
}
