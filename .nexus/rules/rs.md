# Rust File and Directory Naming

## File Naming

1. **One item per file** - Each function, component, struct, enum, type, or const gets its own file
2. **Filename matches item name** - Use snake_case (e.g., `PlatformInfo` → `platform_info.rs`, `AuthError` → `auth_error.rs`)
3. **Function names include context** - `render_navigation_tree_item` not just `render_item`

## Directory Structure

4. **`main.rs` is minimal** - Only contains `main()` function and module imports
5. **Group files by purpose** - Use directories like `components/`, `services/`, `database/`, `api/`, `types/`
6. **Frontend components go in `src/components/`** - All UI components live here
7. **Structs and complex enums get their own directory** - Structs always use directory structure; enums only when they have multiple impl blocks, methods, or trait implementations
8. **Use `mod.rs` for re-exports** - Each directory has a `mod.rs` that exports its contents

## Crate Names

9. **Cargo.toml uses kebab-case** - Package names use hyphens: `nexus-implement-cli`
10. **Code uses snake_case** - Import with underscores: `use nexus_implement_cli::discovery;`
11. **Auto-conversion** - Cargo automatically converts hyphens to underscores in code
# Rust Type Organization

## Type Organization

1. **Type definition in `mod.rs`** - Only the type definition (struct or complex enum), no impl blocks
2. **`methods/`** - Instance methods that do work (`&self`, `&mut self`)
3. **`constructors/`** - Constructors and builders (`new`, `builder`, `with_*`)
4. **`traits/`** - Standard library and custom trait implementations
5. **`enums/`** - Related enums that belong to this type (each enum in its own file)
6. **NO EXCEPTIONS for structs** - Even simple data holders with 1-2 methods must follow this structure
7. **Complex enums only** - Enums with multiple impl blocks, methods, or traits use this structure; simple enums use single file

## Implementation Style

7. **Each file = one impl block** - Single responsibility per file
8. **Standalone functions where possible** - `render_navigation_tree_item(item)` not `self.render_item(item)`
9. **Impl methods call standalone functions** - Keep `impl` thin, delegate to file functions

## When to Use Directory Structure

**ALWAYS use directory structure for:**
- ALL structs (even pure data holders with only fields and derives)
- Complex enums with multiple impl blocks, methods, or trait implementations

