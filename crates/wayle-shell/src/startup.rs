//! Startup timing utilities with visual progress indicators.

use std::time::{Duration, Instant};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

const SLOW_THRESHOLD: Duration = Duration::from_millis(100);
const MODERATE_THRESHOLD: Duration = Duration::from_millis(50);

/// Tracks service initialization timing with visual feedback.
pub struct StartupTimer {
    start: Instant,
    spinner_style: ProgressStyle,
}

impl StartupTimer {
    /// Creates a new startup timer and prints the startup header.
    #[allow(clippy::expect_used)]
    pub fn new() -> Self {
        eprintln!("\n{}\n", style("Wayle Shell Starting...").bold());

        let spinner_style = ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .expect("hardcoded template")
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈");

        Self {
            start: Instant::now(),
            spinner_style,
        }
    }

    /// Times an async operation with a spinner, then prints the result.
    pub async fn time<T, E, F>(&self, name: &'static str, fut: F) -> Result<T, E>
    where
        F: std::future::Future<Output = Result<T, E>>,
    {
        let pb = ProgressBar::new_spinner();
        pb.set_style(self.spinner_style.clone());
        pb.set_message(format!("Loading {name}..."));
        pb.enable_steady_tick(Duration::from_millis(80));

        let start = Instant::now();
        let result = fut.await;
        let duration = start.elapsed();

        pb.finish_and_clear();

        let duration_str = format!("({}ms)", duration.as_millis());

        if result.is_ok() {
            let (check, timing) = if duration >= SLOW_THRESHOLD {
                (style("✓").red().bold(), style(duration_str).red())
            } else if duration >= MODERATE_THRESHOLD {
                (style("✓").yellow().bold(), style(duration_str).yellow())
            } else {
                (style("✓").green().bold(), style(duration_str).dim())
            };
            eprintln!("{check} {name} {timing}");
        } else {
            eprintln!("{} {name}", style("✗").red().bold());
        }

        result
    }

    /// Prints the final startup summary with total time.
    pub fn finish(self) {
        let total_ms = self.start.elapsed().as_millis();
        let time_str = if total_ms >= 1000 {
            format!("{:.2}s", total_ms as f64 / 1000.0)
        } else {
            format!("{total_ms}ms")
        };
        eprintln!(
            "\n{}\n",
            style(format!("Started in {time_str}")).green().bold()
        );
    }
}

impl Default for StartupTimer {
    fn default() -> Self {
        Self::new()
    }
}
