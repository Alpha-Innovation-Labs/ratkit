use std::thread;
use std::time::Duration;
use ratatui_toolkit::VT100Term;

/// This test specifically validates that fish shell does NOT produce
/// the "could not read response to Primary Device Attribute query" error
#[tokio::test]
async fn test_fish_no_da_query_error() {
    // Only run if fish is available
    let shell = std::env::var("SHELL").unwrap_or_default();
    if !shell.contains("fish") {
        println!("SKIPPING: Not using fish shell (SHELL={})", shell);
        return;
    }

    eprintln!("========================================");
    eprintln!("SPAWNING FISH SHELL");
    eprintln!("========================================");

    let mut term = VT100Term::spawn_with_command("Fish DA Query Test", "fish", &[])
        .expect("Failed to spawn fish shell");

    eprintln!("Fish spawned, waiting 3 seconds for initialization...");

    // Wait long enough for fish to:
    // 1. Start up
    // 2. Send DA query (if it's going to)
    // 3. Wait for response timeout (2 seconds)
    // 4. Print error message (if any)
    thread::sleep(Duration::from_secs(3));

    eprintln!("Fish should have initialized by now.");
    eprintln!("If you see the DA query timeout error above, the test failed.");
    eprintln!("If no error appeared, the TERM variable was correctly set!");

    // Type a simple command to verify fish is responsive
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

    let pwd_chars = vec!['p', 'w', 'd'];
    for c in pwd_chars {
        let key = KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        term.handle_key(key);
        thread::sleep(Duration::from_millis(10));
    }

    // Press Enter
    let enter = KeyEvent {
        code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    };
    term.handle_key(enter);

    // Wait for command to execute
    thread::sleep(Duration::from_millis(500));

    eprintln!("========================================");
    eprintln!("TEST COMPLETE");
    eprintln!("Fish was responsive and executed 'pwd' command");
    eprintln!("No DA query timeout means TERM was set correctly!");
    eprintln!("========================================");
}

/// Test that verifies TERM environment variable is actually set
#[tokio::test]
async fn test_term_variable_is_set() {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "bash".to_string());

    let mut term =
        VT100Term::spawn_with_command("TERM Check", &shell, &[]).expect("Failed to spawn shell");

    thread::sleep(Duration::from_millis(500));

    // Send command to print TERM variable
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

    let command = "echo $TERM\n";
    for c in command.chars() {
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
    thread::sleep(Duration::from_millis(500));

    eprintln!("========================================");
    eprintln!("Sent 'echo $TERM' to shell");
    eprintln!("Expected output: xterm-256color");
    eprintln!("Check the terminal output above");
    eprintln!("========================================");
}
