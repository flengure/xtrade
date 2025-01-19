# api module Architecture Diagram

```mermaid
flowchart TD

    S1[state.rs<br>Shared state<br>API Server]:::core
    S2[state.rs<br>Saved state<br>File Storage]:::offline

    A1[api.rs]:::online
    R1[rest.rs]:::online

    ON[online.rs]:::online
    OF[offline.rs]:::offline
    F1(((Terminal<br>CLI))):::terminal
    F2(((Terminal<br>CLI))):::terminal
    F3(((Terminal<br>CLI))):::terminal
    G(((Browsers<br>Web UI Client<br>curl<br>app))):::browser
    I1[ipc.rs]:::ipc

	S1 <-- Restore<br>Save --> S2
    S1 <-- Data struct --> A1
    S1 <-- Data struct --> I1
    S2 <-- Data struct --> OF
    OF <-- Text --> F2

    A1 <-- Request\nResponse --> R1
    R1 <-- Request\nResponse --> G
    R1 <-- Request\nResponse --> ON
    I1 <-- Text --> F3
    ON <--> F1

    classDef core fill:#ffeeba,stroke:#856404,stroke-width:2px,font-size:14px,color:#856404,font-weight:bold;
    classDef online fill:#d1e7dd,stroke:#0f5132,stroke-width:2px,font-size:14px,color:#0f5132,font-weight:bold;
    classDef offline fill:#f8d7da,stroke:#842029,stroke-width:2px,font-size:14px,color:#842029,font-weight:bold;
    classDef ipc fill:#cce5ff,stroke:#004085,stroke-width:2px,font-size:14px,color:#004085,font-weight:bold;
    classDef browser fill:#0a735f,stroke:#0ae87c,stroke-width:2px,font-size:14px,color:#ffffff,font-weight:bold;
    classDef terminal fill:#5f0a73,stroke:#7c3aed,stroke-width:2px,font-size:14px,color:#ffffff,font-weight:bold;
```

# Application Architecture Documentation
## Modules Overview

### 1. `state.rs`

- **Purpose:**
    Serves as the centralized state management module for both online (`Shared state`) and offline (`Saved state`) operations.

- **Responsibilities:**

    - Manages shared application state (`Shared state` for online operations).
    - Manages saved application state (`Saved state` for offline operations).

---
### 2. `api.rs`

- **Purpose:**
    Defines REST endpoints and processes requests from the Web UI or external tools (e.g., curl).

- **Responsibilities:**

    - Handles REST API requests and routes them to the appropriate logic in `rest.rs`.
    - Provides IPC command support for the CLI.

---
### 3. `rest.rs`

- **Purpose:**
    Handles REST-based interactions for the Web UI and external clients.

- **Responsibilities:**

    - Processes REST API calls forwarded by `api.rs`.
    - Provides an interface between REST clients and the shared state.

---
### 4. `ipc.rs`

- **Purpose:**
    Facilitates direct communication between the CLI and the shared state.

- **Responsibilities:**

    - Receives CLI commands and performs state operations directly on `state.rs`.

---

### 5. `online.rs` / `offline.rs`

- **Purpose:**
    Handles mode-specific logic for online and offline operations.

- **Responsibilities:**

    - `online.rs`: Handles online mode operations and interactions with the REST API.
    - `offline.rs`: Handles offline mode operations and interacts directly with the saved state.

---

### 6. CLI and Web UI

- **CLI:**
    Provides terminal-based interaction with the system via `ipc.rs`.
- **Web UI:**
    Communicates with the REST API to perform operations on the shared state.

---

### Notes

- **Concurrency Handling:**
    Use `RwLock` or `Mutex` in `state.rs` to manage concurrent access from REST and IPC pathways.
- **Scalability:**
    Modularize the application into separate crates if the project grows in complexity.
