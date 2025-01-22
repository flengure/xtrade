use clap::Args;

#[derive(Args, Clone, Debug)]
pub struct ServerStartupArgs {
    #[arg(long)]
    pub api_port: Option<u16>,
    #[arg(long)]
    pub api_bind_address: Option<String>,
    #[arg(long)]
    pub state_file_path: Option<String>,
    /// Enable the Web UI (default)
    #[arg(long, conflicts_with = "no_web_client")]
    pub web_client: bool,
    /// Disable the Web UI
    #[arg(long, conflicts_with = "web_client")]
    pub no_web_client: bool,
    #[arg(long)]
    pub web_client_port: Option<u16>,
    #[arg(long)]
    pub web_client_bind_address: Option<String>,
    #[arg(long)]
    pub static_files_path: Option<String>,
    /// Enable the Web UI (default)
    #[arg(long, conflicts_with = "no_webhook")]
    pub webhook: bool,
    /// Disable the Web UI
    #[arg(long, conflicts_with = "webhook")]
    pub no_webhook: bool,
    #[arg(long)]
    pub webhook_port: Option<u16>,
    #[arg(long)]
    pub webhook_bind_address: Option<String>,
}
//
//#[derive(Args, Clone, Debug)]
//pub struct ServerStartupArgs {
//    #[arg(long)]
//    pub port: Option<u16>,
//    #[arg(long)]
//    pub bind: Option<String>,
//    #[arg(long)]
//    pub state: Option<String>,
//    /// Enable the Web UI (default)
//    #[arg(long, conflicts_with = "no_web")]
//    pub web: bool,
//    /// Disable the Web UI
//    #[arg(long, conflicts_with = "web")]
//    pub no_web: bool,
//    #[arg(long)]
//    pub web_port: Option<u16>,
//    #[arg(long)]
//    pub web_bind: Option<String>,
//    #[arg(long)]
//    pub web_path: Option<String>,
//}
//
