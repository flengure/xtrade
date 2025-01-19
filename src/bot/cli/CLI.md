

> [!NOTE] Title
> Contents




```mermaid
flowchart TD
    A[state.rs\nProvides shared state]
    B[api.rs\nDefines API endpoints]
    C[rest.rs\nHandles REST requests]
    D[offline.rs\nManages offline mode]
    E[online.rs\nManages online mode]
    F((Terminal / CLI)):::ui
    G(((Browser / curl))):::ui

    A <-- Data struct --> B
    A <-- Data struct --> D
    B <-- Request\nResponse --> C
    C <-- Request\nResponse --> G
    C <-- Request\nResponse --> E
    E <-- Text --> F
    D <-- Text --> F

    classDef ui fill:#005f73,stroke:#0a9396,stroke-width:2px,font-size:14px,color:#ffffff,font-weight:bold;
```




```