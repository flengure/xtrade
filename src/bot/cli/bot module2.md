
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

    classDef ui fill:#005f73,stroke:#0a9396,stroke-width:2px,font-size:14px,color:#ffffff,font-weight:bold;
    classDef online fill:#d1e7dd,stroke:#0f5132,stroke-width:2px,font-size:14px,color:#0f5132;
    classDef offline fill:#f8d7da,stroke:#842029,stroke-width:2px,font-size:14px,color:#842029;
    classDef ipc fill:#cce5ff,stroke:#004085,stroke-width:2px,font-size:14px,color:#004085;
    classDef central fill:#ffeeba,stroke:#856404,stroke-width:2px,font-size:14px,color:#856404,font-weight:bold;
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

