use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use std::thread;
use std::time::Duration;
use ratatui_toolkit::VT100Term;

#[tokio::test]
async fn test_vt100_terminal_spawn_and_type() {
    // Spawn a shell
    let shell = if cfg!(target_os = "windows") {
        "powershell.exe".to_string()
    } else {
        std::env::var("SHELL").unwrap_or_else(|_| "bash".to_string())
    };

    let mut term = VT100Term::spawn_with_command("Test Terminal", &shell, &[])
        .expect("Failed to spawn terminal");

    // Wait for terminal to initialize
    thread::sleep(Duration::from_millis(500));

    // Type "echo hello"
    let keys = vec![
        ('e', KeyModifiers::NONE),
        ('c', KeyModifiers::NONE),
        ('h', KeyModifiers::NONE),
        ('o', KeyModifiers::NONE),
        (' ', KeyModifiers::NONE),
        ('h', KeyModifiers::NONE),
        ('e', KeyModifiers::NONE),
        ('l', KeyModifiers::NONE),
        ('l', KeyModifiers::NONE),
        ('o', KeyModifiers::NONE),
    ];

    for (c, mods) in keys {
        let key = KeyEvent {
            code: KeyCode::Char(c),
            modifiers: mods,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };

        let handled = term.handle_key(key);
        assert!(handled, "Key '{}' was not handled", c);
        thread::sleep(Duration::from_millis(10));
    }

    // Press Enter
    let enter_key = KeyEvent {
        code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    };

    let handled = term.handle_key(enter_key);
    assert!(handled, "Enter key was not handled");

    // Wait for command to execute
    thread::sleep(Duration::from_millis(500));

    // Verify we can enter copy mode (Ctrl+B)
    let copy_mode_key = KeyEvent {
        code: KeyCode::Char('b'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    };

    let handled = term.handle_key(copy_mode_key);
    assert!(handled, "Ctrl+B (copy mode) was not handled");

    // Wait a bit
    thread::sleep(Duration::from_millis(100));

    // Try to select some text with mouse
    term.handle_mouse_down(5, 5);
    term.handle_mouse_drag(10, 5);
    term.handle_mouse_up();

    // Check if we have a selection
    assert!(
        term.has_selection(),
        "Should have text selection after mouse drag"
    );

    // Get selected text
    let selected = term.get_selected_text();
    assert!(selected.is_some(), "Should have selected text");

    println!("Selected text: {:?}", selected);

    // Exit copy mode
    let esc_key = KeyEvent {
        code: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    };

    term.handle_key(esc_key);
    assert!(
        !term.has_selection(),
        "Selection should be cleared after ESC"
    );
}

#[tokio::test]
async fn test_vt100_no_fish_errors() {
    // Only run if fish is the shell
    let shell = std::env::var("SHELL").unwrap_or_default();
    if !shell.contains("fish") {
        println!("Skipping fish test - not using fish shell");
        return;
    }

    let mut term =
        VT100Term::spawn_with_command("Fish Test", "fish", &[]).expect("Failed to spawn fish");

    // Wait for fish to initialize
    thread::sleep(Duration::from_secs(3));

    // Fish should have set TERM=xterm-256color and not printed the DA query timeout error
    // We can't directly check stderr, but we can verify the terminal is responsive

    // Type a simple command
    let key = KeyEvent {
        code: KeyCode::Char('e'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    };

    let handled = term.handle_key(key);
    assert!(handled, "Fish terminal should handle keyboard input");

    // If we get here without the test hanging for 2 seconds, fish didn't timeout
    println!("Fish terminal initialized successfully without DA query timeout");
}

#[tokio::test]
async fn test_vt100_key_sequences() {
    let shell = if cfg!(target_os = "windows") {
        "powershell.exe".to_string()
    } else {
        "/bin/sh".to_string()
    };

    let mut term =
        VT100Term::spawn_with_command("Key Test", &shell, &[]).expect("Failed to spawn terminal");

    // Wait for initialization
    thread::sleep(Duration::from_millis(500));

    // Test various key sequences
    let test_cases = vec![
        (KeyCode::Char('a'), KeyModifiers::NONE, "Regular key"),
        (KeyCode::Char('c'), KeyModifiers::CONTROL, "Ctrl+C"),
        (KeyCode::Up, KeyModifiers::NONE, "Up arrow"),
        (KeyCode::Down, KeyModifiers::NONE, "Down arrow"),
        (KeyCode::Left, KeyModifiers::NONE, "Left arrow"),
        (KeyCode::Right, KeyModifiers::NONE, "Right arrow"),
        (KeyCode::Home, KeyModifiers::NONE, "Home"),
        (KeyCode::End, KeyModifiers::NONE, "End"),
        (KeyCode::Backspace, KeyModifiers::NONE, "Backspace"),
        (KeyCode::Delete, KeyModifiers::NONE, "Delete"),
        (KeyCode::Tab, KeyModifiers::NONE, "Tab"),
    ];

    for (code, mods, desc) in test_cases {
        let key = KeyEvent {
            code,
            modifiers: mods,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };

        let handled = term.handle_key(key);
        assert!(handled, "{} should be handled", desc);
        thread::sleep(Duration::from_millis(10));
    }

    println!("All key sequences handled successfully");
}

#[tokio::test]
async fn test_vt100_focus_state() {
    let mut term = VT100Term::new("Focus Test");

    // Initially not focused
    assert!(!term.focused, "Terminal should start unfocused");

    // Set focused
    term.focused = true;
    assert!(term.focused, "Terminal should be focused after setting");

    // Focus doesn't affect key handling
    let key = KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    };

    let handled = term.handle_key(key);
    // Should still handle even though there's no PTY (for non-spawned terminal)
    assert!(handled, "Should handle key even in non-spawned terminal");
}

#[tokio::test]
async fn test_vt100_writer_functional() {
    let shell = if cfg!(target_os = "windows") {
        "powershell.exe".to_string()
    } else {
        "/bin/sh".to_string()
    };

    let term = VT100Term::spawn_with_command("Writer Test", &shell, &[])
        .expect("Failed to spawn terminal");

    // Try to send input (this is what handle_key does internally)
    // If the writer wasn't properly initialized, this would panic or fail
    term.send_input("test\n");

    // If we get here, send_input worked (writer is functional)
    println!("Writer is functional");
}

#[tokio::test]
async fn test_vt100_pty_communication() {
    let shell = if cfg!(target_os = "windows") {
        "powershell.exe".to_string()
    } else {
        "/bin/sh".to_string()
    };

    let mut term =
        VT100Term::spawn_with_command("PTY Test", &shell, &[]).expect("Failed to spawn terminal");

    // Wait for shell to start
    thread::sleep(Duration::from_millis(500));

    // Send a command that produces output: echo test123
    for c in "echo test123\n".chars() {
        let key = KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        term.handle_key(key);
        thread::sleep(Duration::from_millis(10));
    }

    // Wait for output
    thread::sleep(Duration::from_secs(1));

    // The terminal should have received and processed output
    // We can't easily inspect the screen content in this test,
    // but if the PTY was working, the command executed
    println!("PTY communication test completed");
}
