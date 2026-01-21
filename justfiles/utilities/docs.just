# Run Fumadocs documentation site
# Usage: just docs

# Build documentation from rustdoc + copy README files
docs-build:
    @echo "\033[1;36mBuilding documentation...\033[0m"
    @mkdir -p ../logs
    @cd docs && npm run build

# Run dev server (assumes docs already built)
# Usage: just docs
docs:
    @echo "\033[1;36mStarting dev server...\033[0m"
    @mkdir -p ../logs
    @cd docs && npm run dev >> ../logs/server.log 2>&1

# Build for production
# Usage: just docs-build-prod
docs-build-prod:
    @echo "\033[1;36mBuilding production documentation...\033[0m"
    @mkdir -p ../logs
    @cd docs && NODE_ENV=production npm run build

# Clean build artifacts
# Usage: just docs-clean
docs-clean:
    @echo "\033[1;36mCleaning docs build artifacts...\033[0m"
    @rm -rf docs/.next docs/out
    @echo "\033[1;32m✓ Cleaned\033[0m"

# Install dependencies
# Usage: just docs-install
docs-install:
    @echo "\033[1;36mInstalling dependencies...\033[0m"
    @cd docs && npm install

# View logs
# Usage: just docs-logs
docs-logs:
    @tail -f ../logs/server.log
