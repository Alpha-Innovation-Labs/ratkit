---
context_id: RKC_003
title: Ratkit Core Crate Packaging
project: ratkit-core
created: "2026-02-05"
---

# RKC_003: Ratkit Core Crate Packaging

## Desired Outcome

The `ratkit` crate provides the core runtime by default, always including the Runner and Layout Manager for consumers. All primitives, widgets, and services remain behind feature flags so users can opt into only what they need, while an `all` feature enables the full set for convenience. Diagnostics overlays are optional helpers, not part of the default runtime. The crate documentation clearly describes the default core runtime and how to enable optional components.

## Next Actions

| Description | Test |
|-------------|------|
| Expose Runner and Layout Manager as part of the default ratkit public API | `ratkit_core_exports_available` |
| Ensure the core runtime is included without requiring feature flags | `ratkit_core_included_by_default` |
| Gate primitives, widgets, and services behind individual feature flags | `ratkit_optional_components_gated` |
| Provide an `all` feature that enables every optional component | `ratkit_all_feature_enables_all` |
| Provide optional diagnostics helpers for FPS/redraws | `ratkit_diagnostics_helpers_available` |
| Document how to enable optional features and the default core runtime | `ratkit_feature_docs_updated` |
