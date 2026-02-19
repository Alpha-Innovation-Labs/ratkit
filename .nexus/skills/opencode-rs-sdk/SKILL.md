# opencode-sdk

> Rust SDK for OpenCode HTTP API with SSE streaming support. Provides ergonomic async client, 15 REST API modules, 40+ event types, and managed server lifecycle.

Use this file to understand the SDK structure, feature flags, and correct API usage patterns. All examples assume async context with tokio runtime.

## Agent Operating Rules

1. **Always check feature flags** before using APIs - `http` and `sse` are enabled by default, `server` and `cli` require explicit enabling.
2. **Unix platforms only** - Windows will fail at compile time with `compile_error!`.
3. **Subscribe before sending** - For streaming workflows, create SSE subscription before sending async prompts to avoid missing early events.
4. **Use convenience methods** - Prefer `Client::run_simple_text()` and `send_text_async_and_wait_for_idle()` for common workflows.
5. **Handle all error variants** - Match on `OpencodeError` variants, using helper methods like `is_not_found()` and `is_validation_error()`.
6. **Drop subscriptions to cancel** - `SseSubscription` and `RawSseSubscription` cancel on drop; explicitly call `.close()` for early termination.
7. **Server processes auto-kill** - `ManagedServer` kills child process on drop; call `.stop()` for graceful shutdown.

## Environment and Version Constraints

| Constraint | Value | Impact |
|------------|-------|--------|
| Platform | Unix only (Linux/macOS) | Windows compilation fails |
| Rust Edition | 2024 | Requires Rust 1.85+ |
| Default Features | `http`, `sse` | Always available unless disabled |
| Optional Features | `server`, `cli` | Requires explicit `--features` |
| Full Feature Set | `full` = all features | Use for complete functionality |
| Default Server URL | `http://127.0.0.1:4096` | ClientBuilder default |
| Timeout Default | 300 seconds | Suitable for long AI requests |

## Quick Task Playbooks

### Send a Simple Text Prompt and Wait for Response
```rust
let client = Client::builder().build()?;
let session = client.run_simple_text("Hello, AI!").await?;
let response = client.wait_for_idle_text(&session.id, Duration::from_secs(60)).await?;
```

### Create Session with Custom Title
```rust
let session = client.create_session_with_title("My Coding Task").await?;
```

### Stream Events for a Session
```rust
let mut subscription = client.subscribe_session(&session.id).await?;
while let Some(event) = subscription.recv().await {
    match event {
        Event::MessagePartUpdated { properties } => println!("{}", properties.delta.unwrap_or_default()),
        Event::SessionIdle { .. } => break,
        _ => {}
    }
}
```

### Start Managed Server
```rust
let server = ManagedServer::start(ServerOptions::new().port(8080)).await?;
let client = Client::builder().base_url(server.url()).build()?;
// Server auto-stops when `server` is dropped
```

## Getting Started

Add to `Cargo.toml`:
```toml
[dependencies]
opencode-sdk = "0.1"
# Or with all features:
opencode-sdk = { version = "0.1", features = ["full"] }
```

Basic client setup:
```rust
use opencode_sdk::{Client, ClientBuilder};

let client = Client::builder()
    .base_url("http://127.0.0.1:4096")
    .directory("/path/to/project")
    .timeout_secs(300)
    .build()?;
```

## Workspace Overview

```
src/
├── lib.rs           # Crate root, re-exports, feature gates
├── client.rs        # Client, ClientBuilder - ergonomic API entrypoint
├── error.rs         # OpencodeError, Result<T> - error handling
├── sse.rs           # SseSubscriber, SseSubscription, SessionEventRouter
├── server.rs        # ManagedServer, ServerOptions - server feature
├── cli.rs           # CliRunner, RunOptions, CliEvent - cli feature
├── runtime.rs       # ManagedRuntime - server+http features
├── http/            # HTTP API modules (requires http feature)
│   ├── mod.rs       # HttpClient, HttpConfig
│   ├── sessions.rs  # SessionsApi - 18 endpoints
│   ├── messages.rs  # MessagesApi - 6 endpoints
│   ├── files.rs     # FilesApi
│   ├── tools.rs     # ToolsApi
│   └── ...          # 11 more API modules
└── types/           # Data models
    ├── mod.rs       # Type re-exports
    ├── session.rs   # Session, SessionCreateOptions
    ├── message.rs   # Message, Part, PromptRequest
    ├── event.rs     # Event enum (40 variants)
    └── ...          # 11 more type modules
```

