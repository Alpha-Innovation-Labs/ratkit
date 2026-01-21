# Start documentation server
# Usage: just docs
# Example: just docs

docs:
    @echo "Generating documentation from rustdoc..."
    python3 docs/scripts/generate_docs.py
    @echo "Starting documentation server on port 3000..."
    mkdir -p logs
    cd docs && npm run dev >> ../logs/docs.log 2>&1
