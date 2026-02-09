# IN-DEPTH ANALYSIS: TermTui vs Mprocs Terminal Emulation

## Executive Summary

I've identified **ONE CRITICAL BUG** in termtui that explains why full-screen apps like yazi and opencode don't work correctly. Additionally, I've found several architectural differences that should be addressed to match mprocs behavior.

---

## 1. CRITICAL BUG: Alternate Screen Mode Not Implemented (Issue #1)

### Problem
**Location**: `crates/primitives/termtui/src/screen.rs:498`

```rust
fn handle_mode(&mut self, mode: Mode) {
    // Standard ANSI modes - not commonly used
    let _ = mode;  // ← BUG: Discards all mode changes!
}
```

The `handle_mode` function **completely ignores all escape sequence mode changes**. This means termtui never processes critical sequences like:

- **CSI ? 1049 h/l** - Alternate Screen Buffer (THE ROOT CAUSE)
- **CSI ? 47 h/l** - Alternate Screen Buffer (legacy)
- **CSI ? 2004 h/l** - Bracketed Paste Mode
- And many others...

### Why This Breaks Full-Screen Apps

When an app like yazi or opencode starts, it sends the escape sequence:
```
ESC [ ? 1049 h
```

This tells the terminal: "Switch to alternate screen buffer and clear it."

In **mprocs**, this is handled correctly:

```rust
// src/vt100/screen.rs:1245
"?1049" => {
    // Alternate Screen Buffer, With Cursor Save and Clear on Enter
    if set {
        self.decsc();
        self.alternate_grid.clear();
        self.enter_alternate_grid();  // ← Sets MODE_ALTERNATE_SCREEN flag
    } else {
        self.exit_alternate_grid();
        self.decrc();
    }
}
```

The grid selection in mprocs correctly switches between grids:

```rust
fn grid_mut(&mut self) -> &mut crate::vt100::grid::Grid {
    if self.mode(MODE_ALTERNATE_SCREEN) {
        &mut self.alternate_grid  // ← Use alternate grid
    } else {
        &mut self.grid           // ← Use primary grid
    }
}
```

In **termtui**, the 1049 sequence is parsed by termwiz, but the `handle_mode` function ignores it, so:

1. `MODE_ALTERNATE_SCREEN` flag is never set
2. The grid selection still uses the primary grid
3. Full-screen content is written to the wrong grid (which has scrollback)
4. Rendering shows the wrong grid content

### Fix Required

Replace the incomplete `handle_mode` function with proper mode handling. The best approach is to implement custom escape sequence parsing similar to mprocs, which processes raw bytes and has explicit control over mode handling.

---

## 2. Architecture Comparison

### Escape Sequence Parser

| Aspect          | mprocs                                                | termtui                                       |
| ---------------- | ----------------------------------------------------- | --------------------------------------------- |
| **Parser Type**     | Custom parser in `vt100/screen.rs:627`                  | termwiz library                               |
| **Parser Approach** | Byte-by-byte manual parsing, handles 1049 explicitly  | Delegate to termwiz, which provides Mode enum |
| **Mode Handling**   | Explicit parsing of private modes (`?1049`, `?47`, `?2004`) | Mode enum provided but not handled            |
| **Control**         | Full control over escape sequence processing          | Dependent on termwiz's Action enum            |

**Implication**: mprocs has explicit control and can easily add support for new escape sequences. termtui is more limited by termwiz's abstraction.

### Grid Management

Both use a similar VecDeque-based architecture, but mprocs has proper mode switching:

```rust
// mprocs - src/vt100/screen.rs:17
pub struct Screen {
    grid: Grid,              // Primary grid with scrollback
    alternate_grid: Grid,   // Alternate grid (no scrollback)
    modes: u8,              // Contains MODE_ALTERNATE_SCREEN flag
    // ...
}
```

termtui has the same structure:

```rust
// termtui - src/screen.rs:22
pub struct Screen {
    grid: Grid,              // Primary grid
    alternate_grid: Grid,   // Alternate grid
    modes: u8,              // Contains MODE_ALTERNATE_SCREEN flag
    // ...
}
```

The grids are identical in structure, but termtui never sets the `MODE_ALTERNATE_SCREEN` flag when the 1049 sequence is received.

