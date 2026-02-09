TermTui vs mprocs Terminal Emulator Comparison

Scope
- Compare @crates/primitives/termtui with /tmp/mprocs terminal implementation.
- Focus on terminal emulator behavior, full-screen apps, and fish startup error.

Key Differences (Terminal Emulator Core)
- Parser/emulator stack
  - termtui uses termwiz for parsing plus a minimal Screen implementation. Screen::handle_mode is a no-op, so DEC private modes are ignored. See crates/primitives/termtui/src/screen.rs.
  - mprocs uses a custom VT100 parser and emulator with explicit DEC mode handling. See /tmp/mprocs/src/vt100/screen.rs and /tmp/mprocs/src/vt100/parser.rs.

- Alternate screen handling
  - termtui defines MODE_ALTERNATE_SCREEN but never sets it; handle_mode ignores DECSET/DECRST. The alternate grid is effectively unused.
  - mprocs toggles the alternate screen on ?47 and ?1049, saving/restoring cursor and clearing the alt grid on entry. See /tmp/mprocs/src/vt100/screen.rs (process_csi handling for ?47/?1049).

- Scroll regions, origin mode, insert mode
  - termtui lacks DECSTBM (CSI r), origin mode (?6), insert mode (CSI 4), and related grid behavior.
  - mprocs supports these and routes them through grid operations. See /tmp/mprocs/src/vt100/grid.rs and /tmp/mprocs/src/vt100/screen.rs.

- Wrap logic and wide chars
  - termtui uses a simplified pending_wrap model and only partial wide/combining support.
  - mprocs uses col_wrap + wide continuation rules, matching tmux behavior. See /tmp/mprocs/src/vt100/screen.rs (text) and /tmp/mprocs/src/vt100/grid.rs.

- Resizing behavior (PTY vs renderer)
  - termtui spawns the PTY at fixed size 24x80 and never stores the master PTY. TermTui::resize cannot actually resize the PTY.
  - mprocs runs in the real terminal and reads size via SIGWINCH, so the emulator and apps stay in sync. See /tmp/mprocs/src/term/term_driver.rs.

Root Causes of the Reported Issues
- Full-screen apps not filling the area in termtui
  - The PTY size stays at 24x80 because _master is never stored. The widget may resize the parser and rendering area, but the subprocess still believes it is 24x80 and only draws to that size.

- Alternate screen behavior incorrect or missing
  - termtui ignores DECSET/DECRST, so ?47/?1049 are not handled, and apps never enter alternate screen mode.

- fish error on startup (likely)
  - Without the exact error message, the most likely causes are:
    - Missing DECSET/DECRST or other DEC mode handling.
    - Incorrect PTY size reporting causing terminal capability checks to fail.

Update Plan to Align termtui with mprocs
1) Fix PTY sizing
  - Store the master PTY in TermTui and use it in TermTui::resize.
  - Accept initial size in spawn_with_command and create PTY with that size.
  - Result: full-screen apps fill the area because the subprocess sees correct terminal dimensions.

2) Implement DEC private mode handling
  - Map termwiz Mode changes to mprocs behavior:
    - ?47 and ?1049: enter/exit alternate grid, save/restore cursor, clear alt grid on enter.
    - ?25: show/hide cursor.
    - ?6: origin mode.
    - ?2004: bracketed paste.

3) Add scroll region and insert mode support
  - Implement CSI r (DECSTBM) and CSI 4 (insert mode), aligning with mprocs grid behavior.

4) Improve wrap and wide-char handling
  - Port mprocs col_wrap + wide continuation behavior to termtui Screen/Grid.

5) Optional: adopt mprocs vt100 module
  - For maximum parity, transplant /tmp/mprocs/src/vt100/* into termtui and use the same parser + screen logic.

Primary Files Referenced
- termtui
  - crates/primitives/termtui/src/lib.rs
  - crates/primitives/termtui/src/screen.rs
  - crates/primitives/termtui/src/grid.rs
  - crates/primitives/termtui/src/parser.rs
  - crates/primitives/termtui/src/widget.rs

- mprocs
  - /tmp/mprocs/src/vt100/screen.rs
  - /tmp/mprocs/src/vt100/parser.rs
  - /tmp/mprocs/src/vt100/grid.rs
  - /tmp/mprocs/src/term/term_driver.rs

Notes for Follow-up
- Provide the exact fish error message to diagnose the startup failure precisely.
