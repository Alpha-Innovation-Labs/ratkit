---
context_id: RKC_002
title: Runner
project: ratkit-core
created: "2026-02-05"
---

# RKC_002: Runner

## Desired Outcome

A Runner system that owns the event loop and render orchestration for the ratkit core runtime. The Runner polls for events, dispatches mouse, keyboard, tick, and resize inputs, and coordinates with the Layout Manager to recompute geometry when needed. Keyboard events route only to the focused element, mouse events route by z-order (with capture support), tick events route to scheduled handlers, and resize events route to the Layout Manager with coalescing. The Runner determines redraws when layout is dirty or any element is dirty, renders all visible elements on each redraw, and clears the layout dirty flag only after a successful render. Diagnostics overlays (FPS/redraws) are optional and must be explicitly enabled.

## Reference

See `.nexus/context/ratkit-core/_reference/layout-manager-plan.md` for runtime event loop flow and dirty flag lifecycle.

## Next Actions

| Description | Test |
|-------------|------|
| Implement `Runner` struct that owns event dispatch and rendering coordination | `runner_struct_created` |
| Poll for events and dispatch based on mouse, keyboard, tick, and resize types | `runner_event_dispatches` |
| Route keyboard events only to the focused element | `runner_keyboard_focus_routing` |
| Route mouse events by z-order with capture support | `runner_mouse_z_order_routing` |
| Route tick events only to scheduled handlers | `runner_tick_routing` |
| Coalesce resize events and route to the Layout Manager | `runner_resize_coalesced` |
| Trigger redraw when layout is dirty or any element is dirty | `runner_redraw_when_dirty` |
| Render all visible elements on each redraw | `runner_renders_all_visible` |
| Clear layout dirty flag only after successful render | `runner_clears_layout_dirty_after_render` |
| Collect async tick results without blocking render loop | `runner_collects_async_tick_results` |
| Provide optional diagnostics overlay for FPS/redraws | `runner_diagnostics_optional` |