### PTY Library

| Aspect         | mprocs                                                        | termtui                                |
| -------------- | ------------------------------------------------------------- | -------------------------------------- |
| **Library**        | `mprocs-pty` 0.7.0 (fork of portable-pty)                       | `portable-pty` 0.9                       |
| **Size Passing**   | Uses `rustix::termios::tcsetwinsize` with proper Winsize struct | Uses `portable_pty::MasterPty::resize()` |
| **Initialization** | Fork PTY, set flags (O_NONBLOCK, FD_CLOEXEC)                  | Similar, uses portable-pty APIs        |

**Key Difference**: mprocs uses a custom fork that may have bug fixes or optimizations for terminal emulation.

---

## 3. Hardcoded PTY Size (Potential Fish Error)

**Location**: `crates/primitives/termtui/src/lib.rs:109-110`

```rust
pub fn spawn_with_command(...) -> Result<Self> {
    let rows = 24;  // ← HARDCODED
    let cols = 80;  // ← HARDCODED
```

### Problem

The PTY is always created with 24x80 size, regardless of the actual terminal size. This causes:

1. **Fish initialization issues**: Fish queries terminal size at startup and may fail if the PTY reports a different size
2. **Apps query wrong size**: Apps that query terminal (like `tput cols`) get 80, not the actual size
3. **Resize lag**: Until `resize()` is called, apps think they're in an 80-column terminal

### mprocs Approach

mprocs receives the actual size and passes it to the PTY:

```rust
// src/proc/inst.rs:44
pub async fn spawn(
    size: &Size,  // ← Actual size passed in
    ...
) -> anyhow::Result<Self> {
    let vt = crate::vt100::Parser::new(size.height, size.width, scrollback_len);
    // PTY gets actual size through rustix::termios::tcsetwinsize
}
```

### Fix Required

Change `spawn_with_command` to accept size parameters and query the actual terminal size before spawning, or provide a separate `resize()` method that should be called immediately after creation.

---

## 4. Terminal Initialization Sequences

### TERM Environment Variable

**mprocs**: Does NOT explicitly set `TERM`. Relies on system defaults.

**termtui**: Sets `TERM=xterm-256color` explicitly:

```rust
// src/lib.rs:128
cmd.env("TERM", "xterm-256color");
```

**Implication**: Setting `TERM=xterm-256color` is generally safe and recommended. This shouldn't be an issue for fish.

### Shell Initialization

The fish error you mentioned could be related to:
1. PTY size mismatch (24x80 hardcoded)
2. Missing alternate screen handling causing initialization output to be lost
3. Non-blocking I/O setup differences

mprocs uses `rustix` for all low-level operations, which is more modern and may handle edge cases better than `portable-pty`.

---

## 5. Screen Rendering Comparison

### mprocs Rendering Flow

```rust
// src/ui_term.rs:95
fn render_screen(
    screen: &Screen,
    copy_mode: &CopyMode,
    area: Rect,
    grid: &mut Grid,
) {
    for row in 0..area.height {
        for col in 0..area.width {
            // Direct access to screen cells
            if let Some(cell) = screen.cell(row, col) {
                *to_cell = cell.clone();
            }
        }
    }
}
```

### termtui Rendering Flow

```rust
// src/widget.rs:54
for (row_idx, row) in self.screen.visible_rows().enumerate() {
    // Uses iterator over visible rows
    // Calls screen.visible_rows() → grid().visible_rows()
}
```

**Key Difference**: termtui's `visible_rows()` calls `self.grid()`, which correctly switches between primary/alternate grids based on mode flag. However, since the mode flag is never set for alternate screen, it always shows the primary grid.

---

## 6. Mode Flag Implementation Comparison

### mprocs Mode Constants

```rust
// src/vt100/screen.rs:10-14
const MODE_APPLICATION_KEYPAD: u8 = 0b0000_0001;
const MODE_APPLICATION_CURSOR: u8 = 0b0000_0010;
const MODE_HIDE_CURSOR: u8 = 0b0000_0100;
const MODE_ALTERNATE_SCREEN: u8 = 0b0000_1000;
const MODE_BRACKETED_PASTE: u8 = 0b0001_0000;
```

### termtui Mode Constants

