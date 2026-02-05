---
context_id: RKC_001
title: Layout Manager
project: ratkit-core
created: "2026-02-05"
---

# RKC_001: Layout Manager

## Desired Outcome

A Layout Manager system that owns geometry computation, z-order management, and focus stack coordination for all UI elements in the ratkit core runtime. The system assigns per-element Rects, maintains z-order for hit testing and mouse routing, and manages focus transitions with fallback handling. Elements declare their region (Top/Center/Bottom), visibility, fixed heights for top/bottom, and optional z-order for center overlays; the Layout Manager recalculates layouts on resize, element registration changes, or visibility toggles using fixed-region sizing (top/bottom fixed, center fills remaining space). Focus ordering follows Top → Center (by z-order) → Bottom, mouse routing uses a z-order snapshot with optional capture semantics, and terminal sizes are clamped to minimum bounds. The Layout Manager sets the layout dirty flag but never reads or clears it, while the Runner reads and clears it after a successful render. The system exposes geometry, z-order, and focus state for the Runner's event dispatch and rendering decisions.

## Reference

See `.nexus/context/ratkit-core/_reference/layout-manager-plan.md` for complete architecture specification including:
- System diagram and event loop flow
- Layout model with Top/Center/Bottom regions
- Focus and input routing rules
- Z-order and hit testing semantics
- Dirty flag lifecycle (Layout Manager sets, Runner reads and clears)
- Dynamic registration with UUID-based IDs

## Next Actions

| Description | Test |
|-------------|------|
| Implement `LayoutManager` struct with geometry computation for Top/Center/Bottom regions | `layout_manager_struct_created` |
| Create element metadata types (region, visibility, height, z_order) | `element_metadata_types_defined` |
| Apply fixed-region sizing rules with top/bottom fixed heights and center fill | `layout_region_sizing_applied` |
| Implement z-order maintenance and hit testing API | `z_order_hit_testing_works` |
| Mouse routing uses a z-order snapshot for hit testing | `mouse_routing_uses_z_order_snapshot` |
| Implement focus stack with push/pop/cycle operations | `focus_stack_operations_work` |
| Focus order follows Top then Center by z-order then Bottom | `focus_order_follows_regions` |
| Implement focus fallback when focused element unregisters | `focus_fallback_works` |
| Implement layout dirty flag (set by Layout Manager, read by Runner) | `dirty_flag_lifecycle_works` |
| Layout Manager never reads or clears its own dirty flag | `layout_manager_dirty_flag_one_way` |
| Implement resize handling with debounced invalidation | `resize_debounce_works` |
| Clamp terminal size to minimum bounds for layout computation | `terminal_size_clamped` |
| Implement dynamic element registration and deregistration | `dynamic_registration_works` |
| Integrate Layout Manager with Runner event loop | `runner_integration_works` |
| Create diagnostic API for z-order, rects, and focus stack inspection | `diagnostics_api_works` |
| Implement mouse capture mode with timeout and validation | `mouse_capture_works` |
