// Re-export primitives for backward compatibility
#[cfg(feature = "button")]
pub use crate::primitives::button::*;

#[cfg(feature = "dialog")]
pub use crate::primitives::dialog::*;

#[cfg(feature = "menu-bar")]
pub use crate::primitives::menu_bar::*;

#[cfg(feature = "pane")]
pub use crate::primitives::pane::*;

#[cfg(feature = "resizable-grid")]
pub use crate::primitives::resizable_grid::*;

#[cfg(feature = "scroll")]
pub use crate::primitives::scroll::*;

#[cfg(feature = "statusline")]
pub use crate::primitives::statusline::*;

#[cfg(feature = "termtui")]
pub use crate::primitives::termtui::*;

#[cfg(feature = "toast")]
pub use crate::primitives::toast::*;

#[cfg(feature = "tree-view")]
pub use crate::primitives::tree_view::*;

#[cfg(feature = "widget-event")]
pub use crate::primitives::widget_event::*;

// Re-export widgets for backward compatibility
#[cfg(feature = "ai-chat")]
pub use crate::widgets::ai_chat::*;

#[cfg(feature = "code-diff")]
pub use crate::widgets::code_diff::*;

#[cfg(feature = "file-system-tree")]
pub use crate::widgets::file_system_tree::*;

#[cfg(feature = "hotkey-footer")]
pub use crate::widgets::hotkey_footer::*;

#[cfg(feature = "markdown-preview")]
pub use crate::widgets::markdown_preview::*;

#[cfg(feature = "theme-picker")]
pub use crate::widgets::theme_picker::*;

// Widget modules
#[cfg(feature = "ai-chat")]
pub mod ai_chat;

#[cfg(feature = "code-diff")]
pub mod code_diff;

#[cfg(feature = "file-system-tree")]
pub mod file_system_tree;

#[cfg(feature = "hotkey-footer")]
pub mod hotkey_footer;

#[cfg(feature = "markdown-preview")]
pub mod markdown_preview;

#[cfg(feature = "theme-picker")]
pub mod theme_picker;
