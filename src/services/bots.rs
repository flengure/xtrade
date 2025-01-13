// src/services/bots.rs
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Bot;
    use std::collections::HashMap;

    fn mock_bot(id: &str, name: &str) -> Bot {
        Bot {
            bot_id: id.to_string(),
            name: name.to_string(),
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

    #[test]
    fn test_create_bot() {
        let mut bots = HashMap::new();
        let bot = mock_bot("bot1", "TestBot");

        let bot_id = create_bot(&mut bots, bot.clone());
        assert_eq!(bot_id, "bot1");
        assert_eq!(bots.len(), 1);
        assert_eq!(bots.get("bot1").unwrap().name, "TestBot");
    }

    #[test]
    fn test_list_bots() {
        let mut bots = HashMap::new();
        bots.insert("bot1".to_string(), mock_bot("bot1", "TestBot1"));
        bots.insert("bot2".to_string(), mock_bot("bot2", "TestBot2"));

        let bot_list = list_bots(&bots);
        assert_eq!(bot_list.len(), 2);
        assert!(bot_list.iter().any(|bot| bot.name == "TestBot1"));
        assert!(bot_list.iter().any(|bot| bot.name == "TestBot2"));
    }

    #[test]
    fn test_get_bot() {
        let mut bots = HashMap::new();
        bots.insert("bot1".to_string(), mock_bot("bot1", "TestBot"));

        let bot = get_bot(&bots, "bot1");
        assert!(bot.is_some());
        assert_eq!(bot.unwrap().name, "TestBot");

        let non_existent_bot = get_bot(&bots, "bot2");
        assert!(non_existent_bot.is_none());
    }

    #[test]
    fn test_update_bot_success() {
        let mut bots = HashMap::new();
        bots.insert("bot1".to_string(), mock_bot("bot1", "OldBot"));

        let updated_bot = mock_bot("bot1", "UpdatedBot");
        let result = update_bot(&mut bots, "bot1", updated_bot.clone());
        assert!(result.is_ok());
        assert_eq!(bots.get("bot1").unwrap().name, "UpdatedBot");
    }

    #[test]
    fn test_update_bot_not_found() {
        let mut bots = HashMap::new();

        let updated_bot = mock_bot("bot1", "UpdatedBot");
        let result = update_bot(&mut bots, "bot1", updated_bot.clone());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Bot with ID bot1 not found");
    }

    #[test]
    fn test_delete_bot_success() {
        let mut bots = HashMap::new();
        bots.insert("bot1".to_string(), mock_bot("bot1", "TestBot"));

        let result = delete_bot(&mut bots, "bot1");
        assert!(result.is_ok());
        assert!(bots.get("bot1").is_none());
    }

    #[test]
    fn test_delete_bot_not_found() {
        let mut bots = HashMap::new();

        let result = delete_bot(&mut bots, "bot1");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Bot with ID bot1 not found");
    }
}
