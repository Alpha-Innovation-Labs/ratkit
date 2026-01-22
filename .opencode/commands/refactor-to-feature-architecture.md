# Orchestrator: Feature-Based Architecture Refactoring

## Mission
Refactor a Rust crate from its current modular structure to a clean **feature-based architecture** inspired by Bulletproof React patterns.

## Inputs
- **Source Crate**: The Rust crate to refactor (provided as argument)
- **Current Structure**: The existing modular organization to analyze
- **Domain Context**: What the crate does (provided as argument)

## Phase 0: Structure Design (Consensus First)

Before any refactoring, the orchestrator must **design the target structure** by spawning 3 architect agents in parallel. They must **reach consensus** before proceeding.

### Step 0.1: Spawn 3 Architecture Designers

**Agent A - "Simplicity Advocate"**
- Analyze the current structure
- Propose a minimal feature-based structure (fewest directories)
- Argue for keeping things flat and simple

**Agent B - "Scalability Advocate"**
- Analyze the current structure  
- Propose a structure that scales for growth
- Argue for clear separation of concerns

**Agent C - "Domain-Driven Advocate"**
- Analyze the current structure
- Propose a structure based on domain boundaries
- Argue for feature-based colocation

### Step 0.2: Facilitate Discussion

The orchestrator collects all 3 proposals and facilitates discussion:
```
1. Each agent presents their proposed structure (with directory tree)
2. Each agent identifies strengths of other proposals
3. Each agent identifies concerns with other proposals
4. Agents discuss and iterate toward agreement
```

### Step 0.3: Reach Consensus

The orchestrator guides agents to consensus on:
- **Top-level directories** (what's at the root level)
- **Feature boundaries** (what constitutes a "feature")
- **Colocation rules** (what belongs together)
- **Directory depth limit** (max nesting level)
- **Naming conventions** (state.rs, widget.rs, mod.rs, etc.)

**Consensus Rule**: All 3 agents must agree OR identify specific tradeoffs they're accepting.

### Step 0.4: Output Target Structure

Once consensus is reached, document the agreed structure:

```
[CRATE_NAME]/
├── [TOP_LEVEL_DIR_1]/
│   └── ...
├── [TOP_LEVEL_DIR_2]/
│   └── ...
├── [TOP_LEVEL_DIR_3]/
│   └── ...
├── ...
└── mod.rs

WITH:
- Max depth: N levels
- Feature pattern: [description]
- Colocation rule: [description]
- Naming convention: [description]
```

## Phase 1: Writer Agents

After Phase 0 produces an agreed target structure, spawn **one writer agent per major component**.

### Writer Agent Task

```
Given:
- Target structure from Phase 0
- Your assigned component/domain
- Current source files to refactor

Your Task:
1. Analyze all files in your component's domain
2. Map them to the new structure
3. Create new files following the agreed patterns
4. Consolidate split code (state + methods together, no more type-based splitting)
5. Preserve ALL functionality, public APIs, and behavior
6. Output:
   - List of files created
   - Mapping from old files to new locations
   - Any questions or edge cases encountered
```

### Writer Assignment Strategy

The orchestrator identifies natural boundaries and assigns:
- One writer per feature area
- One writer per core/shared module
- One writer per foundation/primitive module

## Phase 2: Validator Agents (Round 1)

For **each component** from Phase 1, spawn a validator agent.

### Validator Agent Task

```
Given:
- Original source files
- Refactored source files
- Target structure from Phase 0

Your Task:
1. Verify functionality is preserved:
   - All original types exist
   - All original methods exist
   - All public APIs unchanged
2. Verify structure compliance:
   - Files are in correct locations
   - No more than agreed max depth
   - Naming conventions followed
3. Verify code quality:
   - No unused imports
   - No broken references
   - Code compiles (if possible)
4. If issues found, fix them OR report them
5. Output:
   - Validation status (PASS/FAIL)
   - List of issues found
   - List of fixes applied
```

## Phase 2: Validator Agents (Round 2)

Spawn **different validator agents** (not the same as Round 1) for each component.

### Second Validator Agent Task

```
Given:
- Refactored source files (after Round 1 fixes)
- Target structure from Phase 0

Your Task:
1. Independent review - don't trust Round 1
2. Check for issues Round 1 might have missed:
   - Circular dependencies
   - Missing re-exports
   - Style inconsistencies
   - Unused code
   - Broken imports
3. Verify Round 1 fixes were complete
4. Output:
   - Final validation status
   - Any remaining issues
   - Recommendations for Phase 3
```

## Phase 3: Final Confirmation

Spawn one agent to review the **entire refactoring**.

### Final Confirmation Agent Task

```
Given:
- All refactored files
- Target structure from Phase 0
- All validation reports from Phase 2

Your Task:
1. Verify all components are present
2. Verify module exports resolve correctly
3. Verify no files were accidentally deleted
4. Verify the overall structure matches the agreed target
5. Check cross-component imports work
6. Identify any remaining issues
7. Output:
   - FINAL_STATUS: COMPLETE / INCOMPLETE / NEEDS_WORK
   - Summary of refactoring
   - Any remaining issues to address
```

## Orchestrator Responsibilities

### Progress Tracking

| Phase | What | Parallel? |
|-------|------|-----------|
| 0.1 | 3 architecture designers | ✅ Yes |
| 0.2-0.4 | Discussion to consensus | Sequential |
| 1 | Writer agents (1 per component) | ✅ Yes |
| 2.1 | Round 1 validators (1 per component) | ✅ Yes |
| 2.2 | Round 2 validators (1 per component) | ✅ Yes |
| 3 | Final confirmation | N/A |

### Communication Protocol

At each phase boundary, report:

```
=== PHASE [X] COMPLETE ===

Components processed: N
Files created: NN
Files modified: NN
Issues found: NN
Issues resolved: NN

Status: [GREEN/YELLOW/RED]

Next phase: [description]
```

### Error Handling

| Situation | Action |
|-----------|--------|
| Writer agent fails | Retry with new agent |
| Architecture designers can't agree | Orchestrator facilitates trade-off discussion, may need user input |
| Validator finds major issues | Spawn new writer to fix, then re-validate |
| Validator contradictions | Spawn third-party review, may need user input |
| Phase blocked | Report to user, await guidance |

## Refactoring Principles

### What MUST Be Preserved
- ✅ All functionality
- ✅ All public API signatures
- ✅ All behavior (no logic changes)
- ✅ All documentation

### What SHOULD Change
- ✅ Directory structure (feature-based)
- ✅ File organization (colocation by feature)
- ✅ Module indirection (flatten where possible)
- ✅ Naming conventions (consistent patterns)

### What MUST NOT Happen
- ❌ Deleting functionality
- ❌ Changing public APIs
- ❌ Modifying logic (only restructuring)
- ❌ Breaking compilation

## Starting The Orchestrator

The user provides:
```
- Crate path: [path to crate]
- Crate purpose: [what it does]
- Any constraints: [optional preferences]
```

The orchestrator then:
1. ✅ Spawns 3 architecture designers
2. ✅ Facilitates consensus on target structure
3. ✅ Spawns writers for each component
4. ✅ Spawns validators (2 rounds)
5. ✅ Spawns final confirmation
6. ✅ Reports complete status

## Success Criteria

| Criterion | Measure |
|-----------|---------|
| Architecture consensus | All 3 Phase 0 agents agree |
| All functionality preserved | All validators confirm |
| Structure matches target | Final agent verifies |
| No broken code | Code compiles (if checkable) |
| No files lost | All original code accounted for |

## Begin

When given a crate path and purpose, start immediately with Phase 0.
