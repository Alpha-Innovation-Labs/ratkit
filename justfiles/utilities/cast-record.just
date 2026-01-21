# Record the automated demo (VHS - no icons support)
# Usage: just cast-record
# Note: Icons won't show in VHS-generated GIF. For icons, use cast-interactive.
cast-record:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "📹 Recording automated demo (VHS)..."
    echo "⚠️  Note: Icons require Nerd Fonts which VHS doesn't support."
    echo "   For icons, use: just cast-interactive"

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
    echo ""
    echo "💡 Tip: For icons in your demo, run: just cast-interactive"