## Core Client API

The `Client` struct ([`src/client.rs`](src/client.rs)) is the primary ergonomic API.

### ClientBuilder

Chain configuration before building:
- `base_url(url)` - Server URL (default: `http://127.0.0.1:4096`)
- `directory(dir)` - Set `x-opencode-directory` header
- `timeout_secs(secs)` - Request timeout (default: 300)
- `build()` - Create the client

### HTTP API Accessor Methods

Each returns an API client for the corresponding endpoint group:
```rust
let sessions = client.sessions();     // SessionsApi
let messages = client.messages();     // MessagesApi
let parts = client.parts();           // PartsApi
let permissions = client.permissions(); // PermissionsApi
let questions = client.questions();   // QuestionsApi
let files = client.files();           // FilesApi
let find = client.find();             // FindApi
let providers = client.providers();   // ProvidersApi
let mcp = client.mcp();               // McpApi
let pty = client.pty();               // PtyApi
let config = client.config();         // ConfigApi
let tools = client.tools();           // ToolsApi
let project = client.project();       // ProjectApi
let worktree = client.worktree();     // WorktreeApi
let misc = client.misc();             // MiscApi
```

### SSE Subscription Methods (requires sse feature)
```rust
let sub = client.subscribe().await?;                    // All events for directory
let sub = client.subscribe_session(id).await?;          // Filtered to session
let sub = client.subscribe_global().await?;             // Global events (all dirs)
let raw = client.subscribe_raw().await?;                // Raw JSON frames
let router = client.session_event_router().await?;      // Get cached router
```

### Convenience Methods
```rust
// Create session and send text (returns immediately, use SSE for response)
let session = client.run_simple_text("Hello").await?;

// Create session with title
let session = client.create_session_with_title("Task").await?;

// Send text asynchronously (empty response, use SSE)
client.send_text_async(&session.id, "Hello", None).await?;

// Subscribe, send, and wait for idle with text collection
let text = client.send_text_async_and_wait_for_idle(&session.id, "Hello", None, Duration::from_secs(60)).await?;

// Wait for idle on existing subscription
let text = client.wait_for_idle_text(&session.id, Duration::from_secs(60)).await?;
```

## HTTP Endpoints

All API modules require the `http` feature (enabled by default).

### SessionsApi ([`src/http/sessions.rs`](src/http/sessions.rs))

| Method | Endpoint | Description |
|--------|----------|-------------|
| `create(req)` | POST /session | Create new session |
| `create_with(opts)` | POST /session | Create with convenience options |
| `get(id)` | GET /session/{id} | Get session by ID |
| `list()` | GET /session | List all sessions |
| `delete(id)` | DELETE /session/{id} | Delete session |
| `fork(id)` | POST /session/{id}/fork | Fork session |
| `update(id, req)` | PATCH /session/{id} | Update session |
| `share(id)` | POST /session/{id}/share | Share session |
| `unshare(id)` | DELETE /session/{id}/share | Unshare session |
| `revert(id, req)` | POST /session/{id}/revert | Revert to message |
| `summarize(id, req)` | POST /session/{id}/summarize | Summarize session |
| `compact(id)` | POST /session/{id}/compact | Compact session |
| `diff(id)` | GET /session/{id}/diff | Get session diff |
| `export(id)` | GET /session/{id}/export | Export session |
| `status()` | GET /status | Get server status |
| `todo(id)` | GET /session/{id}/todo | Get todo items |
| `mark_todo(id, todo_id)` | POST /session/{id}/todo/{todo_id} | Mark todo done |
| `unmark_todo(id, todo_id)` | DELETE /session/{id}/todo/{todo_id} | Unmark todo |

