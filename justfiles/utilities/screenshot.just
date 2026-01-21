# Screenshot Generation Commands

# Generate all screenshots
screenshot-gen-all:
    @echo "\033[1;36mGenerating screenshots for all components...\033[0m"
    cargo run --example screenshot_generator

# Generate screenshot for specific component
screenshot-gen COMPONENT:
    @echo "\033[1;36mGenerating screenshot for: {{COMPONENT}}\033[0m"
    cargo run --example screenshot_generator {{COMPONENT}}

# Clean up generated screenshots
screenshot-clean:
    @echo "\033[1;36mCleaning up screenshots...\033[0m"
    rm -rf docs/screenshots/*.png
    @rm -f docs/screenshots/manifest.json
    @echo "\033[1;32m✓ Screenshots cleaned\033[0m"
