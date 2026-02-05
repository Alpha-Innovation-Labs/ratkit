# ratkit

[![Crates.io](https://img.shields.io/crates/v/ratkit.svg)](https://crates.io/crates/ratkit)
[![Documentation](https://img.shields.io/docsrs/ratkit)](https://docs.rs/ratkit)
[![License](https://img.shields.io/crates/l/ratkit.svg)](LICENSE-MIT)

![ratkit Demo](demo/ratatui-toolkit-demo.gif)

Core runtime and reusable TUI components for [ratatui](https://ratatui.rs/), the Rust terminal UI library.

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
| **HotkeyFooter** | Keyboard shortcut display footer |
| **MenuBar** | Horizontal menu bar with icons |
| **StatusLineStacked** | Neovim-style powerline status |
| **TermTui** | Terminal emulator with mprocs-style copy mode |

## Installation

Add the core runtime to your `Cargo.toml`:

```toml
[dependencies]
ratkit = "0.1"
```

For the full bundle of components:

```toml
[dependencies]
ratkit = { version = "0.1", features = ["all"] }
```

For selected components:

```toml
[dependencies]
ratkit = { version = "0.1", default-features = false, features = ["tree-view", "resizable-grid", "toast"] }
```

## Feature Flags

ratkit ships as a core runtime with optional components. By default only the
core runtime is enabled; opt in to specific components or use the `all` feature
to pull everything.

```toml
ratkit = { version = "0.1", default-features = false, features = ["tree-view", "toast"] }
```

| Feature | Description |
|---------|-------------|
| `default` | Core runtime only (Runner + Layout Manager) |
| `all` | All widgets and services |
| `full` | Alias for `all` |
| `widgets` | All UI widgets |
| `services` | All service components |

| Feature | Component |
|---------|-----------|
| `button` | Button widget |
| `pane` | Pane widget |
| `dialog` | Modal dialog components |
| `toast` | Toast notification system |
| `statusline` | Powerline-style statusline |
| `scroll` | Scrollable content helpers |
| `menu-bar` | Menu bar component |
| `resizable-grid` | Resizable split panels |
| `tree-view` | Generic tree view widget |
| `widget-event` | Widget event helpers |
| `termtui` | Terminal emulator (TermTui) |
| `markdown-preview` | Markdown preview widget |
| `code-diff` | Code diff widget |
| `ai-chat` | AI chat widget |
| `hotkey-footer` | Hotkey footer widget |
| `file-system-tree` | File browser with devicons and sorting |
| `theme-picker` | Theme picker widget |
| `file-watcher` | File watcher service |
| `git-watcher` | Git watcher service |
| `repo-watcher` | Repo watcher service |
| `hotkey-service` | Hotkey service |

## Quick Start

```rust
use ratatui::prelude::*;
use ratkit::{render_markdown, ResizableSplit, SplitDirection, Toast, ToastLevel};
use std::time::Duration;

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
cargo run --example split_demo --features all
cargo run --example tree_view_demo --features tree
cargo run --example toast_manager_demo --features toast
cargo run --example markdown_demo --features markdown
cargo run --example full_app_demo --features all
```

### Resizable Split

```rust
use ratkit::{ResizableSplit, SplitDirection};

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

For a multi-pane layout with drag-resize, see `examples/split_demo.rs`.

### Tree View

```rust
use ratkit::{TreeNode, TreeView, TreeViewState};

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
use ratkit::{render_toasts, Toast, ToastLevel, ToastManager};
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
use ratkit::{
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
use ratkit::{TermTui, TermTuiKeyBindings};

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
use ratkit::{TermTui, TermTuiKeyBindings};
use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};

// Create custom keybindings
let mut bindings = TermTuiKeyBindings::default();
bindings.enter_copy_mode = KeyEvent::new(KeyCode::Char('b'), KeyModifiers::CONTROL);

let term = TermTui::spawn_with_command("Terminal", "bash", &[])?
    .with_keybindings(bindings);
```

### TreeView

```rust
use ratkit::{TreeKeyBindings, TreeNavigator};
use crossterm::event::KeyCode;

let bindings = TreeKeyBindings::new()
    .with_next(vec![KeyCode::Char('n'), KeyCode::Down])
    .with_previous(vec![KeyCode::Char('p'), KeyCode::Up])
    .with_expand(vec![KeyCode::Char('e'), KeyCode::Right])
    .with_collapse(vec![KeyCode::Char('c'), KeyCode::Left]);

let navigator = TreeNavigator::with_keybindings(bindings);
```

### Available Keybinding Configs

| Component | Config Struct | Builder Method |
|-----------|--------------|----------------|
| TermTui | `TermTuiKeyBindings` | `.with_keybindings()` |
| TreeView | `TreeKeyBindings` | `TreeNavigator::with_keybindings()` |

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
| **ratkit** | Comprehensive component library | 17+ components |
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
