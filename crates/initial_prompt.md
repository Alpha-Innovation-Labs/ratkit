Markdown Viewer Crate Specification
Overview
Create a full-featured markdown viewer as an application-level component that uses ratatui_toolkit::MarkdownWidget for content rendering. The viewer provides a ready-to-use interface with a file tree and resizable split panes.
Relationship: 
- ratatui_toolkit (main crate): Provides MarkdownWidget (library component)
- markdown_viewer (new crate): Provides Viewer (application component)
Core Features
File Tree (Left Panel)
- Show only .md files (filter FileSystemTree results)
- Show hidden .md files
- Sort alphabetically (directories first, then files) - already supported by FileSystemTree
- Expand/collapse with h/l keys - already supported by TreeKeyBindings defaults
- Navigate with j/k keys - already supported by TreeKeyBindings defaults
- Filter mode when focused (type to search) - already supported by FileSystemTree
- File icons for markdown files - already supported by FileSystemTree
Markdown Panel (Right Panel)
- Uses MarkdownWidget for rendering
- Shows TOC, statusline, scrollbar by default
- Filter mode (/ key) for searching content - integrate with widget's filter mode
Resizable Panes
- Split view with draggable divider
- Mouse drag to resize
- File tree on left, markdown content on right
State Persistence
- Remember last open file
- Remember pane resize ratio
- Remember file tree expanded directories
- Remember file tree scroll position
Architecture Components
Viewer (Main Orchestrator)
Manages layout, routes events, coordinates sub-components
Fields:
- config: ViewerConfig
- state: &'a mut ViewerState
- layout: ViewerLayout (calculated on each render)
Methods:
- new(state, config)
- with_theme(theme)
- handle_key_event(KeyEvent) -> ViewerEvent
- handle_mouse_event(MouseEvent) -> ViewerEvent
- render(area, buf)
ResizablePanes
Handles split pane calculation and mouse drag resize (similar to code_diff/resize.rs pattern)
Fields:
- split_ratio: f32 (0.0-1.0, default 0.25)
- min_width: u16 (default 20)
- max_width: Option<u16>
- is_dragging: bool
- drag_start_x: u16
- drag_start_ratio: f32
Methods:
- new() / with_split_ratio(ratio)
- handle_mouse(MouseEvent, area) -> bool
- calculate_areas(area) -> (Rect, Rect, Rect) (left, right, divider)
- is_dragging() -> bool
FileSystemTree (Existing Component)
Use existing ratatui_toolkit::FileSystemTree with configuration:
Configuration:
- show_hidden: true (to show hidden .md files)
- use_dark_theme: based on AppTheme
- File filtering: Add wrapper to filter only .md files
Methods to use:
- FileSystemTree::new(root_path)
- with_theme(theme)
- Navigation: j/k, h/l, Enter, g/G (default keybindings)
- Filter mode: enter_filter_mode(), handle_filter_key(), clear_filter()
MarkdownPanel
Wrapper around MarkdownWidget with file loading
Fields:
- widget: MarkdownWidget<'a>
- file_path: Option<PathBuf>
- show_toc: bool (default true)
- show_statusline: bool (default true)
- show_scrollbar: bool (default true)
Methods:
- new(content, state)
- load_file(path) -> Result<()>
- with_toc(bool), with_statusline(bool), with_scrollbar(bool)
- handle_key_event(KeyEvent) -> MarkdownEvent
- handle_mouse_event(MouseEvent) -> MarkdownEvent
- render(area, buf)
ViewerState (Unified State)
Bundles all sub-component states
Fields:
pub struct ViewerState {
    pub panes: ResizablePanesState,
    pub file_tree: FileSystemTreeState,
    pub markdown: MarkdownState,
    pub theme: Option<AppTheme>,
    
    // Persistence fields
    pub last_open_file: Option<PathBuf>,
    pub saved_split_ratio: Option<f32>,
}
Methods:
- new(root_path) -> Self
- load_file(&mut self, path: &Path) -> Result<()>
- save_state(&self) -> Result<()>
- load_state(&mut self) -> Result<()>
ViewerEvent
Events emitted to parent application
Variants:
pub enum ViewerEvent {
    FileSelected { path: PathBuf },
    FileLoaded { path: PathBuf },
    PanesResized { ratio: f32 },
    FileTreeHidden { hidden: bool },
    