**Use single file for:**
- Standalone functions (not in an impl block)
- Simple enums (placed in parent type's `enums/` folder, one file per enum)
- Type aliases (`type Result<T> = std::result::Result<T, Error>;`)
- Constants (group related constants in files like `src/constants.rs` or `src/config/constants.rs`)

**Never put impl blocks in struct mod.rs file, even for:**
- Simple data holders
- DTOs (Data Transfer Objects)
- Types with only derives and no custom implementations
- Types with only 1-2 methods
- "It seems like overkill" cases

## Example Structure

```
src/components/
├── mod.rs
└── navigation_tree/
    ├── mod.rs                          # Struct definition only
    │
    ├── methods/
    │   ├── mod.rs                      # Re-exports all methods
    │   ├── render_navigation_tree.rs
    │   ├── render_navigation_tree_item.rs
    │   └── toggle_expanded.rs
    │
    ├── traits/
    │   ├── mod.rs                      # Re-exports all trait impls
    │   ├── display.rs                  # impl Display for NavigationTree
    │   ├── default.rs                  # impl Default for NavigationTree
    │   └── from_vec.rs                 # impl From<Vec<TreeItem>> for NavigationTree
    │
    ├── constructors/
    │   ├── mod.rs                      # Re-exports all constructor functions
    │   ├── new.rs                      # impl NavigationTree { pub fn new() } + inline tests
    │   └── with_items.rs               # impl NavigationTree { pub fn with_items() } + inline tests
    │
    └── enums/
        ├── mod.rs                      # Re-exports all enums
        ├── tree_node_kind.rs           # pub enum TreeNodeKind { ... }
        └── expand_state.rs             # pub enum ExpandState { ... }
```

## Example: Even Simple Types

**WRONG:**
```rust
// src/types/config.rs
pub struct Config {
    pub name: String,
}

impl Config {
    pub fn new(name: String) -> Self { ... }
}
```

**CORRECT:**
```
src/types/config/
├── mod.rs              # Just: pub struct Config { pub name: String }
└── constructors/
    ├── mod.rs
    └── new.rs          # impl Config { pub fn new(name: String) -> Self { ... } }
```
# Rust Testing Standards

See [E2E Test Standards](e2e_tests.md) for all testing requirements.

**Summary:** This project uses exclusively E2E tests with no unit tests, no integration tests, and no mocks. All tests must be linked to context files.
# Rust Documentation Standards

## Documentation Requirements

1. **Doc Comments Required:** Every public item must have documentation
   - Structs and their fields
   - Enums and their variants
   - Functions and methods
   - Traits and their methods
   - Type aliases
   - Constants and statics
   - Modules

## Struct Documentation

2. **Struct-level documentation:**
   - Brief description (first line) - What the struct represents
   - Purpose and use cases
   - Example usage

3. **Field documentation:**
   - Document all public fields
   - Explain the purpose and valid values
   - Note any constraints or invariants

## Enum Documentation

4. **Enum-level documentation:**
   - Brief description of what the enum represents
   - When to use each variant
   - Example usage

5. **Variant documentation:**
   - Document each variant's purpose
   - Explain any data associated with variants

## Function Documentation

6. **Required Sections:**
   - Brief description (first line)
   - `# Arguments` - Describe each parameter
   - `# Returns` - Describe return value
   - `# Errors` - Describe error conditions (if function returns Result)
   - `# Panics` - Describe panic conditions (if function can panic)
   - `# Safety` - Document safety invariants (required for unsafe code)
   - `# Performance` - Document algorithmic complexity for performance-critical code
   - `# Example` - Code example showing usage

## Module Documentation

7. **Module-level documentation:**
   - Add at the top of `mod.rs` files using `//!`
   - Explain the module's purpose
   - List main types and concepts
   - Provide usage examples

## Trait Documentation

8. **Trait documentation:**
   - Describe the trait's purpose
   - Explain when to implement it
   - Document associated types and methods
   - Provide implementation examples
# Rust Error Handling

## Error Type Selection

1. **Use `Result<T, E>` for recoverable errors** - Operations that can fail in expected ways should return Result
2. **Use `Option<T>` for expected absence** - Use for values that may or may not exist, not for error conditions
3. **Use `panic!` for unrecoverable errors** - Only for bugs, invariant violations, or truly exceptional conditions
4. **Use `thiserror` for library errors** - When building libraries, use thiserror to create custom error types with proper Error trait implementations
5. **Use `anyhow` for application errors** - In application code, use anyhow for convenient error handling and context

## Error Handling Patterns

6. **Document error conditions** - All possible errors must be documented in the `# Errors` section
7. **Use the `?` operator** - Prefer `?` for error propagation over explicit match or unwrap
8. **Add context to errors** - Use `.context()` or `.with_context()` to add meaningful context when propagating errors
9. **Never use `unwrap()` or `expect()` in production code** - Only acceptable in tests or when panic is genuinely the correct behavior with clear documentation

## thiserror Patterns

10. **Derive `Error` and `Debug`** - Use `#[derive(Error, Debug)]` on error enums
11. **Use `#[error("...")]` attribute** - Define error messages on each variant
12. **Include context in variants** - Use tuple or named fields to include error context

## anyhow Patterns

13. **Import `anyhow::Result`** - Use as return type for functions that can fail
14. **Use `.context()` for static messages** - Add context with string literals
15. **Use `.with_context()` for dynamic messages** - Add context with closures that format strings
16. **Chain context calls** - Add multiple layers of context as errors propagate up the call stack
# Rust Visibility and Encapsulation

## Visibility Rules

1. **Default to private** - All items should be private unless there's a specific reason to expose them
2. **Use `pub(crate)` for internal APIs** - Items shared across modules within the crate but not part of the public API
3. **Use `pub(super)` for parent module access** - Items that should only be visible to the parent module
4. **Document why items are public** - Public items should have documentation explaining their purpose in the public API
5. **Minimize public API surface** - Keep the public API as small as possible to maintain flexibility for internal changes

## Encapsulation Patterns

6. **Encapsulate implementation details** - Hide internal fields, use accessor methods when external access is needed
7. **Consider builder patterns for complex public types** - Rather than exposing all fields publicly, use builders for construction
8. **Private fields with public accessors** - Prefer accessor methods over public fields for better encapsulation
9. **Avoid exposing all fields publicly** - Public fields lock you into the current structure and prevent future changes
# Rust Naming Conventions

## Crate Names

1. **Cargo.toml uses kebab-case** - Package names use hyphens: `nexus-implement-cli`
2. **Code uses snake_case** - Import with underscores: `use nexus_implement_cli::discovery;`
3. **Auto-conversion** - Cargo automatically converts hyphens to underscores in code

## Type Naming

4. **Structs, Enums, Traits use PascalCase** - `TaskMetadata`, `ParseError`, `Display`
5. **Type parameters use single uppercase letters** - `T`, `E`, `K`, `V` or descriptive PascalCase like `State`

## Function and Variable Naming

6. **Functions use snake_case** - `parse_task_file`, `render_navigation_tree`
7. **Function names include context** - `render_navigation_tree_item` not just `render_item`
8. **Variables use snake_case** - `task_id`, `file_path`, `user_count`

## Constant Naming

9. **Constants use SCREAMING_SNAKE_CASE** - `MAX_BUFFER_SIZE`, `DEFAULT_TIMEOUT`
10. **Static variables use SCREAMING_SNAKE_CASE** - `GLOBAL_CONFIG`, `INSTANCE_COUNT`

## Module Naming

11. **Modules use snake_case** - `mod database`, `mod api_client`, `mod error_handling`
12. **File names match module names** - `database.rs`, `api_client/mod.rs`

## Example

```rust
// Crate name: nexus-implement-cli
use nexus_implement_cli::discovery;

const MAX_TASKS: usize = 100;

pub struct TaskMetadata {
    task_id: String,
}

pub enum ParseError {
    FileNotFound,
    InvalidFormat,
}

pub fn parse_task_file(path: &Path) -> Result<TaskMetadata, ParseError> {
    let file_content = read_file(path)?;
    Ok(TaskMetadata { task_id: extract_id(&file_content) })
}
```
# UI Architecture (Bulletproof Pattern)

## Core Principles

1. **Separation of concerns** - Layout containers are distinct from content components
2. **Composition over inheritance** - Views are declarative compositions of reusable components
3. **Feature colocation** - Group code by domain/feature, not by technical role

## Directory Structure

4. **Layout module** - Contains structural containers that don't know their content (shells, panes, splits, grids, slots)
5. **UI Components module** - Reusable content blocks with their own state and behavior (chat, preview, list, tree, editor)
6. **Views module** - Composition definitions only, no rendering logic (dashboard, settings, playground)

## Component Design

7. **Components are self-contained** - A component owns its rendering logic, input handling, internal state, and scroll/focus behavior
8. **Components receive configuration, not implementation details** - Callers configure what they want, not how to achieve it
9. **Components implement a common interface** - All components share a consistent contract for rendering, input handling, and focus management

## View Composition

10. **Views are declarative** - A view defines what goes where, not how to render
11. **Views delegate to components** - Views contain no rendering code themselves
12. **Adding a view requires no new rendering logic** - Only composition of existing components

## Layout vs Component Responsibilities

13. **Layout responsibilities** - Border, title, background, spacing, area calculation for content
14. **Component responsibilities** - Actual content rendering, scrolling, input handling, internal state
15. **Layouts wrap components** - Layout provides the frame, component provides the content

## State Organization

16. **State lives with its feature** - Grouped by domain, not in a flat global struct
17. **Component state is reusable** - The same state structure can be used across multiple views

## Anti-Patterns

18. **Never duplicate component logic across views** - If two views need the same content, use one component
19. **Never put rendering code in view files** - Views compose, components render
20. **Never leak layout concerns into components** - Components don't know about their container
21. **Never create view-specific component variants** - Configure the component, don't fork it

# Dioxus Router Navigation

## Navigation Components

1. **Always use `Link` component for navigation** - Never use raw `<a>` tags for internal navigation
2. **Import Route enum** - Add `use crate::router::Route;` to components that need navigation
3. **Type-safe routing** - Use `Link { to: Route::RouteName {}, ... }` syntax
4. **Avoid `<a href>`** - Raw anchor tags cause full page reloads (white flash) instead of SPA navigation

## Link Component Usage

5. **Syntax:** `Link { to: Route::Home {}, class: "btn", "Link Text" }`
6. **Classes work normally** - Apply CSS classes with the `class` attribute
7. **Children supported** - Link can contain text, icons, or other elements
8. **Browser history works** - Back/forward buttons work automatically

## Why Link Over Anchor Tags

9. **Client-side navigation** - `Link` provides instant SPA navigation without page reload
10. **No white flash** - Avoids full WASM reinitialization between pages
11. **Faster transitions** - No network round-trip or asset reloading
12. **Router integration** - Properly integrates with Dioxus router state and hooks

## Example

```rust
use dioxus::prelude::*;
use crate::router::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            // CORRECT: Using Link component
            Link { to: Route::Home {}, class: "nav-link",
                "Home"
            }
            Link { to: Route::Demo {}, class: "nav-link",
                "Demo"
            }
            
            // WRONG: Don't use raw anchor tags for internal routes
            // a { href: "/", "Home" }  // ❌ Causes full page reload
        }
    }
}
```
