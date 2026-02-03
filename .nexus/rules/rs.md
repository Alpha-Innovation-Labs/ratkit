# Rust Project Structure Rules

This file defines the preferred Rust project organization for this repo.

## Core Principles

1. Organize by domain and feature, not by type.
2. Keep orchestration glue in `app/`.
3. Split always-on logic into `core/` and optional capabilities into `features/`.
4. Each subdomain under `core/` and each feature under `features/` must be its own folder with clear boundaries.
5. Only add feature files that the feature actually needs (`state.rs`, `render.rs`, `ui.rs`, `helpers.rs`).
6. Avoid type-based folders like `constructors/`, `methods/`, `traits/`, or `enums/`.

## Top-Level Layout (Typical)

```
src/
├── lib.rs
├── main.rs
├── app/                 # orchestration, wiring, lifecycle
├── core/                # always-on domain + engine
├── features/            # optional capabilities
├── adapters/            # external boundaries (db, http, fs, cli)
├── services/            # infra helpers (watchers, git, timers)
├── config/              # config structs + loaders
└── utils/               # tiny pure helpers only
```

## Top-Level Layout (TUI)

```
src/
├── main.rs
├── app/                 # orchestration, event loop, wiring
├── widgets/             # widget modules (primary organization)
├── screens/             # view composition
├── adapters/            # external boundaries (fs, git, clipboard)
├── services/            # infra helpers (file watching, timers)
├── config/              # config structs + loaders
└── utils/               # tiny pure helpers only
```

## Folder Semantics

- `app/`: glue layer. Composition, wiring, lifecycle, and startup/teardown.
  - Web: `router.rs`, `lifecycle.rs`
  - TUI: `event_loop.rs`, `layout_wiring.rs`
- `core/`: required engine logic that defines the domain (non-TUI).
  - Web: `core/model/`, `core/engine/`
- `features/`: optional or additive capabilities (non-TUI). Each feature is a folder.
  - Web: `features/auth/`, `features/settings/`
  - TUI: `features/toc/`, `features/scrollbar`
- `widgets/`: primary TUI organization unit. Each widget is a bounded folder.
  - TUI: `widgets/markdown_widget/`, `widgets/command_palette/`
- `screens/`: TUI view composition and layout wiring.
  - TUI: `screens/dashboard.rs`, `screens/settings.rs`
- `adapters/`: outbound/inbound boundaries to external systems (storage is a kind of adapter).
  - Web: `adapters/http.rs`, `adapters/storage.rs`
  - TUI: `adapters/fs.rs`, `adapters/git.rs`
- `services/`: internal infrastructure helpers (file watching, git status, timers).
  - Web: `services/timer.rs`, `services/clock.rs`
  - TUI: `services/file_watcher.rs`, `services/git_status.rs`
- `config/`: config structs + loaders.
  - Web: `config/app_config.rs`
  - TUI: `config/tui_config.rs`
- `utils/`: tiny pure helpers only. No IO, no shared state.
  - Web: `utils/format.rs`
  - TUI: `utils/geometry.rs`

## Core Structure (Subdomain Boundaries)

Each core subdomain is its own folder. Keep code flat inside unless it grows.

```
src/core/
├── mod.rs
├── model/
├── engine/
├── parser/
├── render/
├── source/
├── events/
└── types/
```

## Feature Structure (Feature Boundaries)

Each feature is a folder. Add `state.rs`, `render.rs`, `ui.rs`, or `helpers.rs` only when the feature needs them.

```
src/features/
├── mod.rs
├── toc/
│   ├── mod.rs
│   ├── state.rs
│   ├── render.rs
│   └── helpers.rs
├── selection/
│   ├── mod.rs
│   ├── state.rs
│   └── helpers.rs
└── theme/
    ├── mod.rs
    ├── palette.rs
    ├── style.rs
    └── load.rs
```

## File Naming

1. Use snake_case filenames.
2. Filename matches the primary type or concern.
3. One logical concern per file.

## Example Structures

### Example: CLI App

```
src/
├── main.rs
├── app/
│   ├── mod.rs
│   ├── bootstrap.rs
│   └── wiring.rs
├── core/
│   ├── mod.rs
│   ├── model/
│   ├── parser/
│   └── engine/
├── features/
│   ├── mod.rs
│   ├── search/
│   │   ├── mod.rs
│   │   └── state.rs
│   └── export/
│       ├── mod.rs
│       └── helpers.rs
├── adapters/
│   ├── mod.rs
│   ├── cli.rs
│   └── fs.rs
├── services/
│   ├── mod.rs
│   └── file_watcher.rs
└── config/
    ├── mod.rs
    └── settings.rs
```

### Example: Web App (Dioxus)

```
src/
├── main.rs
├── app/
│   ├── mod.rs
│   ├── router.rs
│   └── wiring.rs
├── core/
│   ├── mod.rs
│   ├── model/
│   ├── engine/
│   └── events/
├── features/
│   ├── mod.rs
│   ├── auth/
│   │   ├── mod.rs
│   │   ├── state.rs
│   │   └── ui.rs
│   ├── dashboard/
│   │   ├── mod.rs
│   │   └── ui.rs
│   └── settings/
│       ├── mod.rs
│       └── ui.rs
├── adapters/
│   ├── mod.rs
│   └── http.rs
└── services/
    ├── mod.rs
    └── storage.rs
```

### Example: TUI App (ratatui)

```
src/
├── main.rs
├── app/
│   ├── mod.rs
│   ├── lifecycle.rs
│   └── wiring.rs
├── screens/
│   ├── mod.rs
│   ├── dashboard.rs
│   └── settings.rs
├── widgets/
│   ├── mod.rs
│   ├── markdown_widget/
│   │   ├── mod.rs
│   │   ├── widget/
│   │   ├── state/
│   │   ├── foundation/
│   │   └── extensions/
│   └── command_palette/
│       ├── mod.rs
│       ├── state.rs
│       └── render.rs
├── adapters/
│   ├── mod.rs
│   └── fs.rs
└── services/
    ├── mod.rs
    └── file_watcher.rs
```
