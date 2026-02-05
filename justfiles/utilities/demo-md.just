# Run standalone Markdown Widget demo with all features enabled
# Features: TOC, Statusline, Scrollbar, Selection, Syntax Highlighting
demo-md:
    @echo "Running Markdown Preview Demo..."
    cargo run -p ratkit-markdown-preview --example markdown_preview_demo
