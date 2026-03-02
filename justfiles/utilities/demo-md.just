# Run standalone Markdown Widget demo with all features enabled
# Features: TOC, Statusline, Scrollbar, Selection, Syntax Highlighting
demo-md:
    @echo "Running Markdown Preview Demo..."
    RATKIT_MD_DEMO_FILE=.agents/skills/opencode-rs-sdk/SKILL.md cargo run --example markdown_preview_markdown_preview_demo --features markdown-preview

# Run markdown demo with smaller/default ratkit skill markdown
demo-md-small:
    @echo "Running Markdown Preview Demo (small markdown)..."
    RATKIT_MD_DEMO_FILE=skills/ratkit/SKILL.md cargo run --example markdown_preview_markdown_preview_demo --features markdown-preview
