// src/app_state.rs
use crate::errors::{ApiError, LoadError};
use crate::models::{
    Bot, BotInsert, BotUpdate, Listener, ListenerInsert, ListenerList, ListenerUpdate, ListenerView,
};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use validator::Validate;

/// Represents the application state.
///
/// # Fields
///
/// * `bots` - A collection of trading bots indexed by their unique IDs.
/// * `file` - The file path where the application state is stored.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub bots: HashMap<String, Bot>,
    pub file: Option<PathBuf>,
}

impl Default for AppState {
    /// Provides a default implementation for `AppState`.
    ///
    /// # Returns
    ///
    /// A new `AppState` with an empty collection of bots and the state file set to "state.json".
    fn default() -> Self {
        AppState {
            bots: HashMap::new(),
            file: Some(PathBuf::from("state.json")),
        }
    }
}

impl AppState {
    /// Saves the current state to a JSON file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - An optional path to save the state file.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the state is saved successfully.
    /// * `Err(LoadError)` if an error occurs during saving.
    pub fn save<P: AsRef<Path>>(&self, file_path: Option<P>) -> Result<(), LoadError> {
        // Determine the file path based on the following priority:
        // 1. Use the provided `file_path` if Some.
        // 2. Else, use the `self.file` field if Some.
        // 3. Else, return an error.
        let path = if let Some(p) = file_path {
            p.as_ref().to_path_buf()
        } else if let Some(ref p) = self.file {
            p.clone()
        } else {
            return Err(LoadError::NoFilePathProvided);
        };

        // Serialize the AppState to JSON
        let state_json = serde_json::to_string_pretty(self).map_err(LoadError::JsonParseError)?;

        // Write the JSON to the file
        std::fs::write(&path, state_json).map_err(|e| LoadError::FileReadError {
            source: e,
            path: path.clone(),
        })?;

        info!("State saved to file: {:?}", &path);
        Ok(())
    }

    /// Inserts a new bot into the collection and returns the bot if successful.
    ///
    /// # Arguments
    ///
    /// * `bot` - The `Bot` instance to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(Bot)` if the bot was added successfully.
    /// * `Err(ApiError)` if a bot with the same `bot_id` already exists or if saving fails.
    pub fn insert_bot(&mut self, bot: Bot) -> Result<Bot, ApiError> {
        let bot_id = bot.bot_id.clone();

        // Check if the bot_id already exists in the HashMap
        if self.bots.contains_key(&bot_id) {
            return Err(ApiError::BotAlreadyExists(bot_id));
        }

        // Attempt to insert the new Bot
        // `HashMap::insert` returns `None` if the key was not present
        match self.bots.insert(bot_id.clone(), bot.clone()) {
            None => {
                // Save the state, propagating any errors as ApiError
                self.save::<PathBuf>(None)
                    .map_err(|e| ApiError::SaveError(e.to_string()))?;
                Ok(bot)
            } // Insertion successful
            Some(_) => Err(ApiError::InsertionError), // This case should not occur due to the prior check
        }
    }

