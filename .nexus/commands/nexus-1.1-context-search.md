---
description: Search existing context files for relevant outcomes/actions
---

# Command: Search Contexts

You are searching through existing context files to answer the user's query. You look at the "Desired Outcome" and "Next Actions" sections of all `.nexus/context/` files to find relevant information.

## Purpose

Help users find existing contexts that match their query, understand what's already been specified, and identify related work across the project.

## Workflow

### 1. Understand the Query

- Listen to the user's question or search terms
- Identify key concepts, features, or goals they're asking about

### 2. Scan All Context Files

**Read all context files from `.nexus/context/`** (excluding `_legacy/` and `_reference/` folders):

For each context file, extract:
- **context_id** from frontmatter
- **title** from frontmatter
- **project** from frontmatter
- **Desired Outcome** section content
- **Next Actions** table content

### 3. Analyze for Relevance

Score each context by relevance to the user's query:
- **High relevance**: Directly addresses the query topic
- **Medium relevance**: Related or adjacent topic
- **Low relevance**: Mentioned but not central

Consider:
- Keywords in the Desired Outcome
- Keywords in Next Actions descriptions
- Project area (nexus-tui, nexus-server, etc.)

### 4. Present Results

Format your response as:

```
## Search Results for: "<user query>"

Found N relevant context(s):

### 1. PRJ_NNN: Title (High Relevance)
**Project:** project-name
**File:** `.nexus/context/project-name/PRJ_NNN-description.md`

**Desired Outcome:**
<One paragraph summary from the context file>

**Key Next Actions:**
| Description | Test |
|-------------|------|
| <Most relevant action 1> | `<test_name>` |
| <Most relevant action 2> | `<test_name>` |

---

### 2. PRJ_NNN: Title (Medium Relevance)
...
```

**If no relevant contexts found:**
```
No contexts found matching "<query>".

**Suggestions:**
- Try different keywords
- Check `.nexus/context/_reference/` for background information
- Create a new context with `@nexus-create-context`
```

### 5. Provide Insights

After listing results, add a brief analysis:

```
## Summary

**What I found:**
- N contexts related to [topic]
- Primary work appears to be in [project-name]
- [Any patterns or gaps noticed]

**Recommendations:**
- [Suggest next steps based on what exists]
```

## Example Queries and Responses

### Example 1: Feature Search

**User:** "How do we handle authentication?"

**Response:**
```
## Search Results for: "authentication"

Found 2 relevant context(s):

### 1. SRV_002: Authentication System (High Relevance)
**Project:** nexus-server
**File:** `.nexus/context/nexus-server/SRV_002-authentication.md`

**Desired Outcome:**
Users can authenticate with the Nexus server using API keys or JWT tokens. The system validates credentials, manages sessions, and provides secure access to protected resources.

**Key Next Actions:**
| Description | Test |
|-------------|------|
| Implement API key validation middleware | `api_key_validation` |
| Create JWT token generation and verification | `jwt_token_handling` |
| Add session management with expiration | `session_management` |

---

### 2. CLT_001: Client Architecture (Medium Relevance)
**Project:** nexus-client
**File:** `.nexus/context/nexus-client/CLT_001-client-architecture.md`

**Desired Outcome:**
The client library provides a clean API for interacting with the Nexus server, including automatic authentication handling and request retry logic.

**Key Next Actions:**
| Description | Test |
|-------------|------|
| Add authentication header injection | `auth_header_injection` |
| Implement token refresh on 401 responses | `token_refresh` |
```

### Example 2: Component Search

**User:** "What TUI components exist?"

**Response:**
```
## Search Results for: "TUI components"

Found 4 relevant context(s):

### 1. TUI_003: List Component (High Relevance)
...

### 2. TUI_004: Chat Component (High Relevance)
...

### 3. TUI_005: Markdown Viewer (High Relevance)
...

### 4. TUI_006: Modal Component (High Relevance)
...
```

## Search Tips

When searching, consider:
- **Synonyms**: "auth" ↔ "authentication" ↔ "login"
- **Related concepts**: "database" ↔ "storage" ↔ "SQLite"
- **Project prefixes**: TUI_, SRV_, CLI_, etc.
- **Broader/narrower terms**: "component" vs "button" vs "widget"

## Notes

- Always exclude `_legacy/` and `_reference/` folders from main results
- Mention if relevant reference materials exist in `_reference/`
- If multiple contexts in the same project are found, group them together
- Highlight any contexts that might be duplicates or overlapping
