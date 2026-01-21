# Remove all demo files
# Usage: just cast-clean
# Example: just cast-clean

cast-clean:
    @echo "🧹 Cleaning demo files..."
    @rm -rf demo/
    @echo "✅ Demo files removed"
