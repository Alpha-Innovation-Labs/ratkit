---
context_id: TPR_007
title: Status Line Primitive
project: toolkit-primitives
created: "2026-01-21"
---

# TPR_007: Status Line Primitive

## Desired Outcome

A powerline-style statusline primitive with stacked indicators for terminal applications. The `StatusLineStacked` provides left-aligned and right-aligned stacked indicators with a centered status message, using diagonal powerline slants for visual separation. The `StyledStatusLine` extends this with operational mode styling (green for Operational, yellow for Dire, red for Evacuate) following a reactor-control "Westinghouse" aesthetic.

## Next Actions

| Description | Test |
|-------------|------|
| Implement `StatusLineStacked` with start(), center(), end() methods for content placement | `statusline_stacked_methods_exist` |
| Render powerline-style slants (SLANT_TL_BR, SLANT_BL_TR) as diagonal separators | `powerline_slants_render_correctly` |
| Stack indicators on the left side of the statusline | `statusline_stacks_indicators_left` |
| Stack indicators on the right side of the statusline | `statusline_stacks_indicators_right` |
| Center the status message between left and right indicator stacks | `statusline_centers_message` |
| Apply green styling for Operational mode in StyledStatusLine | `styled_statusline_green_in_operational_mode` |
| Apply yellow styling for Dire mode in StyledStatusLine | `styled_statusline_yellow_in_dire_mode` |
| Apply red styling for Evacuate mode in StyledStatusLine | `styled_statusline_red_in_evacuate_mode` |
| Follow reactor-control "Westinghouse" aesthetic for StyledStatusLine visual design | `styled_statusline_follows_westinghouse_aesthetic` |
