# ratkit

ratkit is a comprehensive Rust TUI (Terminal User Interface) component library built on ratatui 0.29. It provides modular primitives, widgets, and services for building rich terminal applications with features like file system navigation, git monitoring, markdown rendering, AI chat interfaces, and more.

[Usage instructions - how to use this file]

## Agent Operating Rules

1. Implement the CoordinatorApp trait to define application event handling and rendering for all ratkit applications
2. Register UI elements with ElementMetadata specifying their Region (Top/Center/Bottom) before they can receive events
3. Use Element::on_keyboard, on_mouse, on_focus_gain, on_focus_loss, and on_tick callbacks to handle element-specific events
4. State structs (DialogState, TreeViewState, etc.) must be held in the app struct and persisted across renders to maintain selection and interaction state
5. Widget state must be mutably borrowed during rendering and event handling, then stored back in app state
6. Enable features via feature flags in Cargo.toml (e.g., feature = "button")
7. Theme-dependent methods require the 'theme' feature and ratkit-theme dependency
8. WidgetEvent is the unified event system; handle WidgetEvent::None for no-op cases
9. All watchers must be ticked/queried regularly in the event loop to detect changes
10. All stateful widgets require persistent state objects that must be stored between frames
11. Mouse capture must be enabled via crossterm for mouse interactions to function
12. Feature flags in Cargo.toml enable individual primitives; no default features
13. ratatui 0.29 is the underlying rendering library for all widgets
14. Use `just check` before commits (verifies format, lint, and tests)

## Environment and Version Constraints

- Rust 1.70+ required (workspace.rust-version)
- sccache wrapper required for builds (export CARGO_BUILD_RUSTC_WRAPPER=sccache)
- Python 3 required for documentation generation scripts
- npm required for serving documentation
- TTY or script required for `just dev` command
- crossterm 0.28 is used for terminal input/events
- All crates follow modular architecture: types in mod.rs, constructors in new.rs, methods in methods/, traits in traits/

## Quick Task Playbooks

### Add a new primitive/widget/service crate

1. Create new crate directory under crates/{primitives,widgets,services}/
2. Add crate member entry to workspace.members in Cargo.toml
3. Add crate dependencies to workspace.dependencies section
4. Create mod.rs with type definitions (struct/enum only, no impl blocks)
5. Create constructors/new.rs with new() and with_* builders
6. Create methods/ directory for instance methods
7. Create traits/ directory for trait implementations
8. Add re-export in crates/ratkit/src/lib.rs
9. Add feature flag to ratkit Cargo.toml if components should be optional
10. Run `just check` to verify all checks pass

### Add a new example

1. Create example file in examples/ directory
2. Ensure example has proper module-level doc comments
3. Run with `just example <name>` or `just dev` if showcase
4. Test with `just run --example <name> --features <required-features>`

### Run complete verification

1. Run `just fmt` to auto-format code
2. Run `just fmt-check` to verify formatting
3. Run `just lint` for clippy checks
4. Run `just test` for all unit and integration tests
5. Or simply run `just check` which runs all three

### Build and serve documentation

1. Run `just docs` to regenerate and serve docs on port 3000
2. This runs the Python doc generator from docs/scripts/generate_docs.py
3. Serves docs site via npm in docs/ directory

## Getting Started

```toml
# In Cargo.toml
[dependencies]
ratkit = { version = "0.1", features = ["button", "dialog", "pane"] }
```

```rust
use ratkit::{run, CoordinatorApp, RunnerConfig};

struct MyApp;

impl CoordinatorApp for MyApp {
    fn on_event(&mut self, event: ratkit::Event) -> bool {
        false
    }

    fn on_draw(&mut self, frame: &mut ratatui::Frame) {
        // Render your UI here
    }
}

fn main() {
    let config = RunnerConfig::default();
    run(MyApp, config);
}
```

## Workspace Overview

The ratkit workspace contains 23 crates organized into:
- **Primitives** (12 crates): Core UI building blocks (Button, Dialog, MenuBar, TreeView, Pane, Toast, StatusLine, Scroll, ResizableGrid, WidgetEvent, Termtui)
- **Widgets** (6 crates): Higher-level TUI widgets (MarkdownPreview, CodeDiff, AIChat, HotkeyFooter, FileSystemTree, ThemePicker)
- **Services** (4 crates): Reactive monitoring (FileWatcher, GitWatcher, RepoWatcher, HotkeyService)
- **Meta-crate** (1 crate): ratkit aggregates all components

[justfile](justfile) - Main command entry point with aliases for all build, test, and verification commands
[Cargo.toml](Cargo.toml) - Workspace manifest defining 23 member crates and shared dependencies

## Primitives

Core UI building blocks for ratkit TUI applications including interactive widgets, layout components, status indicators, notifications, and utilities. All primitives depend on ratatui for rendering and support optional theming via the 'theme' feature.

### Links

