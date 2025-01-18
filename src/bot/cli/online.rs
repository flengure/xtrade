use crate::bot::cli::Commands;
use crate::bot::rest::{
    BotInsertArgs, BotListArgs, BotUpdateArgs, ListenerInsertArgs, ListenerListArgs,
    ListenerUpdateArgs, RestClient,
};
use crate::errors::ApiError;

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

            let result = client.add_bot(bot_insert_args).await?;
            println!("{}", result);
            Ok(())
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

            let bots = client.get_bots(page, limit, Some(filter_args)).await?;
            println!("{}", bots);
            Ok(())
        }

        Commands::GetBot { bot_id } => {
            let bot = client.get_bot(&bot_id).await?;
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

            let updated_bot = client.update_bot(&bot_id, update_args).await?;
            println!("{}", updated_bot);
            Ok(())
        }

        Commands::DeleteBot { bot_id } => {
            let result = client.delete_bot(&bot_id).await?;
            println!("{}", result);
            Ok(())
        }

        Commands::AddListener {
            bot_id,
            service,
            secret,
            msg,
        } => {
            let listener_args = ListenerInsertArgs::new(service, secret, msg);
            let result = client.add_listener(&bot_id, listener_args).await?;
            println!("{}", result);
            Ok(())
        }

        Commands::ListListeners {
            bot_id,
            listener_id,
            service,
            page,
            limit,
        } => {
            let filter_args = ListenerListArgs::new()
                .listener_id(listener_id)
                .service(service);

            let listeners = client
                .get_listeners(&bot_id, page, limit, Some(filter_args))
                .await?;
            println!("{}", listeners);
            Ok(())
        }

        Commands::GetListener {
            bot_id,
            listener_id,
        } => {
            let listener = client.get_listener(&bot_id, &listener_id).await?;
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
            let update_args = ListenerUpdateArgs::new(listener_id.clone())
                .service(service)
                .secret(secret)
                .msg(msg);

            let updated_listener = client
                .update_listener(&bot_id, &listener_id, update_args)
                .await?;
            println!("{}", updated_listener);
            Ok(())
        }

        Commands::DeleteListener {
            bot_id,
            listener_id,
        } => {
            let result = client.delete_listener(&bot_id, &listener_id).await?;
            println!("{}", result);
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

            let result = client.delete_listeners(&bot_id, Some(delete_args)).await?;
            println!("{}", result);
            Ok(())
        }
    }
}