    /// Creates a new bot using the provided input and inserts it into the collection.
    ///
    /// This method validates the provided input, generates a unique identifier for the bot
    /// if one is not provided, and ensures that a bot with the same identifier does not already exist.
    /// If successful, the bot is inserted into the collection and persisted.
    ///
    /// # Arguments
    ///
    /// * `args` - A [`BotInput`] struct containing the parameters for creating the bot:
    ///   - `bot_id`: Optional unique identifier for the bot. A UUID is generated if not provided.
    ///   - `name`: The name of the bot (required).
    ///   - `exchange`: The exchange the bot operates on (required).
    ///   - `api_key`: Optional API key for the exchange.
    ///   - `api_secret`: Optional API secret for the exchange.
    ///   - `rest_endpoint`: Optional REST API endpoint.
    ///   - `rpc_endpoint`: Optional RPC endpoint.
    ///   - `webhook_secret`: Optional webhook secret.
    ///   - `trading_fee`: Optional trading fee percentage.
    ///   - `private_key`: Optional private key.
    ///   - `contract_address`: Optional contract address.
    ///
    /// # Returns
    ///
    /// * `Ok(Bot)` - The created bot if successfully inserted into the collection.
    /// * `Err(ApiError)` - An error if:
    ///   - The input validation fails.
    ///   - A bot with the same `bot_id` already exists in the collection.
    ///   - There is a failure during the save operation.
    ///
    /// # Errors
    ///
    /// * [`ApiError::InvalidInput`] - If the input fails validation.
    /// * [`ApiError::BotAlreadyExists`] - If a bot with the same `bot_id` already exists.
    /// * [`ApiError::SaveError`] - If the bot cannot be persisted to the state.
    ///
    /// # Example
    ///
    /// ```rust
    /// let args = BotInput {
    ///     bot_id: None,
    ///     name: "TraderBot".to_string(),
    ///     exchange: "Binance".to_string(),
    ///     api_key: Some("api-key".to_string()),
    ///     api_secret: Some("api-secret".to_string()),
    ///     rest_endpoint: Some("https://api.binance.com".to_string()),
    ///     rpc_endpoint: None,
    ///     webhook_secret: Some("webhook-secret".to_string()),
    ///     trading_fee: Some(0.1),
    ///     private_key: None,
    ///     contract_address: None,
    /// };
    ///
    /// match app_state.create_bot(args) {
    ///     Ok(bot) => println!("Created bot: {:?}", bot),
    ///     Err(err) => eprintln!("Failed to create bot: {}", err),
    /// }
    /// ```
    pub fn create_bot(&mut self, args: BotInsert) -> Result<Bot, ApiError> {
        // Validate the input
        if let Err(validation_errors) = args.validate() {
            return Err(ApiError::InvalidInput(validation_errors.to_string()));
        }

        // Create and insert the bot
        self.insert_bot(Bot {
            bot_id: args
                .bot_id
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            name: args.name,
            exchange: args.exchange,
            api_key: args.api_key,
            api_secret: args.api_secret,
            rest_endpoint: args.rest_endpoint,
            rpc_endpoint: args.rpc_endpoint,
            webhook_secret: args.webhook_secret,
            trading_fee: args.trading_fee,
            private_key: args.private_key,
            contract_address: args.contract_address,
            listeners: HashMap::new(),
        })
    }

    /// Updates an existing bot's information.
    ///
    /// # Arguments
    ///
    /// * `bot_id` - A string slice that holds the unique identifier of the bot to update.
    /// * `updated_bot` - The `Bot` instance containing updated information.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the bot was updated successfully.
    /// * `Err(ApiError)` if the bot does not exist or if saving the state fails.
    pub fn args(&mut self, args: BotUpdate) -> Result<Bot, ApiError> {
        // Check if the bot exists and get a mutable reference to it
        let updated_bot = match self.bots.get_mut(&args.bot_id) {
            None => return Err(ApiError::BotNotFound(args.bot_id.to_string())), // Bot not found
            Some(bot) => {
                // Update fields
                bot.name = args.name.unwrap_or(bot.name.clone());
                bot.exchange = args.exchange.unwrap_or(bot.exchange.clone());
                bot.api_key = args.api_key.or(bot.api_key.clone());
                bot.api_secret = args.api_secret.or(bot.api_secret.clone());
                bot.rest_endpoint = args.rest_endpoint.or(bot.rest_endpoint.clone());
                bot.rpc_endpoint = args.rpc_endpoint.or(bot.rpc_endpoint.clone());
                bot.trading_fee = args.trading_fee.or(bot.trading_fee);
                bot.private_key = args.private_key.or(bot.private_key.clone());
                bot.contract_address = args.contract_address.or(bot.contract_address.clone());
                bot.clone() // Clone the updated bot to end the mutable borrow
            }
        };

        // Save the updated state
        self.save::<PathBuf>(None)
            .map_err(|e| ApiError::SaveError(e.to_string()))?;

        // Return the updated bot
        Ok(updated_bot)
    }

    /// Lists all bots as a vector.
    ///
    /// # Returns
    ///
    /// A vector containing clones of all `Bot` instances.
    pub fn list_bots(&self) -> Vec<Bot> {
        self.bots.values().cloned().collect()
    }

    /// Retrieves a bot by its unique identifier.
    ///
    /// # Arguments
    ///
    /// * `bot_id` - A string slice that holds the unique identifier of the bot to retrieve.
    ///
    /// # Returns
    ///
    /// * `Some(&Bot)` if a bot with the specified ID exists.
    /// * `None` if no bot with the specified ID is found.
    pub fn get_bot(&self, bot_id: &str) -> Option<&Bot> {
        self.bots.get(bot_id)
    }

    /// Deletes a bot from the application state.
    ///
    /// # Arguments
    ///
    /// * `bot_id` - A string slice that holds the unique identifier of the bot to delete.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the bot was deleted successfully.
    /// * `Err(ApiError)` if the bot does not exist or if saving the state fails.
    pub fn delete_bot(&mut self, bot_id: &str) -> Result<(), ApiError> {
        // Attempt to remove the bot
        if self.bots.remove(bot_id).is_none() {
            // Bot not found
            return Err(ApiError::BotNotFound(bot_id.to_string()));
        }

        // Save the updated state
        self.save::<PathBuf>(None)
            .map_err(|e| ApiError::SaveError(e.to_string()))?;

        info!("Deleted bot with ID: {}", bot_id);
        Ok(())
    }