| Path | Annotation |
|------|------------|
| [crates/primitives/button/src/lib.rs](crates/primitives/button/src/lib.rs) | Button widget for clickable buttons with hover states |
| [crates/primitives/pane/src/lib.rs](crates/primitives/pane/src/lib.rs) | Styled panel component with title, icon, padding, and optional footer |
| [crates/primitives/dialog/src/lib.rs](crates/primitives/dialog/src/lib.rs) | Modal dialog windows with customizable buttons and dialog types |
| [crates/primitives/toast/src/lib.rs](crates/primitives/toast/src/lib.rs) | Toast notification system with success/error/info/warning levels |
| [crates/primitives/statusline/src/lib.rs](crates/primitives/statusline/src/lib.rs) | Status bar with PowerLine-style stacked indicators |
| [crates/primitives/scroll/src/lib.rs](crates/primitives/scroll/src/lib.rs) | Scroll offset calculation algorithms for visible viewport management |
| [crates/primitives/menu-bar/src/lib.rs](crates/primitives/menu-bar/src/lib.rs) | Menu bar widget with items, icons, and click actions |
| [crates/primitives/resizable-grid/src/lib.rs](crates/primitives/resizable-grid/src/lib.rs) | Resizable grid layouts with mouse interaction for split panes |
| [crates/primitives/tree-view/src/lib.rs](crates/primitives/tree-view/src/lib.rs) | Tree view widget for hierarchical data with navigation |
| [crates/primitives/widget-event/src/lib.rs](crates/primitives/widget-event/src/lib.rs) | Common event types emitted by interactive widgets |
| [crates/primitives/termtui/src/lib.rs](crates/primitives/termtui/src/lib.rs) | VT100 terminal emulation for embedded terminal rendering |

## Widgets

Collection of TUI widgets for ratatui applications including markdown rendering, code diff viewing, AI chat interfaces, file system navigation, theme selection, and hotkey displays. All widgets follow ratatui's stateful rendering pattern with persistent state management.

### Links

| Path | Annotation |
|------|------------|
| [crates/widgets/markdown-preview/src/lib.rs](crates/widgets/markdown-preview/src/lib.rs) | Markdown rendering widget with TOC, syntax highlighting, and scroll state management |
| [crates/widgets/code-diff/src/lib.rs](crates/widgets/code-diff/src/lib.rs) | Code diff viewer widget for VS Code-style side-by-side diffs |
| [crates/widgets/ai-chat/src/lib.rs](crates/widgets/ai-chat/src/lib.rs) | AI chat interface with multi-line input and file attachments |
| [crates/widgets/hotkey-footer/src/lib.rs](crates/widgets/hotkey-footer/src/lib.rs) | Aerospace-tui style hotkey footer bar component |
| [crates/widgets/file-system-tree/src/lib.rs](crates/widgets/file-system-tree/src/lib.rs) | File system tree view with icons, expand/collapse, and filter mode |
| [crates/widgets/theme-picker/src/lib.rs](crates/widgets/theme-picker/src/lib.rs) | Modal theme picker with search and live preview |

## Services

Services domain provides reactive monitoring and event management capabilities for ratkit TUI applications. Includes file system watchers, git repository monitoring, combined repo watchers, and hotkey management. All services use non-blocking interfaces suitable for TUI event loops.

### Links

| Path | Annotation |
|------|------------|
| [crates/services/file-watcher/src/lib.rs](crates/services/file-watcher/src/lib.rs) | Watches files and directories for changes using notify crate |
| [crates/services/git-watcher/src/lib.rs](crates/services/git-watcher/src/lib.rs) | Monitors .git directory for repository state changes |
| [crates/services/repo-watcher/src/lib.rs](crates/services/repo-watcher/src/lib.rs) | Combines git and file watchers with git status integration |
| [crates/services/hotkey-service/src/lib.rs](crates/services/hotkey-service/src/lib.rs) | Centralized hotkey registration with scope support |

## Core Runtime

The core runtime provides the main event loop, layout coordination, focus management, and mouse routing for ratkit TUI applications. It orchestrates LayoutManager for three-region geometry (Top/Center/Bottom), FocusManager for keyboard focus traversal, and MouseRouter for mouse event routing with capture semantics.

### Links

| Path | Annotation |
|------|------------|
| [crates/ratkit/src/lib.rs](crates/ratkit/src/lib.rs) | Main crate re-exports |
| [crates/ratkit/src/runner_helper.rs](crates/ratkit/src/runner_helper.rs) | Runner struct and configuration |
| [crates/ratkit/src/coordinator.rs](crates/ratkit/src/coordinator.rs) | LayoutCoordinator orchestrating layout, focus, and mouse |
| [crates/ratkit/src/layout.rs](crates/ratkit/src/layout.rs) | LayoutManager for element geometry |
| [crates/ratkit/src/focus.rs](crates/ratkit/src/focus.rs) | FocusManager for keyboard focus |
| [crates/ratkit/src/mouse_router.rs](crates/ratkit/src/mouse_router.rs) | MouseRouter for mouse routing |