### MessagesApi ([`src/http/messages.rs`](src/http/messages.rs))

| Method | Endpoint | Description |
|--------|----------|-------------|
| `prompt(session_id, req)` | POST /session/{id}/message | Send prompt |
| `prompt_async(session_id, req)` | POST /session/{id}/prompt_async | Async prompt (empty response) |
| `send_text_async(session_id, text, model)` | POST /session/{id}/prompt_async | Convenience text sender |
| `list(session_id)` | GET /session/{id}/message | List messages |
| `get(session_id, message_id)` | GET /session/{id}/message/{mid} | Get message |
| `remove(session_id, message_id)` | DELETE /session/{id}/message/{mid} | Remove message |

### Other API Modules

- **PartsApi** (`src/http/parts.rs`) - Message part CRUD operations
- **PermissionsApi** (`src/http/permissions.rs`) - Permission management
- **QuestionsApi** (`src/http/questions.rs`) - Question operations
- **FilesApi** (`src/http/files.rs`) - File operations
- **FindApi** (`src/http/find.rs`) - Search operations
- **ProvidersApi** (`src/http/providers.rs`) - Provider management
- **McpApi** (`src/http/mcp.rs`) - MCP operations
- **PtyApi** (`src/http/pty.rs`) - PTY operations
- **ConfigApi** (`src/http/config.rs`) - Configuration
- **ToolsApi** (`src/http/tools.rs`) - Tool operations
- **ProjectApi** (`src/http/project.rs`) - Project operations
- **WorktreeApi** (`src/http/worktree.rs`) - Worktree operations
- **MiscApi** (`src/http/misc.rs`) - Miscellaneous endpoints

## Types and Models

### Session Types ([`src/types/session.rs`](src/types/session.rs))

```rust
pub struct Session {
    pub id: String,
    pub project_id: Option<String>,
    pub directory: Option<String>,
    pub parent_id: Option<String>,
    pub summary: Option<SessionSummary>,
    pub share: Option<ShareInfo>,
    pub title: String,
    pub version: String,
    pub time: Option<SessionTime>,
    pub permission: Option<Ruleset>,
    pub revert: Option<RevertInfo>,
}

pub struct SessionCreateOptions { /* builder pattern */ }
pub struct CreateSessionRequest { parent_id, title, permission, directory }
pub struct UpdateSessionRequest { title }
pub struct SummarizeRequest { provider_id, model_id, auto }
pub struct RevertRequest { message_id, part_id }
pub struct SessionStatus { active_session_id, busy }
pub struct SessionDiff { diff, files }
pub struct TodoItem { id, content, completed, priority }
```

### Message Types ([`src/types/message.rs`](src/types/message.rs))

```rust
pub struct Message {
    pub info: MessageInfo,
    pub parts: Vec<Part>,
}

pub struct MessageInfo {
    pub id: String,
    pub session_id: Option<String>,
    pub role: String,  // "user", "assistant", "system"
    pub time: MessageTime,
    pub agent: Option<String>,
    pub variant: Option<String>,
}

pub enum Part {  // 12 variants
    Text { id, text, synthetic, ignored, metadata },
    File { id, mime, url, filename, source },
    Tool { id, call_id, tool, input, state, metadata },
    Reasoning { id, text, metadata },
    StepStart { id, snapshot },
    StepFinish { id, reason, snapshot, cost, tokens },
    Snapshot { id, snapshot },
    Patch { id, hash, files },
    Agent { id, name, source },
    Retry { id, attempt, error },
    Compaction { id, auto },
    Subtask { id, prompt, description, agent, command },
    Unknown,
}

pub enum PromptPart {
    Text { text, synthetic, ignored, metadata },
    File { mime, url, filename },
    Agent { name },
    Subtask { prompt, description, agent, command },
}

pub struct PromptRequest {
    pub parts: Vec<PromptPart>,
    pub message_id: Option<String>,
    pub model: Option<ModelRef>,
    pub agent: Option<String>,
    pub no_reply: Option<bool>,
    pub system: Option<String>,
    pub variant: Option<String>,
}

pub enum ToolState {  // 5 variants
    Pending(ToolStatePending),
    Running(ToolStateRunning),
    Completed(ToolStateCompleted),
    Error(ToolStateError),
    Unknown(serde_json::Value),
}
```

