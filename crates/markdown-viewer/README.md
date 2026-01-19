# markdown-viewer

A full-featured markdown viewer application component for ratatui-toolkit.

## Features

- **File Tree Navigation**: Browse and select markdown files with a tree view
- **Markdown Rendering**: Display markdown content with syntax highlighting
- **Resizable Split Panes**: Drag to resize file tree and content panels
- **State Persistence**: Remember last open file, split ratio, and visibility preferences
- **Keyboard Shortcuts**: Vim-style navigation and standard keybindings

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
markdown-viewer = "0.1"
```

## Quick Start

```rust,ignore
use ratatui::prelude::*;
use ratatui::crossterm::{event::{Event, KeyCode, KeyEventKind}};
use markdown_viewer::prelude::*;

fn main() -> anyhow::Result<()> {
    // Create state
    let mut state = ViewerState::new("/path/to/markdown/files");

    // Create viewer
    let mut viewer = Viewer::new(&mut state);

    // Load a file
    viewer.load_file(Path::new("readme.md"))?;

    // Handle events
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('q') => break,
            _ => {
                let _ = viewer.handle_key_event(key);
            }
        }
    }

    // Save state
    viewer.save_state()?;

    Ok(())
}
```

## Configuration

```rust,ignore
let config = ViewerConfig::new()
    .with_split_ratio(0.3)
    .with_min_pane_width(20)
    .with_file_tree_visible(true);

let viewer = Viewer::with_config(&mut state, config);
```

## Keyboard Shortcuts

| Key | Action |
|-----|---------|
| `[` | Toggle file tree |
| `h/l` | Switch focus (file tree ↔ markdown) |
| `j/k` | Navigate up/down |
| `Enter` | Select file |
| `/` | Enter filter mode |
| `q` | Quit |

## Architecture

- `Viewer`: Main orchestrator managing layout and routing events
- `ResizablePanes`: Split pane calculation and mouse drag resize
- `MarkdownPanel`: Wrapper around `ratatui_toolkit::render_markdown`
- `ViewerState`: Unified state with persistence support
- `Persistence`: Saves/loads state to `~/.config/markdown-viewer/`

## License

MIT
