// src/bot/state/mod.rs
//! # State Management Module
//!
//! This module is responsible for managing the core application state of bots and listeners,
//! serving as a centralized interface for CRUD operations and state persistence. It is utilized
//! by both the REST API endpoints and the CLI's offline mode, ensuring a consistent experience
//! across different usage contexts.
//!
//! ## Key Responsibilities
//! - **State Management**: Provides utilities to manipulate bots and listeners in memory.
//! - **Persistence**: Ensures that changes to the state are saved to disk, enabling durability.
//! - **Validation**: Enforces constraints on inputs, such as non-empty IDs and other field-level checks.
//! - **Error Handling**: Propagates detailed and structured errors to the calling modules (REST API or CLI).
//!
//! ## Integration Points
//! - **REST API**:
//!   - Used by `crate::bot::api::endpoint::<function_name>` to handle online operations.
//! - **CLI Offline Mode**:
//!   - Used by `crate::bot::cli::offline::run` for offline command execution.
//!
//! ## Structure
//! - **Input Module** (`input`):
//!   - Defines argument types (`BotInsertArgs`, `ListenerInsertArgs`, etc.) for input handling.
//! - **Output Module** (`output`):
//!   - Defines views (`BotView`, `ListenerView`, etc.) for serializable output structures.
//! - **Server Module** (`server`):
//!   - Manages arguments and configurations for server startup (`ServerStartupArgs`).
//!
//! ## Purpose
//! The `AppState` struct within this module encapsulates the in-memory state of the application
//! and provides methods to interact with bots and listeners. This ensures a clear separation of
//! state management logic from other parts of the application, fostering maintainability and reusability.
//!
//! ## Why This Matters
//! This module serves as the backbone of the application's state management. Two years from now, this doc should
//! remind you that this module:
//! - Acts as the single source of truth for bot and listener data.
//! - Bridges the CLI and REST API with consistent logic and error propagation.
//! - Handles critical operations like persistence, validation, and filtering with reliability.
//!
//! ## Example Usage
//! ### Adding a Bot
//! ```rust
//! let mut state = AppState::new();
//! let args = BotInsertArgs::new("TestBot", "Binance");
//! let bot = state.add_bot(args)?;
//! println!("Added bot: {}", bot);
//! ```
//!
//! ### Listing Bots
//! ```rust
//! let bots = state.list_bots(None)?;
//! println!("Available bots: {:?}", bots);
//! ```
//!
//! ### Clearing All Listeners
//! ```rust
//! state.clear_listeners()?;
//! println!("All listeners cleared.");
//! ```
pub mod input;
pub mod output;
pub mod registry;
pub mod server;

pub use input::{BotDeleteArgs, BotGetArgs, BotInsertArgs, BotListArgs, BotUpdateArgs};
pub use input::{
    ListenerDeleteArgs, ListenerGetArgs, ListenerInsertArgs, ListenerListArgs, ListenerUpdateArgs,
    ListenersDeleteArgs,
};
pub use output::{BotListView, BotView};
pub use output::{ListenerListView, ListenerView};
pub use server::ServerStartupArgs;

pub use registry::BotRegistry;

pub use crate::app_state::AppState;
pub use crate::bot::model::{Bot, Listener};
