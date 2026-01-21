#!/usr/bin/env python3
"""
Script to generate MDX documentation from Rust source code with rustdoc comments.
"""

import os
import re
from pathlib import Path
from typing import List, Dict, Optional

RUST_SOURCE_DIR = Path("crates/ratatui-toolkit/src")
DOCS_OUTPUT_DIR = Path("docs/content/docs")


def clean_rustdoc(doc_string: str) -> str:
    """Clean rustdoc comments by removing //!, /// prefixes."""
    if not doc_string:
        return ""
    lines = []
    for line in doc_string.split("\n"):
        if line.strip().startswith("//!"):
            lines.append(line.lstrip("//! ").strip())
        elif line.strip().startswith("///"):
            lines.append(line.lstrip("/// ").strip())
        else:
            lines.append(line.strip())
    return "\n".join(line for line in lines if line)


def format_rustdoc_to_markdown(doc_text: str) -> str:
    """Format rustdoc text to proper markdown."""
    if not doc_text:
        return ""

    lines = []
    code_block = False

    for line in doc_text.split("\n"):
        stripped = line.strip()

        # Track code blocks
        if stripped.startswith("```"):
            code_block = not code_block
            # Fix rust,no_run -> rust
            if "rust,no_run" in stripped or "rust,ignore" in stripped:
                stripped = stripped.replace("rust,no_run", "rust").replace(
                    "rust,ignore", "rust"
                )
                lines.append(stripped)
                continue
            lines.append(line)
            continue

        # Skip empty lines
        if not stripped:
            lines.append("")
            continue

        # Handle rustdoc headings (#)
        if stripped.startswith("#") and not code_block:
            level = len(stripped) - len(stripped.lstrip("#"))
            # Increase level by 1 for main doc
            lines.append("#" * (level + 1) + " " + stripped.lstrip("#").strip())
        # Handle rustdoc lists (-, *)
        elif stripped.startswith("-") and not code_block:
            lines.append(line)
        elif stripped.startswith("*") and not code_block:
            lines.append("-" + stripped[1:])
        # Handle rustdoc links [name](path)
        elif stripped.startswith("[") and "]" in stripped and "(" in stripped:
            # Keep markdown links as-is
            lines.append(line)
        # Handle raw text
        elif not code_block:
            lines.append(line)
        else:
            lines.append(line)

    return "\n".join(lines)


def extract_from_file(file_path: Path) -> Dict:
    """Extract docs from a single Rust file."""
    content = file_path.read_text()

    result = {"module_doc": "", "structs": [], "impls": [], "functions": []}

    # Extract module-level documentation (//!)
    module_doc_pattern = r"^(//!.*(?:\n//!.*)*)"
    module_matches = re.findall(module_doc_pattern, content, re.MULTILINE)
    if module_matches:
        result["module_doc"] = clean_rustdoc("\n".join(module_matches))

    # Extract struct definitions with their docs
    struct_pattern = r"(?P<docs>(?:///[^\n]*\n\s*)+)\s*pub struct (?P<name>\w+)\s*\{(?P<body>[^}]*)\}"
    struct_matches = re.finditer(struct_pattern, content, re.MULTILINE)

    for match in struct_matches:
        docs = clean_rustdoc(match.group("docs"))
        name = match.group("name")
        body = match.group("body")

        # Parse fields
        fields = []
        for line in body.split("\n"):
            line = line.strip()
            if line and not line.startswith("//"):
                # Parse "field_name: Type," or "field_name: Type"
                field_match = re.match(r"(\w+)\s*:\s*([^,]+)", line)
                if field_match:
                    field_name, field_type = field_match.groups()
                    fields.append({"name": field_name, "type": field_type.strip()})

        result["structs"].append({"name": name, "docs": docs, "fields": fields})

    # Extract impl blocks with methods
    impl_pattern = r"(?P<docs>(?:///[^\n]*\n\s*)*)impl\s+\w+\s*\{(?P<body>[^}]*)\}"
    impl_matches = re.finditer(impl_pattern, content, re.MULTILINE)

    for match in impl_matches:
        body = match.group("body")

        # Extract methods from impl block
        method_pattern = r"(?P<docs>(?:///[^\n]*\n\s*)*)pub\s+(?:async\s+)?fn\s+(?P<name>\w+)\s*\((?P<params>[^)]*)\)\s*(?P<return>->\s*[^{;]*)?"

        for method_match in re.finditer(method_pattern, body, re.MULTILINE):
            method_docs = clean_rustdoc(method_match.group("docs"))
            method_name = method_match.group("name")
            params = method_match.group("params")
            return_type = method_match.group("return") or "-> ()"

            result["impls"].append(
                {
                    "name": method_name,
                    "docs": method_docs,
                    "params": params.strip(),
                    "return": return_type.strip(),
                }
            )

    # Extract standalone pub functions
    func_pattern = r"(?P<docs>(?:///[^\n]*\n\s*)*)pub\s+(?:async\s+)?fn\s+(?P<name>\w+)\s*\((?P<params>[^)]*)\)\s*(?P<return>->\s*[^{;]*)?"
    func_matches = re.finditer(func_pattern, content, re.MULTILINE)

    for match in func_matches:
        func_docs = clean_rustdoc(match.group("docs"))
        func_name = match.group("name")
        params = match.group("params")
        return_type = match.group("return") or "-> ()"

        result["functions"].append(
            {
                "name": func_name,
                "docs": func_docs,
                "params": params.strip(),
                "return": return_type.strip(),
            }
        )

    return result


