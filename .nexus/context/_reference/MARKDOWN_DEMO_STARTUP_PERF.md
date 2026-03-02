# Markdown Demo Startup Performance Findings

## Problem Summary

`just demo-md` (loading `.agents/skills/opencode-rs-sdk/SKILL.md`) had a much slower time-to-first-render than `just demo-md-small` (loading `skills/ratkit/SKILL.md`).

## File Size Context

- Large input: `.agents/skills/opencode-rs-sdk/SKILL.md`
  - 1067 lines, 37504 bytes
- Small input: `skills/ratkit/SKILL.md`
  - 511 lines, 24199 bytes

Large file also contains far more code fences:

- Large: 78 fence lines (`^````)
- Small: 4 fence lines

## Reproduction Method

To measure startup consistently, run the markdown demo in a fixed-size pseudo terminal and use startup probe mode:

```bash
script -q /dev/null sh -c "stty cols 120 rows 40; env RATKIT_MD_DEMO_FILE=.agents/skills/opencode-rs-sdk/SKILL.md target/debug/examples/markdown_preview_markdown_preview_demo --startup-probe"
```

The demo prints:

```text
MARKDOWN_DEMO_READY_MS=<ms>
```

Equivalent small-file command:

```bash
script -q /dev/null sh -c "stty cols 120 rows 40; env RATKIT_MD_DEMO_FILE=skills/ratkit/SKILL.md target/debug/examples/markdown_preview_markdown_preview_demo --startup-probe"
```

## Measured Results

Before parser optimization:

- Large (`demo-md`): ~1293 to 1298 ms (about 1.29s)
- Small (`demo-md-small`): ~70 to 72 ms

After parser optimization:

- Large (`demo-md`): mostly ~150 to 152 ms (one outlier at 228 ms)
- Small (`demo-md-small`): ~47 to 49 ms

## Root Cause

In `src/widgets/markdown_preview/widgets/markdown_widget/foundation/parser.rs`, code block text handling instantiated `SyntaxHighlighter::new()` repeatedly inside the parser event loop for code block text.

`SyntaxHighlighter::new()` loads syntect defaults:

- `SyntaxSet::load_defaults_newlines()`
- `ThemeSet::load_defaults()`

That initialization is expensive, and was being repeated many times for markdown with many fenced blocks.

## Implemented Optimization

Moved highlighter creation to once per parse call:

- Create `let highlighter = SyntaxHighlighter::new();` before iterating parser events.
- Reuse that highlighter for all code lines in the parse.

This removed repeated syntect setup and reduced startup cost substantially.

## Notes

- Startup probe support was added to `examples/markdown_preview_markdown_preview_demo.rs` via `--startup-probe` so future startup performance checks can be repeated quickly.
- Cached rendering and parsed cache still provide additional runtime wins after first draw.
