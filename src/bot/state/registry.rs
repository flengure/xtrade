use crate::bot::state::{
    AppState, Bot, BotDeleteArgs, BotGetArgs, BotInsertArgs, BotListArgs, BotListView,
    BotUpdateArgs, BotView, Listener, ListenerDeleteArgs, ListenerGetArgs, ListenerInsertArgs,
    ListenerListArgs, ListenerListView, ListenerUpdateArgs, ListenerView, ListenersDeleteArgs,
};
use crate::errors::AppError;
use log::info;
use std::path::PathBuf;

pub trait BotRegistry {
    // Bot-related utils
    fn get_bot_mut(&mut self, bot_id: &str) -> Result<&mut Bot, AppError>;
    fn get_bot_ref(&self, bot_id: &str) -> Result<&Bot, AppError>;
    fn add_bot(&mut self, args: BotInsertArgs) -> Result<BotView, AppError>;
    fn list_bots(&self, args: Option<BotListArgs>) -> Result<BotListView, AppError>;
    fn get_bot(&self, args: BotGetArgs) -> Result<BotView, AppError>;
    fn update_bot(&mut self, args: BotUpdateArgs) -> Result<BotView, AppError>;
    fn delete_bot(&mut self, args: BotDeleteArgs) -> Result<BotView, AppError>;
    fn validate_bot_id(&self, bot_id: &str) -> Result<(), AppError>;

    // Listener-related methods
    fn get_listener_mut(
        &mut self,
        bot_id: &str,
        listener_id: &str,
    ) -> Result<&mut Listener, AppError>;
    fn get_listener_ref(&self, bot_id: &str, listener_id: &str) -> Result<&Listener, AppError>;
    fn add_listener(&mut self, args: ListenerInsertArgs) -> Result<ListenerView, AppError>;
    fn list_listeners(&self, args: ListenerListArgs) -> Result<ListenerListView, AppError>;
    fn get_listener(&self, args: ListenerGetArgs) -> Result<ListenerView, AppError>;
    fn update_listener(&mut self, args: ListenerUpdateArgs) -> Result<ListenerView, AppError>;
    fn delete_listener(&mut self, args: ListenerDeleteArgs) -> Result<ListenerView, AppError>;
    fn delete_listeners(&mut self, args: ListenersDeleteArgs)
        -> Result<ListenerListView, AppError>;

    // Utility methods for clearing data
    fn clear_bots(&mut self) -> Result<(), AppError>;
    fn clear_listeners(&mut self) -> Result<(), AppError>;
}

/// These are the primary state management functions
/// They use input and output templates
/// Errors ready for propagation
/// called by
///  crate::bot::api::endpoint::<function name> - REST endpoint
///  crate::bot::cli::offline::run - CLI offline mode
///
impl BotRegistry for AppState {
    // Bot methods
    fn get_bot_mut(&mut self, bot_id: &str) -> Result<&mut Bot, AppError> {
        self.bots
            .get_mut(bot_id)
            .ok_or_else(|| AppError::BotNotFound(format!("Bot with ID '{}' not found.", bot_id)))
    }

    fn get_bot_ref(&self, bot_id: &str) -> Result<&Bot, AppError> {
        self.bots
            .get(bot_id)
            .ok_or_else(|| AppError::BotNotFound(format!("Bot with ID '{}' not found.", bot_id)))
    }

    fn get_listener_mut(
        &mut self,
        bot_id: &str,
        listener_id: &str,
    ) -> Result<&mut Listener, AppError> {
        self.get_bot_mut(bot_id)?
            .listeners
            .get_mut(listener_id)
            .ok_or_else(|| {
                AppError::BotNotFound(format!(
                    "Listener with ID '{}' and Bot ID '{}' not found.",
                    listener_id, bot_id
                ))
            })
    }