## Usage Cards

### Primitives Usage Cards

#### Button

**Use when:** Need a clickable button with hover states and style customization

**Enable:** feature = "button" in Cargo.toml

**Import:** `use ratkit_button::Button`

**Minimal flow:**
1. Create `Button::new("Label")`
2. Call `update_hover(column, row)` on mouse move events
3. Call `is_clicked(column, row)` on click events
4. Use `render()` or `render_with_title()` in frame.render_widget()

**APIs:** Button::new(), Button::normal_style(), Button::hover_style(), Button::update_hover(), Button::is_clicked(), Button::render(), Button::render_with_title()

**Pitfalls:** State must be held in app struct for persistence; button area is set during render

**Source:** [crates/primitives/button/src/lib.rs](crates/primitives/button/src/lib.rs)

#### Pane

**Use when:** Need a styled panel container with title, icon, padding, and optional footer

**Enable:** feature = "pane" in Cargo.toml

**Import:** `use ratkit_pane::Pane`

**Minimal flow:**
1. Create `Pane::new("Title")`
2. Chain builder methods for styling (with_icon, with_padding, border_style)
3. Use `Pane::render()` or `inner_content()` in frame.render_widget()

**APIs:** Pane::new(), Pane::with_icon(), Pane::with_padding(), Pane::with_uniform_padding(), Pane::with_text_footer(), Pane::border_style(), Pane::border_type(), Pane::title_style(), Pane::footer_style()

**Pitfalls:** Padding reduces available inner content area; footer_height reserves space but doesn't auto-calculate

**Source:** [crates/primitives/pane/src/lib.rs](crates/primitives/pane/src/lib.rs)

#### Dialog

**Use when:** Need modal dialog windows for user confirmation or information display

**Enable:** feature = "dialog" in Cargo.toml

**Import:** `use ratkit_dialog::{Dialog, DialogState, DialogType}`

**Minimal flow:**
1. Create `Dialog::new(title, message)` or use builder (Dialog::info, Dialog::confirm)
2. Set buttons with `.buttons(vec!["Yes", "No"])`
3. Create `DialogState::new()` to track selected button
4. Handle keyboard events to navigate/select buttons
5. Use `DialogWidget::render()` in frame.render_widget()

**APIs:** Dialog::new(), Dialog::info(), Dialog::warning(), Dialog::error(), Dialog::success(), Dialog::confirm(), Dialog::buttons(), Dialog::style(), Dialog::border_color(), Dialog::width_percent(), DialogState::new()

**Pitfalls:** DialogState must be stored in app state; overlay blocks input to underlying widgets

**Source:** [crates/primitives/dialog/src/lib.rs](crates/primitives/dialog/src/lib.rs)

#### Toast

**Use when:** Need temporary notification messages (success, error, info, warning) that auto-dismiss

**Enable:** feature = "toast" in Cargo.toml

**Import:** `use ratkit_toast::{Toast, ToastManager, ToastLevel, render_toasts}`

**Minimal flow:**
1. Create `Toast { message, level, created_at, duration }`
2. Add to `ToastManager::toasts` vector
3. Call `ToastManager::cleanup()` to remove expired toasts
4. Use `render_toasts()` in frame.render_widget() to render all toasts

**APIs:** Toast::new(), ToastLevel::{Success, Error, Info, Warning}, ToastManager::default(), ToastManager::add(), ToastManager::cleanup(), render_toasts()

**Pitfalls:** ToastManager doesn't auto-expire; call cleanup() before render; DEFAULT_TOAST_DURATION is 3 seconds

**Source:** [crates/primitives/toast/src/lib.rs](crates/primitives/toast/src/lib.rs)

#### StatusLineStacked

**Use when:** Need a status bar with PowerLine-style separators and stacked left/right indicators

**Enable:** feature = "statusline" in Cargo.toml

**Import:** `use ratkit_statusline::{StatusLineStacked, SLANT_TL_BR, SLANT_BL_TR}`

**Minimal flow:**
1. Create `StatusLineStacked::new()`
2. Add left indicators with `.start()`
3. Add right indicators with `.end()`
4. Set center text with `.center()`
5. Use frame.render_widget() to render

**APIs:** StatusLineStacked::new(), StatusLineStacked::start(), StatusLineStacked::end(), StatusLineStacked::center(), SLANT_TL_BR, SLANT_BL_TR

**Pitfalls:** PowerLine characters require Nerd Font; separators are decorative only

**Source:** [crates/primitives/statusline/src/lib.rs](crates/primitives/statusline/src/lib.rs)

#### calculate_scroll_offset

**Use when:** Need to auto-scroll to keep selected item visible and centered in scrollable widgets

**Enable:** feature = "scroll" in Cargo.toml

**Import:** `use ratkit_scroll::calculate_scroll_offset`

**Minimal flow:**
1. Call `calculate_scroll_offset(selected_index, visible_count, total_count)`
2. Use returned offset when rendering visible items
3. Skip rendering items before the offset index

