# Markdown Widget - Architecture Overview

## What It Does

The markdown widget is a **feature-rich, scrollable, interactive markdown viewer** for ratatui-based TUI applications. It renders markdown with support for:

- **Scrolling**: Keyboard (j/k/arrows/PageUp/PageDown) and mouse wheel
- **Line highlighting**: Click to focus, current line highlighted
- **Collapsible sections**: Click headings to collapse/expand
- **Collapsible frontmatter**: Click frontmatter to toggle
- **Expandable content blocks**: "Show more"/"Show less" for long content
- **Text selection**: Drag to select text, copy with `y` or `Ctrl+Shift+C`
- **Double-click detection**: Detects double-clicks on lines
- **Table of Contents (TOC)**: Clickable navigation overlay
- **Statusline**: Shows mode (Normal/Drag/Filter) and scroll position
- **Scrollbar**: Visual scroll position indicator
- **Filter mode**: Type `/` to filter/search document
- **Git stats integration**: Display git blame/info (optional)
- **Line numbers**: Optional line numbering
- **Theming**: Supports custom themes and code block syntax highlighting

## Key Ways of Calling It

### 1. From Unified State (Recommended)

```rust
use ratatui_toolkit::{MarkdownState, MarkdownWidget};

// Create unified state
let mut state = MarkdownState::default();
state.source.set_content("# Hello World");
state.display.set_show_line_numbers(true);

// Create widget in render loop
let widget = MarkdownWidget::from_state(state.content(), &mut state)
    .show_toc(true)
    .show_statusline(true)
    .show_scrollbar(true)
    .with_theme(&app_theme);

frame.render_widget(widget, area);
```

### 2. Event Handling Pattern

```rust
// Keyboard events
let content = state.content().to_string();
let (event, sync_state) = {
    let mut widget = MarkdownWidget::from_state(&content, &mut state);
    let event = widget.handle_key_event(key_event);
    let sync_state = widget.get_state_sync();
    (event, sync_state)
};

// Sync state back
sync_state.apply_to(&mut state);

// Handle events
match event {
    MarkdownEvent::Copied { text } => { /* Show toast */ }
    MarkdownEvent::DoubleClick { line_number, .. } => { /* Handle */ }
    _ => {}
}
```

### 3. Mouse Events Pattern

```rust
// Batch scroll events for performance
let mut scroll_delta = 0;
let mut other_events = Vec::new();

while event::poll(Duration::from_millis(0))? {
    let evt = event::read()?;
    match evt {
        Event::Mouse(mouse) => match mouse.kind {
            MouseEventKind::ScrollUp => scroll_delta -= 1,
            MouseEventKind::ScrollDown => scroll_delta += 1,
            _ => other_events.push(Event::Mouse(mouse)),
        },
        _ => other_events.push(evt),
    }
}

// Apply scroll
if scroll_delta != 0 {
    state.scroll.scroll_down(scroll_delta.abs() as usize);
}

// Handle other mouse events
for evt in other_events {
    if let Event::Mouse(mouse) = evt {
        let sync_state = {
            let mut widget = MarkdownWidget::from_state(&content, &mut state)
                .show_toc(true);
            widget.handle_toc_hover(&mouse, area);
            widget.handle_toc_click(&mouse, area);
            widget.handle_mouse_event(&mouse, area);
            widget.get_state_sync()
        };
        sync_state.apply_to(&mut state);
    }
}
```

## Architecture

### Core Components

#### 1. **MarkdownWidget Struct**
Location: `src/markdown_widget/widget/mod.rs:56`

Holds mutable references to all state components:
- `content`: The markdown string
- `scroll`: Scroll position/viewport/current line
- `source`: Content source (file path or string)
- `cache`: Parsed and rendered cache
- `display`: Display settings (line numbers, themes)
- `collapse`: Section collapse tracking
- `expandable`: Expandable content blocks
- `git_stats_state`: Git statistics
- `vim`: Vim keybinding state (for `gg`, `G`)
- `selection`: Text selection state
- `double_click`: Double-click detection
- `toc_state`: Optional TOC state
- `app_theme`: Optional theme for styling
- Various UI flags (`show_toc`, `show_statusline`, `show_scrollbar`, `bordered`)

#### 2. **MarkdownState (Unified State)**
Location: `src/markdown_widget/state/markdown_state/mod.rs:32`

