pub mod input;
pub mod output;

pub use input::{BotInsertArgs, BotListArgs, BotUpdateArgs};
pub use input::{ListenerInsertArgs, ListenerListArgs, ListenerUpdateArgs};
pub use output::{BotListView, BotView};
pub use output::{ListenerListView, ListenerView};

pub use crate::bot::model::{Bot, Listener};
use crate::errors::ApiError;
pub use crate::state::AppState;
use log::info;
use std::path::PathBuf;

impl AppState {
    /// Utility: Get a mutable reference to a bot by ID.
    fn get_bot_mut(&mut self, bot_id: &str) -> Result<&mut Bot, ApiError> {
        self.bots
            .get_mut(bot_id)
            .ok_or_else(|| ApiError::BotNotFound(format!("Bot with ID '{}' not found.", bot_id)))
    }

    /// Utility: Get a reference to a bot by ID.
    fn get_bot_ref(&self, bot_id: &str) -> Result<&Bot, ApiError> {
        self.bots
            .get(bot_id)
            .ok_or_else(|| ApiError::BotNotFound(format!("Bot with ID '{}' not found.", bot_id)))
    }

    /// Utility: Validate a bot ID.
    fn validate_bot_id(&self, bot_id: &str) -> Result<(), ApiError> {
        if bot_id.trim().is_empty() {
            Err(ApiError::ValidationError(
                "Bot ID cannot be empty.".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    /// Clear all bots and save the updated state.
    pub fn clear_bots(&mut self) -> Result<(), ApiError> {
        self.bots.clear();
        self.save::<PathBuf>(None)
            .map_err(|e| ApiError::SaveError(e.to_string()))?;
        info!("Successfully cleared all bots.");
        Ok(())
    }

    /// Clear all listeners from all bots and save the updated state.
    pub fn clear_listeners(&mut self) -> Result<(), ApiError> {
        for bot in self.bots.values_mut() {
            bot.listeners.clear();
        }
        self.save::<PathBuf>(None)
            .map_err(|e| ApiError::SaveError(e.to_string()))?;
        info!("Successfully cleared all listeners.");
        Ok(())
    }

    /// Add a bot to the application state.
    pub fn add_bot(&mut self, args: BotInsertArgs) -> Result<BotView, ApiError> {
        let bot: Bot = args.into();
        if self.bots.contains_key(&bot.bot_id) {
            return Err(ApiError::BotAlreadyExists(bot.bot_id.clone()));
        }

        self.bots.insert(bot.bot_id.clone(), bot.clone());
        println!("hello");
        self.save::<PathBuf>(None)
            .map_err(|e| ApiError::SaveError(e.to_string()))?;
        Ok(bot.into())
    }

    /// List all bots, optionally filtering by provided arguments.
    pub fn list_bots(&self, args: Option<BotListArgs>) -> Result<BotListView, ApiError> {
        let filtered_bots: Vec<BotView> = self
            .bots
            .values()
            .filter(|bot| args.as_ref().map_or(true, |filters| filters.matches(bot)))
            .map(|bot| bot.clone().into())
            .collect();

        if filtered_bots.is_empty() {
            return Err(ApiError::NotFound("No bots found.".to_string()));
        }

        Ok(BotListView(filtered_bots))
    }

    /// Get a specific bot by ID.
    pub fn get_bot(&self, bot_id: &str) -> Result<BotView, ApiError> {
        let bot = self.get_bot_ref(bot_id)?;
        Ok(bot.clone().into())
    }

    /// Update an existing bot.
    pub fn update_bot(&mut self, bot_id: &str, args: BotUpdateArgs) -> Result<BotView, ApiError> {
        let bot_clone = {
            let bot = self.get_bot_mut(bot_id)?;
            args.apply(bot);
            bot.clone()
        };

        self.save::<PathBuf>(None)
            .map_err(|e| ApiError::SaveError(e.to_string()))?;
        Ok(bot_clone.into())
    }

    /// Delete a bot and return its view.
    pub fn delete_bot(&mut self, bot_id: &str) -> Result<BotView, ApiError> {
        let bot = self
            .bots
            .remove(bot_id)
            .ok_or_else(|| ApiError::BotNotFound(format!("Bot with ID '{}' not found.", bot_id)))?;

        self.save::<PathBuf>(None)
            .map_err(|e| ApiError::SaveError(e.to_string()))?;
        Ok(bot.into())
    }

    /// Add a listener to a bot.
    pub fn add_listener(
        &mut self,
        bot_id: &str,
        args: ListenerInsertArgs,
    ) -> Result<ListenerView, ApiError> {
        let bot = self.get_bot_mut(bot_id)?;
        let listener_id = args
            .listener_id
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let listener = Listener {
            service: args.service,
            secret: args.secret.unwrap_or_default(),
            msg: args.msg.unwrap_or_default(),
        };

        bot.listeners.insert(listener_id.clone(), listener.clone());
        self.save::<PathBuf>(None)
            .map_err(|e| ApiError::SaveError(e.to_string()))?;
        Ok((bot_id, listener_id.as_str(), &listener).into())
    }

    /// List listeners for a bot, optionally filtering by arguments.
    pub fn list_listeners(
        &self,
        bot_id: &str,
        args: ListenerListArgs,
    ) -> Result<ListenerListView, ApiError> {
        self.validate_bot_id(bot_id)?;
        let bot = self.get_bot_ref(bot_id)?;

        let filtered_listeners: Vec<ListenerView> = bot
            .listeners
            .iter()
            .filter(|(id, listener)| args.matches(id, listener))
            .map(|(id, listener)| (bot_id, id.as_str(), listener).into())
            .collect();

        if filtered_listeners.is_empty() {
            return Err(ApiError::ListenerNotFound(
                "No matching listeners found.".to_string(),
            ));
        }

        Ok(ListenerListView(filtered_listeners))
    }

    /// Get a specific listener by bot ID and listener ID.
    pub fn get_listener(&self, bot_id: &str, listener_id: &str) -> Result<ListenerView, ApiError> {
        let bot = self.get_bot_ref(bot_id)?;
        let listener = bot.listeners.get(listener_id).ok_or_else(|| {
            ApiError::ListenerNotFound(format!(
                "Listener with ID '{}' not found in bot '{}'.",
                listener_id, bot_id
            ))
        })?;
        Ok((bot_id, listener_id, listener).into())
    }

    /// Update a specific listener by bot ID and listener ID.
    pub fn update_listener(
        &mut self,
        bot_id: &str,
        args: ListenerUpdateArgs,
    ) -> Result<ListenerView, ApiError> {
        let updated_listener_view = {
            let bot = self.get_bot_mut(bot_id)?;
            let listener = bot.listeners.get_mut(&args.listener_id).ok_or_else(|| {
                ApiError::ListenerNotFound(format!(
                    "Listener ID '{}' not found in bot '{}'.",
                    args.listener_id, bot_id
                ))
            })?;

            // Apply updates to the listener
            args.apply(listener);

            // Create an immutable reference to the updated listener
            let listener_ref = &*listener;

            // Convert to ListenerView using the immutable reference
            (bot_id, args.listener_id.as_str(), listener_ref).into()
        };

        // Save the updated state
        self.save::<PathBuf>(None)
            .map_err(|e| ApiError::SaveError(e.to_string()))?;

        Ok(updated_listener_view)
    }

    /// Delete a specific listener by bot ID and listener ID.
    pub fn delete_listener(
        &mut self,
        bot_id: &str,
        listener_id: &str,
    ) -> Result<ListenerView, ApiError> {
        let listener_clone = {
            let bot = self.get_bot_mut(bot_id)?;
            bot.listeners.remove(listener_id).ok_or_else(|| {
                ApiError::ListenerNotFound(format!(
                    "Listener with ID '{}' not found in bot '{}'.",
                    listener_id, bot_id
                ))
            })
        };

        self.save::<PathBuf>(None)
            .map_err(|e| ApiError::SaveError(e.to_string()))?;
        Ok((bot_id, listener_id, &listener_clone?).into())
    }

    /// Delete listeners matching criteria.
    pub fn delete_listeners(
        &mut self,
        bot_id: &str,
        args: ListenerListArgs,
    ) -> Result<ListenerListView, ApiError> {
        let mut deleted_listeners = Vec::new();

        {
            let bot = self.get_bot_mut(bot_id)?;
            bot.listeners.retain(|listener_id, listener| {
                if args.matches(listener_id, listener) {
                    // Create an immutable reference to the listener
                    let listener_ref = &*listener;

                    // Add the listener to the deleted list using the immutable reference
                    deleted_listeners.push((bot_id, listener_id.as_str(), listener_ref).into());
                    false // Remove this listener
                } else {
                    true // Retain this listener
                }
            });
        }

        if deleted_listeners.is_empty() {
            return Err(ApiError::ListenerNotFound(
                "No matching listeners found.".to_string(),
            ));
        }

        self.save::<PathBuf>(None)
            .map_err(|e| ApiError::SaveError(e.to_string()))?;
        Ok(ListenerListView(deleted_listeners))
    }
}