### Event Types ([`src/types/event.rs`](src/types/event.rs))

40 SSE event variants organized by category:

**Server/Instance (4):** `ServerConnected`, `ServerHeartbeat`, `ServerInstanceDisposed`, `GlobalDisposed`

**Session (8):** `SessionCreated`, `SessionUpdated`, `SessionDeleted`, `SessionDiff`, `SessionError`, `SessionCompacted`, `SessionStatus`, `SessionIdle`

**Messages (4):** `MessageUpdated`, `MessageRemoved`, `MessagePartUpdated`, `MessagePartRemoved`

**PTY (4):** `PtyCreated`, `PtyData`, `PtyClosed`, `PtyResized`

**Questions (3):** `QuestionCreated`, `QuestionUpdated`, `QuestionRemoved`

**Permissions (3):** `PermissionRequested`, `PermissionReplied`, `PermissionExecuted`

**Files (2):** `FileChanged`, `FileSynced`

**Worktree (2):** `WorktreeSyncStarted`, `WorktreeSyncCompleted`

**MCP (4):** `MCPServerConnected`, `MCPServerDisconnected`, `MCPError`, `MCPLog`

**Tool (2):** `ToolCallCreated`, `ToolCallUpdated`

**Other (4):** `ConfigUpdated`, `ProviderUpdated`, `FindResultsUpdated`, `Notification`

## SSE Streaming

The SSE module ([`src/sse.rs`](src/sse.rs)) provides robust event streaming with automatic reconnection.

### Key Types

```rust
pub struct SseSubscriber { /* creates subscriptions */ }
pub struct SseSubscription { /* typed event receiver */ }
pub struct RawSseSubscription { /* raw JSON receiver */ }
pub struct SessionEventRouter { /* multiplexes to per-session channels */ }

pub struct SseOptions {
    pub capacity: usize,           // default: 256
    pub initial_interval: Duration, // default: 250ms
    pub max_interval: Duration,     // default: 30s
}

pub struct SessionEventRouterOptions {
    pub upstream: SseOptions,
    pub session_capacity: usize,      // default: 256
    pub subscriber_capacity: usize,   // default: 256
}

pub struct SseStreamStats {
    pub events_in: u64,
    pub events_out: u64,
    pub dropped: u64,
    pub parse_errors: u64,
    pub reconnects: u64,
    pub last_event_id: Option<String>,
}
```

### SseSubscriber Methods

```rust
pub async fn subscribe(&self, opts: SseOptions) -> Result<SseSubscription>;
pub async fn subscribe_typed(&self, opts: SseOptions) -> Result<SseSubscription>;
pub async fn subscribe_global(&self, opts: SseOptions) -> Result<SseSubscription>;
pub async fn subscribe_typed_global(&self, opts: SseOptions) -> Result<SseSubscription>;
pub async fn subscribe_raw(&self, opts: SseOptions) -> Result<RawSseSubscription>;
pub async fn subscribe_session(&self, session_id: &str, opts: SseOptions) -> Result<SseSubscription>;
pub async fn session_event_router(&self, opts: SessionEventRouterOptions) -> Result<SessionEventRouter>;
```

### Subscription Methods

```rust
impl SseSubscription {
    pub async fn recv(&mut self) -> Option<Event>;
    pub fn stats(&self) -> SseStreamStats;
    pub fn close(&self);
}

impl RawSseSubscription {
    pub async fn recv(&mut self) -> Option<RawSseEvent>;
    pub fn stats(&self) -> SseStreamStats;
    pub fn close(&self);
}

impl SessionEventRouter {
    pub async fn subscribe(&self, session_id: &str) -> SseSubscription;
    pub fn stats(&self) -> SseStreamStats;
    pub fn close(&self);
}
```

### Reconnection Behavior

