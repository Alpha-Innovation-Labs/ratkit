# Pick and run an example via fzf
# Usage: just demo
# Example: just demo

demo:
    #!/usr/bin/env bash
    set -euo pipefail

    if ! command -v fzf >/dev/null 2>&1; then
        echo "fzf is required for this command."
        echo "Install it with: brew install fzf"
        exit 1
    fi

    selection=$(rg --files \
        -g 'crates/**/examples/*.rs' \
        -g '!**/_archives/**' \
        -g '!**/target/**' \
        | while IFS= read -r file; do
            crate_dir=$(dirname "$(dirname "$file")")
            manifest="$crate_dir/Cargo.toml"
            if [[ -f "$manifest" ]]; then
                pkg=$(awk -F'"' '/^name =/ {print $2; exit}' "$manifest")
                example=$(basename "$file" .rs)
                if [[ -n "$pkg" && -n "$example" ]]; then
                    printf '%s:%s\n' "$pkg" "$example"
                fi
            fi
        done \
        | sort -u \
        | fzf --prompt="Select example > ")

    if [[ -z "${selection}" ]]; then
        echo "No example selected."
        exit 0
    fi

    pkg=${selection%%:*}
    example=${selection#*:}

    echo "Running $pkg:$example..."
    cargo run -p "$pkg" --example "$example"
