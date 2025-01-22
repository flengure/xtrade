/// Dispatch CLI commands and modes
pub async fn run(cli: Cli, app_state: Arc<Mutex<AppState>>) -> Result<()> {
    match cli.command {
        // Handle the `Server` mode
        Commands::Server {
            port,
            bind,
            state,
            web,
            no_web,
            web_port,
            web_bind,
            web_path,
        } => {
            let server_args = ServerArgs {
                port,
                bind,
                state,
                web,
                no_web,
                web_port,
                web_bind,
                web_root: web_path,
            };
            server::run(server_args, app_state.clone()).await?;
        }

        // Handle the `Offline` mode
        Commands::Offline(offline_args) => {
            // If a state file is provided, update the app_state's file field
            if let Some(state_path) = offline_args.state {
                let mut app_state = app_state.lock().unwrap();
                app_state.file = Some(state_path.into());
            }

            // Dispatch offline management commands
            crate::bot::cli::offline::run(offline_args.command, app_state.clone())
                .await
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        }

        // Handle the `Management` commands in online mode
        Commands::Management(management_command) => {
            let url = cli
                .url
                .unwrap_or_else(|| "http://localhost:7762".to_string());
            let rest_client = RestClient::new(&url);

            // Dispatch online management commands
            crate::bot::cli::online::run(management_command, rest_client)
                .await
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        }
    }

    Ok(())
}
