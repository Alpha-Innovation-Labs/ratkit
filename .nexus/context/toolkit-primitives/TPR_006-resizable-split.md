---
context_id: TPR_006
title: Resizable Split Primitive
project: toolkit-primitives
created: "2026-01-21"
---

# TPR_006: Resizable Split Primitive

## Desired Outcome

A `ResizableSplit` primitive that provides resizable split panels with mouse drag support. Users can drag dividers between panels to adjust their relative sizes, with configurable minimum and maximum size constraints. Vertical splits allow horizontal dragging, and horizontal splits allow vertical dragging. Dividers highlight on hover to indicate interactivity.

## Reference

```
┌─────────────────┬───────┬─────────────────┐
│                 │       │                 │
│    Panel A      │ Divider│    Panel B      │
│  (left/top)     │◀─────▶│  (right/bottom) │
│                 │       │                 │
└─────────────────┴───────┴─────────────────┘

SplitDirection::Vertical   → Panels arranged left/right, divider drags horizontally
SplitDirection::Horizontal → Panels arranged top/bottom, divider drags vertically

Drag Constraints:
  split_percent: 0-100 (position of divider)
  min_percent: 0-100 (minimum allowed position)
  max_percent: 0-100 (maximum allowed position)
```

## Next Actions

| Description | Test |
|-------------|------|
| Implement `SplitDirection` enum with `Vertical` and `Horizontal` variants | `split_direction_defined` |
| Implement `ResizableSplit` struct with `split_percent`, `min_percent`, and `max_percent` fields | `resizable_split_structured` |
| Vertical split arrangement shows panels side-by-side with draggable vertical divider | `split_vertical_layout` |
| Horizontal split arrangement shows panels stacked with draggable horizontal divider | `split_horizontal_layout` |
| Dragging vertical split divider left or right updates panel positions proportionally | `split_drag_updates_position` |
| Dragging horizontal split divider up or down updates panel positions proportionally | `split_horizontal_drag_vertical` |
| Divider position cannot be reduced below configured minimum percentage | `split_respect_minimum_size` |
| Divider position cannot exceed configured maximum percentage | `split_respect_maximum_size` |
| Divider changes visual appearance on hover to indicate interactivity | `divider_hover_highlighted` |
| Cursor changes to resize indicator when hovering over divider | `divider_cursor_indicator` |