- Exponential backoff starting at 250ms, max 30s
- Jitter applied to prevent thundering herd
- Last-Event-ID header sent for resumption
- No max retry limit (infinite reconnection)
- Backoff resets on successful connection

## Server and CLI Features

### ManagedServer (requires server feature)

Spawn and manage `opencode serve` processes:

```rust
pub struct ServerOptions {
    pub port: Option<u16>,           // None = random port
    pub hostname: String,            // default: "127.0.0.1"
    pub directory: Option<PathBuf>,
    pub config_json: Option<String>, // via OPENCODE_CONFIG_CONTENT
    pub startup_timeout_ms: u64,     // default: 5000
    pub binary: String,              // default: "opencode"
}

pub struct ManagedServer {
    pub async fn start(opts: ServerOptions) -> Result<Self>;
    pub fn url(&self) -> &Url;
    pub fn port(&self) -> u16;
    pub async fn stop(mut self) -> Result<()>;
    pub fn is_running(&mut self) -> bool;
}
```

Server automatically stops on drop via kill signal. Waits for "opencode server listening on" in stdout or falls back to `/doc` probe.

### CliRunner (requires cli feature)

Wrap `opencode run --format json`:

```rust
pub struct RunOptions {
    pub format: Option<String>,      // default: "json"
    pub attach: Option<String>,
    pub continue_session: bool,
    pub session: Option<String>,
    pub file: Vec<String>,
    pub share: bool,
    pub model: Option<String>,
    pub agent: Option<String>,
    pub title: Option<String>,
    pub port: Option<u16>,
    pub command: Option<String>,
    pub directory: Option<PathBuf>,
    pub binary: String,              // default: "opencode"
}

pub struct CliEvent {
    pub r#type: String,
    pub timestamp: Option<i64>,
    pub session_id: Option<String>,
    pub data: serde_json::Value,
}

pub struct CliRunner {
    pub async fn start(prompt: &str, opts: RunOptions) -> Result<Self>;
    pub async fn recv(&mut self) -> Option<CliEvent>;
    pub async fn collect_text(&mut self) -> String;
}
```

CliEvent helper methods: `is_text()`, `is_step_start()`, `is_step_finish()`, `is_error()`, `is_tool_use()`, `text()`.

## Usage Cards

### Client Usage Card

**Use when:** Building applications that interact with OpenCode HTTP API

**Enable/Install:** Default features (`http`, `sse`)

**Import/Invoke:**
```rust
use opencode_sdk::{Client, ClientBuilder};
let client = Client::builder().build()?;
```

**Minimal flow:**
1. Build client with `Client::builder().base_url(url).directory(dir).build()`
2. Use API accessors like `client.sessions().create(&req).await`
3. For streaming, create SSE subscription before sending async requests
4. Handle errors using `Result<T>` and `OpencodeError` variants

**Key APIs:**
- `Client::builder()` - Create builder
- `ClientBuilder::build()` - Build client
- `client.sessions()`, `client.messages()` - API accessors
- `client.subscribe_session(id).await` - Per-session SSE
- `client.run_simple_text(text).await` - Quick prompt

**Pitfalls:**
- Forgetting to subscribe before `prompt_async` - events will be lost
- Not handling `OpencodeError::Http` - may miss structured error data

**Source:** [`src/client.rs`](src/client.rs)

---

### SessionsApi Usage Card

**Use when:** Managing sessions (CRUD, forking, sharing, reverting)

**Enable/Install:** `http` feature (default)

**Import/Invoke:**
```rust
let sessions = client.sessions();
```

**Minimal flow:**
1. Create: `sessions.create(&CreateSessionRequest::default()).await`
2. Or with title: `sessions.create_with(SessionCreateOptions::new().with_title("Task")).await`
3. List: `sessions.list().await`
4. Delete: `sessions.delete(&id).await`

**Key APIs:**
- `create(req)`, `create_with(opts)` - Create sessions
- `get(id)`, `list()` - Retrieve sessions
- `delete(id)` - Remove session
- `fork(id)` - Fork session
- `share(id)`, `unshare(id)` - Sharing
- `revert(id, req)` - Revert to message

