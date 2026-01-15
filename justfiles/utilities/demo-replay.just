# Replay the interactive demo
# Usage: just demo-replay
# Example: just demo-replay

demo-replay:
    @echo "▶️  Replaying interactive demo..."
    @if [ ! -f demo/ratatui-toolkit-demo.cast ]; then \
        echo "❌ Error: demo/ratatui-toolkit-demo.cast not found. Run 'just demo-interactive' first"; \
        exit 1; \
    fi
    asciinema play demo/ratatui-toolkit-demo.cast
