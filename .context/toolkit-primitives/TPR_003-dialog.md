---
context_id: TPR_003
title: Dialog Primitive
project: toolkit-primitives
created: "2026-01-21"
---

# TPR_003: Dialog Primitive

## Desired Outcome

The Dialog primitive provides modal dialog widgets for displaying important information and capturing user confirmation. Dialogs render as centered modal windows overlaying the main content with configurable types (Info, Success, Warning, Error, Confirm) that each have distinct visual styling. The primitive supports customizable buttons, footer text, configurable dimensions via percentage, and automatic overlay dimming of background content.

## Next Actions

| Description | Test |
|-------------|------|
| Dialog renders centered modal window with title, message, and buttons | `dialog_renders_modal` |
| Info dialog type displays cyan border and themed styling | `dialog_info_type_shows_cyan_border` |
| Success dialog type displays green border and themed styling | `dialog_success_type_shows_green_border` |
| Warning dialog type displays yellow/orange border and themed styling | `dialog_warning_type_shows_yellow_border` |
| Error dialog type displays red border and themed styling | `dialog_error_type_shows_red_border` |
| Confirm dialog type displays action buttons for user response | `dialog_confirm_type_shows_action_buttons` |
| Click detection on dialog buttons triggers callback handlers | `dialog_button_click_handled` |
| Overlay renders and dims background content behind modal | `dialog_overlay_dims_background` |
| Footer text displays below dialog content | `dialog_footer_text_renders` |
| Dialog width and height configurable via percentage of terminal size | `dialog_configurable_dimensions` |
| Custom colors override default type-specific styling | `dialog_custom_colors_override` |
