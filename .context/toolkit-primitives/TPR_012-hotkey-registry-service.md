---
context_id: TPR_012
title: Hotkey Registry Service
project: toolkit-primitives
created: "2026-01-21"
---

# TPR_012: Hotkey Registry Service

## Desired Outcome

A centralized hotkey management system for `ratatui-toolkit` that provides:

1. **HotkeyRegistry** - A singleton service for registering, querying, and dispatching hotkeys
2. **HotkeyScope** - Enum supporting `Global`, `Modal(String)`, `Tab(String)`, and `Custom(String)` scopes
3. **HasHotkeys trait** - Widgets can declare their hotkeys with scope and descriptions
4. **HotkeyHandler trait** - Widgets can respond to hotkey actions via trait implementation
5. **HotkeyFooter::from_registry()** - Constructor that auto-generates footer from registered hotkeys
6. **Showcase integration** - `just dev` uses the registry for hotkey display and event dispatch

The system enables single source of truth for all hotkeys, context-aware hotkey display, automatic help text generation, and clean separation between hotkey metadata and event handling.

## Reference

### Module Structure

```
src/services/hotkey/
├── mod.rs                    # Module root + re-exports
├── hotkey.rs                 # Hotkey struct
├── hotkey_scope.rs           # HotkeyScope enum
├── traits/
│   ├── mod.rs
│   ├── has_hotkeys.rs        # HasHotkeys trait
│   └── hotkey_handler.rs     # HotkeyHandler trait
└── hotkey_registry/
    ├── mod.rs
    ├── constructors/
    │   └── mod.rs
    ├── methods/
    │   ├── mod.rs
    │   ├── register.rs
    │   ├── lookup.rs
    │   ├── get_hotkeys.rs
    │   └── get_by_scope.rs
    └── traits/
        └── default.rs
```

### Key Types

| Type | Purpose |
|------|---------|
| `Hotkey` | key, description, scope, priority |
| `HotkeyScope` | Global, Modal, Tab, Custom |
| `HotkeyRegistry` | Singleton registry service |
| `HotkeyItem` | Existing footer display type |

### Trait Hierarchy

```
HasHotkeys        → Widget declares its hotkeys
       │
       ▼
HotkeyHandler     → Widget handles hotkey actions
```

### API Pattern

```
HotkeyRegistry::global()     → Access singleton
  .register(hotkey)          → Add hotkey
  .lookup(key_code, scope)   → Find by key
  .get_by_scope(scope)       → Filter by context
```

## Next Actions

| Description | Test |
|-------------|------|
| Create `TPR_012-hotkey-registry-service.md` context file | `context_file_created` |
| Delete experimental code in `src/services/hotkey/` | `experimental_code_deleted` |
| Create `hotkey_scope.rs` with HotkeyScope enum (String variants) | `hotkey_scope_created` |
| Create `hotkey.rs` with Hotkey struct and builder methods | `hotkey_struct_created` |
| Create `hotkey_registry/mod.rs` with HotkeyRegistry singleton | `registry_singleton_created` |
| Add registration methods: `register()`, `register_global()` | `registration_methods_added` |
| Add lookup methods: `lookup()`, `get_by_scope()`, `get_global()` | `lookup_methods_added` |
| Create `traits/has_hotkeys.rs` with HasHotkeys trait | `has_hotkeys_trait_created` |
| Create `traits/hotkey_handler.rs` with HotkeyHandler trait | `hotkey_handler_trait_created` |
| Add `From<&Hotkey> for HotkeyItem` conversion | `conversion_implemented` |
| Add `HotkeyFooter::from_registry()` constructor | `footer_constructor_added` |
| Update `lib.rs` to re-export hotkey service | `lib_re_exports_added` |
| Update `services/mod.rs` to export hotkey module | `services_module_updated` |
| Integrate into showcase: register global hotkeys | `global_hotkeys_integrated` |
| Integrate into showcase: implement HasHotkeys for demo widgets | `widget_traits_implemented` |
| Update showcase footer to use `from_registry()` | `showcase_footer_updated` |
| Verify `just dev` shows hotkeys correctly | `showcase_hotkeys_verified` |
| Verify hotkey dispatch works in `just dev` | `hotkey_dispatch_verified` |
