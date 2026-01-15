# Package for crates.io (dry run)
# Usage: just package
# Example: just package

package:
    @echo "Packaging for crates.io..."
    cargo publish --dry-run
