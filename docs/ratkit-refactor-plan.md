# Ratkit Modular Workspace Refactor Plan

## Executive Summary
Refactor ratatui-toolkit into a ratkit modular workspace of ultra-granular crates with independent versioning. The goal is a clean, maintainable codebase where each primitive, widget, and service is isolated into its own crate, while the ratkit meta-crate provides a unified API surface and authoritative documentation entry point. This plan focuses on the code refactor requirements and the structural changes needed to support a scalable workspace.

## Work Breakdown
### Codebase Inventory and Module Mapping
- Catalog the current primitives, widgets, and services; map each module to its target ratkit-* crate.
- **Granularity**: One crate per widget, one crate per primitive, and one crate per service.
- **Naming Convention**: All crates use the `ratkit-*` prefix (e.g., `ratkit-button`, `ratkit-color`, `ratkit-forms`).
- **Versioning**: Each crate has its own independent version starting at 0.1.0. When a crate is updated, dependent crates must be identified and updated accordingly (dependency management is out of scope for this refactor).
- **Migration Strategy**: Big bang approach. The `ratatui-toolkit` crate will be deprecated and delisted after the refactor.
- Define the authoritative crate list and confirm scope boundaries for each item.
- Dependencies/prereqs: complete module inventory and public API map.
- Risk factors and mitigation: scope drift or mismatched boundaries; mitigate with a published module map and boundary review checks.

### Target Structure

```
crates/
├── ratkit/                          # Meta-crate (re-exports all)
│   └── Cargo.toml with optional deps
│
├── Primitives (ultra-granular)
│   ├── ratkit-button/
│   ├── ratkit-pane/
│   ├── ratkit-dialog/
│   ├── ratkit-toast/
│   ├── ratkit-statusline/
│   ├── ratkit-scroll/
│   ├── ratkit-menu-bar/
│   ├── ratkit-resizable-grid/
│   ├── ratkit-tree-view/
│   ├── ratkit-border-style/
│   ├── ratkit-widget-event/
│   └── ratkit-termtui/
│
├── Widgets
│   ├── ratkit-markdown-preview/     # Former markdown_widget
│   ├── ratkit-code-diff/
│   ├── ratkit-ai-chat/
│   ├── ratkit-hotkey-footer/
│   ├── ratkit-file-system-tree/
│   └── ratkit-theme-picker/
│
└── Services
    ├── ratkit-file-watcher/
    ├── ratkit-git-watcher/
    ├── ratkit-repo-watcher/
    └── ratkit-hotkey-service/
```

**Naming Convention**: All crates prefixed with `ratkit-`

### Workspace Structure and Crate Boundaries
- Define the workspace layout and create the crate scaffolds for all ratkit-* crates and the ratkit meta-crate.
- Establish clear ownership boundaries and avoid cross-crate leakage.
- Dependencies/prereqs: finalized crate list and module mapping.
- Risk factors and mitigation: tangled dependency graph; mitigate with a strict dependency policy and layering rules.

### Shared Types and Cross-Crate Contracts
- **ratkit meta-crate**: Pure re-export layer with no implementations; provides unified API surface.
- Define and document cross-crate contracts to prevent duplicated types and incompatible trait implementations.
- Dependencies/prereqs: identification of shared types and trait usage.
- Risk factors and mitigation: circular dependencies; mitigate by enforcing dependency direction (widgets/services depend on primitives, never the reverse) and consolidating shared types into the most appropriate primitive crate.

### Meta-Crate as Unified API Surface
- Define the ratkit meta-crate as the canonical API surface for re-exports and module organization.
- Ensure re-exports are curated and stable to keep the top-level API coherent.
- Dependencies/prereqs: public API map and crate boundaries.
- Risk factors and mitigation: re-export drift; mitigate with explicit re-export rules and validation checks.

### Dependency and Feature Management
- Define optional dependency policy per crate with explicit feature flags and clear defaults.
- Establish the meta-crate feature matrix and its mapping to underlying crates.
- Dependencies/prereqs: dependency inventory and optional feature list.
- Risk factors and mitigation: feature-flag explosion; mitigate with feature bundles and minimal-build validation.

### Validation and Quality Gates
- Define acceptance criteria for each crate: API completeness, code ownership boundaries, and integration into the meta-crate.
- Establish CI checks for per-crate build integrity and workspace-level coherence.
- **Migration Checklist**: Verify all ratatui-toolkit functionality is migrated before delisting.
- **Deprecation Notice**: Add deprecation warning to ratatui-toolkit crate pointing users to ratkit.
- Dependencies/prereqs: crate metadata and dependency policy.
- Risk factors and mitigation: publishing and build regressions; mitigate with package validation checks and consistent release hygiene.
