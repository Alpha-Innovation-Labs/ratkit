# MarkdownWidget

A feature-rich markdown rendering widget for ratatui TUI applications. Provides interactive markdown viewing with syntax highlighting, table of contents navigation, text selection, and more.

## Features

- **Full Markdown Rendering**: Supports GFM (GitHub Flavored Markdown) including tables, code blocks, blockquotes, lists, and more
- **Syntax Highlighting**: Automatic syntax highlighting for code blocks using syntect with multiple theme support
- **Table of Contents (TOC)**: Interactive sidebar showing document headings with click-to-navigate
- **Text Selection**: Click-and-drag text selection with automatic clipboard copy
- **Click-to-Highlight**: Click any line to highlight it (useful for tracking position while reading)
- **Section Collapsing**: Click on headings to collapse/expand document sections
- **Expandable Content**: Long content blocks show "Show more/Show less" functionality
- **Scroll Tracking**: Accurate scroll position with scrollbar visualization
- **Line Numbers**: Optional document line numbers and per-element line numbers
- **Vim Keybindings**: Optional vim-style navigation (gg, G, j, k)
- **Git Integration**: Display git status and blame information for files
- **Theme Support**: Light/dark mode themes with custom color palette loading from JSON
- **Event System**: Rich event emission for application integration

## Mouse Capture Requirement

For mouse interactions (click, drag, hover, scroll wheel) to work, you must enable mouse capture with crossterm:

```rust
use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();

    // On startup:
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // ... your application code ...

    // On cleanup:
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}
```

Without `EnableMouseCapture`, scroll wheel events may still work (terminal-dependent), but click events will not be received by the application.

## Architecture

The crate follows a modular architecture with four main modules:

```
markdown_widget/
├── state/          # State management modules
├── foundation/     # Core rendering and parsing
├── extensions/     # Optional feature extensions
└── widget/         # Main widget implementation
```

### State Module

Contains all state management for the widget:

- `MarkdownState` - Unified state (recommended for most use cases)
- `ScrollState` - Scroll position, viewport, current line
- `SourceState` - Content source (file or string) management
- `CacheState` - Parsed and render caching
- `DisplaySettings` - Display configuration (line numbers, themes)
- `CollapseState` - Section collapse tracking
- `ExpandableState` - Expandable content state
- `GitStatsState` - Git file statistics
- `VimState` - Vim keybinding state
- `SelectionState` - Text selection state
- `TocState` - Table of contents state
- `DoubleClickState` - Double-click detection

### Foundation Module

Core functionality that's always available:

- `MarkdownSource` - Content source abstraction (string or file)
- `MarkdownElement` - Renderable markdown element struct
- `render_markdown_to_elements()` - Parse markdown to elements
- `render()` - Render a single element
- `MarkdownEvent` - Events emitted by the widget
- `render_markdown()` - High-level rendering function

### Extensions Module

Optional extensions that can be enabled/disabled:

- `Toc` - Table of contents sidebar
- `CustomScrollbar` - Visual scrollbar with click support
- `SyntaxHighlighter` - Code block syntax highlighting
- `ColorPalette` - Theme color management
- `MarkdownTheme` - Complete theme configuration

### Widget Module

The main `MarkdownWidget` struct that ties everything together.

## Usage

### Basic Example (Recommended)

Use the unified `MarkdownState` for simpler state management:

```rust
use ratatui_toolkit::{MarkdownWidget, MarkdownState};

let mut state = MarkdownState::default();
state.source.set_content("# Hello World\n\nWelcome to the markdown widget!");

let content = state.content().to_string();
let widget = MarkdownWidget::from_state(&content, &mut state)
    .show_toc(true)
    .show_statusline(true)
    .show_scrollbar(true);
```

### Advanced Example (Individual State Modules)

For more control, use individual state modules:

```rust
use ratatui_toolkit::markdown_widget::{MarkdownWidget, state::*};

let mut scroll = ScrollState::default();
let mut source = SourceState::default();
let mut cache = CacheState::default();
let display = DisplaySettings::default();
let mut collapse = CollapseState::default();
let mut expandable = ExpandableState::default();
let mut git_stats = GitStatsState::default();
let mut vim = VimState::default();
let mut selection = SelectionState::default();
let mut double_click = DoubleClickState::default();

let widget = MarkdownWidget::new(
    content,
    &mut scroll,
    &mut source,
    &mut cache,
    &display,
    &mut collapse,
    &mut expandable,
    &mut git_stats,
    &mut vim,
    &mut selection,
    &mut double_click,
)
.show_toc(true)
.show_statusline(true)
.show_scrollbar(true);
```

