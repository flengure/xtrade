```mermaid
flowchart TD
    A[state.rs<br>Provides Shared/Saved state]:::central
    A1[state.rs<br>Provides Shared state]:::online
    A2[state.rs<br>Provides Saved state]:::offline
    B1[api.rs<br>Defines API endpoints]:::online
    C1[rest.rs<br>Handles REST requests]:::online
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
    C1 <-- Request\nResponse --> G
    C1 <-- Request\nResponse --> D2
    D2 <-- Text --> F2
    D1 <-- Text --> F1

    classDef ui fill:#005f73,stroke:#0a9396,stroke-width:2px,font-size:14px,color:#ffffff,font-weight:bold;
    classDef online fill:#d1e7dd,stroke:#0f5132,stroke-width:2px,font-size:14px,color:#0f5132;
    classDef offline fill:#f8d7da,stroke:#842029,stroke-width:2px,font-size:14px,color:#842029;
    classDef central fill:#ffeeba,stroke:#856404,stroke-width:2px,font-size:14px,color:#856404,font-weight:bold;
```
```
Offline mode is a maintenance mode just to maintain the offline data
does not represent the actual state

Cli commands need to use online mode for the actial state
```