Bundles all component states into a single struct:
```rust
pub struct MarkdownState {
    pub scroll: ScrollState,
    pub source: SourceState,
    pub cache: CacheState,
    pub display: DisplaySettings,
    pub collapse: CollapseState,
    pub expandable: ExpandableState,
    pub git_stats: GitStatsState,
    pub vim: VimState,
    pub selection: SelectionState,
    pub double_click: DoubleClickState,
    pub toc_hovered: bool,
    pub toc_hovered_entry: Option<usize>,
    pub toc_scroll_offset: usize,
    pub selection_active: bool,
    pub cached_git_stats: Option<GitStats>,
    pub rendered_lines: Vec<Line<'static>>,
    pub filter: Option<String>,
    pub filter_mode: bool,
}
```

#### 3. **WidgetStateSync (State Synchronization)**
Location: `src/markdown_widget/widget/methods/sync_state_back.rs:11`

Captures transient widget state that needs to sync back:
- `toc_hovered`: TOC hover state
- `toc_hovered_entry`: Which TOC entry is hovered
- `toc_scroll_offset`: TOC scroll position
- `selection_active`: Whether text selection is active
- `last_double_click`: Last double-click info
- `filter`: Current filter text
- `filter_mode`: Whether filter mode is active

**Pattern**: Create widget → Handle events → Get sync state → Apply to MarkdownState

### Event Flow

```
Input (Key/Mouse)
    ↓
MarkdownWidget::handle_key_event() / handle_mouse_event()
    ↓
Internal state updates (scroll, selection, etc.)
    ↓
Returns MarkdownEvent enum
    ↓
Application handles event (show toast, etc.)
    ↓
widget.get_state_sync() → sync_state.apply_to(&mut state)
    ↓
State persisted for next frame
```

### Rendering Pipeline

Location: `src/markdown_widget/widget/traits/widget.rs:27`

1. **Reserve layout**: Border (if enabled) → Statusline → Content area
2. **Calculate TOC area**: Compact (collapsed) or expanded (hovered)
3. **Check cache**:
   - If render cache valid (same content/width/theme): Use cached lines
   - Else if parsed cache valid: Use parsed elements, re-render
   - Else: Parse markdown → Cache parsed → Render to lines
4. **Filter lines**: If filter mode active, only show matching elements
5. **Apply selection highlighting**: If selection active, highlight selected text
6. **Render content**: Draw visible lines to buffer with current line highlight
7. **Render TOC**: Draw TOC overlay if enabled
8. **Render statusline**: Draw mode/scroll info if enabled
9. **Render scrollbar**: Draw scrollbar last (on top)

### Caching Strategy

Two-level caching for performance:

1. **Parsed Cache**: Markdown → Elements (ast-like structure)
   - Key: Content hash
   - Invalidates when content changes

2. **Render Cache**: Elements → Vec<Line> (final render output)
   - Key: Content hash + width + line_numbers + theme + app_theme + heading_collapse
   - Invalidates when any rendering config changes

**Resize optimization**: Uses stale cache during resize (set `is_resizing: true`)

### State Modules

Located in: `src/markdown_widget/state/`

- `scroll_state`: Position (scroll_offset), viewport (viewport_height), current line (current_line)
- `source_state`: Content source (file path or inline string)
- `cache_state`: Parsed and render cache
- `display_settings`: Display config (line numbers, themes, show_heading_collapse)
- `collapse_state`: Track collapsed sections by line index
- `expandable_state`: Track expandable content blocks
- `git_stats_state`: Git blame/info integration
- `vim_state`: Vim keybinding state (for `gg` detection)
- `selection_state`: Text selection (start/end positions)
- `double_click_state`: Double-click detection (timestamp, last line)
- `toc_state`: TOC entries and navigation state

### Widget Modes

Location: `src/markdown_widget/widget/enums/markdown_widget_mode.rs:5`

- **Normal**: Default viewing mode
- **Drag**: Text selection active (dragging mouse)
- **Filter**: Search/filter mode active

### Event Types

Location: `src/markdown_widget/foundation/events/markdown_event.rs:8`

- `None`: No event
- `FocusedLine { line }`: Current focused line changed
- `HeadingToggled { level, text, collapsed }`: Heading collapsed/expanded
- `DoubleClick { line_number, line_kind, content }`: Double-click detected
- `Copied { text }`: Text copied to clipboard
- `SelectionStarted`: Drag started
- `SelectionEnded`: Selection ended
- `Scrolled { offset, direction }`: Content scrolled
- `FilterModeChanged { active, filter }`: Filter mode entered/changed
- `FilterModeExited { line }`: Filter mode exited with Enter

