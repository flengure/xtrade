use crate::bot::api::ApiResponse;
use crate::bot::cli::Commands;
use crate::bot::rest::{BotListView, BotView, ListenerListView, ListenerView, RestClient};
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

    //println!("Raw response body: {}", body); // Optional: Debugging raw response

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
        Commands::AddBot(bot_insert_args) => {
            process_and_display_response::<BotView>(client.add_bot(bot_insert_args).await?).await
        }

        Commands::ListBots(bot_list_args) => {
            process_and_display_response::<BotListView>(
                client
                    .get_bots(bot_list_args.page, bot_list_args.limit, Some(bot_list_args))
                    .await?,
            )
            .await
        }

        Commands::GetBot(bot_get_args) => {
            process_and_display_response::<BotView>(client.get_bot(&bot_get_args.bot_id).await?)
                .await
        }

        Commands::UpdateBot(bot_update_args) => {
            process_and_display_response::<BotView>(
                client
                    .update_bot(&bot_update_args.bot_id.to_string(), bot_update_args)
                    .await?,
            )
            .await
        }

        Commands::DeleteBot(bot_delete_args) => {
            process_and_display_response::<BotView>(
                client.delete_bot(&bot_delete_args.bot_id).await?,
            )
            .await
        }

        Commands::AddListener(listener_insert_args) => {
            process_and_display_response::<ListenerView>(
                client
                    .add_listener(
                        &listener_insert_args.bot_id.to_string(),
                        listener_insert_args,
                    )
                    .await?,
            )
            .await
        }

        Commands::ListListeners(listener_list_args) => {
            process_and_display_response::<ListenerListView>(
                client
                    .get_listeners(
                        &listener_list_args.bot_id.to_string(),
                        listener_list_args.page,
                        listener_list_args.limit,
                        Some(listener_list_args),
                    )
                    .await?,
            )
            .await
        }

        Commands::GetListener(listener_get_args) => {
            process_and_display_response::<ListenerView>(
                client
                    .get_listener(&listener_get_args.bot_id, &listener_get_args.listener_id)
                    .await?,
            )
            .await
        }

        Commands::UpdateListener(listener_update_args) => {
            process_and_display_response::<ListenerView>(
                client
                    .update_listener(
                        &listener_update_args.bot_id.to_string(),
                        &listener_update_args.listener_id.to_string(),
                        listener_update_args,
                    )
                    .await?,
            )
            .await
        }

        Commands::DeleteListener(listener_delete_args) => {
            process_and_display_response::<ListenerView>(
                client
                    .delete_listener(
                        &listener_delete_args.bot_id,
                        &listener_delete_args.listener_id,
                    )
                    .await?,
            )
            .await
        }

        Commands::DeleteListeners(listeners_delete_args) => {
            process_and_display_response::<ListenerListView>(
                client
                    .delete_listeners(
                        &listeners_delete_args.bot_id.to_string(),
                        Some(listeners_delete_args),
                    )
                    .await?,
            )
            .await
        }

        _ => Err(ApiError::InvalidInput(
            "The provided command is not valid for online mode.".to_string(),
        )),
    }
}