**Pitfalls:**
- Session IDs are strings - don't assume UUID format
- `create_with` is more ergonomic than manual `CreateSessionRequest`

**Source:** [`src/http/sessions.rs`](src/http/sessions.rs)

---

### MessagesApi Usage Card

**Use when:** Sending prompts or managing messages

**Enable/Install:** `http` feature (default)

**Import/Invoke:**
```rust
let messages = client.messages();
```

**Minimal flow:**
1. Send prompt: `messages.prompt(&session_id, &PromptRequest::text("Hello")).await`
2. Or async (no response body): `messages.prompt_async(&session_id, &req).await`
3. List: `messages.list(&session_id).await`
4. Get: `messages.get(&session_id, &message_id).await`

**Key APIs:**
- `prompt(session_id, req)` - Send prompt (sync-like)
- `prompt_async(session_id, req)` - Async send (use with SSE)
- `send_text_async(session_id, text, model)` - Convenience method
- `list(session_id)`, `get(session_id, message_id)` - Retrieve messages
- `remove(session_id, message_id)` - Delete message

**Pitfalls:**
- `prompt_async` returns empty body - must use SSE for response
- `PromptRequest::text("...").with_model("provider", "model")` for model selection

**Source:** [`src/http/messages.rs`](src/http/messages.rs)

---

### SseSubscription Usage Card

**Use when:** Streaming real-time events from OpenCode

**Enable/Install:** `sse` feature (default)

**Import/Invoke:**
```rust
use opencode_sdk::sse::SseOptions;
let subscription = client.subscribe_session(&session_id).await?;
```

**Minimal flow:**
1. Subscribe: `let mut sub = client.subscribe_session(&id).await?`
2. Receive: `while let Some(event) = sub.recv().await { /* process */ }`
3. Stats: `let stats = sub.stats()` for diagnostics
4. Drop or `sub.close()` to cancel

**Key APIs:**
- `SseSubscription::recv().await` - Get next event
- `SseSubscription::stats()` - Stream diagnostics
- `SseSubscription::close()` - Cancel subscription
- `SessionEventRouter::subscribe(id).await` - Multiplexed session subscription

**Pitfalls:**
- Not subscribing before async operations - miss early events
- Channel lag causes dropped events - check `stats().dropped`
- Parse errors in `stats().parse_errors` indicate schema mismatch

**Source:** [`src/sse.rs`](src/sse.rs)

---

### ManagedServer Usage Card

**Use when:** Programmatically starting OpenCode server for tests or automation

**Enable/Install:** `server` feature (NOT default)

**Import/Invoke:**
```rust
use opencode_sdk::server::{ManagedServer, ServerOptions};
let server = ManagedServer::start(ServerOptions::new().port(8080)).await?;
```

**Minimal flow:**
1. Configure: `ServerOptions::new().port(8080).directory("/project")`
2. Start: `let server = ManagedServer::start(opts).await?`
3. Get URL: `let url = server.url()`
4. Create client: `let client = Client::builder().base_url(url).build()?`
5. Stop (or drop): `server.stop().await?`

**Key APIs:**
- `ServerOptions::new()` - Create options
- `ServerOptions::port()`, `::hostname()`, `::directory()`, `::config_json()` - Configure
- `ManagedServer::start(opts).await` - Spawn server
- `ManagedServer::url()`, `::port()` - Get connection info
- `ManagedServer::stop().await` - Graceful shutdown

**Pitfalls:**
- Server kills on drop - hold `ManagedServer` reference while using
- `config_json` sets env var `OPENCODE_CONFIG_CONTENT` - server must support this
- Startup timeout defaults to 5s - increase for slow systems

**Source:** [`src/server.rs`](src/server.rs)

---

### CliRunner Usage Card

**Use when:** Falling back to CLI when HTTP API unavailable

**Enable/Install:** `cli` feature (NOT default)

**Import/Invoke:**
```rust
use opencode_sdk::cli::{CliRunner, RunOptions};
let mut runner = CliRunner::start("Hello", RunOptions::new()).await?;
```

