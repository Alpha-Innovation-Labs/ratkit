---
context_id: TPR_004
title: Menu Bar Primitive
project: toolkit-primitives
created: "2026-01-21"
---

# TPR_004: Menu Bar Primitive

## Desired Outcome

The MenuBar primitive provides a horizontal menu bar with selectable items. Menu items display a name and optional icon (Nerd Font icon or emoji), and can be selected or hovered with distinct visual styling for each state. Click detection enables item selection, and area tracking supports interactive mouse behavior.

## Next Actions

| Description | Test |
|-------------|------|
| MenuBar renders horizontally with all items in a single row | `menu_bar_renders_horizontally` |
| MenuItem displays name text correctly | `menu_item_displays_name` |
| MenuItem with Nerd Font icon renders the icon character | `menu_item_with_nerd_font_icon_displays_icon` |
| MenuItem with emoji renders the emoji character | `menu_item_with_emoji_displays_icon` |
| Selected MenuItem displays different visual style than unselected | `menu_item_selection_changes_visual` |
| Hovered MenuItem displays different visual style than non-hovered | `menu_item_hover_changes_visual` |
| MenuItem in selected-hover state displays combined styling | `menu_item_selected_hover_combines_styles` |
| Mouse hover over MenuItem updates hover state | `mouse_hover_highlights_item` |
| Click on MenuItem updates selection state | `click_selects_menu_item` |
| MenuBar tracks area for each item enabling click detection | `menu_bar_tracks_item_areas` |
| Multiple MenuItems are rendered with consistent spacing | `menu_items_render_with_consistent_spacing` |
