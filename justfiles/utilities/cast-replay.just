# Replay the interactive demo
# Usage: just cast-replay
# Example: just cast-replay

cast-replay:
    @echo "▶️  Replaying interactive demo..."
    @if [ ! -f demo/ratatui-toolkit-demo.cast ]; then \
        echo "❌ Error: demo/ratatui-toolkit-demo.cast not found. Run 'just cast-interactive' first"; \
        exit 1; \
    fi
    asciinema play demo/ratatui-toolkit-demo.cast
