# Ratatui E2E Testing Standards

## Purpose

This rule specifies the **mandatory testing framework** for all E2E tests in the ratkit project. All TUI (Terminal User Interface) E2E tests must use `ratatui-testlib` for PTY-based testing of terminal applications.

## Mandatory Framework: ratatui-testlib

1. **All TUI E2E tests must use `ratatui-testlib`** - No exceptions. This library provides PTY-based testing that runs your app in a real pseudo-terminal.

2. **Installation** - Add to `[dev-dependencies]` in `Cargo.toml`:
   ```toml
   [dev-dependencies]
   ratatui-testlib = "0.1"
   portable-pty = "0.8"
   ```

3. **Feature flags** - Enable based on testing needs:
   - `async-tokio` - For async TUI applications
   - `sixel` - For testing Sixel graphics positioning
   - `snapshot-insta` - For snapshot testing integration
   - `headless` - For CI/CD without X11/Wayland

## Why ratatui-testlib?

**Unit testing limitations** that `ratatui-testlib` solves:
- ❌ Mock backends can't test PTY-specific behavior (terminal size negotiation, TTY detection)
- ❌ Can't test graphics protocols (Sixel, iTerm2 images, Kitty graphics)
- ❌ Can't test real terminal escape sequence processing
- ❌ Can't test split-process architectures (daemon + client)
- ❌ Can't test user interaction flows in actual terminal context

**ratatui-testlib** runs your TUI app in a real pseudo-terminal (PTY), captures output using a terminal emulator, and provides an ergonomic API for assertions.

## Basic Test Structure

```rust
use ratatui_testlib::{TuiTestHarness, KeyCode};
use portable_pty::CommandBuilder;

#[test]
fn test_my_tui_app() -> ratatui_testlib::Result<()> {
    // Create a test harness with 80x24 terminal
    let mut harness = TuiTestHarness::new(80, 24)?;

    // Spawn your TUI application
    let mut cmd = CommandBuilder::new("./my-tui-app");
    harness.spawn(cmd)?;

    // Wait for initial render
    harness.wait_for(|state| {
        state.contents().contains("Welcome")
    })?;

    // Send keyboard input
    harness.send_key(KeyCode::Char('q'))?;

    // Verify result
    let contents = harness.screen_contents();
    assert!(contents.contains("Goodbye"));

    Ok(())
}
```

## Key Testing Patterns

### 4. Waiting for Content

```rust
// Wait for specific text to appear
harness.wait_for_text("Main Menu")?;

// Wait with custom condition
harness.wait_for(|state| state.contents().contains("Loaded"))?;
```

### 5. Sending Input

```rust
// Send single key
harness.send_key(KeyCode::Down)?;
harness.send_key(KeyCode::Enter)?;

// Send text (for input fields)
harness.send_text("hello world")?;

// Send special keys
harness.send_key(KeyCode::Esc)?;
harness.send_key(KeyCode::Ctrl('c'))?;
```

### 6. Screen Assertions

```rust
// Get full screen contents
let contents = harness.screen_contents();
assert!(contents.contains("Expected text"));

// Check specific region
let region = harness.state().region((0, 0, 80, 1)); // Top line
assert!(region.contains("Header"));
```

### 7. Async Testing (Tokio)

```rust
use ratatui_testlib::AsyncTuiTestHarness;

#[tokio::test]
async fn test_async_app() -> ratatui_testlib::Result<()> {
    let mut harness = AsyncTuiTestHarness::new(80, 24).await?;
    let mut cmd = CommandBuilder::new("./my-async-app");
    harness.spawn(cmd).await?;
    
    harness.wait_for_text("Ready").await?;
    harness.send_key(KeyCode::Char('q')).await?;
    
    Ok(())
}
```

### 8. Sixel Graphics Testing

```rust
#[test]
fn test_sixel_renders_in_preview_area() -> ratatui_testlib::Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?;
    // ... spawn app and trigger Sixel rendering ...

    // Define the preview area where Sixel graphics should appear
    let preview_area = (5, 40, 35, 15); // (row, col, width, height)

    // Assert: Sixel graphics within bounds
    harness.assert_sixel_within_bounds(preview_area)?;
    
    // Or use helper for standard layout
    harness.assert_preview_has_sixel()?;

    Ok(())
}
```

## Testing Ratkit Examples

For testing the ratkit example demos:

```rust
use ratatui_testlib::{TuiTestHarness, KeyCode};
use portable_pty::CommandBuilder;
use std::path::PathBuf;

#[test]
fn test_button_demo() -> ratatui_testlib::Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?;
    
    // Build the example binary path
    let example_path = PathBuf::from("target/debug/examples/button_demo");
    
    let mut cmd = CommandBuilder::new(example_path.to_str().unwrap());
    harness.spawn(cmd)?;
    
    // Wait for UI to render
    harness.wait_for_text("Button")?;
    
    // Interact with the button
    harness.send_key(KeyCode::Char('q'))?;
    
    Ok(())
}
```

## CI/CD Configuration

### 9. Headless Mode

Enable headless testing for GitHub Actions (no X11/Wayland required):

```toml
[dev-dependencies]
ratatui-testlib = { version = "0.1", features = ["headless"] }
```

### 10. GitHub Actions Example

```yaml
name: E2E Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --examples
      - run: cargo test --test e2e
```

## Anti-Patterns

11. **Don't use `TestBackend` for E2E tests** - `TestBackend` is for unit tests. E2E tests must use `ratatui-testlib` with real PTY.

12. **Don't mock terminal behavior** - The whole point of E2E is testing real terminal behavior. Never mock crossterm or terminal APIs.

13. **Don't test internal state** - Test observable screen output, not internal widget state.

## Migration from Other Approaches

If you have existing tests using:
- `TestBackend` → Migrate to `TuiTestHarness`
- Manual PTY setup → Use `ratatui-testlib` abstractions
- Snapshot tests → Add `snapshot-insta` feature

## Resources

- **Documentation**: https://docs.rs/ratatui-testlib
- **Repository**: https://github.com/raibid-labs/ratatui-testlib
- **Crate**: https://crates.io/crates/ratatui-testlib
