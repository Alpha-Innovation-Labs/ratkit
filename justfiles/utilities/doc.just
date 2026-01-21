# Start documentation server
# Usage: just doc
# Example: just doc

doc:
    @echo "Starting documentation server..."
    cd docs && npm run dev
