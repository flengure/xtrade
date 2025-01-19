# Application Architecture Documentation

## Modules Overview

### 1. `state.rs`
Responsible for core logic related to online and offline state manipulation, as well as local data management.

- **Key Responsibilities:**
  - Defines operations on application state (e.g., CRUD operations for bots and listeners).
  - Provides state to other components depending on the mode (online/offline).
- **Modes:**
  - **Online:**
    - State is loaded from a file and provided to the API server.
    - Utilizes `Arc<Mutex>` for shared state management.
  - **Offline:**
    - State is loaded from a file and provided to the CLI.
- **Important Note:**
  - All internal processes must interact with `state.rs` indirectly through `rest.rs` for now.
  - A future enhancement may replace this with a more efficient mechanism such as **internal hooks** or **RPC**.

---

### 2. `api.rs`
Defines HTTP endpoints and acts as the bridge between external requests and internal logic.

- **Key Responsibilities:**
  - Defines REST API endpoints.
  - Calls `rest.rs` for state-related operations.

---

### 3. `rest.rs`
Acts as the intermediary between internal processes and `state.rs`.

- **Key Responsibilities:**
  - Processes REST-based client operations.
  - Handles all interactions with `state.rs`.
  - Ensures state is accessed consistently and securely by internal processes.

---

### 4. `online.rs`
Contains logic specific to the online mode.

- **Key Responsibilities:**
  - Contains handler functions for interacting with the online REST client (`rest.rs`).
  - Implements CLI commands that operate in the online mode.

---

### 5. `offline.rs`
Contains logic specific to the offline mode.

- **Key Responsibilities:**
  - Contains handler functions for interacting with the saved application state (`state.rs`) via `rest.rs`.
  - Implements CLI commands that operate in the offline mode.

---

## Architecture Diagram

The following diagram illustrates the relationships between the key components of the application architecture.
**Note:** All interactions with `state.rs` are mediated through `rest.rs`:

```mermaid
flowchart TD
    A[state.rs<br>Provides Shared/Saved state]:::central
    A1[state.rs<br>Provides Shared state]:::online
    A2[state.rs<br>Provides Saved state]:::offline
    B1[api.rs<br>Defines API endpoints]:::online
    C1[rest.rs<br>Handles REST requests<br>Mediates access to state.rs]:::online
    D1[offline.rs<br>Manages offline mode]:::offline
    D2[online.rs<br>Manages online mode]:::online
    F1((Terminal / CLI)):::ui
    F2((Terminal / CLI)):::ui
    G((Browser / curl)):::ui

    A --> A1
    A --> A2
    A1 <-- Data struct --> B1
    A2 <-- Data struct --> D1
    B1 <-- Request\nResponse --> C1
    C1 <-- Mediates --> A
    C1 <-- Request\nResponse --> G
    C1 <-- Request\nResponse --> D2
    D2 <-- Text --> F2
    D1 <-- Text --> F1

    classDef ui fill:#005f73,stroke:#0a9396,stroke-width:2px,font-size:14px,color:#ffffff,font-weight:bold;
    classDef online fill:#d1e7dd,stroke:#0f5132,stroke-width:2px,font-size:14px,color:#0f5132;
    classDef offline fill:#f8d7da,stroke:#842029,stroke-width:2px,font-size:14px,color:#842029;
    classDef central fill:#ffeeba,stroke:#856404,stroke-width:2px,font-size:14px,color:#856404,font-weight:bold;