**APIs:** calculate_scroll_offset(selected_index, visible_count, total_count) -> usize

**Pitfalls:** Returns 0 when all items fit; returns max offset when selected at end; visible_count is items per viewport, not pixels

**Source:** [crates/primitives/scroll/src/lib.rs](crates/primitives/scroll/src/lib.rs)

#### MenuBar

**Use when:** Need a horizontal menu bar with selectable items, icons, and click handlers

**Enable:** feature = "menu-bar" in Cargo.toml

**Import:** `use ratkit_menu_bar::{MenuBar, MenuItem}`

**Minimal flow:**
1. Create `MenuItem::new(name, value)` for each item
2. Build `MenuBar::new(items_vec)`
3. Handle mouse events via `handle_mouse(column, row)` or `handle_click()`
4. Use `render()` or `render_centered()` in frame.render_widget()

**APIs:** MenuItem::new(), MenuItem::with_icon(), MenuItem::with_action(), MenuBar::new(), MenuBar::normal_style(), MenuBar::selected_style(), MenuBar::update_hover(), MenuBar::handle_mouse(), MenuBar::render()

**Pitfalls:** handle_mouse returns WidgetEvent::MenuSelected; MenuItem action is consumed (Once) on use

**Source:** [crates/primitives/menu-bar/src/lib.rs](crates/primitives/menu-bar/src/lib.rs)

#### ResizableGrid

**Use when:** Need resizable split panes with mouse draggable dividers

**Enable:** feature = "resizable-grid" in Cargo.toml

**Import:** `use ratkit_resizable_grid::{ResizableGrid, PaneId, ResizableGridWidget, ResizableGridWidgetState}`

**Minimal flow:**
1. Create `ResizableGrid::new(initial_pane_count)`
2. Create `ResizableGridWidgetState` for tracking dividers
3. Handle mouse events to detect divider drag
4. Use `ResizableGridWidget::render()` in frame.render_widget()

**APIs:** ResizableGrid::new(), ResizableGridWidget::new(), ResizableGridWidgetState::new(), PaneId

**Pitfalls:** ResizableGridWidgetState must be stored in app state; mouse interaction requires event loop integration

**Source:** [crates/primitives/resizable-grid/src/lib.rs](crates/primitives/resizable-grid/src/lib.rs)

#### TreeView

**Use when:** Need hierarchical data display with expand/collapse, selection, and keyboard navigation

**Enable:** feature = "tree-view" in Cargo.toml

**Import:** `use ratkit_tree_view::{TreeNode, TreeView, TreeViewState, TreeNavigator}`

**Minimal flow:**
1. Create `TreeNode::with_children()` or `TreeNode::new()` for data
2. Create `TreeView::new(nodes)` with render_fn
3. Create `TreeViewState::new()` for selection/expansion
4. Use `TreeNavigator` for keyboard event handling
5. Use frame.render_widget() to render

**APIs:** TreeNode::new(), TreeNode::with_children(), TreeView::new(), TreeView::render_fn(), TreeViewState::new(), TreeViewState::select(), TreeViewState::toggle_expansion(), TreeNavigator::new(), get_visible_paths()

**Pitfalls:** TreeViewState must be persisted; TreeNavigator handles all keyboard navigation; render_fn borrows data

**Source:** [crates/primitives/tree-view/src/lib.rs](crates/primitives/tree-view/src/lib.rs)

#### WidgetEvent

**Use when:** Need unified event handling from interactive widgets across the application

**Enable:** feature = "widget-event" in Cargo.toml

**Import:** `use ratkit_widget_event::WidgetEvent`

**Minimal flow:**
1. Match on WidgetEvent variants in event loop
2. Handle `Selected { path }` for item selection
3. Handle `Toggled { path, expanded }` for expand/collapse
4. Handle `Scrolled { offset, direction }` for scroll events
5. Handle `MenuSelected { index, action }` for menu clicks
6. Handle `FilterModeChanged/FilterModeExited` for tree view filtering

**APIs:** WidgetEvent::None, WidgetEvent::Selected, WidgetEvent::Toggled, WidgetEvent::Scrolled, WidgetEvent::FilterModeChanged, WidgetEvent::FilterModeExited, WidgetEvent::MenuSelected

**Pitfalls:** WidgetEvent::MenuSelected action is consumed (Once); path is Vec<usize> from root to item

**Source:** [crates/primitives/widget-event/src/lib.rs](crates/primitives/widget-event/src/lib.rs)

#### Termtui

**Use when:** Need embedded terminal emulation (VT100) for rendering terminal output within TUI

**Enable:** feature = "termtui" in Cargo.toml

**Import:** `use ratkit_termtui::{Screen, VtEvent, render_screen, write_screen_diff}`

**Minimal flow:**
1. Create Screen with size/layers
2. Process VT100 escape sequences to update Screen state
3. Use `render_screen()` or `write_screen_diff()` to render in ratatui frame
4. Handle VtEvent for terminal events

