# Remove all demo files
# Usage: just demo-clean
# Example: just demo-clean

demo-clean:
    @echo "🧹 Cleaning demo files..."
    @rm -rf demo/
    @echo "✅ Demo files removed"