```rust
// crates/primitives/termtui/src/screen.rs:11-19
const MODE_CURSOR_VISIBLE: u8 = 1 << 0;
const MODE_ALTERNATE_SCREEN: u8 = 1 << 1;
#[allow(dead_code)]
const MODE_APPLICATION_CURSOR: u8 = 1 << 2;
#[allow(dead_code)]
const MODE_BRACKETED_PASTE: u8 = 1 << 3;
const MODE_AUTO_WRAP: u8 = 1 << 4;
#[allow(dead_code)]
const MODE_ORIGIN: u8 = 1 << 5;
```

Both implementations have the `MODE_ALTERNATE_SCREEN` flag defined, but only mprocs actually sets/clears it.

---

## 7. Escape Sequence Processing Comparison

### mprocs Processing

mprocs implements a custom `process()` function that:
1. Maintains a `feed_buf` for incomplete sequences
2. Parses bytes one-by-one
3. Handles control codes directly (0x07, 0x08, 0x0A, etc.)
4. Parses CSI sequences including private modes (`?1049`, `?47`, etc.)
5. Generates `VtEvent` for bells and replies

```rust
// src/vt100/screen.rs:627
pub fn process(&mut self, data: &[u8], events: &mut Vec<VtEvent>) {
    self.feed_buf.extend_from_slice(data);
    let buf = std::mem::take(&mut self.feed_buf);
    
    let mut pos = 0;
    'process: while pos < buf.len() {
        match buf[pos] {
            0x07 => events.push(VtEvent::Bell),
            0x1B => /* Parse escape sequences including ?1049 */
            // ...
        }
    }
}
```

### termtui Processing

termtui uses termwiz:
1. termwiz's parser handles byte parsing
2. Generates `Action` enum variants
3. `Screen.handle_action()` delegates to specific handlers
4. `handle_mode()` receives `Mode` enum but ignores it

```rust
// crates/primitives/termtui/src/screen.rs:151
pub fn handle_action(&mut self, action: Action) {
    match action {
        Action::CSI(csi) => self.handle_csi(csi),  // CSI includes Mode
        // ...
    }
}

fn handle_csi(&mut self, csi: CSI) {
    match csi {
        CSI::Mode(mode) => self.handle_mode(mode),  // ← Mode enum
        // ...
    }
}
```

**The issue**: termwiz's `Mode` enum may not include all private modes like `?1049`, or termtui doesn't extract them properly.

---

## 8. Key Encoding Comparison

### mprocs Key Encoding

mprocs uses a sophisticated `encode_key()` function that:
- Handles CSI-u encoding (modern terminal support)
- Supports application cursor keys
- Handles control key ambiguities
- Normalizes shift to uppercase
- Implements modifier encoding (SHIFT, ALT, CONTROL)

```rust
// src/encode_term.rs:33
pub fn encode_key(key: &Key, modes: KeyCodeEncodeModes) -> Result<String> {
    // Complex encoding with CSI-u support
    // Handles xterm compatibility
    // ...
}
```

### termtui Key Encoding

termtui has a simpler implementation:
- Basic CSI encoding for arrow keys
- Control character mapping
- ALT prefix handling
- Limited modifier support

```rust
// crates/primitives/termtui/src/lib.rs:355
fn key_to_terminal_input(&self, key: crossterm::event::KeyEvent) -> String {
    match key.code {
        KeyCode::Char(c) => { /* basic handling */ }
        KeyCode::Up => "\x1b[A".to_string(),
        // ...
    }
}
```

**Implication**: termtui may not send keys correctly to all applications, especially those that expect CSI-u encoding.

---

## 9. Resize Handling Comparison

### mprocs Resize

```rust
// src/proc/inst.rs:129
pub fn resize(&mut self, size: &Size) {
    let rows = size.height;
    let cols = size.width;

    self.process.resize(Winsize { ... });
    
    if let Ok(mut vt) = self.vt.write() {
        vt.set_size(rows, cols);  // Update parser size
    }
}
```

### termtui Resize

```rust
// crates/primitives/termtui/src/lib.rs:575
pub fn resize(&mut self, rows: u16, cols: u16) {
    if let Some(ref master) = self._master {
        let _ = master.resize(PtySize { rows, cols, ... });
    }
    let mut parser = self.parser.lock().unwrap();
    parser.resize(rows as usize, cols as usize);
}
```

