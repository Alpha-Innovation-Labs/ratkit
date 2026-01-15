# Check formatting
# Usage: just fmt-check
# Example: just fmt-check

fmt-check:
    @echo "Checking formatting..."
    cargo fmt --check
