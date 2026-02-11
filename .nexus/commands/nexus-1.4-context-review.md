---
description: Review context files for standards compliance
---

# Command: Review Contexts

You are reviewing existing context files to ensure they follow proper requirements, identify issues, and suggest improvements.

## Purpose

Context files can degrade over time - they might contain code snippets, become too verbose, try to do too much, or violate the context specification format. This command audits context files and recommends corrections.

## Context File Requirements

A proper context file should:

✅ **HAVE:**
- YAML frontmatter with context_id, title, project, created date
- Summary (1 paragraph, describes WHAT and WHY)
- Goals (3 specific, measurable outcomes)
- File System Diff (tree structure showing expected changes)
- Lessons Learned section (can be empty or populated)
- Validation section (commands that must succeed)
- **Proper location**: Must be in `.nexus/context/tasks/<project-folder>/CONTEXT_XXX-description.md`

❌ **NOT HAVE:**
- Code snippets or implementation details
- Step-by-step implementation instructions
- More than 3-4 goals (too broad)
- Verbose descriptions (should be concise)
- Mixed concerns (doing multiple unrelated things)
- **Contexts directly in `.nexus/context/tasks/` root** (must be in a project subfolder)

## Instructions

**CRITICAL RULES:**
1. **SCOPE SELECTION** - Ask user which folder to review, or review all contexts
2. **SCAN CONTEXTS** - Review context files in specified folder or all of `.nexus/context/tasks/`
3. **IDENTIFY ISSUES** - Find violations of context requirements
4. **ONE-BY-ONE RECOMMENDATIONS** - Present each issue individually with recommended fixes
5. **CONTEXT SPLITTING** - Identify contexts that are too broad and suggest splitting them
6. **FINAL SUMMARY** - Show all approved changes for final confirmation before applying

### Workflow:

1. **Select Review Scope**:
   - Use the `question` tool:
     ```json
     {
       "questions": [{
         "question": "Which contexts do you want to review?",
         "header": "Review Scope",
         "options": [
           {"label": "All contexts", "description": "Review everything in .nexus/context/tasks/"},
           {"label": "Specific folder", "description": "Review a specific project folder"}
         ]
       }]
     }
     ```
   - If user selects "Specific folder", use the `question` tool again:
     ```json
     {
       "questions": [{
         "question": "Which project folder would you like to review?",
         "header": "Select Folder",
         "options": []
       }]
     }
     ```
   - Validate the folder exists in `.nexus/context/tasks/`
   - Set the scan path accordingly:
     - All contexts: `.nexus/context/tasks/` (recursive)
     - Specific folder: `.nexus/context/tasks/<folder-name>/`

2. **Scan Context Files**:
   - Read all context files from the selected scope
   - For each context, check for:
     - **File location violation**: Context files in `.nexus/context/tasks/` root instead of a project subfolder
     - **Wrong project folder**: Context's frontmatter `project` field doesn't match the folder it's in
     - **Format violations**: Missing sections, incorrect structure
     - **Code snippets**: Any code blocks in Summary, Goals, or other sections (except File System Diff)
     - **Verbosity**: Overly detailed descriptions, implementation steps
     - **Scope creep**: Too many goals (>4), mixed concerns, unrelated objectives
     - **Invalid content**: How-to instructions, implementation details instead of outcomes
   - Track all issues found across all contexts

3. **Categorize Issues**:
   - Group issues by type:
     - **Critical**: Code snippets, missing required sections
     - **Important**: Scope too broad, format violations
     - **Minor**: Verbose descriptions, too many goals
   - Prioritize critical issues first

4. **Check for Context Splitting Candidates**:
   - Identify contexts that are doing too much:
     - More than 4 goals
     - Goals that are unrelated to each other
     - File System Diff spanning too many unrelated areas
     - Summary trying to describe multiple distinct features
   - For each splitting candidate:
     - Analyze the goals and identify logical groupings
     - Propose how to split into multiple focused contexts
     - Suggest names for the split contexts

5. **Present Issues One-by-One** using the `question` tool:
   - For each issue found, present it individually:
     ```json
     {
       "questions": [{
         "question": "Issue [N/TOTAL]: <context-file> - <Issue Type>\n\nWhat I noticed: <Describe the specific issue found>\n\nExample from context:\n```\n<Show the problematic content>\n```\n\nProposed fix:\n```\n<Show what the section will look like after the fix>\n```",
         "header": "Context Issue",
         "options": [
           {"label": "Apply fix", "description": "Apply the recommended fix"},
           {"label": "Skip", "description": "Leave as is"},
           {"label": "Different fix", "description": "I'll specify a different fix"}
         ]
       }]
     }
     ```
   - Wait for user response for EACH issue using the `question` tool
   - Track all approved fixes

