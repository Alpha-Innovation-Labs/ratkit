---
context_id: TPR_013
title: TUI Documentation Screenshots
project: toolkit-primitives
created: "2026-01-21"
---

# TPR_013: TUI Documentation Screenshots

## Desired Outcome

Documentation has consistent, up-to-date screenshots for TUI components captured as real terminal images using Ghostty, stored under the docs screenshots directory for the website.

## Next Actions

| Description | Test |
|-------------|------|
| Define screenshot runner script location as `docs/scripts/ghostty-screenshot.sh` | `ghostty_runner_location` |
| Define default output directory as `docs/screenshots` | `ghostty_output_directory` |
| Create a Ghostty automation script that runs every `demo-*` command and saves a PNG per demo | `ghostty_demo_screenshots_created` |
| Require the script to save screenshots under `docs/screenshots` so docs can reference them | `ghostty_screenshots_in_docs` |
| Require the script to run each demo from the project root using `bash -lc` | `ghostty_uses_project_root` |
| Add fullscreen capture support so screenshots use the terminal at maximum size | `ghostty_fullscreen_capture` |
| Add a configurable crop setting to shave a fixed number of pixels from each edge | `ghostty_crop_applied` |
| Require the script to quit each demo with `q` before starting the next capture | `ghostty_demo_quit_between_runs` |
| Ensure the Ghostty window closes after a successful capture batch | `ghostty_window_closed` |