### Constructor Methods

Located in: `src/markdown_widget/widget/constructors/`

- `from_state()`: Create from MarkdownState (main entry point)
- `new()`: Create from individual state components
- `show_toc(bool)`: Enable/disable TOC
- `show_statusline(bool)`: Enable/disable statusline
- `show_scrollbar(bool)`: Enable/disable scrollbar
- `with_theme(&AppTheme)`: Apply theme
- `bordered(bool)`: Enable/disable border
- `toc_config(TocConfig)`: Configure TOC appearance
- `scrollbar_config(ScrollbarConfig)`: Configure scrollbar
- `with_toc_state(&TocState)`: Use provided TOC state
- `toc_hovered(bool)`: Set TOC hover state
- `selection_active(bool)`: Set selection mode

### Event Handler Methods

Located in: `src/markdown_widget/widget/methods/`

- `handle_key_event(KeyEvent)`: Process keyboard input
- `handle_mouse_event(MouseEvent)`: Process mouse clicks/drag
- `handle_toc_hover(MouseEvent)`: Handle TOC hover detection
- `handle_toc_click(MouseEvent)`: Handle TOC click navigation
- `get_state_sync()`: Capture state to sync back
- `sync_state_back(&mut MarkdownState)`: Sync state to MarkdownState

### Key Behaviors

**Line Navigation**:
- `j`/`Down`: Move down, scroll when near bottom
- `k`/`Up`: Move up, scroll when near top
- `gg`: Go to top
- `G`/`End`: Go to bottom
- `PageUp`/`PageDown`: Scroll by viewport height

**Filter Mode** (`/`):
- Type to filter document
- `j`/`k` to navigate filtered results
- `Esc`: Exit filter mode, clear filter
- `Enter`: Exit filter mode, focus current line

**Text Selection**:
- Drag mouse to select text
- `y` or `Ctrl+Shift+C`: Copy selection to clipboard
- `Esc`: Exit selection mode

**Collapsible Sections**:
- Click heading to collapse/expand
- Click frontmatter to toggle
- Collapsed sections skipped in rendering

**TOC Navigation**:
- Hover TOC to expand
- Click entry to scroll to heading
- Scroll TOC if many entries

## Quick Reference: File Locations

| Component | Location |
|-----------|----------|
| Main widget | `src/markdown_widget/widget/mod.rs` |
| Widget render | `src/markdown_widget/widget/traits/widget.rs` |
| Unified state | `src/markdown_widget/state/markdown_state/mod.rs` |
| Event types | `src/markdown_widget/foundation/events/markdown_event.rs` |
| Key handling | `src/markdown_widget/widget/methods/handle_key_event.rs` |
| Mouse handling | `src/markdown_widget/widget/methods/handle_mouse_event.rs` |
| State sync | `src/markdown_widget/widget/methods/sync_state_back.rs` |
| Demo usage | `examples/markdown_demo.rs` |
| Constructors | `src/markdown_widget/widget/constructors/mod.rs` |

## Common Patterns

### Adding a New Feature

1. Add state to appropriate state module (or create new one in `state/`)
2. Add to `MarkdownState` struct
3. Add fields to `MarkdownWidget` struct
4. Update `from_state()` constructor to initialize
5. Add constructor method (e.g., `with_my_feature()`)
6. Add event handler in `methods/`
7. Emit `MarkdownEvent` variant if needed
8. Add field to `WidgetStateSync` if needed
9. Update `render()` in `traits/widget.rs` if UI changes
10. Add to `sync_state_back()` if state needs persistence

### Debugging Rendering

- Check `cache.render` for cached rendered lines
- Check `cache.parsed` for parsed elements
- Check `rendered_lines` in MarkdownWidget for current render
- Add logging in `render()` to trace pipeline stages
- Verify cache keys (hash, width, theme) for invalidation

### Fixing State Issues

- Ensure all state modifications happen inside widget methods
- Always call `get_state_sync()` and `apply_to()` after event handling
- Check that `WidgetStateSync` includes all transient state
- Verify state fields are correctly initialized in `from_state()`
- Ensure mutable borrows are properly scoped (widget drops before sync)

### Performance Optimization

- Leverage two-level cache (parsed + render)
- Use stale cache during resize (`is_resizing` flag)
- Batch scroll events (combine multiple scroll events)
- Check cache validity before re-parsing
- Minimize re-renders by syncing state properly
