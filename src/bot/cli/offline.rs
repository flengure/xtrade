use crate::bot::cli::Commands;
use crate::bot::state::{
    AppState, BotInsertArgs, BotListArgs, BotUpdateArgs, ListenerInsertArgs, ListenerListArgs,
    ListenerUpdateArgs,
};
use crate::errors::ApiError;
use std::sync::{Arc, Mutex};

/// Handle CLI commands in offline mode
pub async fn run(args: Commands, state: Arc<Mutex<AppState>>) -> Result<(), ApiError> {
    // Acquire the lock on the AppState
    let mut app_state = state.lock().map_err(|_| {
        ApiError::InternalServerError("Failed to acquire lock on AppState.".to_string())
    })?;

    match args {
        Commands::Server { .. } => Err(ApiError::InvalidInput(
            "The `Server` command cannot be used in offline mode.".to_string(),
        )),
        Commands::ClearAll { target } => match target.as_str() {
            "bots" => {
                app_state.clear_bots()?;
                println!("All bots cleared.");
                Ok(())
            }
            "listeners" => {
                app_state.clear_listeners()?;
                println!("All listeners cleared.");
                Ok(())
            }
            _ => Err(ApiError::InvalidInput(format!(
                "Invalid target: {}. Use 'bots' or 'listeners'.",
                target
            ))),
        },

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

            let added_bot = app_state.add_bot(bot_insert_args)?;
            println!("{}", added_bot);
            Ok(())
        }

        Commands::ListBots {
            page: _,
            limit: _,
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

            let bots = app_state.list_bots(Some(filter_args))?;
            if bots.0.is_empty() {
                println!("No bots available.");
            } else {
                println!("{}", bots);
            }
            Ok(())
        }

        Commands::GetBot { bot_id } => {
            let bot = app_state.get_bot(&bot_id)?;
            println!("{}", bot);
            Ok(())
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

            let updated_bot = app_state.update_bot(&bot_id, update_args)?;
            println!("{}", updated_bot);
            Ok(())
        }

        Commands::DeleteBot { bot_id } => {
            let deleted_bot = app_state.delete_bot(&bot_id)?;
            println!("{}", deleted_bot);
            Ok(())
        }

        Commands::AddListener {
            bot_id,
            service,
            secret,
            msg,
        } => {
            let listener_args = ListenerInsertArgs::new(service, secret, msg);
            let listener = app_state.add_listener(&bot_id, listener_args)?;
            println!("{}", listener);
            Ok(())
        }

        Commands::ListListeners {
            bot_id,
            listener_id,
            service,
            page: _,
            limit: _,
        } => {
            let filter_args = ListenerListArgs::new()
                .listener_id(listener_id)
                .service(service);

            let listeners = app_state.list_listeners(&bot_id, filter_args)?;
            if listeners.0.is_empty() {
                println!("No listeners found for Bot ID: {}", bot_id);
            } else {
                println!("{}", listeners);
            }
            Ok(())
        }

        Commands::GetListener {
            bot_id,
            listener_id,
        } => {
            let listener = app_state.get_listener(&bot_id, &listener_id)?;
            println!("{}", listener);
            Ok(())
        }

        Commands::UpdateListener {
            bot_id,
            listener_id,
            service,
            secret,
            msg,
        } => {
            let update_args = ListenerUpdateArgs::new(listener_id)
                .service(service)
                .secret(secret)
                .msg(msg);

            let updated_listener = app_state.update_listener(&bot_id, update_args)?;
            println!("{}", updated_listener);
            Ok(())
        }

        Commands::DeleteListener {
            bot_id,
            listener_id,
        } => {
            let deleted_listener = app_state.delete_listener(&bot_id, &listener_id)?;
            println!("{}", deleted_listener);
            Ok(())
        }

        Commands::DeleteListeners {
            bot_id,
            listener_id,
            service,
        } => {
            let delete_args = ListenerListArgs::new()
                .listener_id(listener_id)
                .service(service);

            let deleted_listeners = app_state.delete_listeners(&bot_id, delete_args)?;
            println!("{}", deleted_listeners);
            Ok(())
        }
    }
}
