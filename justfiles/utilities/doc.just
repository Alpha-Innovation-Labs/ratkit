# Start documentation server
# Usage: just docs
# Example: just docs

docs:
    @echo "🧹 Cleaning generated documentation..."
    @rm -rf docs/content/docs/primitives docs/content/docs/services docs/content/docs/widgets
    @echo "Generating documentation from rustdoc..."
    python3 docs/scripts/generate_docs.py
    @echo "Starting documentation server on port 3000..."
    mkdir -p logs
    cd docs && npm run dev 2>&1 | tee -a ../logs/docs.log
