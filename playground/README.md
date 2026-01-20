# ratatui-interact Demo

A comprehensive demo of the **ratatui-interact** library showcasing interactive widgets.

## Features Demonstrated

### 🎯 Interactive Widgets
- **SplitPane** - Resizable split panes with drag-to-resize
- **Input** - Text input field with cursor management
- **CheckBox** - Toggleable checkbox
- **Select** - Dropdown selection box

### 🎨 Display Widgets
- **Spinner** - Animated loading indicators (Braille frames)
- **Progress** - Progress bar with label

## Installation

```bash
cd playground
cargo build
```

## Running the Demo

### Interactive Mode
```bash
cargo run
```

### Automated Test
```bash
echo "q" | cargo run --quiet
```

## Controls

| Key | Action |
|------|---------|
| `Tab` / `Shift+Tab` | Cycle focus between elements |
| `Space` | Toggle checkbox |
| `Enter` | Select dropdown option |
| `←` / `→` | Resize panes (when divider focused) |
| Mouse Drag | Resize panes by dragging divider |
| `Q` / `Esc` | Quit |

## Keyboard Shortcuts

- **Resize Panes**: Use Left/Right arrow keys when the divider is focused
- **Mouse Support**: Click to focus elements, drag to resize panes
- **Quit**: Press Q or Esc at any time to exit

## Screenshot

The demo features:
- Left pane: Interactive input widgets (text field, checkbox, select dropdown)
- Right pane: Animated display widgets (spinner, progress bar)
- Draggable divider: Resize panes by dragging the center bar

## Library Information

- **Name**: ratatui-interact
- **Version**: 0.2
- **Repository**: https://github.com/Brainwires/ratatui-interact
- **License**: MIT

### Components Used

- `SplitPane` - Layout widget with resizable panes
- `Spinner` - Animated loading indicator
- `Progress` - Progress bar widget
- `Input` - Text input field
- `CheckBox` - Toggleable checkbox
- `Select` - Dropdown selection

### All Available Components

The library also includes many more widgets not shown in this demo:

- **Interactive**: Button, TextArea, ContextMenu, PopupDialog, HotkeyDialog
- **Display**: ParagraphExt, Toast, MarqueeText
- **Navigation**: ListPicker, TreeView, FileExplorer, Accordion, Breadcrumb
- **Layout**: TabView
- **Viewer**: LogViewer, DiffViewer, StepDisplay

See the [full documentation](https://docs.rs/ratatui-interact) for details.
