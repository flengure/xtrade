use crate::bot::api::ApiResponse;
use crate::bot::cli::Commands;
use crate::bot::rest::{
    BotInsertArgs, BotListArgs, BotListView, BotUpdateArgs, BotView, ListenerInsertArgs,
    ListenerListArgs, ListenerListView, ListenerUpdateArgs, ListenerView, RestClient,
};
// use reqwest::Response;
// use serde::de::DeserializeOwned;

use crate::errors::ApiError;
use reqwest::Response;
use serde::de::DeserializeOwned;

/// Utility function to process API response and handle output
pub async fn process_and_display_response<T>(
    response: Response, // The HTTP response
) -> Result<(), ApiError>
where
    T: DeserializeOwned + std::fmt::Display, // Ensure T can be deserialized and printed
{
    let status = response.status(); // Get the HTTP status code
    let body = response
        .text()
        .await
        .map_err(|e| ApiError::ConnectionError(format!("Failed to read response body: {}", e)))?;

    println!("Raw response body: {}", body); // Optional: Debugging raw response

    if status.is_success() {
        // Parse the body into ApiResponse<T>
        let api_response: ApiResponse<T> = serde_json::from_str(&body).map_err(|e| {
            ApiError::ParseError(format!("Failed to parse response into ApiResponse: {}", e))
        })?;

        // Check if `success` is true
        if !api_response.success {
            return Err(ApiError::SerializationError(
                api_response
                    .error
                    .unwrap_or_else(|| "Unknown error".to_string()),
            ));
        }

        // Extract the `data` field and display it
        if let Some(data) = api_response.data {
            println!("{}", data); // Use the Display implementation of T
            Ok(())
        } else {
            Err(ApiError::ParseError(
                "Response 'data' field is missing despite success=true.".to_string(),
            ))
        }
    } else {
        Err(ApiError::HttpError(status.as_u16(), body))
    }
}

/// Handle CLI commands in online mode
pub async fn run(args: Commands, client: RestClient) -> Result<(), ApiError> {
    match args {
        Commands::Server { .. } => Err(ApiError::InvalidInput(
            "The `Server` command cannot be used in online mode.".to_string(),
        )),
        Commands::ClearAll { target: _ } => {
            println!("The 'ClearAll' command is not supported in online mode.");
            Ok(())
        }

        Commands::AddBot {
            bot_id,
            name,
            exchange,
            api_key,
            api_secret,
            rest_endpoint,
            rpc_endpoint,
            webhook_secret,
            trading_fee,
            private_key,
            contract_address,
        } => {
            let bot_insert_args = BotInsertArgs::new(name, exchange)
                .bot_id(bot_id)
                .api_key(api_key)
                .api_secret(api_secret)
                .rest_endpoint(rest_endpoint)
                .rpc_endpoint(rpc_endpoint)
                .webhook_secret(webhook_secret)
                .trading_fee(trading_fee)
                .private_key(private_key)
                .contract_address(contract_address);

            process_and_display_response::<BotView>(client.add_bot(bot_insert_args).await?).await
        }

        Commands::ListBots {
            page,
            limit,
            bot_id,
            name,
            exchange,
            api_key,
            rest_endpoint,
            rpc_endpoint,
            trading_fee,
            private_key,
            contract_address,
        } => {
            let filter_args = BotListArgs::new()
                .bot_id(bot_id)
                .name(name)
                .exchange(exchange)
                .api_key(api_key)
                .rest_endpoint(rest_endpoint)
                .rpc_endpoint(rpc_endpoint)
                .trading_fee(trading_fee)
                .private_key(private_key)
                .contract_address(contract_address);

            process_and_display_response::<BotListView>(
                client.get_bots(page, limit, Some(filter_args)).await?,
            )
            .await
        }

        Commands::GetBot { bot_id } => {
            let response = client.get_bot(&bot_id).await?;
            process_and_display_response::<BotView>(response).await
        }

        Commands::UpdateBot {
            bot_id,
            name,
            exchange,
            api_key,
            api_secret,
            rest_endpoint,
            rpc_endpoint,
            webhook_secret,
            trading_fee,
            private_key,
            contract_address,
        } => {
            let update_args = BotUpdateArgs::new()
                .name(name)
                .exchange(exchange)
                .api_key(api_key)
                .api_secret(api_secret)
                .rest_endpoint(rest_endpoint)
                .rpc_endpoint(rpc_endpoint)
                .webhook_secret(webhook_secret)
                .trading_fee(trading_fee)
                .private_key(private_key)
                .contract_address(contract_address);

            process_and_display_response::<BotView>(client.update_bot(&bot_id, update_args).await?)
                .await
        }

        Commands::DeleteBot { bot_id } => {
            process_and_display_response::<BotView>(client.delete_bot(&bot_id).await?).await
        }

        Commands::AddListener {
            bot_id,
            service,
            secret,
            msg,
        } => {
            process_and_display_response::<ListenerView>(
                client
                    .add_listener(&bot_id, ListenerInsertArgs::new(service, secret, msg))
                    .await?,
            )
            .await
        }

        Commands::ListListeners {
            bot_id,
            listener_id,
            service,
            page,
            limit,
        } => {
            process_and_display_response::<ListenerListView>(
                client
                    .get_listeners(
                        &bot_id,
                        page,
                        limit,
                        Some(
                            ListenerListArgs::new()
                                .listener_id(listener_id)
                                .service(service),
                        ),
                    )
                    .await?,
            )
            .await
        }

        Commands::GetListener {
            bot_id,
            listener_id,
        } => {
            process_and_display_response::<ListenerView>(
                client.get_listener(&bot_id, &listener_id).await?,
            )
            .await
        }

        Commands::UpdateListener {
            bot_id,
            listener_id,
            service,
            secret,
            msg,
        } => {
            process_and_display_response::<ListenerView>(
                client
                    .update_listener(
                        &bot_id,
                        &listener_id,
                        ListenerUpdateArgs::new(listener_id.clone())
                            .service(service)
                            .secret(secret)
                            .msg(msg),
                    )
                    .await?,
            )
            .await
        }

        Commands::DeleteListener {
            bot_id,
            listener_id,
        } => {
            process_and_display_response::<ListenerView>(
                client.delete_listener(&bot_id, &listener_id).await?,
            )
            .await
        }

        Commands::DeleteListeners {
            bot_id,
            listener_id,
            service,
        } => {
            process_and_display_response::<ListenerListView>(
                client
                    .delete_listeners(
                        &bot_id,
                        Some(
                            ListenerListArgs::new()
                                .listener_id(listener_id)
                                .service(service),
                        ),
                    )
                    .await?,
            )
            .await
        }
    }
}