**Minimal flow:**
1. Create options: `RunOptions::new().model("provider/model").agent("code")`
2. Start: `let mut runner = CliRunner::start("prompt", opts).await?`
3. Stream: `while let Some(event) = runner.recv().await { /* process */ }`
4. Or collect: `let text = runner.collect_text().await`

**Key APIs:**
- `RunOptions::new()` - Create options (format defaults to "json")
- `RunOptions::model()`, `::agent()`, `::title()`, `::attach()` - Configure
- `CliRunner::start(prompt, opts).await` - Start CLI
- `CliRunner::recv().await` - Get next event
- `CliRunner::collect_text().await` - Aggregate text events
- `CliEvent::is_text()`, `::text()` - Event inspection

**Pitfalls:**
- CLI outputs NDJSON to stdout - `format` must be "json"
- stderr inherited - CLI errors visible but not captured
- Session sharing requires `share: true` in options

**Source:** [`src/cli.rs`](src/cli.rs)

## API Reference

### Core Types

| Type | Location | Description |
|------|----------|-------------|
| `Client` | [`src/client.rs:15`](src/client.rs:15) | Main ergonomic API client |
| `ClientBuilder` | [`src/client.rs:27`](src/client.rs:27) | Builder for Client |
| `OpencodeError` | [`src/error.rs:9`](src/error.rs:9) | Error enum (13 variants) |
| `Result<T>` | [`src/error.rs:6`](src/error.rs:6) | Type alias for Result |

### SSE Types

| Type | Location | Description |
|------|----------|-------------|
| `SseSubscriber` | [`src/sse.rs:331`](src/sse.rs:331) | Creates SSE subscriptions |
| `SseSubscription` | [`src/sse.rs:134`](src/sse.rs:134) | Typed event subscription |
| `RawSseSubscription` | [`src/sse.rs:155`](src/sse.rs:155) | Raw JSON subscription |
| `SessionEventRouter` | [`src/sse.rs:195`](src/sse.rs:195) | Multiplexes to sessions |
| `SseOptions` | [`src/sse.rs:62`](src/sse.rs:62) | Subscription options |
| `SessionEventRouterOptions` | [`src/sse.rs:163`](src/sse.rs:163) | Router options |
| `SseStreamStats` | [`src/sse.rs:83`](src/sse.rs:83) | Diagnostics snapshot |
| `RawSseEvent` | [`src/sse.rs:143`](src/sse.rs:143) | Raw SSE frame |

### Server/CLI Types

| Type | Location | Description |
|------|----------|-------------|
| `ManagedServer` | [`src/server.rs:88`](src/server.rs:88) | Managed server process |
| `ServerOptions` | [`src/server.rs:14`](src/server.rs:14) | Server configuration |
| `CliRunner` | [`src/cli.rs:186`](src/cli.rs:186) | CLI wrapper |
| `RunOptions` | [`src/cli.rs:13`](src/cli.rs:13) | CLI options |
| `CliEvent` | [`src/cli.rs:133`](src/cli.rs:133) | CLI output event |

### HTTP Types

| Type | Location | Description |
|------|----------|-------------|
| `HttpClient` | [`src/http/mod.rs:43`](src/http/mod.rs:43) | Low-level HTTP client |
| `HttpConfig` | [`src/http/mod.rs:32`](src/http/mod.rs:32) | HTTP configuration |
| `SessionsApi` | [`src/http/sessions.rs:15`](src/http/sessions.rs:15) | Sessions API client |
| `MessagesApi` | [`src/http/messages.rs:14`](src/http/messages.rs:14) | Messages API client |

### Model Types

