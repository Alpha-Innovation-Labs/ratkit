# Markdown Widget Module Refactor Plan

## Overview

Refactor `src/markdown_renderer/` to `src/markdown_widget/` with a clean architecture.

## Goals

1. Rename module from `markdown_renderer` to `markdown_widget`
2. Organize into clear top-level concepts: `widget/`, `foundation/`, `state/`, `extensions/`
3. All toggleable functionality lives under `extensions/`
4. Follow `.nexus/rules/rs.md` conventions (struct directories, one item per file)
5. Ensure clean dependency graph with no cycles

---

## Dependency Graph

```
┌─────────────────────────────────────────────────────────────────┐
│                           widget/                                │
│                 (owns references to state)                       │
│           (consumes events, extensions, foundation)              │
└─────────────────────────────────────────────────────────────────┘
        │ uses                              │ uses
        ▼                                   ▼
┌─────────────────────┐           ┌─────────────────────┐
│      state/         │           │    extensions/      │
│  (scroll, toc,      │           │  (minimap, toc,     │
│   selection)        │           │   theme, selection) │
└─────────────────────┘           └─────────────────────┘
        │ uses                              │ uses
        └──────────────┬────────────────────┘
                       ▼
        ┌─────────────────────────────────────────────────┐
        │                  foundation/                     │
        │    (elements, parser, source, events, types)     │
        └─────────────────────────────────────────────────┘

RULES:
- widget/ owns state references, never the reverse
- extensions/ receive state as parameters, never hold widget references
- state/ and extensions/ ONLY depend on foundation/
- foundation/ has ZERO dependencies on widget/, state/, or extensions/
- Shared types (GitStats, CodeBlockTheme, events) live in foundation/
```

---

## Target Structure

