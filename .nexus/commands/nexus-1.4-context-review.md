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
- **Proper location**: Must be in `.context/tasks/<project-folder>/CONTEXT_XXX-description.md`

❌ **NOT HAVE:**
- Code snippets or implementation details
- Step-by-step implementation instructions
- More than 3-4 goals (too broad)
- Verbose descriptions (should be concise)
- Mixed concerns (doing multiple unrelated things)
- **Contexts directly in `.context/tasks/` root** (must be in a project subfolder)

## Instructions

**CRITICAL RULES:**
1. **SCOPE SELECTION** - Ask user which folder to review, or review all contexts
2. **SCAN CONTEXTS** - Review context files in specified folder or all of `.context/tasks/`
3. **IDENTIFY ISSUES** - Find violations of context requirements
4. **ONE-BY-ONE RECOMMENDATIONS** - Present each issue individually with recommended fixes
5. **CONTEXT SPLITTING** - Identify contexts that are too broad and suggest splitting them
6. **FINAL SUMMARY** - Show all approved changes for final confirmation before applying

### Workflow:

1. **Select Review Scope**:
   - First, ask the user:
     ```
     **Which contexts do you want to review?**
     
     **Recommended:** Option A - Review all contexts
     
     | Option | Description |
     |--------|-------------|
     | A | All contexts - Review everything in .context/tasks/ |
     | B | Specific folder - I'll specify which project folder |
     | Short | Specify folder name (e.g., "context-core") |
     
     You can reply with: "A" to review all, "B" to specify a folder, or just type the folder name.
     ```
   - Wait for user response
   - If user specifies a folder name, validate it exists in `.context/tasks/`
   - Set the scan path accordingly:
     - All contexts: `.context/tasks/` (recursive)
     - Specific folder: `.context/tasks/<folder-name>/`

2. **Scan Context Files**:
   - Read all context files from the selected scope
   - For each context, check for:
     - **File location violation**: Context files in `.context/tasks/` root instead of a project subfolder
     - **Wrong project folder**: Context's frontmatter `project` field doesn't match the folder it's in
     - **Format violations**: Missing sections, incorrect structure
     - **Code snippets**: Any code blocks in Summary, Goals, or other sections (except File System Diff)
     - **Verbosity**: Overly detailed descriptions, implementation steps
     - **Scope creep**: Too many goals (>4), mixed concerns, unrelated objectives
     - **Invalid content**: How-to instructions, implementation details instead of outcomes
   - Track all issues found across all contexts

2. **Categorize Issues**:
   - Group issues by type:
     - **Critical**: Code snippets, missing required sections
     - **Important**: Scope too broad, format violations
     - **Minor**: Verbose descriptions, too many goals
   - Prioritize critical issues first

3. **Check for Context Splitting Candidates**:
   - Identify contexts that are doing too much:
     - More than 4 goals
     - Goals that are unrelated to each other
     - File System Diff spanning too many unrelated areas
     - Summary trying to describe multiple distinct features
   - For each splitting candidate:
     - Analyze the goals and identify logical groupings
     - Propose how to split into multiple focused contexts
     - Suggest names for the split contexts

4. **Present Issues One-by-One** (Use Standardized Format):
   - For each issue found, present it individually:
     ```
     **Issue [N/TOTAL]**: CONTEXT_XXX-description.md - <Issue Type>
     
     **What I noticed:** <Describe the specific issue found>
     
     **Example from context:**
     ```
     <Show the problematic content>
     ```
     
     **Recommended:** Option A - <Brief description of recommended fix>
     
     | Option | Description |
     |--------|-------------|
     | A | <Recommended fix> |
     | B | Skip - Leave as is |
     | C | Different fix - I'll specify |
     | Short | Provide custom action (≤5 words) |
     
     **Proposed fix:**
     ```
     <Show what the section will look like after the fix>
     ```
     
     You can reply with: "A"/"yes" to accept, "B" to skip, "C" to provide different fix, or your custom action.
     ```
   - Wait for user response for EACH issue
   - Track all approved fixes

