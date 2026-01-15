# ratatui-toolkit justfile

# Default recipe - show help
default:
    @just --list

# Run the showcase demo with all components
dev:
    cargo run --example showcase --features full

# Run a specific example
example name:
    cargo run --example {{name}} --features full

# Build with all features
build:
    cargo build --all-features

# Run all tests
test:
    cargo test --all-features

# Run clippy linter
lint:
    cargo clippy --all-features -- -D warnings

# Format code
fmt:
    cargo fmt

# Check formatting
fmt-check:
    cargo fmt --check

# Build documentation
doc:
    cargo doc --all-features --no-deps --open

# Run all checks (format, lint, test)
check: fmt-check lint test

# Package for crates.io (dry run)
package:
    cargo publish --dry-run

# Clean build artifacts
clean:
    cargo clean