```
src/markdown_widget/
├── mod.rs                            # Public API exports only

├── widget/                           # Main MarkdownWidget (orchestrator)
│   ├── mod.rs                        # MarkdownWidget struct definition
│   ├── constructors/
│   │   ├── mod.rs
│   │   ├── new.rs
│   │   ├── with_toc.rs
│   │   ├── with_minimap.rs
│   │   ├── with_selection.rs
│   │   ├── with_theme.rs
│   │   ├── with_file_watcher.rs
│   │   ├── show_scrollbar.rs
│   │   └── show_statusline.rs
│   ├── methods/
│   │   ├── mod.rs
│   │   ├── handle_key_event.rs
│   │   ├── handle_mouse_event.rs
│   │   ├── is_resizing.rs
│   │   └── mode.rs
│   ├── traits/
│   │   ├── mod.rs
│   │   └── widget.rs                 # impl Widget for MarkdownWidget
│   └── enums/
│       ├── mod.rs
│       └── markdown_widget_mode.rs   # MarkdownWidgetMode only

├── foundation/                       # Zero-dependency building blocks
│   ├── mod.rs
│   │
│   ├── elements/                     # MarkdownElement representation
│   │   ├── mod.rs                    # MarkdownElement struct
│   │   ├── methods/
│   │   │   ├── mod.rs
│   │   │   ├── render.rs             # render_element()
│   │   │   └── render_with_options.rs # render_element_with_options()
│   │   ├── enums/
│   │   │   ├── mod.rs
│   │   │   ├── element_kind.rs
│   │   │   ├── text_segment.rs
│   │   │   ├── code_block_border_kind.rs
│   │   │   ├── code_block_theme.rs   # CodeBlockTheme lives HERE (element representation)
│   │   │   ├── code_block_colors.rs
│   │   │   ├── column_alignment.rs
│   │   │   └── table_border_kind.rs
│   │   ├── constants.rs              # HEADING_ICONS, BULLET_MARKERS
│   │   ├── render_options.rs         # RenderOptions struct
│   │   └── tests.rs
│   │
│   ├── parser/                       # Markdown string → Elements
│   │   ├── mod.rs
│   │   ├── render_markdown_to_elements.rs
│   │   ├── helpers/
│   │   │   ├── mod.rs
│   │   │   └── parse_frontmatter.rs
│   │   └── tests.rs
│   │
│   ├── source/                       # MarkdownSource (String | File)
│   │   ├── mod.rs                    # MarkdownSource enum
│   │   ├── constructors/
│   │   │   ├── mod.rs
│   │   │   ├── from_string.rs
│   │   │   └── from_file.rs
│   │   ├── methods/
│   │   │   ├── mod.rs
│   │   │   ├── content.rs
│   │   │   ├── reload.rs
│   │   │   ├── path.rs
│   │   │   ├── is_file.rs
│   │   │   ├── is_string.rs
│   │   │   └── set_content.rs
│   │   ├── traits/
│   │   │   ├── mod.rs
│   │   │   ├── default.rs
│   │   │   ├── from_str.rs
│   │   │   └── from_string.rs
│   │   └── tests.rs
│   │
│   ├── events/                       # Events crossing component boundaries
│   │   ├── mod.rs
│   │   ├── markdown_event.rs         # MarkdownEvent enum
│   │   └── markdown_double_click_event.rs
│   │
│   ├── types/                        # Shared types used across layers
│   │   ├── mod.rs
│   │   ├── git_stats.rs              # GitStats struct (used by widget + scroll_manager)
│   │   └── selection_pos.rs          # SelectionPos (used by selection extension)
│   │
│   ├── helpers/                      # Shared utility functions
│   │   ├── mod.rs
│   │   ├── hash_content.rs           # Generic hashing
│   │   ├── is_in_area.rs             # Geometry check (Rect containment)
│   │   ├── element_to_plain_text.rs  # Element conversion
│   │   └── get_line_at_position.rs   # Line lookup utility
│   │
│   └── functions.rs                  # render_markdown(), render_markdown_with_style()

├── state/                            # State managers (used by widget)
│   ├── mod.rs
│   │
│   ├── scroll_manager/               # MarkdownScrollManager
│   │   ├── mod.rs
│   │   ├── constructors/
│   │   │   ├── mod.rs
│   │   │   └── new.rs
│   │   ├── methods/
│   │   │   ├── mod.rs
│   │   │   ├── scroll.rs
│   │   │   ├── viewport.rs
│   │   │   ├── cache.rs
│   │   │   ├── source.rs
│   │   │   └── expandable.rs
│   │   ├── traits/
│   │   │   └── mod.rs
│   │   ├── parsed_cache/             # ParsedCache (caching concern, not parsing)
│   │   │   ├── mod.rs
│   │   │   ├── constructors/
│   │   │   └── methods/
│   │   ├── render_cache/             # RenderCache
│   │   │   ├── mod.rs
│   │   │   ├── constructors/
│   │   │   └── methods/
│   │   ├── expandable_state/         # ExpandableState STRUCT (not enum)
│   │   │   ├── mod.rs
│   │   │   └── constructors/
│   │   └── tests.rs
│   │
│   ├── toc_state/                    # TocState - SINGLE source of truth for TOC
│   │   ├── mod.rs                    # scroll_offset, hovered_entry, hovered, entries
│   │   ├── constructors/
│   │   │   ├── mod.rs
│   │   │   └── new.rs
│   │   ├── methods/
│   │   │   ├── mod.rs
│   │   │   ├── scroll.rs
│   │   │   ├── hover.rs
│   │   │   └── entries.rs
│   │   └── enums/
│   │       ├── mod.rs
│   │       └── toc_entry.rs          # TocEntry
│   │
│   ├── selection_state/              # SelectionState
│   │   ├── mod.rs
│   │   ├── constructors/
│   │   │   ├── mod.rs
│   │   │   └── new.rs
│   │   ├── methods/
│   │   │   ├── mod.rs
│   │   │   ├── enter.rs
│   │   │   ├── exit.rs
│   │   │   ├── update_cursor.rs
│   │   │   ├── get_selection.rs
│   │   │   ├── get_selected_text.rs
│   │   │   └── is_in_selection.rs
│   │   └── tests.rs
│   │
│   └── double_click_state/           # DoubleClickState
│       ├── mod.rs
│       ├── constructors/
│       │   ├── mod.rs
│       │   └── new.rs
│       ├── methods/
│       │   ├── mod.rs
│       │   ├── clear_pending.rs
│       │   └── check_pending_timeout.rs
│       ├── traits/
│       │   ├── mod.rs
│       │   └── default.rs
│       └── tests.rs

├── extensions/                       # Toggleable UI extensions
│   ├── mod.rs
│   │
│   ├── minimap/                      # .with_minimap()
│   │   ├── mod.rs                    # Minimap struct
│   │   ├── constructors/
│   │   │   ├── mod.rs
│   │   │   ├── new.rs
│   │   │   ├── config.rs
│   │   │   ├── width.rs
│   │   │   ├── viewport.rs
│   │   │   ├── text_style.rs
│   │   │   └── viewport_style.rs
│   │   ├── methods/
│   │   │   ├── mod.rs
│   │   │   ├── render_to_lines.rs
│   │   │   ├── click_to_line.rs
│   │   │   ├── is_in_viewport.rs
│   │   │   ├── line_densities.rs
│   │   │   └── max_line_width.rs
│   │   ├── traits/
│   │   │   ├── mod.rs
│   │   │   └── widget.rs
│   │   ├── helpers/
│   │   │   ├── mod.rs
│   │   │   ├── density_to_braille.rs
│   │   │   ├── density_pair_to_braille.rs
│   │   │   └── line_to_density.rs
│   │   ├── enums/
│   │   │   ├── mod.rs
│   │   │   └── minimap_config.rs
│   │   └── tests.rs
│   │
│   ├── toc/                          # .with_toc() - UI widget only
│   │   ├── mod.rs                    # Toc struct (receives &TocState)
│   │   ├── constructors/
│   │   │   ├── mod.rs
│   │   │   └── new.rs
│   │   ├── methods/
│   │   │   ├── mod.rs
│   │   │   ├── render_compact.rs
│   │   │   └── render_expanded.rs
│   │   ├── traits/
│   │   │   ├── mod.rs
│   │   │   └── widget.rs
│   │   ├── enums/
│   │   │   ├── mod.rs
│   │   │   └── toc_config.rs         # TocConfig (display options)
│   │   └── tests.rs
│   │
│   ├── theme/                        # .with_theme()
│   │   ├── mod.rs
│   │   │
│   │   ├── markdown_theme/           # MarkdownTheme struct (orchestration)
│   │   │   ├── mod.rs
│   │   │   ├── constructors/
│   │   │   └── methods/
│   │   │
│   │   ├── markdown_style/           # MarkdownStyle struct (raw values)
│   │   │   ├── mod.rs                # Icon/color configuration values
│   │   │   └── traits/
│   │   │       ├── mod.rs
│   │   │       └── default.rs
│   │   │
│   │   ├── color_palette/            # ColorPalette struct
│   │   │   ├── mod.rs
│   │   │   ├── constructors/
│   │   │   │   ├── mod.rs
│   │   │   │   └── new.rs
│   │   │   └── methods/
│   │   │       ├── mod.rs
│   │   │       ├── add_color.rs
│   │   │       ├── get.rs
│   │   │       └── get_or_default.rs
│   │   │
│   │   ├── color_mapping/            # ColorMapping struct
│   │   │   ├── mod.rs
│   │   │   └── methods/
│   │   │       ├── mod.rs
│   │   │       └── get_color.rs
│   │   │
│   │   ├── syntax_highlighter/       # SyntaxHighlighter struct
│   │   │   ├── mod.rs
│   │   │   ├── constructors/
│   │   │   └── methods/
│   │   │
│   │   ├── enums/
│   │   │   ├── mod.rs
│   │   │   ├── theme_variant.rs
│   │   │   └── syntax_theme_variant.rs
│   │   │
│   │   ├── palettes/                 # Built-in palettes
│   │   │   ├── mod.rs
│   │   │   ├── dark_default.rs
│   │   │   ├── light_default.rs
│   │   │   └── opencode_dark.rs
│   │   │
│   │   ├── load_theme_from_json.rs
│   │   ├── get_effective_theme_variant.rs
│   │   └── tests.rs
│   │
│   ├── selection/                    # .with_selection() - handlers only
│   │   ├── mod.rs
│   │   ├── handlers/
│   │   │   ├── mod.rs
│   │   │   ├── handle_mouse_event.rs
│   │   │   ├── handle_mouse_event_with_double_click.rs
│   │   │   └── handle_click.rs       # Click-specific logic
│   │   ├── helpers/
│   │   │   ├── mod.rs
│   │   │   └── should_render_line.rs # Rendering logic for selection
│   │   └── tests.rs
│   │
│   └── file_watcher/                 # .with_file_watcher()
│       ├── mod.rs                    # MarkdownFileWatcher struct
│       ├── constructors/
│       │   ├── mod.rs
│       │   └── new.rs
│       ├── methods/
│       │   ├── mod.rs
│       │   ├── watch.rs
│       │   ├── unwatch.rs
│       │   ├── check_for_changes.rs
│       │   └── drain_events.rs
│       ├── traits/
│       │   ├── mod.rs
│       │   └── debug.rs
│       ├── helpers/
│       │   ├── mod.rs
│       │   └── is_relevant_event.rs
│       └── tests.rs

└── internal.rs                       # pub(crate) re-exports for cross-module use
```

