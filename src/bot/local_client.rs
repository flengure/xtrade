use super::cli::OfflineCmds;
use crate::app_config::AppConfig;
use crate::app_state::AppState;
use crate::bot::state::BotRegistry;
use crate::errors::AppError;
use std::path::Path;

/// Handle CLI commands in offline mode
pub async fn run(state_file: Option<&Path>, args: OfflineCmds) -> Result<(), AppError> {
    // Load AppConfig
    let mut app_config = AppConfig::load::<&Path>(None)?;

    // Override the state file if provided via CLI
    if let Some(v) = state_file {
        app_config.local_cli.state_file = v.to_path_buf();
    }

    // Initialize the application state directly
    let mut app_state = AppState::load(app_config.clone())?;

    match args {
        OfflineCmds::ClearAll { target } => match target.as_str() {
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
            _ => Err(AppError::InvalidInput(format!(
                "Invalid target: {}. Use 'bots' or 'listeners'.",
                target
            ))),
        },
        OfflineCmds::AddBot(args) => Ok(println!("{}", app_state.add_bot(args)?)),
        OfflineCmds::ListBots(args) => Ok(println!("{}", app_state.list_bots(Some(args))?)),
        OfflineCmds::GetBot(args) => Ok(println!("{}", app_state.get_bot(args)?)),
        OfflineCmds::UpdateBot(args) => Ok(println!("{}", app_state.update_bot(args)?)),
        OfflineCmds::DeleteBot(args) => Ok(println!("{}", app_state.delete_bot(args)?)),
        OfflineCmds::AddListener(args) => Ok(println!("{}", app_state.add_listener(args)?)),
        OfflineCmds::ListListeners(args) => Ok(println!("{}", app_state.list_listeners(args)?)),
        OfflineCmds::GetListener(args) => Ok(println!("{}", app_state.get_listener(args)?)),
        OfflineCmds::UpdateListener(args) => Ok(println!("{}", app_state.update_listener(args)?)),
        OfflineCmds::DeleteListener(args) => Ok(println!("{}", app_state.delete_listener(args)?)),
        OfflineCmds::DeleteListeners(args) => Ok(println!("{}", app_state.delete_listeners(args)?)),
    }
}
