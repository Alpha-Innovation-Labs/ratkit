---
project_id: project-name
title: Project Title
created: "YYYY-MM-DD"
status: active  # active, complete, on-hold
dependencies: []  # List of project_ids this depends on
---

<!-- 
SOURCE OF TRUTH: .nexus/rules/context.md

PROJECT INDEX: .context/<project-name>/index.md
This file provides operational knowledge for a project (crate, package, module).

STRUCTURE:
.context/<project-name>/
├── index.md           # This file (project overview + operational knowledge)
└── PRJ_NNN-*.md       # Context specifications for this project

CRITICAL RULES:
- NO code - operational tips only
- Focus on WHAT the project does and HOW to use it
- Include architecture diagrams, CLI usage, debugging guides
- Keep dependencies accurate for build ordering
-->

# Project Title

## Overview

<!-- What does this project do? Who uses it? What problem does it solve? -->

<One paragraph describing the project's purpose and users.>

## Architecture

<!-- ASCII diagram showing components and data flow -->

```
┌─────────────────────────────────────────────────────────────────────┐
│                          Project Name                                │
├─────────────────────────────────────────────────────────────────────┤
│  Component A  ──>  Component B  ──>  Component C                    │
└─────────────────────────────────────────────────────────────────────┘
```

## CLI Usage

<!-- How do users interact with this project? -->

```bash
# Example commands
command subcommand [options]
```

## Key Dependencies

<!-- External crates/packages this project relies on -->

| Crate/Package | Purpose |
|---------------|---------|
| example       | Description of why it's used |

## Environment Variables

<!-- Configuration via environment -->

| Variable | Default | Description |
|----------|---------|-------------|
| `EXAMPLE_VAR` | `default` | What it controls |

## Debugging & Troubleshooting

### Common Issue

- Symptom: What the user sees
- Cause: Why it happens
- Fix: How to resolve it
