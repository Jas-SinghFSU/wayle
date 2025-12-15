use std::{
    env, fs,
    io::{Error, ErrorKind},
    path::PathBuf,
};

/// Utility struct for managing configuration file paths
///
/// Provides methods to locate configuration directories and files following
/// the XDG Base Directory specification
pub struct ConfigPaths;

impl ConfigPaths {
    /// Returns the configuration directory path for the application
    ///
    /// Follows the XDG Base Directory specification:
    /// - First checks `XDG_CONFIG_HOME`
    /// - Falls back to `$HOME/.config`
    /// - Appends "wayle" to the base config directory
    ///
    /// # Errors
    /// Returns an error if neither `XDG_CONFIG_HOME` nor `HOME` environment variables are set
    pub fn config_dir() -> Result<PathBuf, Error> {
        let config_home = env::var("XDG_CONFIG_HOME")
            .or_else(|_| env::var("HOME").map(|home| format!("{home}/.config")))
            .map_err(|e| {
                Error::new(
                    ErrorKind::NotFound,
                    format!("Neither XDG_CONFIG_HOME nor HOME environment variable found: {e}"),
                )
            })?;

        Ok(PathBuf::from(config_home).join("wayle"))
    }

    /// Returns the application data directory path
    ///
    /// Creates the directory if it doesn't exist.
    ///
    /// # Errors
    /// Returns an error if HOME environment variable is not set or directory cannot be created
    pub fn app_data_dir() -> Result<PathBuf, Error> {
        let data_dir = env::var("HOME")
            .map(|home| format!("{home}/.wayle"))
            .map_err(|e| {
                Error::new(
                    ErrorKind::NotFound,
                    format!("HOME environment variable not found: {e}"),
                )
            })?;

        let app_dir = PathBuf::from(data_dir);

        if !app_dir.exists() {
            fs::create_dir_all(&app_dir)?;
        }

        Ok(app_dir)
    }

    /// Get the application log directory
    ///
    /// Creates the directory if it doesn't exist.
    ///
    /// # Errors
    /// Returns error if directory cannot be created
    pub fn log_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let app_dir = Self::app_data_dir()?;
        let log_dir = app_dir.join("logs");

        if !log_dir.exists() {
            fs::create_dir_all(&log_dir)?;
        }

        Ok(log_dir)
    }

    /// Returns the path to the main configuration file
    ///
    /// # Panics
    /// Panics if neither HOME nor XDG_CONFIG_HOME environment variables are set
    #[allow(clippy::panic)]
    pub fn main_config() -> PathBuf {
        match Self::config_dir() {
            Ok(dir) => dir.join("config.toml"),
            Err(_) => {
                panic!("Failed to determine config directory - is $HOME or $XDG_CONFIG_HOME set?")
            }
        }
    }

    /// Returns the path to the GUI configuration file
    ///
    /// # Panics
    /// Panics if neither HOME nor XDG_CONFIG_HOME environment variables are set
    #[allow(clippy::panic)]
    pub fn runtime_config() -> PathBuf {
        match Self::config_dir() {
            Ok(dir) => dir.join("runtime.toml"),
            Err(_) => {
                panic!("Failed to determine config directory - is $HOME or $XDG_CONFIG_HOME set?")
            }
        }
    }
}
