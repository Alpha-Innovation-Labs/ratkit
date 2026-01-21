# Upload demo to asciinema.org
# Usage: just cast-upload
# Example: just cast-upload

cast-upload:
    @echo "☁️  Uploading demo to asciinema.org..."
    @if [ ! -f demo/ratatui-toolkit-demo.cast ]; then \
        echo "❌ Error: demo/ratatui-toolkit-demo.cast not found. Run 'just cast-interactive' first"; \
        exit 1; \
    fi
    asciinema upload demo/ratatui-toolkit-demo.cast
    @echo "✅ Demo uploaded!"