**APIs:** Screen::new(), Screen::size(), VtEvent, render_screen(), write_screen_diff()

**Pitfalls:** Termtui is for embedding real terminal output; not a terminal emulator UI widget

**Source:** [crates/primitives/termtui/src/lib.rs](crates/primitives/termtui/src/lib.rs)

### Widgets Usage Cards

#### MarkdownWidget

**Use when:** Need to render markdown content with syntax highlighting, TOC, and scroll state

**Enable:** feature = "markdown-preview" in Cargo.toml

**Import:** `use ratkit::MarkdownWidget`

**Minimal flow:**
1. Create `MarkdownState::default()` and store it persistently
2. Create `MarkdownWidget::from_state(content, &mut state)`
3. Call `frame.render_stateful_widget(widget, area, &mut state.scroll)`

**APIs:** MarkdownWidget::from_state(), MarkdownState::default(), MarkdownWidget::show_toc(), MarkdownWidget::show_scrollbar()

**Pitfalls:** Scroll state must persist across frames; Mouse capture required for click handling

**Source:** [crates/widgets/markdown-preview/src/widgets/widget/mod.rs](crates/widgets/markdown-preview/src/widgets/widget/mod.rs)

#### CodeDiff

**Use when:** Displaying side-by-side code diffs with syntax highlighting

**Enable:** feature = "code-diff" in Cargo.toml

**Import:** `use ratkit_code_diff::{CodeDiff, DiffConfig, get_git_diff}`

**Minimal flow:**
1. Create `DiffConfig::default()`
2. Call `get_git_diff(old_text, new_text, &config)`
3. Create `CodeDiff::new(diff_result)`
4. `frame.render_widget(code_diff, area)`

**APIs:** get_git_diff(), DiffConfig::default(), CodeDiff::new(), CodeDiff::render()

**Pitfalls:** Similar crate dependency required for diff algorithm

**Source:** [crates/widgets/code-diff/src/code_diff/mod.rs](crates/widgets/code-diff/src/code_diff/mod.rs)

#### AIChat

**Use when:** Building interactive AI chat interfaces with file attachments and commands

**Enable:** feature = "ai-chat" in Cargo.toml

**Import:** `use ratkit_ai_chat::{AIChat, AIChatEvent, MessageStore}`

**Minimal flow:**
1. Create `AIChat::new()`
2. Handle key events via `AIChat::handle_key(key)`
3. Call `AIChat::render(frame, area)`
4. Process `AIChatEvent::MessageSubmitted` events

**APIs:** AIChat::new(), AIChat::handle_key(), AIChat::render(), AIChatEvent::MessageSubmitted, MessageStore::add()

**Pitfalls:** Ctrl+J required for newlines in input; @ prefix triggers file mode, / prefix triggers command mode

**Source:** [crates/widgets/ai-chat/src/ai_chat.rs](crates/widgets/ai-chat/src/ai_chat.rs)

#### HotkeyFooter

**Use when:** Displaying keyboard shortcuts in a styled footer bar (aerospace-tui style)

**Enable:** feature = "hotkey-footer" in Cargo.toml

**Import:** `use ratkit_hotkey_footer::{HotkeyFooter, HotkeyItem}`

**Minimal flow:**
1. Create `HotkeyItem::new(key, description)` for each shortcut
2. Create `HotkeyFooter::new(items_vec)`
3. Configure colors via `.key_color()`, `.description_color()`, `.background_color()`
4. `frame.render_widget(footer, area)`

**APIs:** HotkeyFooter::new(), HotkeyFooter::render(), HotkeyItem::new(), HotkeyFooter::key_color()

**Pitfalls:** Footer renders as single line - ensure adequate height

**Source:** [crates/widgets/hotkey-footer/src/footer.rs](crates/widgets/hotkey-footer/src/footer.rs)

#### FileSystemTree

**Use when:** Navigating directory structures with icons, selection, and filter mode

**Enable:** feature = "file-system-tree" in Cargo.toml

**Import:** `use ratkit_file_system_tree::{FileSystemTree, FileSystemTreeState}`

**Minimal flow:**
1. Create `FileSystemTree::new(root_path)`
2. Create `FileSystemTreeState::new()`
3. Handle navigation via `select_next()`, `select_previous()`, `toggle_selected()`
4. `frame.render_stateful_widget(tree, area, &mut state)`

**APIs:** FileSystemTree::new(), FileSystemTreeState::new(), FileSystemTree::select_next(), FileSystemTree::toggle_selected(), FileSystemTree::handle_filter_key()

**Pitfalls:** expand_directory returns IO::Result - handle errors; Paths are Vec<usize> indices, not strings

**Source:** [crates/widgets/file-system-tree/src/widget.rs](crates/widgets/file-system-tree/src/widget.rs)

#### ThemePicker

**Use when:** Providing theme selection via modal dialog with search and live preview

**Enable:** feature = "theme-picker" in Cargo.toml

