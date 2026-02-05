---
context_id: TPR_001
title: Multi-Split Layout Primitive
project: toolkit-primitives
created: "2026-01-21"
---

# TPR_001: Multi-Split Layout Primitive

## Desired Outcome

The toolkit provides a multi-split layout primitive and a first-class widget that lets callers create nested horizontal and vertical panes, resize dividers with mouse interactions, and render hover feedback directly through the widget API.

## Reference

None.

## Next Actions

| Description | Test |
|-------------|------|
| Implement `SplitLayout` primitive that exposes a public API for nested horizontal and vertical panes | `split_layout_public_api` |
| Adding a horizontal split to a pane produces two visible panes with correct geometry | `split_add_horizontal` |
| Adding a vertical split to a pane produces two visible panes with correct geometry | `split_add_vertical` |
| Dragging a divider in the `SplitLayoutWidget` resizes adjacent panes while keeping minimum sizes enforced | `split_widget_resize_with_minimums` |
| Hovering a divider in the `SplitLayoutWidget` applies hover styling visible in the rendered output | `split_widget_hover_styling` |
| Moving a pane to a different split group updates layout ordering without losing pane identity | `split_move_pane` |
| Removing a pane collapses redundant split nodes and leaves a valid layout | `split_remove_pane` |
