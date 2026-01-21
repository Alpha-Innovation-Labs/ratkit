# Run the standalone Terminal Pane demo
demo-term:
    @echo "Running Terminal Pane Demo..."
    cargo run --example showcase --features terminal

# ============================================================================
# Demo Widget Commands
# ============================================================================
import '../utilities/demo-codediff.just'
import '../utilities/demo-aichat.just'
import '../utilities/demo-split.just'