**Import:** `use ratkit_theme_picker::{ThemePicker, ThemePickerEvent, ThemeColors}`

**Minimal flow:**
1. Create `ThemePicker::new()`
2. Call `.show()` to display modal
3. Handle keys via `ThemePicker::handle_key(key)`
4. Process `ThemePickerEvent::Selected`
5. `frame.render_widget(picker, area)`

**APIs:** ThemePicker::new(), ThemePicker::show(), ThemePicker::handle_key(), ThemePickerEvent::Selected, ThemeColors::default()

**Pitfalls:** Must call show() before rendering; handle_key returns None if picker not visible

**Source:** [crates/widgets/theme-picker/src/picker.rs](crates/widgets/theme-picker/src/picker.rs)

### Services Usage Cards

#### FileWatcher

**Use when:** Need to detect file system changes (single file or directory tree)

**Enable:** feature = "file-watcher" in Cargo.toml

**Import:** `use ratkit::FileWatcher`

**Minimal flow:**
1. Create watcher via `FileWatcher::for_directory()` or `FileWatcher::for_file()`
2. Add paths via `watch()` method
3. Call `check_for_changes()` in event loop
4. Retrieve changed paths via `get_changed_paths()`

**APIs:** FileWatcher::for_file(), FileWatcher::for_directory(), FileWatcher::new(), FileWatcher::with_config(), FileWatcher::watch(), FileWatcher::unwatch(), FileWatcher::check_for_changes(), FileWatcher::drain_events(), FileWatcher::get_changed_paths()

**Pitfalls:** Must drain events or check changes regularly; Recursive watching can be resource-intensive; Debounce may mask rapid successive changes

**Source:** [crates/services/file-watcher/src/lib.rs](crates/services/file-watcher/src/lib.rs)

#### GitWatcher

**Use when:** Need to detect git repository state changes (commits, branches, refs)

**Enable:** feature = "git-watcher" in Cargo.toml

**Import:** `use ratkit::GitWatcher`

**Minimal flow:**
1. Create `GitWatcher::new()` or with_config()
2. Watch a git repository path via watch()
3. Poll `check_for_changes()` in event loop
4. Drain events after change detected

**APIs:** GitWatcher::new(), GitWatcher::with_config(), GitWatcher::watch(), GitWatcher::unwatch(), GitWatcher::check_for_changes(), GitWatcher::drain_events()

**Pitfalls:** Only watches .git directory - doesn't detect working tree edits; Won't catch uncommitted file changes - use RepoWatcher instead

**Source:** [crates/services/git-watcher/src/lib.rs](crates/services/git-watcher/src/lib.rs)

#### RepoWatcher

**Use when:** Need complete git repository monitoring including working tree changes

**Enable:** feature = "repo-watcher" in Cargo.toml

**Import:** `use ratkit::RepoWatcher`

**Minimal flow:**
1. Create `RepoWatcher::new()` or with_config()
2. Watch repository root via watch()
3. Call `check_for_changes()` in event loop
4. Get changes via `get_change_set()` returning GitChangeSet

**APIs:** RepoWatcher::new(), RepoWatcher::with_config(), RepoWatcher::watch(), RepoWatcher::check_for_changes(), RepoWatcher::get_change_set()

**Pitfalls:** Requires git binary for git status --porcelain; Returns repository-relative paths - convert to absolute for display

**Source:** [crates/services/repo-watcher/src/lib.rs](crates/services/repo-watcher/src/lib.rs)

#### HotkeyRegistry

**Use when:** Need centralized hotkey management with scope-based filtering

**Enable:** feature = "hotkey-service" in Cargo.toml

**Import:** `use ratkit::{Hotkey, HotkeyRegistry, HotkeyScope}`

**Minimal flow:**
1. Create `HotkeyRegistry::new()`
2. Register hotkeys via register() with `Hotkey::new()`
3. Process crossterm key events separately
4. Query active hotkeys via `get_hotkeys()` or `lookup()`

**APIs:** Hotkey::new(), HotkeyRegistry::new(), HotkeyRegistry::register(), HotkeyRegistry::get_hotkeys(), HotkeyRegistry::lookup(), HotkeyRegistry::set_active_scope(), HotkeyRegistry::get_active_scope()

**Pitfalls:** Registry doesn't consume/process input - handle crossterm events separately; Scopes must match exactly for filtering; Priority only resolves conflicts between same-key hotkeys

**Source:** [crates/services/hotkey-service/src/lib.rs](crates/services/hotkey-service/src/lib.rs)

### Core Runtime Usage Cards

#### Runner

**Use when:** Setting up the main application event loop and terminal

**Enable:** Default feature (no feature flag needed)

**Import:** `use ratkit::{run, CoordinatorApp, RunnerConfig}`

**Minimal flow:**
1. Define struct MyApp; impl CoordinatorApp for MyApp { on_event(), on_draw() }
2. Configure `RunnerConfig::default()` or custom tick/layout_debounce
3. Call `run(app, config)` to enter the event loop

