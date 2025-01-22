# state Module

The state module handles the application’s runtime state and configuration management. It provides mechanisms to load, save, and manipulate both the application state (AppState) and configuration settings (AppConfig). This modular approach ensures that state and configuration are isolated but easily integrated into the broader application.

## Module Overview

### state.rs

This file contains the core logic for managing the runtime application state, including bots and configuration. The state is stored in a JSON file (state.json by default) and is loaded or created when the application starts. This state includes information about:
	•	Bots: Details about each bot the application manages (e.g., name, exchange, configurations).
	•	Configuration (AppConfig): The runtime configuration settings for various subsystems (API, WebClient, Webhook).

Key Features:
	1.	Loading State:
	•	Loads the application state from a JSON file.
	•	If the file is missing, it creates a new blank file and writes an empty JSON object ({}).
	•	Validates that the file is writable before proceeding.
	2.	Saving State:
	•	Serializes the AppState to JSON and writes it back to the file.
	•	Ensures that configuration settings (AppConfig) are also saved.

Limitations:
	•	Memory Constraints: Since the state (including all bots) is kept in memory as a HashMap, scaling to thousands of bots could become infeasible. In such cases, the state management should migrate to a database.

### settings.rs

This file focuses on configuration management using AppConfig. It provides a centralized way to manage settings for different subsystems like:
	•	API Server:
	•	Port, bind address, and state file path.
	•	Web Client:
	•	Static file path, port, and whether it is enabled.
	•	Webhook Server:
	•	Port and bind address.
	•	Remote Server:
	•	URL for online mode.
	•	Local State:
	•	File path for offline mode.

Key Features:
	1.	Default Configuration:
	•	Provides sensible defaults for all configuration settings.
	•	Includes values like default ports, bind addresses, and file paths.
	2.	Loading Configuration:
	•	Loads configuration from a TOML file (config.toml by default).
	•	Falls back to defaults if the file is missing or invalid and creates a new configuration file.
	3.	Saving Configuration:
	•	Serializes the configuration to TOML and saves it to a file.
	•	Handles environment variable overrides and custom file paths.

Integration Between state.rs and settings.rs
	1.	State-Configuration Dependency:
	•	AppState depends on AppConfig for runtime settings like file paths and server configurations.
	•	The state_file_path defined in AppConfig determines where the application state is saved or loaded from.
	2.	Error Handling:
	•	Both state.rs and settings.rs include robust error handling for file I/O, JSON parsing, and configuration deserialization.
	3.	Test Coverage:
	•	Comprehensive tests ensure that the state is loaded, saved, and created correctly.
	•	Tests verify the integration between state.rs and settings.rs.

TODOs and Future Enhancements
	1.	State Migration to a Database:
	•	Replace the in-memory HashMap storage for bots with a database (e.g., SQLite, PostgreSQL) to handle larger datasets and improve scalability.
	2.	Dynamic Configuration Reload:
	•	Implement mechanisms to reload AppConfig at runtime without restarting the application.
	3.	Validation and Schema Enforcement:
	•	Add strict validation for AppState and AppConfig to ensure data consistency.
	4.	Concurrency Improvements:
	•	Introduce locks or asynchronous state management for better performance in multi-threaded environments.
	5.	Custom Configuration Locations:
	•	Extend support for runtime-specified configuration paths via CLI arguments or environment variables.

Example Configurations and State Files

config.toml
```toml

[api_server]
port = 7762
bind_address = "127.0.0.1"
state_file_path = "state.json"

[web_client]
is_enabled = true
port = 7763
bind_address = "0.0.0.0"
static_files_path = "src/webui/dist"

[webhook_server]
is_enabled = true
port = 7764
bind_address = "0.0.0.0"

[remote_server]
api_url = "http://localhost:7762"

[local_state]
state_file_path = "state.json"
```
Example state.json
```json
{
  "bots": {
    "bot1": {
      "name": "Test Bot",
      "exchange": "Test Exchange"
    }
  },
  "config": {
    "api_server": {
      "port": 7762,
      "bind_address": "127.0.0.1",
      "state_file_path": "state.json"
    }
  }
}
```
This modular structure allows the application to manage its state and configuration effectively while being ready for future scalability improvements.