6. **Present Context Splitting Recommendations** using the `question` tool:
   - If contexts need splitting, present each one:
     ```json
     {
       "questions": [{
         "question": "Recommendation [N/TOTAL]: Split <context-file> - Context Too Broad\n\nWhat I noticed: This context has N goals covering multiple unrelated features: <list them>\n\nProposed split:\n\n1. **CONTEXT_XXX-1: <Name>**\n   - Goals: <list goals>\n\n2. **CONTEXT_XXX-2: <Name>**\n   - Goals: <list goals>",
         "header": "Split Context",
         "options": [
           {"label": "Split contexts", "description": "Split into M focused contexts"},
           {"label": "Keep as one", "description": "Reduce goals instead"},
           {"label": "Different approach", "description": "I'll specify"}
         ]
       }]
     }
     ```
   - Wait for user response using the `question` tool
   - If user approves split, mark for context creation

7. **Final Summary & Confirmation** using the `question` tool:
   - After ALL issues have been reviewed, present summary:
     ```json
     {
       "questions": [{
         "question": "Summary of Changes:\n\nI will make the following changes:\n\n**Files to Update:**\n1. CONTEXT_XXX-description.md\n   - Remove code snippets from Summary section\n   - Reduce goals from 6 to 3\n\n**Contexts to Split:**\n1. CONTEXT_ZZZ-big-context.md → Split into N contexts\n\n**Contexts to Create:**\n- N new context files from splits\n\nProceed with these changes?",
         "header": "Confirm Changes",
         "options": [
           {"label": "Yes, apply all", "description": "Apply all changes"},
           {"label": "No, cancel", "description": "Cancel all changes"},
           {"label": "Review", "description": "Show me specific changes again"}
         ]
       }]
     }
     ```

8. **Apply Changes**:
   - If user approves:
     - **For file location violations**:
       - Move context files from `.nexus/context/tasks/` root to appropriate project subfolder
       - Use the `project` field from frontmatter to determine destination folder
       - If no `project` field, use the `question` tool to ask which folder to move it to
       - Create project folder if it doesn't exist
     - **For file updates**:
       - Read each context file
       - Apply approved corrections
       - Maintain proper formatting
       - Save updated file
     - **For context splits**:
       - Create new context files with auto-incremented IDs in the same project folder
       - Distribute goals/content appropriately
       - Update original context with reference to split contexts
       - Or archive original context (use `question` tool to ask user preference)
       - Confirm all changes made
   - If user cancels:
     - Do not modify any files

9. **Check AGENTS.md Learning Opportunities**:
   - After all context corrections are done, review the lessons learned and changes made
   - For each project folder that had contexts updated, check if there's operational knowledge to add to AGENTS.md
   - Ask the user ONE TIME (not per-context) using the `question` tool:
     ```json
     {
       "questions": [{
         "question": "AGENTS.md Update Opportunity\n\nDuring this review, I worked on contexts in the following project folders:\n- <project-folder-1>\n- <project-folder-2>\n\nBased on the lessons learned and validation commands in these contexts, I could add operational knowledge to AGENTS.md files.\n\nWould you like to update AGENTS.md?",
         "header": "Update AGENTS.md",
         "options": [
           {"label": "Yes, update", "description": "Help me update AGENTS.md with operational knowledge"},
           {"label": "No, skip", "description": "Skip AGENTS.md updates"}
         ]
       }]
     }
     ```
   - If user approves:
     - For each project folder, read the AGENTS.md (or note if missing)
     - Identify operational knowledge from contexts:
       - Common validation commands → "Running & Operations" section
       - Major lessons learned → "Major Lessons" section
       - New features from context summaries → "Major Features" section
     - Present proposed additions ONE-BY-ONE using the `question` tool
     - Apply approved changes
   - If AGENTS.md doesn't exist, offer to create it from template

10. **Completion**:
    - Show summary of what was done:
      ```
      ✅ Context review complete:
      
      **Updated:**
      - N context files corrected
      - M issues fixed
      - X AGENTS.md files updated with operational knowledge
      
      **Created:**
      - Y new focused context files from splits
      
      **Summary:**
      All contexts now comply with context specification requirements.
      Next: Run 'nexus' to see updated context list.
      ```

---

## Appendix A: Issue Detection Rules

### Critical Issues (Must Fix)

1. **File Location Violations**
   - Context file in `.nexus/context/tasks/` root instead of a project subfolder
   - Must be: `.nexus/context/tasks/<project-name>/CONTEXT_XXX-description.md`
   - **Action**: Move to appropriate project folder based on frontmatter `project` field

2. **Project Folder Mismatch**
   - Context's frontmatter `project` field doesn't match the folder it's in
   - Example: Context in `context-core/` but frontmatter says `project: different-project`
   - **Action**: Either move context to correct folder or update frontmatter

3. **Code Snippets in Wrong Sections**
   - Code blocks in Summary, Goals, or Lessons Learned
   - Implementation details instead of outcomes
   - **Exception**: File System Diff should have code-like tree structure

