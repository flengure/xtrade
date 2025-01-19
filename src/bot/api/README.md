# bot module Architecture Documentation

## Architecture Diagram

The following diagram illustrates the relationships between the key components of the application architecture.
**Note:** All interactions with `state.rs` are mediated through `rest.rs`:

```mermaid
flowchart TD

    A1[state.rs<br>Provides Shared state]:::online
    A2[state.rs<br>Provides Saved state]:::offline

    B1[api.rs<br>Defines API endpoints<br>Handles IPC commands]:::online
    C1[rest.rs<br>Handles REST requests<br>Manages state interactions]:::online

    D1[online.rs<br>Manages online mode]:::online
    D2[offline.rs<br>Manages offline mode]:::online
    F1((Terminal / CLI)):::ui
    F2((Terminal / CLI)):::ui
    F3((Terminal / CLI)):::ui
    G((Browser / curl)):::ui
    I1[ipc.rs<br>IPC Interface]:::ipc



    A1 <-- Data struct --> B1
    A1 <-- Data struct --> I1
    A2 <-- Data struct --> D2
    D2 <-- Text --> F2

    B1 <-- Request\nResponse --> C1
    C1 <-- Request\nResponse --> G
    C1 <-- Request\nResponse --> D1
    I1 <-- Text --> F3
    D1 <--> F1


---
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
  - All internal processes must interact with `state.rs` through `rest.rs` for now.
  - future enhancement may provide **Supplementary Mechanisms:**
    - **IPC Channels:** Lightweight, asynchronous communication for internal state access.
    - **RPC (Remote Procedure Calls):** For specific, higher-level interactions across components.

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

    classDef ui fill:#005f73,stroke:#0a9396,stroke-width:2px,font-size:14px,color:#ffffff,font-weight:bold;
    classDef online fill:#d1e7dd,stroke:#0f5132,stroke-width:2px,font-size:14px,color:#0f5132;
    classDef offline fill:#f8d7da,stroke:#842029,stroke-width:2px,font-size:14px,color:#842029;
    classDef ipc fill:#cce5ff,stroke:#004085,stroke-width:2px,font-size:14px,color:#004085;
    classDef central fill:#ffeeba,stroke:#856404,stroke-width:2px,font-size:14px,color:#856404,font-weight:bold;
