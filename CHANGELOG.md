# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.11] - 2026-01-16

### Added

- **Per-Pane Focus Mode** - `requires_focus_mode()` method on `PaneContent` trait
  - Allows hybrid auto-focus behavior where some panes auto-passthrough and others require explicit focus
  - Customizable icons via `focus_icon()` and `selected_icon()` methods
  - Visual indicator shows "Press Enter" for panes requiring explicit focus

### Changed

- **Module Architecture** - All components restructured to follow consistent organization
  - Type definitions in `mod.rs` (struct/enum only, no impl blocks)
  - Constructors in `constructors/` directory (`new`, `with_*`, builders)
  - Instance methods in `methods/` directory
  - Trait implementations in `traits/` directory
  - Components restructured: Button, Dialog, FileSystemTree, FuzzyFinder, HotkeyFooter, MenuBar, Pane, ResizableSplit, StatusBar, StatusLineStacked

- **Markdown Renderer** - Complete rewrite with new `styled_line` system
  - Added syntax highlighting support for code blocks
  - Added theme configuration via `MarkdownStyle`
  - Added `MarkdownScrollManager` for scroll state management
  - Added `MarkdownWidget` as a stateful widget
  - Improved code block rendering with borders
  - Added expandable/collapsible section support

- **Toast Module** - Refactored into `constructors/` and `methods/` for better organization

### Removed

- Removed deprecated `vt100_term` module (replaced by `termtui`)
- Removed unused `alac_term` module
- Removed unused `ai_chat` module

## [0.1.0] - 2026-XX-XX

### Added

- Initial release of ratatui-toolkit
- **Core Components**
  - `Button` - Clickable button with hover states and click detection
  - `Dialog` - Modal dialogs with Info/Success/Warning/Error/Confirm types
  - `Pane` - Styled panel component with padding and optional footer
  - `Toast` - Toast notifications with auto-expiry and severity levels
  - `ToastManager` - Manager for multiple simultaneous toasts

- **Layout Components**
  - `ResizableSplit` - Draggable split panels (vertical/horizontal)
  - `MasterLayout` - Application shell with tabs, panes, and vim-like navigation
  - `NavigationBar` - Tab navigation component
  - `Tab` - Tab container for grouped panes

- **Widget Components**
  - `TreeView` - Generic tree widget with expand/collapse and navigation
  - `FileSystemTree` - File browser with devicons and sorting
  - `FuzzyFinder` - PTY-based fuzzy search popup
  - `MenuBar` - Horizontal menu bar with icon support
  - `StatusBar` - Customizable status bar
  - `StatusLineStacked` - Neovim-style powerline status
  - `HotkeyFooter` - Keyboard shortcut display footer

- **Markdown Rendering**
  - Full markdown to ratatui `Text` conversion
  - Support for headings, code blocks, lists, quotes
  - Customizable styling with `MarkdownStyle`

- **Terminal Emulation**
  - `TermTui` - Terminal emulator with mprocs-style architecture
  - Copy mode support with text selection
  - OSC 52 clipboard integration

- **Feature Flags**
  - `default` - Core components (markdown, tree, dialog, toast, split, menu, statusbar, hotkey)
  - `full` - All features including terminal and fuzzy finder
  - `terminal` - Terminal emulation components
  - `fuzzy` - Fuzzy finder component
  - `file-tree` - File system tree with devicons
  - `master-layout` - Full application layout framework

### Documentation

- Comprehensive crate-level documentation
- Module-level documentation with examples
- 10 runnable examples demonstrating each major component
- README with quick start guide and feature comparison

[Unreleased]: https://github.com/alpha-innovation-labs/ratatui-toolkit/compare/v0.1.11...HEAD
[0.1.11]: https://github.com/alpha-innovation-labs/ratatui-toolkit/compare/v0.1.0...v0.1.11
[0.1.0]: https://github.com/alpha-innovation-labs/ratatui-toolkit/releases/tag/v0.1.0
