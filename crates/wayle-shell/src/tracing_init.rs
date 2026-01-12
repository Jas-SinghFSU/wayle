//! Tracing initialization for the shell daemon.

use std::{env, error::Error, io, mem};

use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use wayle_config::infrastructure::paths::ConfigPaths;

/// Initializes tracing with console and file output.
///
/// Console output respects RUST_LOG (defaults to "warn").
/// File output uses WAYLE_FILE_LOG level (defaults to "info").
///
/// # Errors
///
/// Returns error if log directory creation or subscriber init fails.
pub fn init() -> Result<(), Box<dyn Error>> {
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
        .filename_prefix("wayle-shell")
        .filename_suffix("log")
        .build(&log_dir)?;
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let format = env::var("WAYLE_LOG_FORMAT").unwrap_or_else(|_| String::from("pretty"));

    let registry = tracing_subscriber::registry();

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

    mem::forget(_guard);

    Ok(())
}