Both implementations properly resize both the PTY and the screen parser. However, termtui's initial spawn creates the PTY with the wrong size.

---

## Recommended Implementation Plan

### Phase 1: Fix Critical Alternate Screen Bug (HIGHEST PRIORITY)

1. **Implement proper mode handling** in `screen.rs`:
   - Parse raw CSI sequences before termwiz processes them
   - Handle `?1049` (alternate screen) explicitly
   - Handle `?47` (legacy alternate screen)
   - Handle `?2004` (bracketed paste)
   - Handle `?25` (hide/show cursor)

2. **Options for implementation**:
   
   **Option A: Custom Parser (Recommended - Match mprocs)**
   - Implement custom `process()` function in `screen.rs`
   - Parse CSI sequences manually, including private modes
   - Maintain feed buffer for incomplete sequences
   - Similar to mprocs/src/vt100/screen.rs:627
   
   **Option B: Enhance termwiz integration**
   - Parse raw bytes to detect `?1049` before termwiz
   - Set/clear `MODE_ALTERNATE_SCREEN` flag appropriately
   - Still use termwiz for standard sequences

3. **Update `handle_mode` to at least handle critical modes**:
   - Set/clear `MODE_ALTERNATE_SCREEN` for 1049
   - Set/clear `MODE_BRACKETED_PASTE` for 2004
   - Set/clear `MODE_HIDE_CURSOR` for 25

### Phase 2: Fix PTY Initialization

1. **Remove hardcoded size** from `spawn_with_command`:
   ```rust
   // OLD:
   let rows = 24;
   let cols = 80;
   
   // NEW:
   let rows = area.height;  // Or pass as parameter
   let cols = area.width;
   ```

2. **Accept size as parameter** or query actual terminal size

3. **Ensure resize is called immediately** after spawning to set correct size

### Phase 3: Enhance Key Encoding

1. **Implement CSI-u encoding** support:
   - Modern terminals (kitty, wezterm) prefer CSI-u
   - Provides unambiguous key encoding
   - Better support for modified keys

2. **Add application cursor key modes**:
   - DECCKM (application cursor mode)
   - DECKPAM (application keypad mode)

### Phase 4: PTY Library Evaluation

Consider switching from `portable-pty` to `rustix`-based approach:
- More modern APIs
- Better error handling
- Consistent with mprocs's approach
- May have fixes for edge cases

### Phase 5: Comprehensive Testing

1. Test full-screen apps:
   - yazi
   - opencode
   - htop
   - vim/nvim
   - less/most

2. Test shell initialization:
   - fish
   - bash
   - zsh

3. Test resize behavior:
   - Window resize
   - Full-screen app resize

---

## Summary of Issues

| Issue                                    | Severity | Impact                                                |
| ---------------------------------------- | -------- | ----------------------------------------------------- |
| **Alternate screen mode (1049) not handled** | **CRITICAL** | Full-screen apps don't work - main issue you reported |
| Hardcoded 24x80 PTY size                 | HIGH     | Fish errors, wrong app initialization                 |
| Mode changes ignored                     | HIGH     | Many terminal features don't work                     |
| Limited key encoding                    | MEDIUM   | Some apps may not receive correct key input         |
| Dependent on termwiz limitations         | MEDIUM   | Harder to add new features                            |

---

## What Makes mprocs Work

1. **Explicit 1049 handling** - Switches to alternate grid when requested
2. **Proper mode flags** - Uses flags to switch between grids
3. **Actual PTY size** - Passes real terminal dimensions to PTY
4. **Custom parser** - Full control over escape sequences
5. **rustix integration** - Modern, reliable system calls
6. **CSI-u key encoding** - Modern, unambiguous key sequences
7. **Comprehensive mode support** - Handles all private DEC modes

---

## Next Steps

1. Implement alternate screen mode handling (CRITICAL)
2. Fix hardcoded PTY size initialization
3. Consider adopting mprocs-style parser for better control
4. Add comprehensive testing for full-screen applications
5. Evaluate switching to rustix for PTY operations

---

**Date**: 2025-02-06  
**Analysis of**: crates/primitives/termtui vs /tmp/mprocs
