# Run the showcase demo with all components
# Usage: just dev
# Example: just dev

dev:
    @echo "Starting showcase demo..."
    @if tty -s; then \
        cargo run --example showcase --features full; \
    else \
        script -q /dev/null cargo run --example showcase --features full; \
    fi
