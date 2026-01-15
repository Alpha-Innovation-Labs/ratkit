# Build documentation
# Usage: just doc
# Example: just doc

doc:
    @echo "Building documentation..."
    cargo doc --all-features --no-deps --open
