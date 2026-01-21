#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/../.." && pwd)"
content_dir="${repo_root}/docs/content/docs/widgets"

mkdir -p "${content_dir}"

printf "Copying README.md files from crates to docs/content/docs/widgets...\n\n"

widgets=(
  "ai-chat:crates/ratatui-toolkit/src/widgets/ai_chat/README.md"
  "markdown-renderer:crates/ratatui-toolkit/src/widgets/markdown_widget/README.md"
  "tree-view:crates/ratatui-toolkit/src/widgets/tree_view/README.md"
  "file-system-tree:crates/ratatui-toolkit/src/widgets/file_system_tree/README.md"
  "resizable-split:crates/ratatui-toolkit/src/widgets/split_layout/README.md"
  "menu-bar:crates/ratatui-toolkit/src/widgets/menu_bar/README.md"
  "status-line-stacked:crates/ratatui-toolkit/src/widgets/status_line/README.md"
  "toast:crates/ratatui-toolkit/src/widgets/toast/README.md"
  "button:crates/ratatui-toolkit/src/primitives/button/README.md"
  "dialog:crates/ratatui-toolkit/src/widgets/dialog/README.md"
  "hotkey-footer:crates/ratatui-toolkit/src/widgets/hotkey_footer/README.md"
  "term-tui:crates/ratatui-toolkit/src/widgets/term_tui/README.md"
)

for entry in "${widgets[@]}"; do
  name="${entry%%:*}"
  source_rel="${entry#*:}"
  source_path="${repo_root}/${source_rel}"

  if [[ -f "${source_path}" ]]; then
    dest_path="${content_dir}/${name}.mdx"
    {
      printf "---\n"
      printf "title: %s\n" "${name}"
      printf "description: Component usage and examples\n"
      printf "---\n\n"
      cat "${source_path}"
    } > "${dest_path}"
    printf "✓ Copied: %s/README.md → content/docs/widgets/%s.mdx\n" "${name}" "${name}"
  else
    printf "⊘ Skipping: %s (source not found)\n" "${name}"
  fi
done

printf "\n✅ README.md copying complete!\n"
