# Overview

Ratkit core runtime contexts covering the runner, layout manager, and core crate packaging.

# Architecture

```
ratkit-core
├── runtime (runner, dispatch, scheduler)
├── layout (geometry, focus, z-order)
└── crate packaging (core exports and feature flags)
```

# CLI Usage

Not applicable.

# Key Dependencies

| Dependency | Purpose |
|------------|---------|
| ratatui | Core TUI rendering and layout engine |

# Environment Variables

None.

# Debugging & Troubleshooting

If redraw or focus routing behaves unexpectedly, verify layout dirty flag flow and event dispatch order.
