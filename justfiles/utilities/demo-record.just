# Record the automated demo
# Usage: just demo-record
# Example: just demo-record

demo-record:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "📹 Recording automated demo..."

    # Check if VHS is installed
    if ! command -v vhs >/dev/null 2>&1; then
        echo "⚠️  VHS not found. Installing via Homebrew..."
        brew install charmbracelet/tap/vhs
    fi

    # Create demo directory if it doesn't exist
    mkdir -p demo

    # Check if demo.tape exists
    if [ ! -f demo/demo.tape ]; then
        echo "❌ Error: demo/demo.tape not found"
        exit 1
    fi

    # Record the demo
    vhs --output demo/ratatui-toolkit-demo.gif demo/demo.tape

    echo "✅ Demo recorded!"
    echo "📹 GIF: demo/ratatui-toolkit-demo.gif ($(du -h demo/ratatui-toolkit-demo.gif | cut -f1 2>/dev/null || echo "N/A"))"