4. **Missing Required Sections**
   - No Summary
   - No Goals
   - No Validation
   - No File System Diff

5. **Invalid Format**
   - Missing or incorrect YAML frontmatter
   - Incorrect file naming (not CONTEXT_XXX-description.md)

### Important Issues (Should Fix)

1. **Scope Too Broad**
   - More than 4 goals
   - Goals that are unrelated to each other
   - Trying to accomplish multiple distinct features

2. **Implementation Instead of Specification**
   - Step-by-step instructions ("First do X, then Y, then Z")
   - "How-to" content instead of "what" and "why"
   - Detailed technical implementation instead of outcomes

3. **Format Violations**
   - Goals not in bullet list format
   - Validation commands not in code block or bullet list
   - File System Diff not showing tree structure

### Minor Issues (Nice to Fix)

1. **Verbosity**
   - Summary longer than 2-3 sentences
   - Goals with too much detail
   - Overly detailed descriptions

2. **Too Many Goals**
   - 4-5 goals (consider reducing or splitting)
   - Redundant goals

3. **Unclear Validation**
   - Vague validation commands
   - Missing common validations (just dev, just test)

---

## Appendix B: Context Splitting Guidelines

### When to Split a Context

Split if:
- **6+ goals** - Too many objectives
- **Unrelated goals** - Goals cover different features/areas
- **Mixed concerns** - Frontend + Backend + Database in one context
- **File System Diff too large** - Changes span many unrelated directories
- **Summary describes multiple features** - Can't be summarized in one clear sentence

### How to Split

1. **Identify logical groupings**:
   - Group related goals together
   - Separate by layer (frontend, backend, database)
   - Separate by feature (authentication, authorization, session management)

2. **Propose focused contexts**:
   - Each split context should have 2-3 goals
   - Each should have clear, single focus
   - Each should be independently implementable (when possible)

3. **Handle dependencies**:
   - If splits have dependencies, note them in the context
   - Recommend implementation order
   - Add dependencies to Validation section

### Naming Split Contexts

- Original: `CONTEXT_005-user-management-system.md`
- Split 1: `CONTEXT_005-user-authentication.md`
- Split 2: `CONTEXT_006-user-authorization.md`
- Split 3: `CONTEXT_007-user-session-management.md`

---

## Appendix C: Example Issue Detection

### Example 1: File Location Violation

**Issue:**
```
File location: .nexus/context/tasks/CONTEXT_005-user-authentication.md
```

**What I noticed:** Context file is in `.nexus/context/tasks/` root instead of a project subfolder

**Fix:**
```
Move to: .nexus/context/tasks/nexus-app/CONTEXT_005-user-authentication.md
```

Based on frontmatter `project: nexus-app`, this context should be in the `nexus-app/` folder.

### Example 2: Project Folder Mismatch

**Issue:**
```
File location: .nexus/context/tasks/context-core/CONTEXT_003-api-design.md

Frontmatter:
---
project: nexus-app
---
```

**What I noticed:** Context is in `context-core/` folder but frontmatter says `project: nexus-app`

**Fix Options:**
- Move context to `.nexus/context/tasks/nexus-app/CONTEXT_003-api-design.md`, OR
- Update frontmatter to `project: context-core`

### Example 3: Code Snippet in Summary

**Issue:**
```markdown
## Summary

This context implements user authentication using JWT tokens:

\`\`\`rust
pub struct User {
    id: Uuid,
    email: String,
}
\`\`\`
```

**Fix:**
```markdown
## Summary

This context implements JWT-based user authentication to secure API endpoints and manage user sessions.
```

### Example 4: Context Too Broad

**Issue:**
```markdown
## Goals

- Implement user authentication with JWT
- Add user registration flow
- Create admin dashboard
- Set up email notifications
- Implement password reset
- Add OAuth integration
```

**Fix:** Split into 3 contexts:
- CONTEXT_XXX-user-authentication.md (JWT + login)
- CONTEXT_YYY-user-registration.md (registration + email + password reset)
- CONTEXT_ZZZ-admin-dashboard.md (admin UI + OAuth)

### Example 5: Implementation Steps Instead of Outcomes

**Issue:**
```markdown
## Goals

- First, create the database schema
- Then, implement the API endpoints
- After that, add frontend components
- Finally, write tests
```

**Fix:**
```markdown
## Goals

- User authentication system with JWT token management
- Secure API endpoints requiring authentication
- Frontend login/logout interface with session persistence
```

---

## Best Practices

- Review ALL contexts, not just recent ones
- Prioritize critical issues over minor ones
- For context splits, suggest sensible groupings based on actual goals
- Show examples of problematic content when presenting issues
- Always show proposed fix so user knows exactly what will change
- Be conservative with splits - only suggest when context is clearly too broad
- Allow user to skip fixes if they disagree
- Collect all approvals before making ANY changes to files