    fn get_listener_ref(&self, bot_id: &str, listener_id: &str) -> Result<&Listener, AppError> {
        self.get_bot_ref(bot_id)?
            .listeners
            .get(listener_id)
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "Listener with ID '{}' not found in bot '{}'.",
                    listener_id, bot_id
                ))
            })
    }

    /// Utility: Validate a bot ID.
    fn validate_bot_id(&self, bot_id: &str) -> Result<(), AppError> {
        (!bot_id.trim().is_empty())
            .then_some(())
            .ok_or_else(|| AppError::ValidationError("Bot ID cannot be empty.".to_string()))
    }

    /// Clear all bots and save the updated state.
    fn clear_bots(&mut self) -> Result<(), AppError> {
        self.bots.clear();
        self.save::<PathBuf>(None)?;
        info!("Successfully cleared all bots.");
        Ok(())
    }

    /// Clear all listeners from all bots and save the updated state.
    fn clear_listeners(&mut self) -> Result<(), AppError> {
        for bot in self.bots.values_mut() {
            bot.listeners.clear();
        }
        self.save::<PathBuf>(None)?;
        info!("Successfully cleared all listeners.");
        Ok(())
    }

    /// Add a bot to the application state.
    /// base add_bot function
    fn add_bot(&mut self, args: BotInsertArgs) -> Result<BotView, AppError> {
        let bot: Bot = args.into();
        if self.bots.contains_key(&bot.bot_id) {
            return Err(AppError::BotAlreadyExists(bot.bot_id.clone()));
        }
        self.bots.insert(bot.bot_id.clone(), bot.clone());
        self.save::<PathBuf>(None)?;
        Ok(bot.into())
    }

    /// List all bots, optionally filtering by provided arguments.
    fn list_bots(&self, args: Option<BotListArgs>) -> Result<BotListView, AppError> {
        let filtered_bots: Vec<BotView> = self
            .bots
            .values()
            .filter(|bot| args.as_ref().map_or(true, |filters| filters.matches(bot)))
            .map(|bot| bot.clone().into())
            .collect();
        if filtered_bots.is_empty() {
            return Err(AppError::NotFound("No bots found.".to_string()));
        }
        Ok(BotListView(filtered_bots))
    }

    /// Get a specific bot by ID.
    fn get_bot(&self, args: BotGetArgs) -> Result<BotView, AppError> {
        self.get_bot_ref(&args.bot_id).map(|bot| bot.clone().into())
    }

    /// Update an existing bot.
    fn update_bot(&mut self, args: BotUpdateArgs) -> Result<BotView, AppError> {
        let bot_clone = {
            // Retrieve the bot mutably and apply updates
            let bot = self.get_bot_mut(&args.bot_id)?;
            args.apply(bot);
            bot.clone() // Clone the updated bot for the response
        };

        // Save the updated state to the persistent storage
        self.save::<PathBuf>(None)?;

        // Return the updated bot as a `BotView`
        Ok(bot_clone.into())
    }

    /// Delete a bot and return its view.
    fn delete_bot(&mut self, args: BotDeleteArgs) -> Result<BotView, AppError> {
        let bot = self.bots.remove(&args.bot_id).ok_or_else(|| {
            AppError::BotNotFound(format!("Bot with ID '{}' not found.", &args.bot_id))
        })?;
        self.save::<PathBuf>(None)?;
        Ok(bot.into())
    }

    /// Add a listener to a bot.
    fn add_listener(&mut self, args: ListenerInsertArgs) -> Result<ListenerView, AppError> {
        let bot = self.get_bot_mut(&args.bot_id)?;
        let listener_id = args
            .listener_id
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let listener = Listener {
            service: args.service,
            secret: args.secret.unwrap_or_default(),
            msg: args.msg.unwrap_or_default(),
        };

        bot.listeners.insert(listener_id.clone(), listener.clone());
        self.save::<PathBuf>(None)?;
        Ok((&args.bot_id, listener_id.as_str(), &listener).into())
    }

    /// List listeners for a bot, optionally filtering by arguments.
    fn list_listeners(&self, args: ListenerListArgs) -> Result<ListenerListView, AppError> {
        self.validate_bot_id(&args.bot_id)?;
        let bot = self.get_bot_ref(&args.bot_id)?;

        let filtered_listeners: Vec<ListenerView> = bot
            .listeners
            .iter()
            .filter(|(id, listener)| args.matches(id, listener))
            .map(|(id, listener)| (&args.bot_id, id.as_str(), listener).into())
            .collect();

        if filtered_listeners.is_empty() {
            return Err(AppError::ListenerNotFound(
                "No matching listeners found.".to_string(),
            ));
        }
        Ok(ListenerListView(filtered_listeners))
    }

    /// Get a specific listener by bot ID and listener ID.
    fn get_listener(&self, args: ListenerGetArgs) -> Result<ListenerView, AppError> {
        Ok((
            args.bot_id.clone(),
            args.listener_id.clone(),
            self.get_listener_ref(&args.bot_id, &args.listener_id)?,
        )
            .into())
    }

    /// Update a specific listener by bot ID and listener ID.
    fn update_listener(&mut self, args: ListenerUpdateArgs) -> Result<ListenerView, AppError> {
        let updated_listener_view = {
            let listener = self.get_listener_mut(&args.bot_id, &args.listener_id)?;
            // Apply updates to the listener
            args.apply(listener);
            // Create an immutable reference to the updated listener
            let listener_ref = &*listener;
            // Convert to ListenerView using the immutable reference
            (&args.bot_id, args.listener_id.as_str(), listener_ref).into()
        };
        // Save the updated state
        self.save::<PathBuf>(None)?;
        Ok(updated_listener_view)
    }

    /// Delete a specific listener by bot ID and listener ID.
    fn delete_listener(&mut self, args: ListenerDeleteArgs) -> Result<ListenerView, AppError> {
        let bot = self.get_bot_mut(&args.bot_id)?;
        let listener = bot.listeners.remove(&args.listener_id).ok_or_else(|| {
            AppError::ListenerNotFound(format!(
                "Listener with ID '{}' not found in bot '{}'.",
                args.listener_id, args.bot_id
            ))
        })?;
        Ok((&args.bot_id, &args.listener_id, &listener).into())
    }

    /// Delete listeners matching criteria.
    fn delete_listeners(
        &mut self,
        args: ListenersDeleteArgs,
    ) -> Result<ListenerListView, AppError> {
        let mut deleted_listeners = Vec::new();

        // Retrieve the bot and validate its existence
        let bot = self.get_bot_mut(&args.bot_id)?;

        // Use `retain` to remove non-matching listeners and collect the deleted ones
        bot.listeners.retain(|listener_id, listener| {
            if args.matches(listener_id, listener) {
                // Collect the matching listener into the deleted list
                deleted_listeners.push((&args.bot_id, listener_id.as_str(), &*listener).into());
                false // Remove this listener
            } else {
                true // Retain this listener
            }
        });

        // Check if any listeners were deleted
        if deleted_listeners.is_empty() {
            return Err(AppError::ListenerNotFound(
                "No matching listeners found.".to_string(),
            ));
        }

        // Save the updated state
        self.save::<PathBuf>(None)?;
        Ok(ListenerListView(deleted_listeners))
    }
}
