// src/services/listeners.rs

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

/// List all listeners for a bot, flattened with their IDs included.
pub fn list_listeners(&self, bot_id: &str) -> Result<Vec<FlattenedListener>, ApiError> {
    if let Some(bot) = self.bots.get(bot_id) {
        // Transform each listener into a FlattenedListener
        let flattened_listeners: Vec<FlattenedListener> = bot
            .listeners
            .iter()
            .map(|(listener_id, listener)| FlattenedListener {
                listener_id: listener_id.clone(),
                service: listener.service.clone(),
                secret: listener.secret.clone(),
                msg: listener.msg.clone(),
            })
            .collect();

        Ok(flattened_listeners)
    } else {
        Err(ApiError::BotNotFound(bot_id.to_string()))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Bot, Listener};
    use serde_json::json;
    use std::collections::HashMap;

    /// Helper function to create a mock bot
    fn mock_bot(bot_id: &str) -> Bot {
        Bot {
            bot_id: bot_id.to_string(),
            name: "TestBot".to_string(),
            exchange: "Binance".to_string(),
            api_key: Some("api_key".to_string()),
            api_secret: Some("api_secret".to_string()),
            rest_endpoint: Some("http://rest_endpoint".to_string()),
            rpc_endpoint: Some("http://rpc_endpoint".to_string()),
            webhook_secret: Some("webhook_secret".to_string()),
            trading_fee: Some(0.1),
            private_key: Some("private_key".to_string()),
            contract_address: Some("contract_address".to_string()),
            listeners: Vec::new(),
        }
    }

    /// Helper function to create a mock listener
    fn mock_listener(listener_id: &str) -> Listener {
        Listener {
            listener_id: listener_id.to_string(),
            service: "TestService".to_string(),
            secret: "secret123".to_string(),
            msg: json!({"key": "value"}),
        }
    }

    #[test]
    fn test_add_listener_success() {
        let mut bots = HashMap::new();
        let bot_id = "bot123".to_string();
        bots.insert(bot_id.clone(), mock_bot(&bot_id));

        let listener = mock_listener("listener123");
        let result = add_listener(&mut bots, &bot_id, listener.clone());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "listener123");
        assert_eq!(bots[&bot_id].listeners.len(), 1);
        assert_eq!(bots[&bot_id].listeners[0].service, "TestService");
    }

    #[test]
    fn test_add_listener_bot_not_found() {
        let mut bots = HashMap::new();
        let listener = mock_listener("listener123");
        let result = add_listener(&mut bots, "nonexistent_bot", listener);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Bot with ID nonexistent_bot not found");
    }

    #[test]
    fn test_list_listeners_success() {
        let mut bots = HashMap::new();
        let bot_id = "bot123".to_string();
        let mut bot = mock_bot(&bot_id);
        bot.listeners.push(mock_listener("listener123"));
        bots.insert(bot_id.clone(), bot);

        let result = list_listeners(&bots, &bot_id);
        assert!(result.is_ok());
        let listeners = result.unwrap();
        assert_eq!(listeners.len(), 1);
        assert_eq!(listeners[0].listener_id, "listener123");
    }

    #[test]
    fn test_list_listeners_bot_not_found() {
        let bots = HashMap::new();
        let result = list_listeners(&bots, "nonexistent_bot");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Bot with ID nonexistent_bot not found");
    }

    #[test]
    fn test_get_listener_success() {
        let mut bots = HashMap::new();
        let bot_id = "bot123".to_string();
        let mut bot = mock_bot(&bot_id);
        bot.listeners.push(mock_listener("listener123"));
        bots.insert(bot_id, bot);

        let result = get_listener(&bots, "listener123");
        assert!(result.is_some());
        assert_eq!(result.unwrap().listener_id, "listener123");
    }

    #[test]
    fn test_get_listener_not_found() {
        let bots = HashMap::new();
        let result = get_listener(&bots, "nonexistent_listener");

        assert!(result.is_none());
    }

    #[test]
    fn test_update_listener_success() {
        let mut bots = HashMap::new();
        let bot_id = "bot123".to_string();
        let mut bot = mock_bot(&bot_id);
        bot.listeners.push(mock_listener("listener123"));
        bots.insert(bot_id, bot);

        let updated_listener = Listener {
            listener_id: "listener123".to_string(),
            service: "UpdatedService".to_string(),
            secret: "new_secret".to_string(),
            msg: json!({"updated_key": "updated_value"}),
        };

        let result = update_listener(&mut bots, "listener123", updated_listener.clone());
        assert!(result.is_ok());
        let listener = bots
            .values()
            .flat_map(|bot| &bot.listeners)
            .find(|l| l.listener_id == "listener123")
            .unwrap();
        assert_eq!(listener.service, "UpdatedService");
        assert_eq!(listener.secret, "new_secret");
    }

    #[test]
    fn test_update_listener_not_found() {
        let mut bots = HashMap::new();
        let updated_listener = mock_listener("listener123");

        let result = update_listener(&mut bots, "listener123", updated_listener);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Listener with ID listener123 not found"
        );
    }

    #[test]
    fn test_delete_listener_success() {
        let mut bots = HashMap::new();
        let bot_id = "bot123".to_string();
        let mut bot = mock_bot(&bot_id);
        bot.listeners.push(mock_listener("listener123"));
        bots.insert(bot_id, bot);

        let result = delete_listener(&mut bots, "listener123");
        assert!(result.is_ok());
        let listeners = bots
            .values()
            .flat_map(|bot| &bot.listeners)
            .collect::<Vec<_>>();
        assert!(listeners.is_empty());
    }

    #[test]
    fn test_delete_listener_not_found() {
        let mut bots = HashMap::new();

        let result = delete_listener(&mut bots, "nonexistent_listener");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Listener with ID nonexistent_listener not found"
        );
    }
}
