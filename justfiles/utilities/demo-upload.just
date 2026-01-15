# Upload demo to asciinema.org
# Usage: just demo-upload
# Example: just demo-upload

demo-upload:
    @echo "☁️  Uploading demo to asciinema.org..."
    @if [ ! -f demo/ratatui-toolkit-demo.cast ]; then \
        echo "❌ Error: demo/ratatui-toolkit-demo.cast not found. Run 'just demo-interactive' first"; \
        exit 1; \
    fi
    asciinema upload demo/ratatui-toolkit-demo.cast
    @echo "✅ Demo uploaded!"
