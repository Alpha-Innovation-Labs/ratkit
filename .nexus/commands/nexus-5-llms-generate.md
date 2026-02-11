---
description: Orchestrates subagents to generate a single high-quality llms.txt instruction corpus
agent: general
---

You are an orchestrator. Your job is to generate a single, high-quality `llms.txt` by decomposing the repository into major knowledge domains and delegating domain drafts to subagents.

This command must create one file only: `llms.txt` at repository root.

## Core Behavior

1. Inspect the full repository tree first.
2. Infer the best decomposition strategy from actual structure and content.
3. Spawn subagents for each major domain you discovered.
4. Merge subagent outputs into one coherent `llms.txt` instruction corpus.

Do not hardcode domain categories up front. The decomposition must be inferred from the repository itself.

## Objective

Produce an `llms.txt` that helps coding agents:
- understand architecture and boundaries quickly
- find the correct crate/module/docs for a task
- follow repository conventions and constraints
- avoid common implementation mistakes
- generate compile-ready, style-consistent changes

## Non-Negotiable Constraints

1. Generate exactly one artifact: `llms.txt`.
2. Do not create `llms-full.txt` or any sidecar files.
3. Use verified facts only; do not invent paths, crates, commands, features, or conventions.
4. Verify link targets exist before including them.
5. Keep writing concise, technical, and instruction-first.

## Orchestration Workflow

### Phase 1: Global scan and planning

- Scan the repository tree and identify major documentation/code domains.
- Determine the minimum useful number of subagents based on discovered domains.
- Use more than one subagent when the repository has clearly separable major areas.
- Before spawning, produce a short internal plan listing:
  - chosen domain split
  - why this split is appropriate
  - subagent assignments

### Phase 2: Subagent execution

For each domain, spawn a subagent using the Task tool.

Each subagent must:
- read only the files relevant to its assigned domain plus shared root context
- produce domain-specific material for `llms.txt`
- include:
  - key links with one-line usage annotations
  - actionable rules/constraints discovered in that domain
  - common pitfalls specific to that domain
  - usage cards for the domain's primary public entities
  - uncertainties or conflicts that need orchestrator resolution
- avoid writing files directly

### Phase 3: Orchestrator merge and normalization

- Consolidate all subagent drafts into one unified `llms.txt`.
- Resolve overlaps, duplicates, and contradictions.
- Normalize tone and formatting across sections.
- Remove speculative or unverifiable claims.

## Required `llms.txt` Shape

Use this section order:

`# <Project Name>`

`> 1-2 sentence operational summary for coding agents.`

One short paragraph explaining how agents should use this file.

`## Agent Operating Rules`

Numbered, repository-specific, actionable rules.

`## Environment and Version Constraints`

Only verified constraints that materially affect generated code.

`## Quick Task Playbooks`

Short playbooks for the most common repository tasks discovered during scan.
Each playbook must include:
- where to edit
- what related files to update
- what to validate before completion

`## Getting Started`
`## Workspace Overview`
`## <Domain 1>`
`## <Domain 2>`
`...` (domain sections inferred by orchestrator)
`## Usage Cards`
`## API Reference`
`## Common Pitfalls`
`## Optional`

For link-heavy sections, use:
- `[Title](path-or-url)`: what it contains + when to use

## Usage Card Requirements (adaptive, not hardcoded)

Do not assume the project is component-based. Infer the dominant public entity type from the repository and label cards accordingly.

Examples of valid inferred card types:
- Component Usage Cards
- API Endpoint Usage Cards
- CLI Command Usage Cards
- Library Module Usage Cards
- Service Usage Cards
- Data Model Usage Cards

Each card must be concise and include:
- `Use when`: decision boundary for choosing this entity
- `Enable/Install`: feature flag, dependency, binary, or setup requirement (if applicable)
- `Import/Invoke`: canonical import path, command, or entrypoint
- `Minimal flow`: 2-4 steps showing correct usage lifecycle
- `Key APIs`: top methods/functions/options/flags
- `Pitfalls`: 1-2 concrete mistakes to avoid
- `Source`: verified internal path(s), and public docs link if available

Card coverage rules:
- cover all major public entities in each inferred domain
- prioritize high-usage and high-risk entities first
- avoid boilerplate cards for internal-only or trivial entities
- do not include unverifiable API names

## Quality Gate (must pass before writing file)

- exactly one H1 at top
- no broken internal links
- no duplicate entries
- no nonexistent files/directories
- no stale/incorrect counts (workspace crate counts must match current state)
- no unverifiable claims
- no generic filler text
- usage cards reflect inferred entity type and include executable usage guidance

## Finalization

1. Write final content to repository root as `llms.txt`.
2. Return a brief completion note including:
   - detected domain split
   - number of subagents used
   - validations performed

After presenting the result, use the `reporting` tool with:
- input: the final completion note
- sound: /System/Library/Sounds/Glass.aiff
- notificationTitle: "LLMS Generate"
- notificationBody: the first lines of the completion note
