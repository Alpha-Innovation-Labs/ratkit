---
description: Orchestrates subagents to generate a single high-quality llms.txt instruction corpus
---

You are the primary execution agent for this command and you must orchestrate the workflow yourself. Your job is to generate a single, high-quality `llms.txt` by decomposing the repository into major knowledge domains and delegating domain drafts to subagents.

This command must always produce `llms.txt` at repository root. It may also maintain a generation state file used for incremental updates.

## Core Behavior

1. Determine generation mode first: full generation or incremental refresh.
2. In full mode, inspect the full repository tree and infer the best decomposition strategy.
3. In incremental mode, compute changes since last successful generation and scope work to impacted sections/entities.
4. Spawn subagents for each impacted major domain.
5. Merge subagent outputs into one coherent `llms.txt` instruction corpus.

Do not spawn a separate orchestration agent. You (the command agent) perform orchestration directly, including scan, planning, delegation, merge, validation, and final write.

Do not hardcode domain categories up front. The decomposition must be inferred from the repository itself.

## Objective

Produce an `llms.txt` that helps coding agents:
- understand architecture and boundaries quickly
- find the correct crate/module/docs for a task
- follow repository conventions and constraints
- avoid common implementation mistakes
- generate compile-ready, style-consistent changes

## Non-Negotiable Constraints

1. Generate exactly one documentation artifact: `llms.txt`.
2. Do not create `llms-full.txt`.
3. A state file is allowed only for generation bookkeeping: `.nexus/llms-state.json`.
4. Use verified facts only; do not invent paths, crates, commands, features, or conventions.
5. Verify link targets exist before including them.
6. Keep writing concise, technical, and instruction-first.
7. Use committed history only as input for generation decisions.
8. Never use unstaged, staged-but-uncommitted, or untracked working-tree changes to drive updates.

## Orchestration Workflow

Before spawning any subagent, read and treat `.nexus/rules/llms-txt.md` as the source of truth for generation and validation rules.

### Phase 0: Mode selection and baseline

- Run a working-tree cleanliness check first (`git status --porcelain`).
- If any uncommitted changes exist, stop immediately and return a blocking message: generation is commit-based only and requires a clean working tree.
- Check if `llms.txt` exists.
- Check if `.nexus/llms-state.json` exists and has `last_generated_commit`.
- If either is missing, run full mode.
- If both exist, run incremental mode using git diff from `last_generated_commit` to current `HEAD`.
- In incremental mode, source changes only from committed history (`last_generated_commit..HEAD`), never from working tree.

State file schema:

```json
{
  "llms_path": "llms.txt",
  "last_generated_commit": "<sha>",
  "generated_at": "<ISO-8601>"
}
```

### Phase 1A: Full mode scan and planning

- Scan the repository tree and identify major documentation/code domains.
- Determine the minimum useful number of subagents based on discovered domains.
- Use more than one subagent when the repository has clearly separable major areas.
- Before spawning, produce a short internal plan listing:
  - chosen domain split
  - why this split is appropriate
  - subagent assignments

### Phase 1B: Incremental mode impact analysis

- Compute changed files with git since `last_generated_commit`.
- Include adds, edits, renames, and deletions in impact analysis.
- Map changed paths to impacted `llms.txt` sections and usage cards.
- Determine subagent assignments only for impacted domains.
- If structural files changed (for example workspace manifest, root docs topology, or broad feature map), escalate to full mode.
- If impact is broad enough that partial refresh risks inconsistency, escalate to full mode.

### Phase 2: Subagent execution

For each domain (full mode) or impacted domain (incremental mode), spawn a subagent using the Task tool.

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

### Phase 3: Draft assembly agent

- The orchestrator must spawn one dedicated assembly subagent.
- Provide this assembly subagent with:
  - all domain/impact outputs from Phase 2
  - mode context (`full` or `incremental`)
  - the current `llms.txt` content (incremental mode)
  - required document shape and usage-card requirements
  - explicit instruction to comply with `.nexus/rules/llms-txt.md`
- Assembly subagent responsibilities:
  - In full mode: consolidate all domain drafts into one unified `llms.txt` draft.
  - In incremental mode: update only impacted sections/cards and preserve untouched sections verbatim.
  - Resolve overlaps, duplicates, and contradictions.
  - Normalize tone and formatting.
  - Remove speculative or unverifiable claims.
  - Return only the proposed final `llms.txt` content plus a short change summary.

### Phase 4: Validator agent

- The orchestrator must spawn one dedicated validator subagent after Phase 3.
- Provide validator input:
  - proposed `llms.txt` content from assembly subagent
  - repository state and relevant source files
  - quality gate and constraints from this command
  - `.nexus/rules/llms-txt.md` as mandatory validation baseline
- Validator subagent responsibilities:
  - first check conformance against `.nexus/rules/llms-txt.md`
  - verify every quality-gate rule
  - verify links, counts, and referenced paths
  - verify usage-card API names against real code symbols
  - verify incremental updates only touched impacted sections unless full-mode escalation occurred
  - return pass/fail with a concrete issue list and required fixes

- If validator returns fail, orchestrator must iterate: re-run assembly with validator feedback, then re-run validator until pass.

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
- all APIs named in usage cards are verified against current codebase symbols
- incremental updates changed only impacted sections unless escalation to full mode occurred
- generation input came from committed history only (clean working tree check passed)

## Finalization

1. Only after validator pass, write final content to repository root as `llms.txt`.
2. Write/update `.nexus/llms-state.json` with current `HEAD` as `last_generated_commit` only after successful validation and write.
3. Return a brief completion note including:
   - mode used (`full` or `incremental`)
   - clean working tree check status
   - baseline commit and current commit
   - changed files considered (incremental mode)
   - detected domain split
   - number of subagents used
   - validations performed

After presenting the result, use the `reporting` tool with:
- input: the final completion note
- sound: /System/Library/Sounds/Glass.aiff
- notificationTitle: "LLMS Generate"
- notificationBody: the first lines of the completion note
