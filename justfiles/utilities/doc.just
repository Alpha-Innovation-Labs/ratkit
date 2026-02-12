# Start documentation server
# Usage: just docs
# Example: just docs

docs:
    @echo "📦 Installing dependencies (if needed)..."
    @cd docs && npm install --legacy-peer-deps
    @echo "🚀 Starting documentation server on port 3000..."
    @mkdir -p logs
    @cd docs && npm run dev 2>&1 | tee -a ../logs/docs.log
