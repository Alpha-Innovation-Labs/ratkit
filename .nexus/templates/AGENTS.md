# Nexus System

Context-Driven Development: contexts, knowledge, and lessons organized by crate/package.

## Structure
```
.context/tasks/<crate>/
├── AGENTS.md      # Operational knowledge (this file, symlinked to code)
└── CONTEXT_*.md   # Context specifications
```

**Folder = ** Rust crate, Python package, or TypeScript package name.

## Commands
- `nexus` → Select context (fuzzy finder)
- `/nexus-create-context` → New context
- `/nexus-extract-lessons` → Save lessons from conversation
- `/nexus-review-contexts` → Audit contexts

## Context Format
```yaml
---
context_id: CONTEXT_001
project: crate-name
---
# Summary: WHAT and WHY (not HOW)
# Goals: 3 specific outcomes
# File System Diff: Expected changes
# Lessons Learned: Populated after work
# Validation: Commands that must pass
```

## Principles
- Contexts = specifications (describe outcomes, not implementation)
- AGENTS.md = operational tips (deployment, debugging, environment)
- Lessons extracted → preserved for future sessions
- Each crate has own CONTEXT_001, CONTEXT_002, etc.
