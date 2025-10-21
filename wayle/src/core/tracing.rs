use std::{env, error::Error, io, mem};

use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::infrastructure::paths::ConfigPaths;

/// Initialize tracing for the application
///
/// Sets up structured logging with info level by default.
/// Uses RUST_LOG environment variable if set, otherwise defaults to "info".
/// Supports both pretty console output and JSON output based on WAYLE_LOG_FORMAT.
///
/// # Errors
/// Returns error if tracing subscriber initialization fails
pub fn init() -> Result<(), Box<dyn Error>> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let format = env::var("WAYLE_LOG_FORMAT").unwrap_or_else(|_| String::from("pretty"));

    let registry = tracing_subscriber::registry().with(env_filter);

    match format.as_str() {
        "json" => {
            registry
                .with(fmt::layer().json().with_target(true).with_level(true))
                .try_init()?;
        }
        _ => {
            registry
                .with(
                    fmt::layer()
                        .pretty()
                        .with_target(true)
                        .with_level(true)
                        .with_thread_ids(true)
                        .with_thread_names(true),
                )
                .try_init()?;
        }
    }

    Ok(())
}

/// Initialize tracing with file output
///
/// Sets up dual logging: console output respects RUST_LOG (defaults to "warn"),
/// while file output uses WAYLE_FILE_LOG level (defaults to "info").
/// File is created in the wayle logs directory.
///
/// # Errors
/// Returns error if file creation or tracing subscriber initialization fails
pub fn init_with_file() -> Result<(), Box<dyn Error>> {
    init_with_file_and_console_option(true)
}

/// Initialize tracing for CLI mode (file-only, no console output)
///
/// Sets up file-only logging for CLI commands to avoid mixing structured logs
/// with command output. Console logs are completely disabled.
///
/// # Errors
/// Returns error if file creation or tracing subscriber initialization fails
pub fn init_cli_mode() -> Result<(), Box<dyn Error>> {
    init_with_file_and_console_option(false)
}

fn init_with_file_and_console_option(enable_console: bool) -> Result<(), Box<dyn Error>> {
    const DAYS_TO_KEEP: usize = 7;

    let console_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
    let file_filter = env::var("WAYLE_FILE_LOG")
        .map(EnvFilter::new)
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let log_dir = ConfigPaths::log_dir()?;

    let file_appender = tracing_appender::rolling::Builder::new()
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .max_log_files(DAYS_TO_KEEP)
        .filename_prefix("wayle")
        .filename_suffix("log")
        .build(&log_dir)?;
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let format = env::var("WAYLE_LOG_FORMAT").unwrap_or_else(|_| String::from("pretty"));

    let registry = tracing_subscriber::registry();

    if enable_console {
        match format.as_str() {
            "json" => {
                registry
                    .with(
                        fmt::layer()
                            .json()
                            .with_target(true)
                            .with_level(true)
                            .with_writer(io::stdout)
                            .with_filter(console_filter),
                    )
                    .with(
                        fmt::layer()
                            .json()
                            .with_target(true)
                            .with_level(true)
                            .with_writer(non_blocking)
                            .with_ansi(false)
                            .with_filter(file_filter),
                    )
                    .try_init()?;
            }
            _ => {
                registry
                    .with(
                        fmt::layer()
                            .pretty()
                            .with_target(true)
                            .with_level(true)
                            .with_thread_ids(true)
                            .with_thread_names(true)
                            .with_writer(io::stdout)
                            .with_filter(console_filter),
                    )
                    .with(
                        fmt::layer()
                            .compact()
                            .with_target(true)
                            .with_level(true)
                            .with_writer(non_blocking)
                            .with_ansi(false)
                            .with_filter(file_filter),
                    )
                    .try_init()?;
            }
        }
    } else {
        registry
            .with(
                fmt::layer()
                    .compact()
                    .with_target(true)
                    .with_level(true)
                    .with_writer(non_blocking)
                    .with_ansi(false)
                    .with_filter(file_filter),
            )
            .try_init()?;
    }

    mem::forget(_guard);

    Ok(())
}
