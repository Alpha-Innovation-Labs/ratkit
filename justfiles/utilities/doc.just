# Start documentation server
# Usage: just doc
# Example: just doc

doc:
    @echo "Starting documentation server..."
    pkill -f "next dev" || true
    cd docs && npm run dev
