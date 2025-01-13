// src/api/bots/clients.rs
use crate::api::send_request;
use reqwest::{blocking::Client, Method};
use serde_json::Value;

/// Add a new bot by sending a POST request to the API
pub fn add_bot(
    client: &Client,
    url: &str,
    name: String,
    exchange: String,
    api_key: Option<String>,
    api_secret: Option<String>,
    rest_endpoint: Option<String>,
    rpc_endpoint: Option<String>,
    webhook_secret: Option<String>,
    trading_fee: Option<f64>,
    private_key: Option<String>,
    contract_address: Option<String>,
) -> Result<(), String> {
    let bot_data = serde_json::json!({
        "bot_id": uuid::Uuid::new_v4().to_string(),
        "name": name,
        "exchange": exchange,
        "api_key": api_key,
        "api_secret": api_secret,
        "rest_endpoint": rest_endpoint,
        "rpc_endpoint": rpc_endpoint,
        "webhook_secret": webhook_secret,
        "trading_fee": trading_fee,
        "private_key": private_key,
        "contract_address": contract_address,
    });

    let response = client
        .post(&format!("{}/bots", url))
        .json(&bot_data)
        .send()
        .map_err(|err| format!("Network error: {}", err))?;

    if response.status().is_success() {
        println!(
            "Bot added successfully: {}",
            response.text().unwrap_or_default()
        );
        Ok(())
    } else {
        Err(format!(
            "Failed to add bot: {}",
            response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string())
        ))
    }
}

/// Fetch the list of bots from the API
pub fn fetch_bots(client: &Client, url: &str) -> Result<Value, String> {
    match send_request::<()>(client, Method::GET, &format!("{}/bots", url), None) {
        Ok(response) => {
            if response.status().is_success() {
                response
                    .json::<Value>()
                    .map_err(|err| format!("Failed to parse JSON: {}", err))
            } else {
                Err(format!(
                    "API returned error: {}",
                    response
                        .text()
                        .unwrap_or_else(|_| "Unknown error".to_string())
                ))
            }
        }
        Err(err) => Err(format!("Network error: {}", err)),
    }
}

/// Fetch a single bot by its ID
pub fn fetch_bot(client: &Client, url: &str, bot_id: &str) -> Result<Value, String> {
    let full_url = format!("{}/bots/{}", url, bot_id);

    match send_request::<()>(client, Method::GET, &full_url, None) {
        Ok(response) => {
            if response.status().is_success() {
                response
                    .json::<Value>()
                    .map_err(|err| format!("Failed to parse JSON: {}", err))
            } else {
                Err(format!(
                    "API returned error: {}",
                    response
                        .text()
                        .unwrap_or_else(|_| "Unknown error".to_string())
                ))
            }
        }
        Err(err) => Err(format!("Network error: {}", err)),
    }
}

/// Update a bot by sending a PUT request to the API
pub fn update_bot(
    client: &Client,
    url: &str,
    bot_id: &str,
    name: Option<String>,
    exchange: Option<String>,
    api_key: Option<String>,
    api_secret: Option<String>,
    rest_endpoint: Option<String>,
    rpc_endpoint: Option<String>,
    webhook_secret: Option<String>,
    trading_fee: Option<f64>,
    private_key: Option<String>,
    contract_address: Option<String>,
) -> Result<(), String> {
    // Construct the bot data as a JSON object
    let bot_data = serde_json::json!({
        "name": name,
        "exchange": exchange,
        "api_key": api_key,
        "api_secret": api_secret,
        "rest_endpoint": rest_endpoint,
        "rpc_endpoint": rpc_endpoint,
        "webhook_secret": webhook_secret,
        "trading_fee": trading_fee,
        "private_key": private_key,
        "contract_address": contract_address,
    });

    // Send the PUT request to update the bot
    let response = client
        .put(&format!("{}/bots/{}", url, bot_id))
        .json(&bot_data)
        .send()
        .map_err(|err| format!("Network error: {}", err))?;

    // Handle the response
    if response.status().is_success() {
        println!(
            "Bot updated successfully: {}",
            response.text().unwrap_or_default()
        );
        Ok(())
    } else {
        Err(format!(
            "Failed to update bot: {}",
            response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string())
        ))
    }
}

/// Delete a bot by sending a DELETE request to the API
pub fn delete_bot(client: &Client, url: &str, bot_id: &str) -> Result<(), String> {
    // Send the DELETE request to remove the bot
    let response = client
        .delete(&format!("{}/bots/{}", url, bot_id))
        .send()
        .map_err(|err| format!("Network error: {}", err))?;

    // Handle the response
    if response.status().is_success() {
        println!(
            "Bot deleted successfully: {}",
            response.text().unwrap_or_default()
        );
        Ok(())
    } else {
        Err(format!(
            "Failed to delete bot: {}",
            response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string())
        ))
    }
}

/// Add a new listener by sending a POST request to the API
pub fn add_listener(
    client: &Client,
    url: &str,
    bot_id: String,
    service: String,
    secret: Option<String>,
    msg: Option<String>,
) -> Result<(), String> {
    let listener_data = serde_json::json!({
        "bot_id": bot_id,
        "service": service,
        "secret": secret,
        "msg": msg,
    });

    let response = client
        .post(&format!("{}/listeners", url))
        .json(&listener_data)
        .send()
        .map_err(|err| format!("Network error: {}", err))?;

    if response.status().is_success() {
        println!(
            "Listener added successfully: {}",
            response.text().unwrap_or_default()
        );
        Ok(())
    } else {
        Err(format!(
            "Failed to add listener: {}",
            response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string())
        ))
    }
}

/// Fetch the list of listeners for a specific bot
pub fn fetch_listeners(client: &Client, url: &str, bot_id: &str) -> Result<Value, String> {
    let response = client
        .get(&format!("{}/bots/{}/listeners", url, bot_id))
        .send()
        .map_err(|err| format!("Network error: {}", err))?;

    if response.status().is_success() {
        response
            .json()
            .map_err(|_| "Failed to parse listeners response.".to_string())
    } else {
        Err(format!(
            "Failed to fetch listeners: {}",
            response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string())
        ))
    }
}

/// Fetch a listener by ID
pub fn fetch_listener(client: &Client, url: &str, listener_id: &str) -> Result<Value, String> {
    let response = client
        .get(&format!("{}/listeners/{}", url, listener_id))
        .send()
        .map_err(|err| format!("Network error: {}", err))?;

    if response.status().is_success() {
        response
            .json()
            .map_err(|_| "Failed to parse listener response.".to_string())
    } else {
        Err(format!(
            "Failed to fetch listener: {}",
            response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string())
        ))
    }
}

/// Update a listener
pub fn update_listener(
    client: &Client,
    url: &str,
    listener_id: &str,
    service: Option<String>,
    secret: Option<String>,
    msg: Option<String>,
) -> Result<(), String> {
    let listener_data = serde_json::json!({
        "service": service,
        "secret": secret,
        "msg": msg,
    });

    let response = client
        .put(&format!("{}/listeners/{}", url, listener_id))
        .json(&listener_data)
        .send()
        .map_err(|err| format!("Network error: {}", err))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to update listener: {}",
            response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string())
        ))
    }
}

/// Delete a listener by ID
pub fn delete_listener(client: &Client, url: &str, listener_id: &str) -> Result<(), String> {
    let response = client
        .delete(&format!("{}/listeners/{}", url, listener_id))
        .send()
        .map_err(|err| format!("Network error: {}", err))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to delete listener: {}",
            response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string())
        ))
    }
}
