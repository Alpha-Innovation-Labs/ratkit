---
context_id: TPR_008
title: Terminal Emulator Primitive
project: toolkit-primitives
created: "2026-01-21"
---

# TPR_008: Terminal Emulator Primitive

## Desired Outcome

A `TermTui` primitive that renders interactive terminal output in a TUI application with behavior matching full-screen terminal apps. Users can view terminal content, scroll through history, enter copy mode to select and copy text, and run interactive processes that correctly resize and occupy the full available area.

## Reference

- `/.nexus/context/toolkit-primitives/_reference/terminal-emulator-parity-notes.md`

## Next Actions

| Description | Test |
|-------------|------|
| Terminal renders VT100 escape sequences as styled text | `terminal_renders_vt100_output` |
| Scrollback buffer preserves and retrieves historical terminal output | `terminal_scrollback_buffer_works` |
| Entering copy mode freezes current screen for selection | `copy_mode_freezes_screen` |
| Mouse drag selects text within copy mode | `copy_mode_selection_works` |
| Arrow keys navigate within copy mode selection | `copy_mode_navigation_keys` |
| Selected text copies to system clipboard | `copy_to_clipboard_succeeds` |
| Mouse wheel scrolls terminal content up and down | `mouse_wheel_scrolls_content` |
| Spawned command runs in PTY and displays output | `command_spawns_process` |
| Full-screen apps render within the full available terminal area | `fullscreen_apps_fill_area` |
| Resizing the container updates the PTY size and visible output | `pty_resize_updates_output` |
| Alternate screen apps enter and exit without leaving artifacts | `alternate_screen_enters_exits_cleanly` |
| Shell startup completes without terminal capability errors | `shell_startup_has_no_errors` |
