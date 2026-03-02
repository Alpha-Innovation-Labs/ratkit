# Bump patch version and sync version references
# Usage: just bump-version
# Example: just bump-version

bump-version:
    #!/usr/bin/env bash
    set -euo pipefail

    CURRENT_VERSION=$(awk -F'"' '/^version = "[0-9]+\.[0-9]+\.[0-9]+"/ { print $2; exit }' Cargo.toml)
    if [[ -z "${CURRENT_VERSION:-}" ]]; then
        echo "Could not read current version from Cargo.toml"
        exit 1
    fi

    IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"
    NEW_VERSION="${MAJOR}.${MINOR}.$((PATCH + 1))"

    echo "Current version: $CURRENT_VERSION"
    echo "New version: $NEW_VERSION"

    awk -v v="$NEW_VERSION" '
        BEGIN { updated = 0 }
        {
            if (!updated && $0 ~ /^version = "[0-9]+\.[0-9]+\.[0-9]+"/) {
                sub(/^version = "[0-9]+\.[0-9]+\.[0-9]+"/, "version = \"" v "\"")
                updated = 1
            }
            print
        }
    ' Cargo.toml > Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml

    awk -v v="$NEW_VERSION" '
        {
            gsub(/ratkit = "[0-9]+\.[0-9]+\.[0-9]+"/, "ratkit = \"" v "\"")
            gsub(/ratkit = \{ version = "[0-9]+\.[0-9]+\.[0-9]+"/, "ratkit = { version = \"" v "\"")
            print
        }
    ' README.md > README.md.tmp && mv README.md.tmp README.md

    awk -v v="$NEW_VERSION" '
        {
            if ($0 ~ /^  version: "[0-9]+\.[0-9]+\.[0-9]+"$/) {
                sub(/^  version: "[0-9]+\.[0-9]+\.[0-9]+"$/, "  version: \"" v "\"")
            }
            if ($0 ~ /^ratkit = \{ version = "[0-9]+\.[0-9]+\.[0-9]+"/) {
                sub(/^ratkit = \{ version = "[0-9]+\.[0-9]+\.[0-9]+"/, "ratkit = { version = \"" v "\"")
            }
            if ($0 ~ /^- Current: [0-9]+\.[0-9]+\.[0-9]+$/) {
                sub(/^- Current: [0-9]+\.[0-9]+\.[0-9]+$/, "- Current: " v)
            }
            print
        }
    ' skills/ratkit/SKILL.md > skills/ratkit/SKILL.md.tmp && mv skills/ratkit/SKILL.md.tmp skills/ratkit/SKILL.md

    awk -v v="$NEW_VERSION" '
        {
            gsub(/ratkit = "[0-9]+\.[0-9]+(\.[0-9]+)?"/, "ratkit = \"" v "\"")
            gsub(/ratkit = \{ version = "[0-9]+\.[0-9]+(\.[0-9]+)?"/, "ratkit = { version = \"" v "\"")
            gsub(/docs\.rs\/ratkit\/[0-9]+\.[0-9]+(\.[0-9]+)?/, "docs.rs/ratkit/" v)
            print
        }
    ' src/lib.rs > src/lib.rs.tmp && mv src/lib.rs.tmp src/lib.rs

    echo "Updated Cargo.toml, README.md, skills/ratkit/SKILL.md, and src/lib.rs"
