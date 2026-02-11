# Run standalone Markdown Widget demo with all features enabled
# Features: TOC, Statusline, Scrollbar, Selection, Syntax Highlighting
demo-md:
    @echo "Running Markdown Preview Demo..."
    cargo run --example markdown_preview_markdown_preview_demo --features markdown-preview