| Type | Location | Description |
|------|----------|-------------|
| `Session` | [`src/types/session.rs:9`](src/types/session.rs:9) | Session model |
| `SessionCreateOptions` | [`src/types/session.rs:152`](src/types/session.rs:152) | Builder for create |
| `Message` | [`src/types/message.rs:45`](src/types/message.rs:45) | Message with parts |
| `MessageInfo` | [`src/types/message.rs:9`](src/types/message.rs:9) | Message metadata |
| `Part` | [`src/types/message.rs:75`](src/types/message.rs:75) | Content part enum (12 variants) |
| `PromptRequest` | [`src/types/message.rs:513`](src/types/message.rs:513) | Prompt request |
| `PromptPart` | [`src/types/message.rs:582`](src/types/message.rs:582) | Prompt part enum |
| `Event` | [`src/types/event.rs:20`](src/types/event.rs:20) | SSE event enum (40 variants) |
| `GlobalEventEnvelope` | [`src/types/event.rs:11`](src/types/event.rs:11) | Global event wrapper |
| `ToolState` | [`src/types/message.rs:423`](src/types/message.rs:423) | Tool execution state |

## Common Pitfalls

### Feature Flag Mismatches

```rust
// ❌ Won't compile without http feature
let client = Client::builder().build()?;  // Error: http feature required

// ✅ Enable http feature in Cargo.toml
opencode-rs-sdk = { version = "0.1", features = ["http"] }
```

### Missing SSE Subscription

```rust
// ❌ Response events lost
client.send_text_async(&session_id, "Hello", None).await?;
let sub = client.subscribe_session(&session_id).await?;  // Too late!

// ✅ Subscribe first
let sub = client.subscribe_session(&session_id).await?;
client.send_text_async(&session_id, "Hello", None).await?;
// Now events are captured
```

### Platform Incompatibility

```rust
// ❌ Compiling on Windows will fail with:
// error: opencode_rs only supports Unix-like platforms (Linux/macOS). Windows is not supported.

// ✅ Use WSL, Docker, or macOS/Linux
```

### Server Lifecycle Management

```rust
// ❌ Server dropped too early
{
    let server = ManagedServer::start(ServerOptions::new()).await?;
    let client = Client::builder().base_url(server.url()).build()?;
} // Server killed here!
// ❌ Client requests will fail

// ✅ Keep server alive
let server = ManagedServer::start(ServerOptions::new()).await?;
let client = Client::builder().base_url(server.url()).build()?;
// Use client while server in scope
server.stop().await?;  // Or let it drop
```

### Error Handling

```rust
// ❌ Generic error handling misses context
if let Err(e) = result {
    eprintln!("Error: {}", e);
}

// ✅ Use helper methods for specific handling
match result {
    Err(e) if e.is_not_found() => println!("Not found"),
    Err(e) if e.is_validation_error() => println!("Validation: {:?}", e.error_name()),
    Err(e) => eprintln!("Other: {}", e),
    Ok(v) => v,
}
```

### Channel Saturation

```rust
// ❌ Not checking for dropped events
let sub = client.subscribe_session(&id).await?;
// Slow processing...
while let Some(event) = sub.recv().await {
    tokio::time::sleep(Duration::from_secs(1)).await;  // Too slow!
}

// ✅ Monitor stats
if sub.stats().dropped > 0 {
    tracing::warn!("Dropped {} events", sub.stats().dropped);
}
```

## Optional

### Additional Resources

- [API Documentation](https://docs.rs/opencode-rs-sdk): Full rustdocs
- [Repository](https://github.com/allisoneer/agentic_auxilary): Source code
- [OpenCode Documentation](https://opencode.ai): Platform docs

### Version History

| Version | Notes |
|---------|-------|
| 0.1.x   | Initial release with HTTP API, SSE streaming, managed server |

### Feature Flag Matrix

| Feature | Dependencies | APIs Enabled |
|---------|--------------|--------------|
| `http` | reqwest, serde_json | All HTTP API modules |
| `sse` | reqwest-eventsource, backon | SSE streaming, subscriptions |
| `server` | tokio/process, portpicker | ManagedServer |
| `cli` | tokio/process | CliRunner |
| `full` | All above | Everything |

### Default Dependencies

- `tokio` (rt-multi-thread, macros, sync, time)
- `serde` (derive)
- `thiserror`
- `url`
- `http`
- `tokio-util`
- `futures`
- `urlencoding`
- `uuid` (v4, serde)
- `chrono` (serde)
- `tracing`

### License

Apache-2.0

---

*Generated for opencode-rs-sdk v0.1.3*
