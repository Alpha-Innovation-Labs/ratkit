# Clean generated documentation
# Usage: just docs-clean
# Example: just docs-clean

docs-clean:
    @echo "🧹 Cleaning generated documentation..."
    @rm -rf docs/content/docs/primitives docs/content/docs/services docs/content/docs/widgets
    @echo "✅ Generated docs removed"
