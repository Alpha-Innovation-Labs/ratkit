# Run Fumadocs documentation site
# Usage: just docs
# Example: just docs

docs:
    @echo "Starting Fumadocs documentation server..."
    @cd docs && npm run dev

# Build Fumadocs documentation for production
# Usage: just docs-build
# Example: just docs-build

docs-build:
    @echo "Building Fumadocs documentation..."
    @cd docs && npm run build

# Install Fumadocs documentation dependencies
# Usage: just docs-install
# Example: just docs-install

docs-install:
    @echo "Installing Fumadocs documentation dependencies..."
    @cd docs && npm install