---

## Issue Resolution Summary

### Issue 1: GitStats Location
**Problem:** `scroll_manager` imports `GitStats` from `markdown_widget`, creating state → widget dependency.

**Solution:** Move `GitStats` to `foundation/types/git_stats.rs`. Both widget and scroll_manager import from foundation.

### Issue 2: TOC State Duplication
**Problem:** TOC state exists in both `MarkdownScrollManager` and `Toc` struct.

**Solution:** Create `state/toc_state/` as single source of truth. The `Toc` extension receives `&TocState` as a parameter and only handles rendering. State mutations happen through `TocState` methods.

```rust
// Before: duplicated state
MarkdownScrollManager { toc_scroll_offset, toc_hovered_entry, toc_hovered }
Toc { scroll_offset, toc_scroll_offset, hovered_index }

// After: single source of truth
TocState { scroll_offset, hovered_entry, hovered, entries }
Toc::new(&toc_state, config)  // receives state reference
```

### Issue 3: ParsedCache Location
**Problem:** Plan incorrectly placed ParsedCache in `foundation/parser/`.

**Solution:** Keep `ParsedCache` in `state/scroll_manager/parsed_cache/`. Caching is a consumer concern, not a producer concern.

### Issue 4: CodeBlockTheme Dependency
**Problem:** `CodeBlockTheme` in `extensions/theme/` would create state → extensions dependency.

