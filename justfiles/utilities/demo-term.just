# Run the standalone Terminal Pane demo
demo-term:
    @echo "Running Terminal Pane Demo..."
    cargo run --example termtui_term_mprocs_demo --features termtui

# ============================================================================
# Demo Widget Commands
# ============================================================================
import '../utilities/demo-codediff.just'
import '../utilities/demo-aichat.just'
import '../utilities/demo-split.just'