**APIs:** run(), run_with_diagnostics(), Runner::new(), Runner::handle_event(), Runner::render(), RunnerConfig::default()

**Pitfalls:** Runner takes ownership of app - wrap shared state in Arc<RwLock<>> or Rc<RefCell<>>

**Source:** [crates/ratkit/src/runner_helper.rs](crates/ratkit/src/runner_helper.rs)

#### LayoutCoordinator

**Use when:** Orchestrating layout, focus, and mouse routing together

**Enable:** Default (part of core runtime)

**Import:** `use ratkit::LayoutCoordinator`

**Minimal flow:**
1. Create `LayoutCoordinator::new(app)`
2. Register elements with `handle_event(Register(metadata, element))`
3. Route events through `handle_event(Keyboard/Mouse/Tick/Resize)`
4. Call `invalidate_layout()` after structural changes

**APIs:** LayoutCoordinator::new(), handle_event(), invalidate_layout(), invalidate_elements(), get_diagnostic_info()

**Pitfalls:** Must call invalidate_layout() after registering/unregistering elements or changes won't recalculate

**Source:** [crates/ratkit/src/coordinator.rs](crates/ratkit/src/coordinator.rs)

#### LayoutManager

**Use when:** Computing element geometry with three-region layout (Top/Center/Bottom)

**Enable:** Default (part of core runtime)

**Import:** `use ratkit::LayoutManager`

**Minimal flow:**
1. Create `LayoutManager::new()`
2. Call `on_resize(width, height)` on terminal resize
3. Register elements via registry().register()
4. Call `recompute()` to recalculate element rects

**APIs:** LayoutManager::new(), on_resize(), recompute(), get_region_area(), hit_test(), terminal_size()

**Pitfalls:** Minimum terminal size enforced (10x5) - resize below threshold returns LayoutError; Fixed heights for Top/Bottom reduce Center region proportionally

**Source:** [crates/ratkit/src/layout.rs](crates/ratkit/src/layout.rs)

#### FocusManager

**Use when:** Managing keyboard focus traversal between interactive elements

**Enable:** Default (part of core runtime)

**Import:** `use ratkit::FocusManager`

**Minimal flow:**
1. Create `FocusManager::new()`
2. Register focusable elements via registry_mut().register()
3. Use `handle_request(FocusRequest::Next/Previous/First/Last)` for traversal
4. Use `capture_focus()` for modal dialogs (overrides normal focus)

**APIs:** FocusManager::new(), handle_request(), focus_next(), focus_previous(), capture_focus(), release_capture(), focused()

**Pitfalls:** Elements must have focusable=true in metadata to appear in focus traversal; Captured focus takes priority over focus stack - remember to release_capture() when modal closes

**Source:** [crates/ratkit/src/focus.rs](crates/ratkit/src/focus.rs)

#### MouseRouter

**Use when:** Routing mouse events to elements with optional capture

**Enable:** Default (part of core runtime)

**Import:** `use ratkit::MouseRouter`

**Minimal flow:**
1. Create `MouseRouter::new()`
2. Call `capture(id)` when element starts drag/scroll operation
3. `route_mouse_event(x, y, layout)` returns elements hit in z-order
4. Call `release_capture()` when operation ends

**APIs:** MouseRouter::new(), capture(), release_capture(), route_mouse_event(), is_captured(), check_capture_expired()

**Pitfalls:** Mouse capture expires after 5 seconds - implement renewal for long operations; Captured element receives all mouse events until released or capture expires

**Source:** [crates/ratkit/src/mouse_router.rs](crates/ratkit/src/mouse_router.rs)

#### ElementRegistry

**Use when:** Managing UI component lifecycle and metadata

**Enable:** Default (part of core runtime)

**Import:** `use ratkit::{ElementRegistry, Element, ElementMetadata}`

**Minimal flow:**
1. Implement Element trait with id(), on_render(), on_keyboard(), on_mouse(), on_focus_gain(), on_focus_loss(), on_tick()
2. Create `ElementMetadata::new(id, Region::Center)`
3. Call `registry_mut().register(metadata, Arc::new(my_element))`
4. Call `unregister(id)` to remove element

**APIs:** ElementRegistry::new(), register(), unregister(), get_metadata(), get_strong_ref(), focusable_elements(), elements_by_region()

**Pitfalls:** Arc<dyn Element> required for registration - element must be Send+Sync; Registry stores weak references - element must be kept alive externally or handle will become None

**Source:** [crates/ratkit/src/registry.rs](crates/ratkit/src/registry.rs)

## API Reference

### Primitives APIs