def scan_component(component_dir: Path) -> Dict:
    """Scan a component directory and extract all documentation."""
    # Collect all data from all Rust files in the directory tree
    combined_data = {"module_doc": "", "structs": [], "impls": [], "functions": []}

    # First get the main mod.rs
    main_mod = component_dir / "mod.rs"
    if main_mod.exists():
        data = extract_from_file(main_mod)
        combined_data["module_doc"] = data["module_doc"]
        combined_data["structs"].extend(data["structs"])

    # Then scan all .rs files in the directory tree
    for rust_file in component_dir.rglob("*.rs"):
        if rust_file.name == "mod.rs":
            continue

        data = extract_from_file(rust_file)
        combined_data["impls"].extend(data["impls"])
        combined_data["functions"].extend(data["functions"])

    return combined_data


def generate_mdx_from_rustdata(name: str, data: Dict, category: str) -> str:
    """Generate MDX content from extracted Rust data."""
    # Format module doc properly
    formatted_doc = format_rustdoc_to_markdown(data["module_doc"])

    mdx = f"""---
title: {name}
description: Auto-generated documentation from rustdoc
---

# {name}

{formatted_doc}

"""

    # Add struct documentation
    for struct in data["structs"]:
        mdx += f"""## Struct: {struct["name"]}

{struct["docs"]}

### Fields

| Field | Type | Description |
|-------|------|-------------|
"""
        for field in struct["fields"]:
            mdx += f"| `{field['name']}` | `{field['type']}` | |\n"

        mdx += "\n"

    # Add constructor functions
    constructors = [
        f
        for f in data["functions"]
        if any(c in f["name"].lower() for c in ["new", "create", "from"])
    ]
    if constructors:
        mdx += "## Constructors\n\n"
        for func in constructors:
            if func["docs"]:
                mdx += f"""### `{func["name"]}`

{func["docs"]}

**Signature:**
```rust
fn {func["name"]}({func["params"]}) {func["return"]}
```

"""

    # Add other public functions
    other_funcs = [f for f in data["functions"] if f not in constructors]
    if other_funcs:
        mdx += "## Functions\n\n"
        for func in other_funcs:
            if func["docs"]:
                mdx += f"""### `{func["name"]}`

{func["docs"]}

**Signature:**
```rust
fn {func["name"]}({func["params"]}) {func["return"]}
```

"""

    # Add method documentation
    if data["impls"]:
        mdx += "## Methods\n\n"
        for method in data["impls"]:
            if method["docs"]:
                mdx += f"""### `{method["name"]}`

{method["docs"]}

**Signature:**
```rust
fn {method["name"]}({method["params"]}) {method["return"]}
```

"""

    return mdx


def scan_directory(category: str, source_dir: Path, output_dir: Path):
    """Scan a directory for component definitions and generate docs."""
    cat_dir = source_dir / category
    if not cat_dir.exists():
        print(f"  {category}/ - directory not found")
        return

    output_cat_dir = output_dir / category
    output_cat_dir.mkdir(exist_ok=True)

    # Find all mod.rs files
    for item in cat_dir.iterdir():
        if item.is_dir() and (item / "mod.rs").exists():
            name = item.name.replace("_", "-")
            print(f"  Processing {category}/{name}...")

            data = scan_component(item)
            mdx_content = generate_mdx_from_rustdata(name.title(), data, category)

            output_file = output_cat_dir / f"{name}.mdx"
            output_file.write_text(mdx_content)
            print(
                f"    Generated: {output_file} ({len(data['impls'])} methods, {len(data['functions'])} functions)"
            )


def main():
    """Main entry point."""
    print("Generating documentation from Rust source...")

    # Generate index files
    (DOCS_OUTPUT_DIR / "primitives" / "index.mdx").write_text("""---
title: Primitives
description: Basic UI building blocks
---

# Primitives

Basic UI building blocks that form the foundation of terminal UI interfaces.
""")

    (DOCS_OUTPUT_DIR / "services" / "index.mdx").write_text("""---
title: Services
description: Utility services and infrastructure
---

# Services

Utility services and infrastructure components.
""")

    (DOCS_OUTPUT_DIR / "widgets" / "index.mdx").write_text("""---
title: Widgets
description: Complex interactive components for advanced terminal UI layouts.
---

# Widgets

Complex interactive components for advanced terminal UI layouts.
""")

    # Process each category
    scan_directory("primitives", RUST_SOURCE_DIR, DOCS_OUTPUT_DIR)
    scan_directory("services", RUST_SOURCE_DIR, DOCS_OUTPUT_DIR)
    scan_directory("widgets", RUST_SOURCE_DIR, DOCS_OUTPUT_DIR)

    print("\n✓ Documentation generation complete!")


if __name__ == "__main__":
    main()
