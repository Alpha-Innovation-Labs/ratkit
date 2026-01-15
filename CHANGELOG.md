# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
  - `ClickableScrollbar` - Scrollbar with mouse interaction support
  - `FuzzyFinder` - PTY-based fuzzy search popup
  - `MenuBar` - Horizontal menu bar with icon support
  - `StatusBar` - Customizable status bar
  - `StatusLineStacked` - Neovim-style powerline status
  - `HotkeyFooter` - Keyboard shortcut display footer
  - `HotkeyModal` - Help overlay for key bindings

- **Markdown Rendering**
  - Full markdown to ratatui `Text` conversion
  - Support for headings, code blocks, lists, quotes
  - Customizable styling with `MarkdownStyle`

- **Terminal Emulation**
  - `AlacTerm` - Alacritty-based embedded terminal
  - `VT100Term` - VT100 terminal emulator with infinite scrollback
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

[Unreleased]: https://github.com/alpha-innovation-labs/ratatui-toolkit/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/alpha-innovation-labs/ratatui-toolkit/releases/tag/v0.1.0
