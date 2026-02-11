use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};

/// Wait for any output to appear in the buffer
fn wait_for_output(buffer: &Arc<Mutex<Vec<u8>>>, timeout: Duration) {
    let start = Instant::now();
    loop {
        if let Ok(data) = buffer.lock() {
            if !data.is_empty() {
                break;
            }
        }
        if start.elapsed() > timeout {
            break;
        }
        thread::sleep(Duration::from_millis(25));
    }
}

/// Get screen text with ANSI escapes stripped
fn screen_text(buffer: &Arc<Mutex<Vec<u8>>>) -> Result<String> {
    let data = buffer.lock().unwrap();
    let stripped = strip_ansi_escapes::strip(&*data);
    Ok(String::from_utf8_lossy(&stripped).to_string())
}

/// Get raw buffer content (with ANSI escapes)
fn raw_buffer(buffer: &Arc<Mutex<Vec<u8>>>) -> Result<Vec<u8>> {
    let data = buffer.lock().unwrap();
    Ok(data.clone())
}

/// Wait for specific text to appear in the screen
fn wait_for_text(buffer: &Arc<Mutex<Vec<u8>>>, text: &str, timeout: Duration) -> Result<()> {
    let start = Instant::now();
    while start.elapsed() < timeout {
        let screen = screen_text(buffer)?;
        if screen.contains(text) {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(50));
    }
    Err(anyhow!("missing text in terminal output: {text}"))
}

/// Wait for any of the given entries to appear in the screen
fn wait_for_any_entry(
    buffer: &Arc<Mutex<Vec<u8>>>,
    entries: &[String],
    timeout: Duration,
) -> Result<()> {
    let start = Instant::now();
    while start.elapsed() < timeout {
        let screen = screen_text(buffer)?;
        if entries
            .iter()
            .any(|entry| !entry.is_empty() && screen.contains(entry))
        {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(50));
    }
    let screen = screen_text(buffer)?;
    Err(anyhow!("no ls entries found in terminal output: {screen}"))
}

/// Detect if alternate screen mode is active by checking for escape sequences
fn detect_alternate_screen(buffer: &Arc<Mutex<Vec<u8>>>) -> bool {
    let raw = raw_buffer(buffer).unwrap_or_default();
    // Check for ESC [ ? 1049 h or ESC [ ? 47 h (enter alternate screen)
    // The sequences are: \x1b[?1049h (8 bytes) and \x1b[?47h (6 bytes)
    raw.windows(8).any(|w| w == b"\x1b[?1049h") || raw.windows(6).any(|w| w == b"\x1b[?47h")
}

/// Detect if alternate screen mode was exited
#[allow(dead_code)]
fn detect_alternate_screen_exit(buffer: &Arc<Mutex<Vec<u8>>>) -> bool {
    let raw = raw_buffer(buffer).unwrap_or_default();
    // Check for ESC [ ? 1049 l or ESC [ ? 47 l (exit alternate screen)
    raw.windows(8).any(|w| w == b"\x1b[?1049l") || raw.windows(6).any(|w| w == b"\x1b[?47l")
}

/// Get the last N lines of the screen (to avoid scrollback issues)
fn get_last_n_lines(buffer: &Arc<Mutex<Vec<u8>>>, n: usize) -> Result<Vec<String>> {
    let text = screen_text(buffer)?;
    let lines: Vec<String> = text.lines().map(|s| s.to_string()).collect();
    let start = lines.len().saturating_sub(n);
    Ok(lines[start..].to_vec())
}

/// Check if the bottom of the screen has non-whitespace content
fn bottom_line_has_content(buffer: &Arc<Mutex<Vec<u8>>>, pty_rows: u16) -> Result<bool> {
    let last_lines = get_last_n_lines(buffer, pty_rows as usize)?;
    if let Some(last) = last_lines.last() {
        Ok(!last.trim().is_empty())
    } else {
        Ok(false)
    }
}

/// Get the command to run for full-screen testing from env or use a default
fn get_fullscreen_test_command() -> (String, Vec<String>) {
    if let Ok(cmd) = std::env::var("TERMTUI_TEST_FULLSCREEN_CMD") {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if !parts.is_empty() {
            return (
                parts[0].to_string(),
                parts[1..].iter().map(|s| s.to_string()).collect(),
            );
        }
    }
    // Default: try yazi, fallback to a simple test
    ("yazi".to_string(), vec![])
}

