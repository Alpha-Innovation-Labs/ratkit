# Convert interactive demo to GIF (requires agg)
# Usage: just demo-gif
# Example: just demo-gif

demo-gif:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "🎨 Converting ratatui-toolkit-demo.cast to GIF..."

    mkdir -p demo

    if [ ! -f demo/ratatui-toolkit-demo.cast ]; then
        echo "❌ Error: demo/ratatui-toolkit-demo.cast not found. Run 'just demo-interactive' first"
        exit 1
    fi

    if ! command -v agg >/dev/null 2>&1; then
        echo "⚠️  agg not found. Installing..."
        cargo install --git https://github.com/asciinema/agg
    fi

    agg demo/ratatui-toolkit-demo.cast demo/ratatui-toolkit-demo.gif
    echo "✅ GIF created: demo/ratatui-toolkit-demo.gif"
    echo "📊 File size: $(du -h demo/ratatui-toolkit-demo.gif | cut -f1)"
