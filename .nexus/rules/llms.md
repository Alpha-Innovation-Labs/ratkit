# LLMs.txt Standards

## Purpose

Provide a concise, LLM-friendly entry point for this repository that follows the llmstxt.org standard. The file should explain what the project is and point to the most useful documentation and examples.

## Location

Create the file at the repo root as `llms.txt`.

## Required Format

Follow the exact llmstxt.org structure:

1. H1 title with the project name.
2. Blockquote summary (1-3 sentences) describing what the project is and who it is for.
3. Optional details section (plain paragraphs, no headings) for short guidance.
4. One or more H2 sections that contain bullet lists of links.
5. Optional `## Optional` section for secondary links that can be skipped.

Example skeleton:

```
# Project Name

> One short paragraph describing the project.

Short guidance paragraph (optional).

## Docs

- [README](README.md): What this project is and how to use it.

## Examples

- [Showcase](crates/ratatui-toolkit/examples/showcase/): Demo application.

## Optional

- [Change Log](CHANGELOG.md): Release history.
```

## Content Guidelines

1. **Be concise** - Keep it short and curated (6-12 links total outside Optional).
2. **Prioritize user entry points** - README, API overview, main examples, and primary crates.
3. **Avoid exhaustive lists** - Do not enumerate every file or module.
4. **Use repo-relative links** - Prefer local paths over external URLs when possible.
5. **Describe each link** - Add a short description after `:`.
6. **Avoid internal implementation detail** - Link to public APIs and user-facing docs.

## Suggested Sections

- `## Docs` - README, crate docs entry point, primary public API overview.
- `## Examples` - Main demo(s) or showcase apps.
- `## Architecture` - High-level design docs if they exist.
- `## Optional` - Deep references, large docs, or change logs.

## Update Rules

1. Keep summaries current with each release.
2. Add or remove links when the primary entry points change.
3. Ensure paths stay valid after refactors.