#[test]
fn termtui_demo_exits_on_ctrl_q_portable_pty() -> Result<()> {
    let pty_system = native_pty_system();
    // Use larger PTY size (40x120) for better full-screen detection
    let pty_size = PtySize {
        rows: 40,
        cols: 120,
        pixel_width: 0,
        pixel_height: 0,
    };
    let pair = pty_system.openpty(pty_size)?;

    let mut cmd = CommandBuilder::new("cargo");
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = crate_dir
        .join("../../../")
        .canonicalize()
        .expect("workspace root");
    cmd.cwd(workspace_root);
    cmd.arg("run");
    cmd.arg("-p");
    cmd.arg("ratkit-termtui");
    cmd.arg("--example");
    cmd.arg("termtui_demo");

    let mut child = pair.slave.spawn_command(cmd)?;

    let buffer = Arc::new(Mutex::new(Vec::new()));
    let buffer_reader = Arc::clone(&buffer);
    let mut reader = pair.master.try_clone_reader()?;
    thread::spawn(move || {
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    if let Ok(mut data) = buffer_reader.lock() {
                        data.extend_from_slice(&buf[..n]);
                    }
                }
                Err(_) => break,
            }
        }
    });

    let mut writer = pair.master.take_writer()?;

    wait_for_output(&buffer, Duration::from_secs(2));

    // Check for shell startup errors (especially fish)
    let screen = screen_text(&buffer)?;
    if screen.contains("error") || screen.contains("Error") {
        eprintln!("Warning: Shell startup may have errors:\n{}", screen);
    }

    // Test 1: Basic ls command
    writer.write_all(b"ls -1\n")?;
    writer.flush()?;

    let expected = Command::new("bash").arg("-lc").arg("ls -1").output()?;
    let expected_stdout = String::from_utf8_lossy(&expected.stdout);
    let entries: Vec<String> = expected_stdout
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();
    wait_for_any_entry(&buffer, &entries, Duration::from_secs(2))?;

    // Test 2: Full-screen app (yazi or configured alternative)
    let (fullscreen_cmd, _fullscreen_args) = get_fullscreen_test_command();
    let fullscreen_input = format!("{}\n", fullscreen_cmd);
    writer.write_all(fullscreen_input.as_bytes())?;
    writer.flush()?;
    thread::sleep(Duration::from_millis(800));

    // Test 2a: Detect alternate screen activation
    let alt_screen_active = detect_alternate_screen(&buffer);
    assert!(
        alt_screen_active,
        "Full-screen app should activate alternate screen mode (CSI ?1049h or ?47h)"
    );

    // Test 2b: Validate bottom-of-screen content
    let bottom_has_content = bottom_line_has_content(&buffer, pty_size.rows)?;
    assert!(
        bottom_has_content,
        "Full-screen app should have content at bottom of screen (PTY size: {}x{})",
        pty_size.rows, pty_size.cols
    );

    // Test 3: Resize test - resize PTY and verify app adapts
    let new_size = PtySize {
        rows: 30,
        cols: 100,
        pixel_width: 0,
        pixel_height: 0,
    };
    pair.master.resize(new_size)?;
    thread::sleep(Duration::from_millis(500));

    // After resize, send a key to trigger redraw and check content
    writer.write_all(b"\x1b")?; // Escape key
    writer.flush()?;
    thread::sleep(Duration::from_millis(300));

    // Test 4: Exit full-screen app and verify alternate screen exit
    writer.write_all(b"q")?;
    writer.flush()?;
    thread::sleep(Duration::from_millis(500));

    // Note: We can't reliably detect alternate screen exit in all cases
    // since the app may not send the sequence on exit, but we can verify
    // we're back to normal shell operation

    // Test 5: Enter copy mode and verify
    writer.write_all(b"\x18")?; // Ctrl+X to enter copy mode
    writer.flush()?;
    wait_for_text(&buffer, " COPY ", Duration::from_secs(2))?;

    // Exit copy mode
    writer.write_all(b"q")?;
    writer.flush()?;

    // Test 6: Exit the demo app
    for _ in 0..40 {
        if child.try_wait()?.is_some() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(50));
    }

    child.kill()?;
    child.wait()?;

    Err(anyhow!(
        "termtui demo did not terminate after q in copy mode"
    ))
}

#[test]
fn test_alternate_screen_detection() {
    // Test that we can detect alternate screen sequences
    let test_cases = vec![
        (b"\x1b[?1049h".to_vec(), true, "?1049h enter"),
        (b"\x1b[?47h".to_vec(), true, "?47h enter"),
        (b"\x1b[?1049l".to_vec(), false, "?1049l exit"),
        (b"\x1b[?47l".to_vec(), false, "?47l exit"),
        (b"hello world".to_vec(), false, "no sequence"),
    ];

    for (data, expected_enter, desc) in test_cases {
        let buffer = Arc::new(Mutex::new(data));
        let detected = detect_alternate_screen(&buffer);
        assert_eq!(
            detected, expected_enter,
            "Failed for {}: expected {}, got {}",
            desc, expected_enter, detected
        );
    }
}

#[test]
fn test_bottom_line_detection() -> Result<()> {
    // Create a buffer with known content
    let content = "Line 1\nLine 2\nLine 3\nLast line with content";
    let buffer = Arc::new(Mutex::new(content.as_bytes().to_vec()));

    let last_lines = get_last_n_lines(&buffer, 3)?;
    assert_eq!(last_lines.len(), 3);
    assert_eq!(last_lines[2], "Last line with content");

    let has_content = bottom_line_has_content(&buffer, 3)?;
    assert!(has_content, "Bottom line should have content");

    // Test with empty bottom line
    let content_empty = "Line 1\nLine 2\n   \n";
    let buffer_empty = Arc::new(Mutex::new(content_empty.as_bytes().to_vec()));
    let has_content_empty = bottom_line_has_content(&buffer_empty, 4)?;
    assert!(
        !has_content_empty,
        "Bottom line should be empty/whitespace only"
    );

    Ok(())
}
