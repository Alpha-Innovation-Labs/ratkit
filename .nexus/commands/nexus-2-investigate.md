---
description: Spawns 5 subagents to investigate a prompt, then summarizes and compares their findings
---

You are an investigator. Your job is to take the user's prompt and spawn 5 separate subagents to explore it thoroughly.

Steps:
1. Spawn 5 independent subagent sessions using the Task tool, all working on the same prompt
2. Use the **Plan** agent type for all subagents
3. **CRITICAL**: Instruct each subagent to NOT write any code or modify any files - they should only think at a high level, analyze, and report their findings back
4. Collect all findings from the 5 subagents

After collecting all findings, analyze and present:
1. A summary of all findings
2. A voting/comparison system showing:
   - What answers and conclusions were consistent or matched between multiple agents
   - What answers and conclusions were contradictory or disagreed upon
   - Confidence levels for each finding based on agreement
3. A final recommendations section grouped by severity:
   - **Critical**
   - **Minor**

For each recommendation, use the standardized question format from `nexus-1.2-context-create.md` Appendix A:

```
**Question [N/TOTAL]**: <problem statement>

**Recommended:** Option [X] - <reasoning>

| Option | Description |
|--------|-------------|
| A | <recommended solution> |
| B | <alternative solution> |
| Short | Provide different answer |

Reply with: option letter, "yes" for recommended, or your own answer.
```

Provide one question per recommendation, grouped under **Critical** and **Minor** headings.

After presenting the findings, use the `reporting` tool with:
- input: the full investigation summary
- sound: /System/Library/Sounds/Glass.aiff
- notificationTitle: "Investigate"
- notificationBody: the first lines of the investigation summary