### Loading from a File

```rust
use ratatui_toolkit::markdown_widget::{MarkdownState, MarkdownWidget};

let mut state = MarkdownState::default();
state.source.set_source_file("path/to/document.md");

let content = state.content().to_string();
let widget = MarkdownWidget::from_state(&content, &mut state)
    .show_toc(true);
```

### Rendering

```rust
use ratatui::{Frame, widgets::Widget};

fn render_markdown_view(frame: &mut Frame, widget: &MarkdownWidget, area: Rect) {
    widget.render(area, frame.buffer_mut());
}
```

### Event Handling

The widget emits `MarkdownEvent` variants for application integration:

```rust
use ratatui_toolkit::markdown_widget::foundation::events::MarkdownEvent;

match event {
    MarkdownEvent::FocusedLine { line } => {
        // User focused a specific line
    }
    MarkdownEvent::HeadingToggled { level, text, collapsed } => {
        // A heading was collapsed/expanded
    }
    MarkdownEvent::DoubleClick { line_number, line_kind, content } => {
        // Double-click detected
    }
    MarkdownEvent::Copied { text } => {
        // Text was copied to clipboard
    }
    MarkdownEvent::SelectionStarted => {
        // User started selecting text
    }
    MarkdownEvent::SelectionEnded => {
        // User finished selecting text
    }
    MarkdownEvent::Scrolled { offset, direction } => {
        // Content was scrolled
    }
    _ => {}
}
```

## Table of Contents

The TOC extension provides navigation sidebar:

```rust
use ratatui_toolkit::markdown_widget::extensions::toc::{Toc, TocConfig};

let toc_config = TocConfig::default()
    .compact_width(3)
    .expanded_width(25)
    .show_border(true);

let toc = Toc::new(&toc_state)
    .expanded(false); // Start in compact mode
```

Features:
- Compact mode: Horizontal lines showing heading positions
- Expanded mode: Full heading text with hierarchy indentation
- Current heading highlight
- Hover interactions
- Click-to-scroll navigation

## Syntax Highlighting

Code blocks are automatically syntax highlighted using syntect:

```rust
use ratatui_toolkit::markdown_widget::extensions::theme::SyntaxHighlighter;

let highlighter = SyntaxHighlighter::new();

// Highlight a line of code
let highlighted = highlighter.highlight("fn main() {}", "rust");
```

Available themes:
- Dark default (GitHub Dark)
- Light default (GitHub Light)
- OpenCode Dark

```rust
use ratatui_toolkit::markdown_widget::extensions::theme::palettes;

let dark_palette = palettes::dark_default();
let light_palette = palettes::light_default();
```

## Display Settings

Configure the widget appearance:

```rust
use ratatui_toolkit::markdown_widget::state::DisplaySettings;

let display = DisplaySettings::default()
    .show_line_numbers(true)
    .show_document_line_numbers(true)
    .show_heading_collapse(true)
    .set_code_block_theme(CodeBlockTheme::DarkDefault);
```

## Text Selection

The widget supports click-and-drag text selection:

```rust
use ratatui_toolkit::markdown_widget::extensions::selection::{
    handle_mouse_event, handle_mouse_event_with_double_click,
};

// In your event loop:
if let Event::Mouse(mouse_event) = event.read()? {
    let event = handle_mouse_event_with_double_click(
        mouse_event,
        content_area,
        &mut state.selection,
        &mut state.double_click,
        &state.scroll,
    );
}
```

## Section Collapsing

Users can click on headings to collapse/expand sections:

```rust
use ratatui_toolkit::markdown_widget::state::CollapseState;

let mut collapse = CollapseState::default();

// Collapse a specific section
collapse.collapse_section(section_id);

// Expand all sections
collapse.expand_all();

// Collapse all sections
collapse.collapse_all();
```

## Dependencies

This crate is part of the `ratatui-toolkit` workspace. Key dependencies:

- `ratatui` - TUI framework
- `crossterm` - Terminal handling
- `pulldown-cmark` - Markdown parsing
- `syntect` - Syntax highlighting
- `unicode-width` - Unicode width calculation
- `tokio` - Async runtime (for file watching)

Enable the `markdown` feature in your `Cargo.toml`:

```toml
[dependencies]
ratatui-toolkit = { version = "0.1", features = ["markdown"] }
```

## Examples

Run the markdown demo:

```bash
cargo run --example markdown_demo --features markdown
```

## License

MIT License - see the parent workspace for details.