| Component | Key Methods |
|-----------|-------------|
| Button | new(), normal_style(), hover_style(), update_hover(), is_clicked(), render(), render_with_title() |
| Pane | new(), with_icon(), with_padding(), with_uniform_padding(), with_text_footer(), border_style(), border_type(), title_style(), footer_style() |
| Dialog | new(), info(), warning(), error(), success(), confirm(), buttons(), style(), border_color(), width_percent() |
| Toast | new(), add(), cleanup() |
| StatusLineStacked | new(), start(), end(), center() |
| calculate_scroll_offset | (selected_index, visible_count, total_count) -> usize |
| MenuBar | new(), normal_style(), selected_style(), update_hover(), handle_mouse(), render() |
| ResizableGrid | new(), ResizableGridWidget::new(), ResizableGridWidgetState::new() |
| TreeView | new(), render_fn(), TreeViewState::new(), select(), toggle_expansion(), TreeNavigator::new() |
| WidgetEvent | None, Selected, Toggled, Scrolled, FilterModeChanged, FilterModeExited, MenuSelected |
| Termtui | Screen::new(), render_screen(), write_screen_diff() |

### Widgets APIs

| Component | Key Methods |
|-----------|-------------|
| MarkdownWidget | from_state(), show_toc(), show_scrollbar() |
| CodeDiff | get_git_diff(), DiffConfig::default(), new(), render() |
| AIChat | new(), handle_key(), render() |
| HotkeyFooter | new(), render(), key_color() |
| FileSystemTree | new(), select_next(), toggle_selected(), handle_filter_key() |
| ThemePicker | new(), show(), handle_key() |

### Services APIs

| Component | Key Methods |
|-----------|-------------|
| FileWatcher | for_file(), for_directory(), watch(), unwatch(), check_for_changes(), drain_events(), get_changed_paths() |
| GitWatcher | new(), watch(), unwatch(), check_for_changes(), drain_events() |
| RepoWatcher | new(), watch(), check_for_changes(), get_change_set() |
| HotkeyRegistry | new(), register(), get_hotkeys(), lookup(), set_active_scope(), get_active_scope() |

### Core Runtime APIs

| Component | Key Methods |
|-----------|-------------|
| Runner | run(), run_with_diagnostics(), RunnerConfig::default() |
| LayoutCoordinator | new(), handle_event(), invalidate_layout(), invalidate_elements() |
| LayoutManager | new(), on_resize(), recompute(), get_region_area(), hit_test() |
| FocusManager | new(), handle_request(), focus_next(), focus_previous(), capture_focus(), release_capture() |
| MouseRouter | new(), capture(), release_capture(), route_mouse_event(), is_captured() |
| ElementRegistry | new(), register(), unregister(), get_metadata(), focusable_elements() |

## Common Pitfalls

### Primitives Pitfalls

1. Dropping widget state between renders causes loss of selection, scroll position, and expansion state
2. Not calling update_hover() on Button/MenuBar before rendering breaks hover state detection
3. Using dialog types without matching button configurations leaves dialogs without interactive buttons
4. TreeView requires TreeViewState to track selection and expansion; creating state on each render resets selection
5. calculate_scroll_offset() returns 0 when total_count <= visible_count (no scroll needed for fit)

### Widgets Pitfalls

1. Creating widget state inside the render loop causes state loss and broken UI
2. Forgetting EnableMouseCapture breaks all mouse interactions including scroll wheel
3. File system tree expand_directory can fail with IO errors that must be handled
4. Theme picker requires handle_key to return events for proper selection handling

### Services Pitfalls

1. Forgetting to call check_for_changes() or drain_events() causes event buffer overflow
2. Watching too many paths without debouncing degrades performance
3. GitWatcher only watches .git directory - won't detect uncommitted working tree changes
4. HotkeyRegistry doesn't handle input - must be paired with crossterm event loop
5. Paths returned by watchers are absolute - normalize for display if needed

### Core Runtime Pitfalls

1. Runner takes ownership of the app - use Rc<RefCell<App>> or Arc<AppState> for shared mutable state across callbacks
2. LayoutManager::recompute() must be called after registering/unregistering elements or changes won't be visible
3. Resize events are debounced (default 16ms) - immediate resize updates require calling LayoutManager::process_pending_resize()
4. Mouse capture expires after 5 seconds by default - implement capture renewal or extend timeout for drag operations
5. Focusable elements must have metadata.with_focusable(true) - FocusManager ignores non-focusable elements during traversal

## Notes

- Feature flags in Cargo.toml enable individual primitives; no default features
- Theme support (AppTheme) requires 'theme' feature and ratkit-theme crate
- ratatui 0.29 is the underlying rendering library for all widgets
- crossterm 0.28 is used for terminal input/events
- All watchers internally use notify crate v6 - handles cross-platform filesystem events
- HotkeyService integrates with crossterm for keyboard event handling
- RepoWatcher spawns both GitWatcher and FileWatcher internally for dual monitoring
- The core runtime uses crossterm for terminal I/O and ratatui for rendering
- All event callbacks return bool indicating if the element handled the event (true) or should propagate (false)
- Layout debouncing prevents excessive recomputation during rapid resize events
- Focus stack maintains focus history; captured focus temporarily overrides the stack
- Markdown widget module structure has changed - widget module is now at widgets/markdown_widget/mod.rs not widgets/widget/mod.rs