5. **Present Context Splitting Recommendations**:
   - If contexts need splitting, present each one:
     ```
     **Recommendation [N/TOTAL]**: Split CONTEXT_XXX - Context Too Broad
     
     **What I noticed:** This context has N goals covering multiple unrelated features: <list them>
     
     **Recommended:** Option A - Split into M focused contexts
     
     | Option | Description |
     |--------|-------------|
     | A | Split into M contexts - <list proposed context names> |
     | B | Keep as one context - Reduce goals instead |
     | C | Different approach - I'll specify |
     | Short | Provide custom action (≤5 words) |
     
     **Proposed split:**
     
     1. **CONTEXT_XXX-1: <Name>**
        - Goals: <list goals>
     
     2. **CONTEXT_XXX-2: <Name>**
        - Goals: <list goals>
     
     You can reply with: "A" to split, "B" to keep as one, or "C" for different approach.
     ```
   - Wait for user response
   - If user approves split, mark for context creation

6. **Final Summary & Confirmation**:
   - After ALL issues have been reviewed, present summary:
     ```
     **Summary of Changes:**
     
     I will make the following changes:
     
     **Files to Update:**
     1. CONTEXT_XXX-description.md
        - Remove code snippets from Summary section
        - Reduce goals from 6 to 3
     
     2. CONTEXT_YYY-other-context.md
        - Fix missing File System Diff section
        - Remove implementation steps
     
     **Contexts to Split:**
     1. CONTEXT_ZZZ-big-context.md → Split into:
        - CONTEXT_ZZZ-1-focused-context-a.md
        - CONTEXT_ZZZ-2-focused-context-b.md
     
     **Contexts to Create:**
     - N new context files from splits
     
     **Proceed with these changes?**
     
     | Option | Description |
     |--------|-------------|
     | A | Yes - Apply all changes |
     | B | No - Cancel all changes |
     | C | Review - Show me specific changes again |
     
     Reply with: "A"/"yes" to proceed, "B"/"no" to cancel, or "C" to review.
     ```

7. **Apply Changes**:
   - If user approves (Option A or "yes"):
     - **For file location violations**:
       - Move context files from `.context/tasks/` root to appropriate project subfolder
       - Use the `project` field from frontmatter to determine destination folder
       - If no `project` field, ask user which folder to move it to
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
       - Or archive original context (ask user preference)
       - Confirm all changes made
   - If user cancels (Option B or "no"):
     - Do not modify any files

8. **Check AGENTS.md Learning Opportunities**:
   - After all context corrections are done, review the lessons learned and changes made
   - For each project folder that had contexts updated, check if there's operational knowledge to add to AGENTS.md
   - Ask the user ONE TIME (not per-context):
     ```
     **AGENTS.md Update Opportunity**
     
     During this review, I worked on contexts in the following project folders:
     - <project-folder-1>
     - <project-folder-2>
     
     **What I noticed:** Based on the lessons learned and validation commands in these contexts, I could add operational knowledge to AGENTS.md files.
     
     **Example additions:**
     - In <project-folder-1>/AGENTS.md: Add "How to run: `just dev check`" (from validation commands)
     - In <project-folder-2>/AGENTS.md: Add lesson about X discovered during context work
     
     **Recommended:** Option A - Review and update AGENTS.md files
     
     | Option | Description |
     |--------|-------------|
     | A | Yes - Help me update AGENTS.md with operational knowledge |
     | B | No - Skip AGENTS.md updates |
     | Short | Custom instruction |
     
     You can reply with: "A"/"yes" to update, "B"/"no" to skip.
     ```
   - If user approves:
     - For each project folder, read the AGENTS.md (or note if missing)
     - Identify operational knowledge from contexts:
       - Common validation commands → "Running & Operations" section
       - Major lessons learned → "Major Lessons" section
       - New features from context summaries → "Major Features" section
     - Present proposed additions ONE-BY-ONE using standard format
     - Apply approved changes
   - If AGENTS.md doesn't exist, offer to create it from template

9. **Completion**:
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
   - Context file in `.context/tasks/` root instead of a project subfolder
   - Must be: `.context/tasks/<project-name>/CONTEXT_XXX-description.md`
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
File location: .context/tasks/CONTEXT_005-user-authentication.md
```

**What I noticed:** Context file is in `.context/tasks/` root instead of a project subfolder

**Fix:**
```
Move to: .context/tasks/nexus-app/CONTEXT_005-user-authentication.md
```

Based on frontmatter `project: nexus-app`, this context should be in the `nexus-app/` folder.

### Example 2: Project Folder Mismatch

**Issue:**
```
File location: .context/tasks/context-core/CONTEXT_003-api-design.md

Frontmatter:
---
project: nexus-app
---
```

**What I noticed:** Context is in `context-core/` folder but frontmatter says `project: nexus-app`

**Fix Options:**
- Move context to `.context/tasks/nexus-app/CONTEXT_003-api-design.md`, OR
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