    // Forwarded events
    Markdown(MarkdownEvent),
    FileTree(FileTreeEvent),
    None,
}
ViewerConfig
Builder pattern for configuration
Fields:
pub struct ViewerConfig {
    pub initial_split_ratio: f32,        // 0.25 (25% for file tree)
    pub min_pane_width: u16,             // 20
    pub max_pane_width: Option<u16>,     // None
    pub show_file_tree: bool,             // true
    pub hide_file_tree_key: KeyCode,      // '['
    pub file_tree_config: FileSystemTreeConfig,
    pub markdown_config: MarkdownPanelConfig,
}
Defaults:
- File tree on left
- 25% width for file tree
- Min width 20 chars
- [ key to toggle file tree visibility
Event Flow
Input (Key/Mouse)
    ↓
Viewer::handle_*_event()
    ↓
Route to sub-component:
- If file tree focused → FileSystemTree
- If markdown focused → MarkdownPanel
- If divider dragged → ResizablePanes
    ↓
Sub-component returns event
    ↓
Viewer returns ViewerEvent
    ↓
Parent application handles event (load file, show toast, etc.)
    ↓
State persisted to ViewerState
State Sync Pattern
Similar to MarkdownWidget:
let (event, sync_state) = {
    let mut viewer = Viewer::new(&mut state, config);
    let event = viewer.handle_key_event(key);
    let sync_state = viewer.get_state_sync();
    (event, sync_state)
};
sync_state.apply_to(&mut state);
Workspace Integration
File Structure
ratatui-toolkit/
├── Cargo.toml                           # Main crate
├── src/
│   ├── markdown_widget/                  # Widget (existing)
│   ├── file_system_tree/                 # File tree (existing)
│   └── ...
├── markdown-viewer/                     # NEW: Viewer crate
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── viewer.rs
│   │   ├── resizable_panes.rs
│   │   ├── markdown_panel.rs
│   │   ├── state.rs
│   │   ├── viewer_event.rs
│   │   ├── config.rs
│   │   └── persistence.rs              # For saving/loading state
│   └── examples/
│       └── viewer_demo.rs
└── Cargo.toml                          # Workspace file
Dependencies
For development (workspace):
# markdown-viewer/Cargo.toml
[dependencies]
ratatui-toolkit = { version = "0.1.0", path = ".." }
For publishing:
- Same dependency works (cargo uses version when published, ignores path)
Publishing Workflow
1. Update versions in both Cargo.toml files
2. Publish ratatui-toolkit first
3. Publish markdown-viewer second
4. Users can install either crate independently
Design Principles
- Separation of concerns: Widget is reusable library, viewer is application component
- Modularity: Each component testable/usable independently
- Clean API: Builder pattern for configuration
- Event-driven: Parent app reacts to events
- State persistence: Remember user preferences across sessions
Keyboard Shortcuts
Global (Viewer-level)
- [: Toggle file tree visibility
File Tree (when focused)
- j/Down: Next item
- k/Up: Previous item
- h/Left: Collapse directory
- l/Right: Expand directory
- Enter: Select file / toggle expand
- g: Go to top
- G: Go to bottom
- / (or type): Enter filter mode
Markdown Panel (when focused)
- j/Down: Scroll down
- k/Up: Scroll up
- gg: Go to top
- G: Go to bottom
- /: Enter filter mode
- y: Copy selection
- Ctrl+Shift+C: Copy selection
- Esc: Exit filter/selection mode
Focus Switching
- Click on panel to focus
- Arrow keys to switch focus (when at edge)
Notes
FileSystemTree Usage
- Already supports: filtering, sorting (dirs first, then alphabetical), navigation keys, expand/collapse
- Need to add: Filter to show only .md files (post-processing)
- Use FileSystemTreeConfig with show_hidden: true
MarkdownPanel Integration
- Direct wrapper around MarkdownWidget
- Handle file loading errors gracefully
- Sync state back to ViewerState
Persistence
- Save to config file (e.g., ~/.config/markdown-viewer/state.toml)
- Persist: last_open_file, split_ratio, expanded directories, scroll position
- Load on startup to restore session
File Tree Filtering (.md only)
// After loading directory, filter nodes
fn filter_md_only(nodes: Vec<TreeNode<FileSystemEntry>>) -> Vec<TreeNode<FileSystemEntry>> {
    // Keep directories and .md files only
    // Need to recursively filter children
}
---