**Solution:** Move `CodeBlockTheme` to `foundation/elements/enums/code_block_theme.rs`. It's element representation, not theming logic.

### Issue 5: MarkdownStyle vs MarkdownTheme
**Problem:** Unclear relationship between the two types.

**Solution:** Keep both in `extensions/theme/`:
- `MarkdownStyle` → raw configuration values (icons, colors, spacing)
- `MarkdownTheme` → orchestration layer that uses MarkdownStyle + ColorPalette + SyntaxHighlighter

```rust
// Relationship
MarkdownTheme {
    style: MarkdownStyle,      // Raw values
    palette: ColorPalette,     // Color definitions
    highlighter: SyntaxHighlighter, // Code highlighting
}
```

### Issue 6: Helper Functions Mixed Concerns
**Problem:** Helpers have different destinations based on their purpose.

**Solution:** Split by concern:

| Helper | Destination | Reason |
|--------|-------------|--------|
| `hash_content.rs` | `foundation/helpers/` | Generic utility |
| `is_in_area.rs` | `foundation/helpers/` | Geometry utility |
| `element_to_plain_text.rs` | `foundation/helpers/` | Element conversion |
| `get_line_at_position.rs` | `foundation/helpers/` | Shared lookup |
| `handle_click.rs` | `extensions/selection/handlers/` | Selection-specific |
| `should_render_line.rs` | `extensions/selection/helpers/` | Selection rendering |

