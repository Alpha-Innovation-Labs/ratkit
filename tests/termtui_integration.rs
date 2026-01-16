//! Integration tests for TermTui

#![cfg(feature = "terminal")]

use ratatui_toolkit::termtui::TermTui;

#[tokio::test]
async fn test_spawn_echo() {
    // Spawn a terminal with echo command
    let term = TermTui::spawn_with_command("Test", "echo", &["hello"]).unwrap();

    // Wait for process to complete
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    // Terminal should exist
    assert_eq!(term.title, "Test");
}

#[tokio::test]
async fn test_spawn_shell_and_send_input() {
    // Spawn a shell
    let term = TermTui::spawn_with_command("Shell", "sh", &["-c", "cat"]).unwrap();

    // Wait for shell to start
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // Send input
    term.send_input("hello\n");

    // Wait for echo
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    assert_eq!(term.title, "Shell");
}

#[tokio::test]
async fn test_copy_mode_on_spawned_terminal() {
    let mut term = TermTui::spawn_with_command("Test", "echo", &["test output"]).unwrap();

    // Wait for output
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    // Enter copy mode
    term.enter_copy_mode();
    assert!(term.copy_mode.is_active());

    // Should be able to get cursor position
    let cursor = term.copy_mode.cursor();
    assert!(cursor.is_some());
}
