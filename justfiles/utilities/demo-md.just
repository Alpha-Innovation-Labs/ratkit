# Run standalone Markdown Widget demo with all features enabled
# Features: TOC, Statusline, Scrollbar, Selection, Syntax Highlighting
demo-md:
    @echo "Running Markdown Widget Demo..."
    cargo run --example markdown_demo --features full
