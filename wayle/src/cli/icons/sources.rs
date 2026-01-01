use wayle_icons::sources;

use crate::cli::CliAction;

/// Lists all available icon sources.
///
/// # Errors
///
/// Returns error if icon manager fails.
pub fn execute() -> CliAction {
    let all_sources = sources::all();

    println!("\nAvailable icon sources:\n");

    for source in all_sources {
        println!(
            "  {:<16} {:<6} {}",
            source.cli_name(),
            format!("{}-", source.prefix()),
            source.description()
        );
        println!("  {:<16}        {}\n", "", source.website());
    }

    Ok(())
}
