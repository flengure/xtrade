# State Management Module

This module is responsible for managing the core application state of bots and listeners, serving as a centralized interface for CRUD operations and state persistence. It is utilized by both the REST API endpoints and the CLI's offline mode, ensuring a consistent experience across different usage contexts.

---

## Key Responsibilities
- **State Management**: Provides utilities to manipulate bots and listeners in memory.
- **Persistence**: Ensures that changes to the state are saved to disk, enabling durability.
- **Validation**: Enforces constraints on inputs, such as non-empty IDs and other field-level checks.
- **Error Handling**: Propagates detailed and structured errors to the calling modules (REST API or CLI).

---

## Integration Points
- **REST API**:
  - Used by `crate::bot::api::endpoint::<function_name>` to handle online operations.
- **CLI Offline Mode**:
  - Used by `crate::bot::cli::offline::run` for offline command execution.

---

## Structure
- **Input Module** (`input`):
  - Defines argument types (`BotInsertArgs`, `ListenerInsertArgs`, etc.) for input handling.
- **Output Module** (`output`):
  - Defines views (`BotView`, `ListenerView`, etc.) for serializable output structures.
- **Server Module** (`server`):
  - Manages arguments and configurations for server startup (`ServerStartupArgs`).

---

## Why This Matters
This module serves as the backbone of the application's state management. It:
- Acts as the single source of truth for bot and listener data.
- Bridges the CLI and REST API with consistent logic and error propagation.
- Handles critical operations like persistence, validation, and filtering with reliability.

---

## Example Usage

### Adding a Bot
```rust
let mut state = AppState::new();
let args = BotInsertArgs::new("TestBot", "Binance");
let bot = state.add_bot(args)?;
println!("Added bot: {}", bot);
```
###
