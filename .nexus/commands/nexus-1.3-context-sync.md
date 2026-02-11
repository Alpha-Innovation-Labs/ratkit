---
description: Syncs Next Actions between tests and context files
agent: plan
---

# Sync Context Command

## Description

Analyzes the current conversation to identify what Next Actions (tests) were implemented or changed, then syncs them to the relevant context file(s) in `.nexus/context/`. This ensures the context file's "Next Actions" table stays in sync with actual test implementations.

## Usage

```bash
/sync-context
```

No parameters needed - the command analyzes the current conversation automatically.

## What This Command Does

1. **Analyzes the conversation** to identify:
   - Which context ID(s) were worked on (e.g., CLI_010, TUI_011)
   - What tests were written, modified, or discussed
   - New Next Actions that were implemented

2. **Finds the relevant context file(s)** in `.nexus/context/`

3. **Compares** the documented Next Actions with what was actually implemented

4. **Updates** the context file's "## Next Actions" table with:
   - Missing actions that were implemented
   - Updated wording for actions that changed

## Workflow

### Phase 1: Conversation Analysis

1. **Identify context IDs** mentioned in the conversation:
   - Look for patterns like "CLI_010", "TUI_011", etc.
   - Check test file names (e.g., `cli_010_context_chat/` → CLI_010)
   - Check context file references (e.g., `.nexus/context/nexus-cli/CLI_010-*.md`)

2. **Identify Next Actions** that were implemented:
   - Look for test functions that were written or modified
   - Look for test descriptions or scenario descriptions discussed
   - Note any new test cases added during the conversation

### Phase 2: Context File Discovery

1. **Find matching context files**:
   ```
   .nexus/context/**/<context_id>*.md
   ```
   For example: CLI_010 → `.nexus/context/nexus-cli/CLI_010-context-chat.md`

2. **Read the context file** and extract:
   - Current "## Next Actions" table
   - List of documented actions (Description + Test columns)

### Phase 3: Action Comparison

1. **Compare documented vs implemented**:
   - Read the test directory (e.g., `crates/nexus-cli/tests/cli_010_*/`)
   - Extract test function names from `test_*.rs` files
   - Compare with actions listed in context file's Next Actions table

2. **Identify gaps**:
   - Actions implemented but not documented
   - Actions with outdated wording
   - (Do NOT remove actions - they might be pending implementation)

### Phase 4: Present Changes for Approval

For each change identified, use the `question` tool:

```json
{
  "questions": [{
    "question": "Change [N/TOTAL]: Update <context-file>.md - Next Actions\n\nWhat I noticed: During this conversation, a new test `<test_name>` was added that verifies <description>.\n\nProposed addition:\nDescription: <action description>\nTest: `<test_name>`",
    "header": "Sync Context",
    "options": [
      {"label": "Add to context", "description": "Add this action to the context file"},
      {"label": "Skip", "description": "This action is already covered or not needed"},
      {"label": "Modify", "description": "Let me provide different wording"}
    ]
  }]
}
```

Wait for user approval for each change using the `question` tool.

### Phase 5: Apply Changes

After all approvals collected:

1. **Show final summary** using the `question` tool:
   ```json
   {
     "questions": [{
       "question": "Summary of Changes:\n\nI will update the following Next Actions:\n\n1. **<context-file>.md**\n   - Add: <action description> | `<test_name>`\n\nProceed with these updates?",
       "header": "Confirm Updates",
       "options": [
         {"label": "Yes, apply all", "description": "Apply all approved changes"},
         {"label": "No, cancel", "description": "Cancel all changes"},
         {"label": "Review", "description": "Show me specific changes again"}
       ]
     }]
   }
   ```

2. **Apply approved changes**:
   - Read the context file
   - Locate "## Next Actions" table
   - Add new rows in the same table format
   - Preserve existing rows (don't remove any)

## Next Actions Table Format

Per `.nexus/context/rules/context.md`, actions use table format:

```markdown
## Next Actions

| Description | Test |
|-------------|------|
| User can create new item | `create_item` |
| System validates input on submit | `validate_input` |
| Error displays when API fails | `api_error_displayed` |
```

Each row should:
- Have a Description (human-readable, starts with verb)
- Have a Test name (snake_case, without `test_` prefix)
- Description uses present tense ("User can...", "Agent creates...", "System displays...")
- Test column uses backticks around the test name

## Example Execution

```markdown
# After working on CLI_010 and adding a new test

/sync-context

# Output:
1. Analyzing conversation...
   - Found context: CLI_010
   - Found new test: test_context_chat_scans_existing_context_files
   
2. Finding context file...
   - Found: .nexus/context/nexus-cli/CLI_010-context-chat.md
   
3. Comparing actions...
   - Context has 10 documented actions
   - Test file has 16 tests
   - New action to add: context file scanning
   
4. Presenting change for approval...
   [Uses question tool to show proposal]
   
5. User approves...

6. Updating context file...
   Added row to CLI_010-context-chat.md:
   | Agent scans .nexus/context/ for existing files | `context_chat_scans_existing_context_files` |
```

## Important Notes

- **Non-destructive**: Never removes existing actions
- **One-by-one approval**: Each change is presented individually using the `question` tool
- **Final confirmation**: All changes summarized before applying
- **Preserves format**: Matches existing table format
- **Test correlation**: `crates/<project>/tests/<context_id>_*/` → `.nexus/context/<project>/<CONTEXT_ID>-*.md`
- **Standard compliance**: Follows `.nexus/context/rules/context.md` format

## Related Commands

- `/cdd-review-contexts` - Audit context files for quality issues
- `/cdd-create-context` - Create a new context file
