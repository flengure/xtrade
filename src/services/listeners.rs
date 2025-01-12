// services/listeners.rs
use crate::models::{Bot, Listener};
use std::collections::HashMap;

/// Add a listener to a bot
pub fn add_listener(
    bots: &mut HashMap<String, crate::models::Bot>,
    bot_id: &str,
    listener: Listener,
) -> Result<String, String> {
    if let Some(bot) = bots.get_mut(bot_id) {
        bot.listeners.push(listener.clone());
        Ok(listener.listener_id.clone())
    } else {
        Err(format!("Bot with ID {} not found", bot_id))
    }
}

/// List all listeners for a bot
pub fn list_listeners<'a>(
    bots: &'a HashMap<String, Bot>,
    bot_id: &str,
) -> Result<Vec<&'a Listener>, String> {
    if let Some(bot) = bots.get(bot_id) {
        Ok(bot.listeners.iter().collect())
    } else {
        Err(format!("Bot with ID {} not found", bot_id))
    }
}

/// Get a specific listener by its ID
pub fn get_listener<'a>(
    bots: &'a HashMap<String, crate::models::Bot>,
    listener_id: &str,
) -> Option<&'a Listener> {
    for bot in bots.values() {
        if let Some(listener) = bot.listeners.iter().find(|l| l.listener_id == listener_id) {
            return Some(listener);
        }
    }
    None
}
/// Update a listener by its ID
pub fn update_listener(
    bots: &mut HashMap<String, crate::models::Bot>,
    listener_id: &str,
    updated_listener: Listener,
) -> Result<(), String> {
    for bot in bots.values_mut() {
        if let Some(listener) = bot
            .listeners
            .iter_mut()
            .find(|l| l.listener_id == listener_id)
        {
            *listener = updated_listener;
            return Ok(());
        }
    }
    Err(format!("Listener with ID {} not found", listener_id))
}

/// Delete a listener by its ID
pub fn delete_listener(
    bots: &mut HashMap<String, crate::models::Bot>,
    listener_id: &str,
) -> Result<(), String> {
    for bot in bots.values_mut() {
        if let Some(index) = bot
            .listeners
            .iter()
            .position(|listener| listener.listener_id == listener_id)
        {
            bot.listeners.remove(index);
            return Ok(());
        }
    }
    Err(format!("Listener with ID {} not found", listener_id))
}
