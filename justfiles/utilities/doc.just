# Start documentation server
# Usage: just doc
# Example: just doc

doc:
    @echo "Starting documentation server on port 3000..."
    mkdir -p logs
    cd docs && npm run dev -- -p 3000 >> ../logs/docs.log 2>&1
