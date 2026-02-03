//! Hotkey registration and management service.
//!
//! Provides a centralized system for registering, managing, and querying
//! hotkeys across the application. Supports context-scoped hotkeys,
//! priorities, and automatic help text generation.
//!
//! # Example
//!
//! ```no_run
//! use ratkit_hotkey_service::{Hotkey, HotkeyRegistry, HotkeyScope};
//!
//! let mut registry = HotkeyRegistry::new();
//!
//! // Register a global hotkey
//! registry.register(Hotkey::new("q", "Quit application")
//!     .scope(HotkeyScope::Global));
//!
//! // Register a tab-specific hotkey
//! registry.register(Hotkey::new("j", "Move down")
//!     .scope(HotkeyScope::Tab("Markdown")));
//!
//! // Get all hotkeys for display
//! let hotkeys = registry.get_hotkeys();
//! for hotkey in hotkeys {
//!     println!("{} - {}", hotkey.key, hotkey.description);
//! }
//! ```

pub mod hotkey_item;
pub mod hotkey_registry;
pub mod hotkey_scope;
pub mod traits;

pub use hotkey_item::Hotkey;
pub use hotkey_registry::HotkeyRegistry;
pub use hotkey_scope::HotkeyScope;
pub use traits::HasHotkeys;
pub use traits::HotkeyHandler;
