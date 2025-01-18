
```paintext
src/
├── bot
│   ├── api/
│   │   ├── mod.rs            # Module declaration for API
│   │   ├── bots.rs           # Bot-related API endpoints
│   │   ├── listeners.rs      # Listener-related API endpoints
│   ├── api.rs
│   ├── cli/
│   │   ├── README.md
│   │   ├── commands.rs
│   │   ├── offline.rs
│   │   ├── online.rs
│   │   ├── mod.rs
│   │   └── cli.rs
│   ├── mod.rs
│   ├── model.rs
│   ├── rest.rs
│   ├── state.rs
│   └── view.rs

```
---
**Reasoning Behind the Structure**

1. Separation of Concerns
	- state.rs
		- Contains the core logic for state manipulation and local data management.
		- Defines operations on application state, such as CRUD operations for bots and listeners.
		- always uses Input Arg structs for arguments other than indexes
		- always use Output View structs for structured return values
	- api.rs
		- focuses on defining HTTP endpoints for REST API.
		- Maps REST calls to underlying state functions or other appropriate logic
	- rest.rs
		- Handles online REST-based client operations, such as sending requests to an external REST API.
		- Provides a unified way to handle remote operations for CLI commands in “online” mode.
	- model.rs
		- Defines the fundamental data structures (e.g., Bot, Listener, etc.).
	- view.rs
		- Contains simplified or user-facing views and inputs of models (e.g., BotView, ListenerView, BotInsertArgs) for input, display and serialization

2. **cli module:**
- Purpose
	- The cli module contains the logic for command-line interactions, with a clear distinction between offline and online operations.
- Structure
	- cli.rs
		- Acts as the entry point for the CLI. Uses clap or another argument parser to parse commands and dispatch them to appropriate handlers.
	- offline.rs:
		- Contains handler functions that interact with the local application state (state.rs).
		- Implements CLI commands that operate in offline mode.
	- online.rs:
		- Contains handler functions that interact with the online REST client (rest.rs).
		- Implements CLI commands that operate in online mode.
	- mod.rs:
		- Serves as the entry point for the CLI module, exporting offline and online functionality.
	- README.md:
		- Documents how to use the CLI, providing examples of commands and usage scenarios.
3. **Centralized CLI Menu**
	- cli.rs:
		- Centralizes command parsing and dispatching.
		- Based on user input, directs commands to either offline (offline.rs) or online (online.rs) handlers.
		- Maintains consistency in command names and arguments across offline and online modes.
