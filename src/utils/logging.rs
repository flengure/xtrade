// src/utils/logging.rs
use colored::Colorize;
use fern::Dispatch;
use log::LevelFilter;
use std::env;

/// Determine log level based on verbosity flags (-v, -vv, --verbose, etc.)
///
/// This function parses the command-line arguments to determine the desired
/// verbosity level for the application. It processes `-v`, `-vv`, and `--verbose`
/// flags:
///
/// - `-v`, `-vv`, `-vvv`, etc.: Each `v` adds one level of verbosity.
/// - `--verbose`: Adds one level of verbosity per occurrence.
///
/// The resulting verbosity level is mapped to one of the following log levels:
///
/// - 0: `LevelFilter::Warn`
/// - 1: `LevelFilter::Info`
/// - 2: `LevelFilter::Debug`
/// - 3+: `LevelFilter::Trace`
///
/// # Returns
/// A `LevelFilter` representing the appropriate log level.
fn determine_log_level() -> LevelFilter {
    let mut verbosity_level = 0;

    // Capture and process command-line arguments
    let mut args: Vec<String> = env::args().collect();

    args.retain(|arg| {
        if arg.starts_with("-v") && arg != "--verbose" {
            verbosity_level += arg.trim_start_matches('-').len(); // Count 'v's in -v, -vv, etc.
            false // Remove verbosity argument
        } else if arg == "--verbose" {
            verbosity_level += 1; // Count each --verbose as 1
            false // Remove verbosity argument
        } else {
            true // Keep other arguments
        }
    });

    // Map verbosity level to log::LevelFilter
    match verbosity_level {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    }
}

/// Configure and set up the logger.
///
/// This function initializes the application's logging system using the `fern` crate.
/// The log level is dynamically determined by `determine_log_level`, which processes
/// verbosity flags from the command line.
///
/// The logger uses the following format for log messages:
/// ```text
/// YYYY-MM-DD HH:MM:SS [LEVEL] <message>
/// ```
/// where:
/// - `LEVEL` is color-coded based on the log level:
///   - `ERROR`: Red
///   - `WARN`: Yellow
///   - `INFO`: Green
///   - `DEBUG`: Blue
///   - `TRACE`: Cyan
/// - `<message>` is the log message.
///
/// # Arguments
/// None.
///
/// # Returns
/// A `Result` indicating whether the logger was successfully set up.
/// If the setup fails, it returns an error of type `fern::InitError`.
///
/// # Errors
/// This function will return an error if the logger initialization fails.
///
/// # Examples
/// ```rust
/// if let Err(e) = setup_logger() {
///     eprintln!("Failed to initialize logger: {}", e);
///     std::process::exit(1);
/// }
/// ```
pub fn setup_logger() -> Result<(), fern::InitError> {
    let level = determine_log_level();

    Dispatch::new()
        .format(|out, message, record| {
            let level_color = match record.level() {
                log::Level::Error => "ERROR".red(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Info => "INFO".green(),
                log::Level::Debug => "DEBUG".blue(),
                log::Level::Trace => "TRACE".cyan(),
            };

            out.finish(format_args!(
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level_color,
                message
            ));
        })
        .level(level) // Dynamically set the log level
        .chain(std::io::stdout())
        .apply()?;

    println!("Set log level to {:?}", level);

    Ok(())
}
