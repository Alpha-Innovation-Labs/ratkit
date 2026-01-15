# ratatui-toolkit

[![Crates.io](https://img.shields.io/crates/v/ratatui-toolkit.svg)](https://crates.io/crates/ratatui-toolkit)
[![Documentation](https://docs.rs/ratatui-toolkit/badge.svg)](https://docs.rs/ratatui-toolkit)
[![License](https://img.shields.io/crates/l/ratatui-toolkit.svg)](LICENSE-MIT)
[![CI](https://github.com/alpha-innovation-labs/ratatui-toolkit/workflows/CI/badge.svg)](https://github.com/alpha-innovation-labs/ratatui-toolkit/actions)

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
| **AlacTerm** | Alacritty-based embedded terminal |
| **VT100Term** | VT100 terminal emulator with scrollback |
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
| `terminal` | ❌ | Terminal emulators (AlacTerm, VT100) |
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
let text = render_markdown("# Hello\n\nThis is **bold** and *italic*");
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
use ratatui_toolkit::{render_markdown, render_markdown_with_style, MarkdownStyle};

// Simple rendering
let text = render_markdown("# Title\n\nParagraph with **bold** text.");

// Custom styling
let style = MarkdownStyle::default()
    .heading_color(Color::Cyan)
    .code_bg(Color::DarkGray);
let text = render_markdown_with_style(markdown_content, &style);

// Render to frame
frame.render_widget(Paragraph::new(text), area);
```

## Architecture

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
