// services/bots.rs
use crate::models::Bot;
use std::collections::HashMap;

/// Create a new bot
pub fn create_bot(bots: &mut HashMap<String, Bot>, bot: Bot) -> String {
    let bot_id = bot.bot_id.clone();
    bots.insert(bot_id.clone(), bot);
    bot_id
}

/// List all bots
pub fn list_bots(bots: &HashMap<String, Bot>) -> Vec<&Bot> {
    bots.values().collect()
}

/// Get a bot by its ID
pub fn get_bot<'a>(bots: &'a HashMap<String, Bot>, bot_id: &str) -> Option<&'a Bot> {
    bots.get(bot_id)
}

/// Update an existing bot
pub fn update_bot(
    bots: &mut HashMap<String, Bot>,
    bot_id: &str,
    updated_bot: Bot,
) -> Result<(), String> {
    if let Some(bot) = bots.get_mut(bot_id) {
        *bot = updated_bot;
        Ok(())
    } else {
        Err(format!("Bot with ID {} not found", bot_id))
    }
}

/// Delete a bot
pub fn delete_bot(bots: &mut HashMap<String, Bot>, bot_id: &str) -> Result<(), String> {
    if bots.remove(bot_id).is_some() {
        Ok(())
    } else {
        Err(format!("Bot with ID {} not found", bot_id))
    }
}
