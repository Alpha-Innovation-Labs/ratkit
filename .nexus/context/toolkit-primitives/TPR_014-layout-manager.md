---
context_id: TPR_014
title: Layout Manager
project: toolkit-primitives
created: "2025-02-05"
---

# TPR_014: Layout Manager

## Desired Outcome

A Layout Manager system that owns geometry computation, z-order management, and focus stack coordination for all UI elements. The system provides per-element Rect assignments for rendering, maintains z-order for hit testing and mouse routing, manages focus transitions with fallback handling, and coordinates dirty flag lifecycle with the Runner. Elements declare their region (Top/Center/Bottom), visibility, and optional z-order; the Layout Manager recalculates layouts on resize, element registration changes, or visibility toggles, and exposes geometry and focus state for the Runner's event dispatch and rendering decisions.

## Reference

See `.nexus/context/_reference/layout-manager-plan.md` for complete architecture specification including:
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
| Implement z-order maintenance and hit testing API | `z_order_hit_testing_works` |
| Implement focus stack with push/pop/cycle operations | `focus_stack_operations_work` |
| Implement focus fallback when focused element unregisters | `focus_fallback_works` |
| Implement layout dirty flag (set by Layout Manager, read by Runner) | `dirty_flag_lifecycle_works` |
| Implement resize handling with debounced invalidation | `resize_debounce_works` |
| Implement dynamic element registration/deregistration | `dynamic_registration_works` |
| Integrate Layout Manager with Runner event loop | `runner_integration_works` |
| Create diagnostic API for z-order, rects, and focus stack inspection | `diagnostics_api_works` |
| Implement mouse capture mode with timeout and validation | `mouse_capture_works` |
