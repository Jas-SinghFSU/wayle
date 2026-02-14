//! Wallust color extraction.
//!
//! Runs wallust with user's config, then generates colors.json using wayle's template.

use std::path::{Path, PathBuf};

use tokio::{fs, process::Command};
use tracing::debug;
use wayle_config::ConfigPaths;

use super::Tool;
use crate::error::Error;

const COLORS_TEMPLATE: &str = r#"{
    "wallpaper": "{{wallpaper}}",
    "background": "{{background}}",
    "foreground": "{{foreground}}",
    "cursor": "{{cursor}}",
    "color0": "{{color0}}",
    "color1": "{{color1}}",
    "color2": "{{color2}}",
    "color3": "{{color3}}",
    "color4": "{{color4}}",
    "color5": "{{color5}}",
    "color6": "{{color6}}",
    "color7": "{{color7}}",
    "color8": "{{color8}}",
    "color9": "{{color9}}",
    "color10": "{{color10}}",
    "color11": "{{color11}}",
    "color12": "{{color12}}",
    "color13": "{{color13}}",
    "color14": "{{color14}}",
    "color15": "{{color15}}"
}
"#;

/// Wallust CLI arguments.
#[derive(Debug)]
pub enum Arg<'a> {
    /// `run <image>` - Run color extraction on the specified image.
    Run(&'a Path),
    /// `-C <path>` - Use specified config file.
    ConfigFile(&'a Path),
    /// `--templates-dir <path>` - Use specified templates directory.
    TemplatesDir(&'a Path),
}

impl Arg<'_> {
    fn apply(&self, cmd: &mut Command) {
        match self {
            Self::Run(path) => {
                cmd.args(["run", &path.to_string_lossy()]);
            }
            Self::ConfigFile(path) => {
                cmd.args(["-C", &path.to_string_lossy()]);
            }
            Self::TemplatesDir(path) => {
                cmd.args(["--templates-dir", &path.to_string_lossy()]);
            }
        }
    }
}

async fn run(tool: Tool, args: &[Arg<'_>]) -> Result<(), Error> {
    let mut cmd = Command::new("wallust");

    for arg in args {
        arg.apply(&mut cmd);
    }

    let output = tool.run(cmd).await?;
    tool.check_success(&output)
}

/// Runs wallust color extraction on the given image.
///
/// Executes wallust twice:
/// 1. With user's config (their templates like kitty run)
/// 2. With wayle's config (generates colors.json for wayle, uses cached colors)
///
/// # Errors
///
/// Returns error if wallust command fails.
pub async fn extract(image_path: &str) -> Result<(), Error> {
    let path = Path::new(image_path);

    run(Tool::Wallust, &[Arg::Run(path)]).await?;

    let (config, templates_dir) = ensure_wayle_config().await?;
    run(
        Tool::WallustTemplates,
        &[
            Arg::Run(path),
            Arg::ConfigFile(&config),
            Arg::TemplatesDir(&templates_dir),
        ],
    )
    .await?;

    debug!(image = %image_path, "Wallust color extraction complete");
    Ok(())
}

async fn ensure_wayle_config() -> Result<(PathBuf, PathBuf), Error> {
    let data_dir = ConfigPaths::data_dir().map_err(|source| Error::ConfigPathError {
        context: "wayle data directory",
        source,
    })?;

    let wallust_dir = data_dir.join("wallust");
    let templates_dir = wallust_dir.join("templates");
    let config_path = wallust_dir.join("wallust.toml");
    let template_path = templates_dir.join("colors.json");

    fs::create_dir_all(&templates_dir)
        .await
        .map_err(|source| Error::ConfigPathError {
            context: "creating wallust templates directory",
            source,
        })?;

    let colors_output = ConfigPaths::wallust_colors().map_err(|source| Error::ConfigPathError {
        context: "wallust colors output path",
        source,
    })?;

    let config_content = format!(
        r#"
palette = "dark"
check_contrast = true
dynamic_threshold = true

[templates]
colors = {{ template = "colors.json", target = "{}" }}
"#,
        colors_output.display()
    );

    fs::write(&config_path, config_content)
        .await
        .map_err(|source| Error::ConfigPathError {
            context: "writing wayle wallust config",
            source,
        })?;

    fs::write(&template_path, COLORS_TEMPLATE)
        .await
        .map_err(|source| Error::ConfigPathError {
            context: "writing colors.json template",
            source,
        })?;

    Ok((config_path, templates_dir))
}
