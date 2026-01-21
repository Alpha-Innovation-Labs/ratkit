---
context_id: TPR_011
title: AI Chat Widget
project: toolkit-primitives
created: "2026-01-21"
---

# TPR_011: AI Chat Widget

## Desired Outcome

A new `AIChat` widget for the ratatui-toolkit that provides an interactive chat interface with an input box for user messages and a message display area. The input supports multi-line text entry via Ctrl+J, file attachments via `@` prefix with fuzzy search in local directory, and command invocation via `/` prefix (starting with `/clear`). User messages appear at the top of the chat area with AI responses displayed below. A loading spinner indicates LLM response generation in progress.

## Next Actions

| Description | Test |
|-------------|------|
| Create `AIChat` struct with `MessageStore` for chat history and `InputState` for input handling | `ai_chat_struct_created` |
| Implement `TextInput` component with multi-line support where Ctrl+J inserts newline | `text_input_multiline` |
| Implement `@` prefix parsing with fuzzy file search in current working directory | `file_attachment_fuzzy_search` |
| Implement `/` prefix parsing with `/clear` command to clear chat history | `command_parser_clear` |
| Render user messages in chat area with distinct styling (e.g., right-aligned or different color) | `user_messages_rendered` |
| Implement loading spinner that appears while awaiting LLM response | `loading_spinner_displayed` |
| Integrate `AIChat` widget into demo application to verify end-to-end functionality | `demo_integration_works` |
