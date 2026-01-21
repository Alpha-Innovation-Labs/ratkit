# Start documentation server
# Usage: just doc
# Example: just doc

doc:
    @echo "Starting documentation server on port 3000..."
    cd docs && npm run dev -- -p 3000