    /// Add a listener to a bot
    pub fn add_listener(&mut self, args: ListenerInsert) -> Result<Listener, ApiError> {
        if let Some(bot) = self.bots.get_mut(&args.bot_id) {
            let listener_id = uuid::Uuid::new_v4().to_string();
            let listener = Listener {
                service: args.service,
                secret: args.secret.unwrap_or_default(),
                msg: args.msg.unwrap_or_default(),
            };

            match bot.listeners.insert(listener_id, listener.clone()) {
                None => {
                    // Save the state, propagating any errors as ApiError
                    self.save::<PathBuf>(None)
                        .map_err(|e| ApiError::SaveError(e.to_string()))?;
                    Ok(listener)
                } // Insertion successful
                Some(_) => Err(ApiError::InsertionError),
            }
        } else {
            Err(ApiError::BotNotFound(args.bot_id.to_string()))
        }
    }

    /// List all listeners for a bot, flattened with their IDs included.
    ///
    /// This function retrieves listeners for a specific bot and optionally filters them based on
    /// `listener_id` and `service`. The `bot_id` is validated within this function.
    ///
    /// # Arguments
    ///
    /// - `args` - The filtering criteria, encapsulated in the `ListenerList` struct.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<ListenerView>)` - A vector of matching listeners, each represented as a `ListenerView`.
    /// - `Err(ApiError)` - If `bot_id` is empty, no matching listeners are found, or the bot ID is invalid.
    pub fn list_listeners(&self, args: ListenerList) -> Result<Vec<ListenerView>, ApiError> {
        // Validate `bot_id` explicitly
        if args.bot_id.trim().is_empty() {
            return Err(ApiError::ValidationError(
                "Bot ID cannot be empty".to_string(),
            ));
        }

        // Retrieve the bot using the provided `bot_id`
        let bot = self
            .bots
            .get(&args.bot_id)
            .ok_or(ApiError::BotNotFound(args.bot_id.clone()))?;

        // Filter listeners based on optional arguments (`listener_id` and `service`)
        let filtered_listeners: Vec<ListenerView> = bot
            .listeners
            .iter()
            .filter(|(listener_id, listener)| {
                // Filter by `listener_id` if provided
                args.listener_id
                    .as_ref()
                    .map_or(true, |id| id == *listener_id)
                    &&
                // Filter by `service` if provided
                args.service
                    .as_ref()
                    .map_or(true, |service| service == &listener.service)
            })
            .map(|(listener_id, listener)| ListenerView {
                bot_id: bot.bot_id.clone(),
                listener_id: listener_id.clone(),
                service: Some(listener.service.clone()),
                secret: Some(listener.secret.clone()),
                msg: Some(listener.msg.clone()),
            })
            .collect();

        // If no matching listeners are found, return an error
        if filtered_listeners.is_empty() {
            Err(ApiError::ListenerNotFound(
                "No matching listeners found".to_string(),
            ))
        } else {
            Ok(filtered_listeners) // Return the filtered listeners
        }
    }

    /// Retrieve a specific listener based on the given criteria.
    ///
    /// This function delegates filtering to `list_listeners` and ensures only a single
    /// matching listener is returned as a `ListenerView`. If no listeners or multiple
    /// listeners match, an appropriate error is returned.
    ///
    /// # Arguments
    ///
    /// - `args`: The filtering criteria encapsulated in the `ListenerList` struct.
    ///
    /// # Returns
    ///
    /// - `Ok(ListenerView)`: The single matching listener as a `ListenerView`.
    /// - `Err(ApiError)`: If no listeners match, multiple listeners match, or other errors occur.
    pub fn get_listener(&self, args: ListenerList) -> Result<ListenerView, ApiError> {
        // Delegate filtering to `list_listeners`
        let listeners = self.list_listeners(args)?;

        match listeners.len() {
            0 => Err(ApiError::ListenerNotFound(
                "No matching listeners found".to_string(),
            )),
            1 => Ok(listeners.into_iter().next().unwrap()), // Safe because len() == 1
            _ => Err(ApiError::NonUniqueResult(
                "Multiple matching listeners found".to_string(),
            )),
        }
    }

