# Record an interactive manual demo session
# Usage: just cast-interactive
# Example: just cast-interactive

cast-interactive:
    @echo "🗑️  Removing existing demo file..."
    @rm -f demo/ratatui-toolkit-demo.cast
    @echo "🎬 Starting interactive demo recording..."
    @echo "📝 You'll control the app manually"
    @echo "⏹️  Press Ctrl+D when done recording"
    @echo ""
    @mkdir -p demo
    asciinema rec demo/ratatui-toolkit-demo.cast
    @echo "✅ Demo recorded to demo/ratatui-toolkit-demo.cast"
    @echo "🎬 Converting to GIF..."
    @just cast-gif
    @echo "▶️  Playing GIF with mpv..."
    @mpv demo/ratatui-toolkit-demo.gif
