---
description: Create new context specifications from user goals
---

# Command: Create Context

You are creating context specification(s) following `.nexus/context/rules/context.md`.

## Workflow

### 1. Understand What The User Wants

- If user already described their goal: acknowledge it and proceed to scanning
- If unclear, use the `question` tool:
  ```json
  {
    "questions": [{
      "question": "What are you trying to accomplish?",
      "header": "User Goal",
      "options": []
    }]
  }
  ```
- Have a brief conversation to understand the desired outcome

### 2. Scan Existing Contexts

Before creating anything, **quickly scan all files in `.nexus/context/`**:

1. **Read all context file Desired Outcomes** - Look for similar desired outcomes
2. **Grep for keywords** - Search for key terms from the user's request
3. **Check Next Actions sections** - See if any existing context already covers this work

**If overlap found:**
- Show the user the existing context
- Use the `question` tool:
  ```json
  {
    "questions": [{
      "question": "I found `PRJ_NNN-name.md` with outcome: '[outcome]'. This seems related. What would you like to do?",
      "header": "Context Overlap",
      "options": [
        {"label": "Update existing", "description": "Update the existing context file"},
        {"label": "Create new", "description": "Create a new context file"},
        {"label": "Explain difference", "description": "Explain the difference between them"}
      ]
    }]
  }
  ```

**If already done:**
- Tell the user: "This appears to already be covered by `PRJ_NNN-name.md`. Would you like to review it instead?"

### 3. Determine If This Needs Multiple Contexts

Apply the core principles:
- **One outcome per context** - If the user's request has multiple distinct outcomes, split them
- **Next action principle** - Each context should be completable in a single session
- **Simplicity** - If it feels complex, split it

If splitting is needed:
- Explain to the user: "This looks like [N] separate outcomes. I'll create [N] context files..."
- List the proposed contexts with their outcomes
- Use the `question` tool:
  ```json
  {
    "questions": [{
      "question": "Does this split make sense? [List the N proposed contexts with outcomes]",
      "header": "Context Split",
      "options": [
        {"label": "Yes, proceed", "description": "Create all N context files as proposed"},
        {"label": "Adjust", "description": "Let me adjust the breakdown"}
      ]
    }]
  }
  ```

### 4. Identify Project and Determine Context ID

**Scan `.nexus/context/` for existing project directories:**
- If adding to an existing project, use that project's prefix
- If creating a new project, ask user for the project name and derive a 3-letter prefix

**Auto-determine Context ID (NEVER ask the user):**
- Scan `.nexus/context/<project>/` for existing `PRJ_NNN-*.md` files
- Find the highest NNN number
- Use the next number for new context(s)

**Maintain logical ordering:**
- If new context logically comes before existing ones, renumber existing files to maintain order
- If there are gaps in numbering, renumber all contexts to be sequential
- Update any cross-references when renumbering
- If existing context names don't match their outcomes well, rename them

### 5. Gather Information

Propose based on the conversation:
- **Desired Outcome**: What success looks like (one paragraph)
- **Next Actions**: Table with Description and Test columns

If clarification is needed, use the `question` tool with appropriate options.

### 6. Final Check

Before creating, use the `question` tool:
```json
{
  "questions": [{
    "question": "Before I create the context(s): Is there anything I'm missing? (e.g., additional constraints, dependencies, edge cases for Next Actions)",
    "header": "Final Check",
    "options": [
      {"label": "No, proceed", "description": "Create the context files as proposed"},
      {"label": "Add details", "description": "Let me add more information"}
    ]
  }]
}
```

### 7. Generate Context File(s)

For each context, create a file following this structure:

```markdown
---
context_id: PRJ_NNN
title: Human-Readable Title
project: project-name
created: "YYYY-MM-DD"
---

<!-- 
SOURCE OF TRUTH: .nexus/context/rules/context.md

FILE NAMING: PRJ_NNN-brief-description.md
- PRJ = 3-letter project prefix (e.g., KNO for knowledge-harvester)
- NNN = zero-padded sequence number
- Example: KNO_001-project-scaffold.md

CRITICAL RULES:
- NO code at all - code belongs in the codebase
- NO implementation details - describe WHAT, not HOW
- Only E2E tests matter - no unit or integration tests
-->

# PRJ_NNN: Title

## Desired Outcome

<One paragraph describing what success looks like when this context is complete>

## Reference

<!-- Optional: Only include if there are diagrams, ASCII art, or links. Remove entire section if empty. -->

## Next Actions

| Description | Test |
|-------------|------|
| Implement `TypeName` for <purpose> | `type_name_implemented` |
| Create `ServiceName` to handle <responsibility> | `service_created` |
| User action results in expected outcome | `action_outcome` |
| Error condition is handled gracefully | `error_handled` |
```

**IMPORTANT FORMAT RULES:**
- Use `## Desired Outcome` (NOT `## Summary`)
- Use `## Next Actions` table format (NOT `## Goals` bullet list)
- Do NOT include `## Lessons Learned` section
- Do NOT include `## Validation` section
- Do NOT include `## E2E Test Scenarios` section (use Next Actions table instead)
- Test column: snake_case without `test_` prefix (prefix added automatically in test files)
- Description column: Start with verbs (Implement, Create, Add, Configure, Require)

### 8. Save Location

- Pattern: `.nexus/context/<project>/PRJ_NNN-brief-description.md`
- Example: `.nexus/context/knowledge-harvester/KNO_001-project-scaffold.md`
- If new project, create the directory first

### 9. Create index.md If Needed

If this is the first context in a new project directory, create `.nexus/context/<project>/index.md` with sections for: Overview, Architecture, CLI Usage, Key Dependencies, Environment Variables, Debugging & Troubleshooting.

### 10. Read Applicable Rules

After creating the context file(s), read any rules in `.nexus/context/rules/` that are relevant to implementation. Common rules:
- `.nexus/context/rules/context.md` - Context file standards
- Any language-specific rules (e.g., `rs.md` for Rust)
- Any tool-specific rules (e.g., `justfiles.md`)

---

## Appendix A: Question Tool Examples

**Example: Goal Discovery**
```json
{
  "questions": [{
    "question": "What are you trying to accomplish?",
    "header": "User Goal",
    "options": []
  }]
}
```

**Example: Context Overlap**
```json
{
  "questions": [{
    "question": "I found an existing context that seems related. What would you like to do?",
    "header": "Context Overlap",
    "options": [
      {"label": "Update existing", "description": "Update the existing context file with new requirements"},
      {"label": "Create new", "description": "Create a new separate context file"},
      {"label": "Explain difference", "description": "Help me understand the difference"}
    ]
  }]
}
```

**Example: Confirmation**
```json
{
  "questions": [{
    "question": "Ready to proceed?",
    "header": "Confirm",
    "options": [
      {"label": "Yes, proceed", "description": "Create the context as proposed"},
      {"label": "Make changes", "description": "Let me adjust something first"}
    ]
  }]
}
```

## Appendix B: Splitting Examples

**User says:** "I want to add user authentication with OAuth and also set up email notifications"

**Response:** "This looks like 2 separate outcomes:
1. **User Authentication** - Users can log in via OAuth
2. **Email Notifications** - System sends email notifications

Use the `question` tool to confirm the split.

**User says:** "Build the entire payment system"

**Response:** "A payment system has multiple outcomes. Let me break this down:
1. **Payment Processing** - Accept payments via Stripe
2. **Invoice Generation** - Generate and store invoices
3. **Refund Handling** - Process refunds and credits

Use the `question` tool to confirm the breakdown.
