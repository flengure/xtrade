use super::OfflineCmds;
use crate::bot::state::AppState;
use crate::bot::state::BotRegistry;
use crate::errors::ApiError;
use std::sync::{Arc, Mutex};

/// Handle CLI commands in offline mode
pub async fn run(args: OfflineCmds, state: Arc<Mutex<AppState>>) -> Result<(), ApiError> {
    // Acquire the lock on the AppState
    let mut app_state = state.lock().map_err(|_| {
        ApiError::InternalServerError("Failed to acquire lock on AppState.".to_string())
    })?;

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
            _ => Err(ApiError::InvalidInput(format!(
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