    /// Delete listener(s) based on the given criteria.
    ///
    /// This function leverages `list_listeners` to identify listeners that match the criteria and removes them.
    ///
    /// # Arguments
    ///
    /// - `args`: The filtering criteria encapsulated in the `ListenerList` struct.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the listener(s) were deleted successfully.
    /// - `Err(ApiError)`: If no listeners match the criteria or other errors occur.
    pub fn delete_listeners(&mut self, args: ListenerList) -> Result<(), ApiError> {
        // Validate `bot_id` explicitly
        if args.bot_id.trim().is_empty() {
            return Err(ApiError::ValidationError(
                "Bot ID cannot be empty".to_string(),
            ));
        }

        // Delegate filtering to `list_listeners` to find matching listeners
        let listeners_to_delete = self.list_listeners(args.clone())?;

        // Retrieve the bot for mutation
        let bot = self
            .bots
            .get_mut(&args.bot_id)
            .ok_or(ApiError::BotNotFound(args.bot_id.clone()))?;

        // Remove the matching listeners
        for listener_view in listeners_to_delete {
            bot.listeners.remove(&listener_view.listener_id);
        }

        // If all listeners are deleted successfully, return Ok
        Ok(())
    }

    /// Update a listener based on the provided criteria and return it as a `ListenerView`.
    ///
    /// If `bot_id` is provided, it updates the listener in that specific bot.
    /// Otherwise, it searches all bots for the listener and updates it.
    ///
    /// # Arguments
    ///
    /// - `args`: The update criteria and new values encapsulated in the `ListenerUpdate` struct.
    ///
    /// # Returns
    ///
    /// - `Ok(ListenerView)`: The updated listener formatted as a `ListenerView`.
    /// - `Err(ApiError)`: If the listener or bot is not found.
    pub fn update_listener(&mut self, args: ListenerUpdate) -> Result<ListenerView, ApiError> {
        // Try to find the listener, either in the specified bot or across all bots
        let mut target_listener: Option<(&mut Listener, String)> = None;

        if let Some(bot_id) = &args.bot_id {
            // Search in the specified bot
            let bot = self
                .bots
                .get_mut(bot_id)
                .ok_or_else(|| ApiError::BotNotFound(bot_id.clone()))?;
            if let Some(listener) = bot.listeners.get_mut(&args.listener_id) {
                target_listener = Some((listener, bot_id.clone()));
            }
        } else {
            // Search across all bots
            for (bot_id, bot) in self.bots.iter_mut() {
                if let Some(listener) = bot.listeners.get_mut(&args.listener_id) {
                    target_listener = Some((listener, bot_id.clone()));
                    break;
                }
            }
        }

        // If no listener was found, return an error
        let (listener, bot_id) = target_listener.ok_or_else(|| {
            ApiError::ListenerNotFound(format!(
                "Listener ID `{}` not found in any bot",
                args.listener_id
            ))
        })?;

        // Update the listener with the new values
        if let Some(service) = args.service {
            listener.service = service;
        }
        if let Some(secret) = args.secret {
            listener.secret = secret;
        }
        if let Some(msg) = args.msg {
            listener.msg = msg;
        }

        // Map the updated listener to ListenerView
        Ok(ListenerView {
            bot_id,
            listener_id: listener.listener_id.clone(),
            service: Some(listener.service.clone()),
            secret: Some(listener.secret.clone()),
            msg: Some(listener.msg.clone()),
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::models::{Bot, Listener};
//     use serde_json::json;
//     //use std::collections::HashMap;
//     use std::fs;
//     use std::path::Path;
//
//     /// Helper function to create a mock bot
//     fn mock_bot(bot_id: &str) -> Bot {
//         Bot {
//             bot_id: bot_id.to_string(),
//             name: "TestBot".to_string(),
//             exchange: "Binance".to_string(),
//             api_key: Some("api_key".to_string()),
//             api_secret: Some("api_secret".to_string()),
//             rest_endpoint: Some("http://rest_endpoint".to_string()),
//             rpc_endpoint: Some("http://rpc_endpoint".to_string()),
//             webhook_secret: Some("webhook_secret".to_string()),
//             trading_fee: Some(0.1),
//             private_key: Some("private_key".to_string()),
//             contract_address: Some("contract_address".to_string()),
//             listeners: Vec::new(),
//         }
//     }
//
//     /// Helper function to create a mock listener
//     fn mock_listener(listener_id: &str) -> Listener {
//         Listener {
//             listener_id: listener_id.to_string(),
//             service: "TestService".to_string(),
//             secret: "secret123".to_string(),
//             msg: json!({"key": "value"}),
//         }
//     }
//
//     #[test]
//     fn test_add_and_list_bots() {
//         let mut app_state = AppState::default();
//         let bot = mock_bot("bot123");
//         app_state.add_bot(bot);
//
//         let bots = app_state.list_bots();
//         assert_eq!(bots.len(), 1);
//         assert_eq!(bots[0].bot_id, "bot123");
//     }
//
//     #[test]
//     fn test_get_bot() {
//         let mut app_state = AppState::default();
//         let bot = mock_bot("bot123");
//         app_state.add_bot(bot);
//
//         let bot = app_state.get_bot("bot123");
//         assert!(bot.is_some());
//         assert_eq!(bot.unwrap().bot_id, "bot123");
//     }
//
//     #[test]
//     fn test_update_bot() {
//         let mut app_state = AppState::default();
//         let bot = mock_bot("bot123");
//         app_state.add_bot(bot);
//
//         let updated_bot = Bot {
//             bot_id: "bot123".to_string(),
//             name: "UpdatedBot".to_string(),
//             ..mock_bot("bot123")
//         };
//
//         let result = app_state.update_bot("bot123", updated_bot.clone());
//         assert!(result.is_ok());
//
//         let bot = app_state.get_bot("bot123").unwrap();
//         assert_eq!(bot.name, "UpdatedBot");
//     }
//
//     #[test]
//     fn test_delete_bot() {
//         let mut app_state = AppState::default();
//         let bot = mock_bot("bot123");
//         app_state.add_bot(bot);
//
//         let result = app_state.delete_bot("bot123");
//         assert!(result.is_ok());
//         assert!(app_state.get_bot("bot123").is_none());
//     }
//
//     #[test]
//     fn test_add_and_list_listeners() {
//         let mut app_state = AppState::default();
//         let bot = mock_bot("bot123");
//         app_state.add_bot(bot);
//
//         let listener = mock_listener("listener123");
//         let result = app_state.add_listener("bot123", listener.clone());
//         assert!(result.is_ok());
//         assert_eq!(result.unwrap(), "listener123");
//
//         let listeners = app_state.list_listeners("bot123").unwrap();
//         assert_eq!(listeners.len(), 1);
//         assert_eq!(listeners[0].listener_id, "listener123");
//     }
//
//     #[test]
//     fn test_get_listener() {
//         let mut app_state = AppState::default();
//         let bot = mock_bot("bot123");
//         app_state.add_bot(bot);
//
//         let listener = mock_listener("listener123");
//         app_state.add_listener("bot123", listener.clone()).unwrap();
//
//         let fetched_listener = app_state.get_listener("listener123");
//         assert!(fetched_listener.is_some());
//         assert_eq!(fetched_listener.unwrap().listener_id, "listener123");
//     }
//
//     #[test]
//     fn test_update_listener() {
//         let mut app_state = AppState::default();
//         let bot = mock_bot("bot123");
//         app_state.add_bot(bot);
//
//         let listener = mock_listener("listener123");
//         app_state.add_listener("bot123", listener.clone()).unwrap();
//
//         let updated_listener = Listener {
//             listener_id: "listener123".to_string(),
//             service: "UpdatedService".to_string(),
//             ..listener
//         };
//
//         let result = app_state.update_listener("listener123", updated_listener.clone());
//         assert!(result.is_ok());
//
//         let listener = app_state.get_listener("listener123").unwrap();
//         assert_eq!(listener.service, "UpdatedService");
//     }
//
//     #[test]
//     fn test_delete_listener() {
//         let mut app_state = AppState::default();
//         let bot = mock_bot("bot123");
//         app_state.add_bot(bot);
//
//         let listener = mock_listener("listener123");
//         app_state.add_listener("bot123", listener.clone()).unwrap();
//
//         let result = app_state.delete_listener("listener123");
//         assert!(result.is_ok());
//
//         let listeners = app_state.list_listeners("bot123").unwrap();
//         assert!(listeners.is_empty());
//     }
//
//     #[test]
//     fn test_load_and_save_state() {
//         let file_path = "test_state.json";
//         let mut app_state = AppState::default();
//
//         // Add a bot
//         let bot = mock_bot("bot123");
//         app_state.add_bot(bot);
//
//         // Save state to file
//         app_state.save_state_to_file(file_path);
//         assert!(Path::new(file_path).exists());
//
//         // Load state from file
//         let loaded_state = AppState::load_state_from_file(file_path).unwrap();
//         assert_eq!(loaded_state.bots.len(), 1);
//         assert_eq!(loaded_state.bots["bot123"].bot_id, "bot123");
//
//         // Clean up
//         fs::remove_file(file_path).unwrap();
//     }
// }
//