### Issue 7: RenderCache Theme Data
**Problem:** RenderCache contains `theme: CodeBlockTheme` creating dependency issue.

**Solution:** Since `CodeBlockTheme` moves to `foundation/elements/`, no dependency violation. RenderCache in `state/` imports from `foundation/`.

### Issue 8: Render Functions Destination
**Problem:** Render functions need explicit homes.

**Solution:**
- `render_markdown()` → `foundation/functions.rs`
- `render_markdown_with_style()` → `foundation/functions.rs`
- `render_element()` → `foundation/elements/methods/render.rs`
- `render_element_with_options()` → `foundation/elements/methods/render_with_options.rs`
- `RenderOptions` → `foundation/elements/render_options.rs`

### Issue 9: Selection State vs Handlers Split
**Problem:** Previous plan split selection state and handlers across directories.

**Solution:**
- State structs (`SelectionState`, `DoubleClickState`) → `state/` (they're state)
- Handlers → `extensions/selection/handlers/` (they're UI logic)

This matches the pattern: state lives in `state/`, UI extensions live in `extensions/`. Extensions receive `&mut SelectionState` as parameters.

### Issue 10: ExpandableState Type
**Problem:** Plan listed it as enum, but it's actually a struct.

**Solution:** Fixed. `ExpandableState` is a struct in `state/scroll_manager/expandable_state/`.

### Issue 11: pub(crate) Re-exports
**Problem:** Internal functions need new homes after flattening.

**Solution:** Create `internal.rs` at module root for pub(crate) re-exports:

```rust
// internal.rs - pub(crate) items for cross-module use
pub(crate) use foundation::helpers::get_line_at_position::find_line_at_position;
pub(crate) use extensions::selection::handlers::handle_mouse_event;
```

### Issue 12: Events Location
**Problem:** Events cross component boundaries, shouldn't be nested under widget.

**Solution:** Move to `foundation/events/`:
- `markdown_event.rs` → `foundation/events/`
- `markdown_double_click_event.rs` → `foundation/events/`

Events are consumed by external applications, produced by widget internals. Foundation is the right neutral ground.

---

## Migration Mapping

### Phase 1: Create Foundation Structure

```bash
mkdir -p src/markdown_widget/foundation/{elements,parser,source,events,types,helpers}
```

| Current Location | New Location |
|------------------|--------------|
| `markdown_elements/` | `foundation/elements/` |
| `markdown_elements/enums/code_block_theme.rs` | `foundation/elements/enums/` (stays) |
| `render_markdown_to_lines/` | `foundation/parser/` |
| `markdown_source/` | `foundation/source/` |
| `markdown_widget/markdown_event.rs` | `foundation/events/` |
| `markdown_widget/markdown_double_click_event/` | `foundation/events/` |
| (new) `GitStats` | `foundation/types/git_stats.rs` |
| (new) `SelectionPos` | `foundation/types/selection_pos.rs` |
| `markdown_widget/helpers/hash_content.rs` | `foundation/helpers/` |
| `markdown_widget/helpers/is_in_area.rs` | `foundation/helpers/` |
| `markdown_widget/helpers/element_to_plain_text.rs` | `foundation/helpers/` |
| `markdown_widget/helpers/get_line_at_position.rs` | `foundation/helpers/` |

### Phase 2: Create State Structure

| Current Location | New Location |
|------------------|--------------|
| `scroll_manager/` | `state/scroll_manager/` |
| `markdown_widget/selection_state/` | `state/selection_state/` |
| `markdown_widget/double_click_state/` | `state/double_click_state/` |
| (new) TOC state from scroll_manager | `state/toc_state/` |

**Note:** Extract TOC-related fields from `MarkdownScrollManager` into new `TocState`.

### Phase 3: Create Extensions Structure

| Current Location | New Location |
|------------------|--------------|
| `minimap/` | `extensions/minimap/` |
| `toc/` | `extensions/toc/` (UI only, receives &TocState) |
| `theme/` | `extensions/theme/` |
| `markdown_style/` | `extensions/theme/markdown_style/` |
| `syntax_highlighter/` | `extensions/theme/syntax_highlighter/` |
| `file_watcher/` | `extensions/file_watcher/` |
| `markdown_widget/handle_mouse_event*.rs` | `extensions/selection/handlers/` |
| `markdown_widget/helpers/handle_click.rs` | `extensions/selection/handlers/` |
| `markdown_widget/helpers/should_render_line.rs` | `extensions/selection/helpers/` |

### Phase 4: Flatten Widget

| Current Location | New Location |
|------------------|--------------|
| `markdown_widget/markdown_widget/mod.rs` | `widget/mod.rs` |
| `markdown_widget/markdown_widget/constructors/` | `widget/constructors/` |
| `markdown_widget/markdown_widget/methods/` | `widget/methods/` |
| `markdown_widget/markdown_widget/traits/` | `widget/traits/` |
| `markdown_widget/markdown_widget/enums/mode.rs` | `widget/enums/markdown_widget_mode.rs` |

**Add builder constructors:**
- `widget/constructors/with_toc.rs`
- `widget/constructors/with_minimap.rs`
- `widget/constructors/with_selection.rs`
- `widget/constructors/with_theme.rs`
- `widget/constructors/with_file_watcher.rs`
- `widget/constructors/show_scrollbar.rs`
- `widget/constructors/show_statusline.rs`

### Phase 5: Distribute Tests

Each module gets a `tests.rs` file with `#[cfg(test)]` module. Delete root `tests/` folder.

### Phase 6: Create Internal Re-exports

Create `internal.rs` for pub(crate) functions used across modules.

### Phase 7: Delete Obsolete

| Delete | Reason |
|--------|--------|
| Root `tests/` folder | Distributed to modules |
| `markdown_widget/` folder | After flattening |

### Phase 8: Rename Module

```bash
mv src/markdown_renderer src/markdown_widget
# Update src/lib.rs imports
```

---

## Public API (New mod.rs)

```rust
//! Markdown rendering widget for ratatui applications.
//!
//! Provides a feature-rich markdown viewer with optional extensions:
//! - Table of contents (TOC)
//! - Minimap navigation
//! - Syntax highlighting
//! - Text selection
//! - File watching

// Foundation (always available)
pub use foundation::elements::{
    MarkdownElement, ElementKind, TextSegment,
    CodeBlockBorderKind, CodeBlockTheme, CodeBlockColors,
    ColumnAlignment, TableBorderKind,
    RenderOptions,
    HEADING_ICONS, BULLET_MARKERS,
};
pub use foundation::elements::methods::{render_element, render_element_with_options};
pub use foundation::parser::render_markdown_to_elements;
pub use foundation::source::MarkdownSource;
pub use foundation::events::{MarkdownEvent, MarkdownDoubleClickEvent};
pub use foundation::types::{GitStats, SelectionPos};
pub use foundation::functions::{render_markdown, render_markdown_with_style};

// Widget
pub use widget::MarkdownWidget;
pub use widget::enums::MarkdownWidgetMode;

// State (always required)
pub use state::scroll_manager::{MarkdownScrollManager, ExpandableState};
pub use state::toc_state::{TocState, TocEntry};
pub use state::selection_state::SelectionState;
pub use state::double_click_state::DoubleClickState;

// Extensions (toggleable)
pub use extensions::minimap::{Minimap, MinimapConfig};
pub use extensions::toc::{Toc, TocConfig};
pub use extensions::theme::{
    MarkdownTheme, MarkdownStyle,
    ColorPalette, ColorMapping, ThemeVariant,
    SyntaxHighlighter, SyntaxThemeVariant,
    get_effective_theme_variant, load_theme_from_json, palettes,
};
pub use extensions::file_watcher::MarkdownFileWatcher;
```

---

## Widget Builder API

```rust
// Minimal usage
let widget = MarkdownWidget::new(content, &mut scroll);

// With TOC (requires TocState)
let widget = MarkdownWidget::new(content, &mut scroll)
    .with_toc(&mut toc_state, TocConfig::default());

// With selection (requires SelectionState + DoubleClickState)
let widget = MarkdownWidget::new(content, &mut scroll)
    .with_selection(&mut selection, &mut double_click);

// With minimap
let widget = MarkdownWidget::new(content, &mut scroll)
    .with_minimap(MinimapConfig::default());

// With theme
let widget = MarkdownWidget::new(content, &mut scroll)
    .with_theme(&app_theme);

// Render options
let widget = MarkdownWidget::new(content, &mut scroll)
    .show_scrollbar(true)
    .show_statusline(true);

// Full-featured
let widget = MarkdownWidget::new(content, &mut scroll)
    .with_toc(&mut toc_state, TocConfig::default())
    .with_selection(&mut selection, &mut double_click)
    .with_minimap(MinimapConfig::default())
    .with_theme(&app_theme)
    .show_scrollbar(true)
    .show_statusline(true);
```

---

## Design Decisions

### Why `foundation/` not `core/`?
`core` in Rust implies std-like low-level functionality. These are foundational building blocks.

### Why `extensions/` not `features/`?
Avoids confusion with Cargo features. Extensions are runtime-toggleable.

### Why events in `foundation/`?
Events cross component boundaries. They're produced by widget internals and consumed by external applications. Foundation is neutral ground that both can depend on.

### Why `GitStats` in `foundation/types/`?
It's used by both widget (display) and scroll_manager (tracking). Shared types prevent circular dependencies.

### Why `CodeBlockTheme` in `foundation/elements/`?
It's element representation (how code blocks look), not theming logic. This prevents state → extensions dependency.

### Why separate `TocState` from `Toc`?
- `TocState` (in state/) = data (scroll position, hover state, entries)
- `Toc` (in extensions/) = UI rendering
This matches the state/UI separation pattern. Toc receives `&TocState`.

### Why `MarkdownStyle` separate from `MarkdownTheme`?
- `MarkdownStyle` = raw values (icon chars, color values, spacing)
- `MarkdownTheme` = orchestration (combines style + palette + highlighter)
Keeps concerns separate.

### Why selection state in `state/` but handlers in `extensions/`?
- State structs are data that persists across renders
- Handlers are UI logic that processes events
Extensions receive `&mut SelectionState` as parameters, consistent with "extensions receive state".

### Thread Safety for Caches
`ParsedCache` and `RenderCache` are **not thread-safe** by design:
- Owned by `MarkdownScrollManager`, borrowed mutably by widget
- `MarkdownFileWatcher` uses polling (`check_for_changes()`), not callbacks
- No async code touches caches

Future async would need `Arc<RwLock<_>>`.

---

## Execution Order

1. **Create foundation/** - elements, parser, source, events, types, helpers
2. **Move shared types** - GitStats, SelectionPos, events to foundation
3. **Move CodeBlockTheme** - to foundation/elements/enums/
4. **Create state/** - scroll_manager, toc_state, selection_state, double_click_state
5. **Extract TocState** - from scroll_manager into separate module
6. **Create extensions/** - minimap, toc, theme, selection, file_watcher
7. **Move MarkdownStyle** - into extensions/theme/
8. **Flatten widget/** - from nested markdown_widget/
9. **Create internal.rs** - pub(crate) re-exports
10. **Update all mod.rs** - re-exports
11. **Update root mod.rs** - public API
12. **Rename** - markdown_renderer → markdown_widget
13. **Update external imports**
14. **cargo check** - fix compilation
15. **cargo test** - verify tests pass
16. **Delete obsolete** - old folders, root tests/

---

## Verification Checklist

After refactor:
- [ ] `cargo check` passes
- [ ] `cargo test` passes
- [ ] `cargo doc` generates clean documentation
- [ ] No dependency cycles (foundation ← state ← widget, foundation ← extensions ← widget)
- [ ] All public API items documented
- [ ] Builder pattern works as designed
- [ ] TOC state is single source of truth
- [ ] CodeBlockTheme accessible without extensions/ dependency
- [ ] Events importable from foundation/
- [ ] GitStats importable from foundation/
