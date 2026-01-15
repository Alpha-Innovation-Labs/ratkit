# Record an interactive manual demo session
# Usage: just demo-interactive
# Example: just demo-interactive

demo-interactive:
    @echo "🎬 Starting interactive demo recording..."
    @echo "📝 You'll control the app manually"
    @echo "⏹️  Press Ctrl+D when done recording"
    @echo ""
    @mkdir -p demo
    asciinema rec demo/ratatui-toolkit-demo.cast
    @echo "✅ Demo recorded to demo/ratatui-toolkit-demo.cast"
