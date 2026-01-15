# Run clippy linter
# Usage: just lint
# Example: just lint

lint:
    @echo "Running clippy..."
    cargo clippy --all-features -- -D warnings
