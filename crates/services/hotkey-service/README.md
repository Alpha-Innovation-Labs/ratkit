# ratkit-hotkey-service

Hotkey registration and management service for ratkit.

## Features

- Centralized hotkey registration and management
- Context-scoped hotkeys (Global, Modal, Tab, Custom)
- Priority-based conflict resolution
- Automatic help text generation
- Trait-based hotkey handling

## Usage

```rust
use ratkit_hotkey_service::{Hotkey, HotkeyRegistry, HotkeyScope};

let mut registry = HotkeyRegistry::new();

// Register a global hotkey
registry.register(Hotkey::new("q", "Quit application")
    .scope(HotkeyScope::Global));

// Register a tab-specific hotkey
registry.register(Hotkey::new("j", "Move down")
    .scope(HotkeyScope::Tab("Markdown")));

// Get all hotkeys for display
let hotkeys = registry.get_hotkeys();
for hotkey in hotkeys {
    println!("{} - {}", hotkey.key, hotkey.description);
}
```

## Dependencies

- `crossterm` - Terminal and keyboard event handling
