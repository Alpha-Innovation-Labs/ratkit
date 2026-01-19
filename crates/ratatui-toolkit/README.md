# ratatui-toolkit

[![Crates.io](https://img.shields.io/crates/v/ratatui-toolkit.svg)](https://crates.io/crates/ratatui-toolkit)
[![Documentation](https://img.shields.io/docsrs/ratatui-toolkit)](https://docs.rs/ratatui-toolkit)
[![License](https://img.shields.io/crates/l/ratatui-toolkit.svg)](LICENSE-MIT)

![ratatui-toolkit Demo](demo/ratatui-toolkit-demo.gif)

A comprehensive collection of reusable TUI components for [ratatui](https://ratatui.rs/), the Rust terminal UI library.

## Features

| Component | Description |
|-----------|-------------|
| **ResizableSplit** | Draggable split panels (vertical/horizontal) with mouse support |
| **TreeView** | Generic tree widget with expand/collapse, navigation, and selection |
| **FileSystemTree** | File browser with devicons and sorting |
| **MarkdownRenderer** | Render markdown to styled ratatui `Text` |
| **Toast** | Toast notifications with auto-expiry and severity levels |
| **Dialog** | Modal dialogs (Info/Success/Warning/Error/Confirm) |
| **Button** | Clickable buttons with hover states |
| **ClickableScrollbar** | Scrollbar with drag and click support |
| **HotkeyFooter** | Keyboard shortcut display footer |
| **HotkeyModal** | Help overlay for key bindings |
| **MenuBar** | Horizontal menu bar with icons |
| **StatusBar** | Customizable status bar |
| **StatusLineStacked** | Neovim-style powerline status |
| **MasterLayout** | Application shell with tabs, panes, vim-like navigation |
| **TermTui** | Terminal emulator with mprocs-style copy mode |
| **FuzzyFinder** | PTY-based fuzzy search popup |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ratatui-toolkit = "0.1"
```

Or with specific features:

```toml
[dependencies]
ratatui-toolkit = { version = "0.1", default-features = false, features = ["tree", "split", "toast"] }
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `markdown` | ✅ | Markdown rendering to ratatui Text |
| `tree` | ✅ | Generic tree view widget |
| `dialog` | ✅ | Modal dialog components |
| `toast` | ✅ | Toast notification system |
| `split` | ✅ | Resizable split panels |
| `menu` | ✅ | Menu bar component |
| `statusbar` | ✅ | Status bar components |
| `hotkey` | ✅ | Hotkey footer and modal |
| `terminal` | ✅ | Terminal emulator (TermTui) |
| `fuzzy` | ❌ | Fuzzy finder component |
| `master-layout` | ❌ | Full application layout framework |
| `file-tree` | ❌ | File system tree with devicons |
| `full` | ❌ | Enable all features |

## Quick Start

```rust
use ratatui::prelude::*;
use ratatui_toolkit::prelude::*;

// Create a resizable split
let split = ResizableSplit::new(SplitDirection::Vertical)
    .ratio(0.3)
    .min_ratio(0.1)
    .max_ratio(0.9);

// Create a toast notification
let toast = Toast::new("File saved successfully!")
    .level(ToastLevel::Success)
    .duration(Duration::from_secs(3));

// Render markdown
let text = render_markdown("# Hello\n\nThis is **bold** and *italic*", None);
```

## Examples

Run examples with:

```bash
cargo run --example resizable_split_demo
cargo run --example tree_view_demo --features tree
cargo run --example toast_manager_demo --features toast
cargo run --example markdown_demo --features markdown
cargo run --example full_app_demo --features full
```

### Resizable Split

```rust
use ratatui_toolkit::{ResizableSplit, SplitDirection};

let mut split = ResizableSplit::new(SplitDirection::Horizontal)
    .ratio(0.5)
    .min_ratio(0.2)
    .max_ratio(0.8);

// Handle mouse events
if let Some(event) = split.handle_mouse(mouse_event, area) {
    // Split ratio was updated by drag
}

// Render
let [left, right] = split.split(area);
frame.render_widget(left_content, left);
frame.render_widget(right_content, right);
```

### Tree View

```rust
use ratatui_toolkit::{TreeView, TreeViewState, TreeNode};

// Build tree structure
let root = TreeNode::new("root", "Root")
    .with_children(vec![
        TreeNode::new("child1", "Child 1"),
        TreeNode::new("child2", "Child 2")
            .with_children(vec![
                TreeNode::new("grandchild", "Grandchild"),
            ]),
    ]);

let mut state = TreeViewState::new(root);
let tree = TreeView::new(&state);

// Handle navigation
state.handle_key(KeyCode::Down); // Move down
state.handle_key(KeyCode::Right); // Expand
state.handle_key(KeyCode::Left); // Collapse
```

### Toast Notifications

```rust
use ratatui_toolkit::{ToastManager, Toast, ToastLevel, render_toasts};
use std::time::Duration;

let mut manager = ToastManager::new()
    .max_toasts(5)
    .default_duration(Duration::from_secs(3));

// Add toasts
manager.push(Toast::info("Information message"));
manager.push(Toast::success("Operation completed!"));
manager.push(Toast::warning("Please check your input"));
manager.push(Toast::error("Something went wrong"));

// Update and render
manager.tick(); // Remove expired toasts
render_toasts(frame, area, &manager);
```

### Markdown Rendering

```rust
use ratatui_toolkit::{
    render_markdown, render_markdown_with_style, render_markdown_interactive,
    MarkdownStyle, MarkdownWidget, MarkdownScrollManager,
};

// Simple rendering
let text = render_markdown("# Title\n\nParagraph with **bold** text.", None);

// Custom styling with syntax highlighting
let style = MarkdownStyle::default();
let text = render_markdown_with_style(markdown_content, &style);

// Interactive widget with scroll management
let mut scroll_manager = MarkdownScrollManager::new();
let widget = MarkdownWidget::new(markdown_content)
    .with_style(style);
frame.render_stateful_widget(widget, area, &mut scroll_manager);
```

**Markdown Features:**
- Syntax highlighting for code blocks
- Configurable themes
- Scroll management for long documents
- Expandable/collapsible sections

### Terminal Emulator (TermTui)

```rust
use ratatui_toolkit::{TermTui, TermTuiKeyBindings};

// Spawn a terminal with default shell
let shell = std::env::var("SHELL").unwrap_or("/bin/sh".into());
let mut term = TermTui::spawn_with_command("Terminal", &shell, &[])?;

// Handle input
term.handle_key(key_event);
term.handle_mouse(mouse_event, terminal_area);

// Render
term.render(frame, area);
```

**Default Keybindings:**
- `Ctrl+X` - Enter copy mode
- `Ctrl+Shift+C` - Copy selection

**Copy Mode:**
- `h/j/k/l` or arrows - Navigate
- `v` or `Space` - Start selection
- `y` or `Enter` - Copy and exit
- `w/b` - Word navigation
- `0/$` - Line start/end
- `g/G` - Top/bottom
- `Esc` or `q` - Exit copy mode

## Customizable Keybindings

All interactive components expose their keybindings through configuration structs, allowing full customization:

### TermTui

```rust
use ratatui_toolkit::{TermTui, TermTuiKeyBindings};
use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};

// Create custom keybindings
let mut bindings = TermTuiKeyBindings::default();
bindings.enter_copy_mode = KeyEvent::new(KeyCode::Char('b'), KeyModifiers::CONTROL);

let term = TermTui::spawn_with_command("Terminal", "bash", &[])?
    .with_keybindings(bindings);
```

### TreeView

```rust
use ratatui_toolkit::{TreeNavigator, TreeKeyBindings};
use crossterm::event::KeyCode;

let bindings = TreeKeyBindings::new()
    .with_next(vec![KeyCode::Char('n'), KeyCode::Down])
    .with_previous(vec![KeyCode::Char('p'), KeyCode::Up])
    .with_expand(vec![KeyCode::Char('e'), KeyCode::Right])
    .with_collapse(vec![KeyCode::Char('c'), KeyCode::Left]);

let navigator = TreeNavigator::with_keybindings(bindings);
```

### MasterLayout

```rust
use ratatui_toolkit::{MasterLayout, MasterLayoutKeyBindings};
use crossterm::event::KeyCode;

let mut bindings = MasterLayoutKeyBindings::default();
bindings.navigate_left = KeyCode::Char('a');
bindings.navigate_right = KeyCode::Char('d');

let layout = MasterLayout::new()
    .with_keybindings(bindings);
```

### Available Keybinding Configs

| Component | Config Struct | Builder Method |
|-----------|--------------|----------------|
| TermTui | `TermTuiKeyBindings` | `.with_keybindings()` |
| TreeView | `TreeKeyBindings` | `TreeNavigator::with_keybindings()` |
| MasterLayout | `MasterLayoutKeyBindings` | `.with_keybindings()` |

## Architecture

### Module Structure

All components follow a consistent module organization pattern for maintainability:

```
src/component_name/
├── mod.rs              # Type definition only (struct/enum)
├── constructors/       # Constructors and builders (new, with_*, builder)
│   ├── mod.rs
│   ├── new.rs
│   └── with_*.rs
├── methods/            # Instance methods (&self, &mut self)
│   ├── mod.rs
│   └── method_name.rs
└── traits/             # Trait implementations (Widget, Default, etc.)
    ├── mod.rs
    └── trait_name.rs
```

This structure ensures:
- **Single responsibility**: Each file contains one impl block
- **Easy navigation**: Find constructors in `constructors/`, methods in `methods/`
- **Consistent patterns**: All components organized identically

### Component Pattern

All components follow ratatui's `Widget` and `StatefulWidget` patterns:

```rust
// Stateless widget
impl Widget for MyComponent {
    fn render(self, area: Rect, buf: &mut Buffer) { ... }
}

// Stateful widget
impl StatefulWidget for MyComponent {
    type State = MyComponentState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) { ... }
}
```

### Mouse Support

Components support mouse interaction through a consistent API:

```rust
// Handle mouse events
if let Some(action) = component.handle_mouse(mouse_event, area) {
    match action {
        ComponentAction::Clicked => { ... }
        ComponentAction::Dragged { delta } => { ... }
    }
}
```

## Comparison with Alternatives

| Crate | Focus | Components |
|-------|-------|------------|
| **ratatui-toolkit** | Comprehensive component library | 17+ components |
| `tui-textarea` | Text editing | Textarea only |
| `tui-tree-widget` | Tree views | Tree only |
| `ratatui-image` | Image rendering | Images only |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

Licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT).
