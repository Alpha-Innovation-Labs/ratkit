# Run a specific example
# Usage: just example <name>
# Example: just example my-example

example name:
    @echo "Running example: {{name}}..."
    cargo run --example {{name}} --features full
