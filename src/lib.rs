//! Wayle - Compositor-agnostic desktop environment framework.
//!
//! Wayle provides a unified framework for building desktop environment components
//! that work across different Wayland compositors. The main features include:
//!
//! - Reactive configuration system with TOML imports
//! - CLI interface for configuration management
//! - Compositor abstraction layer
//! - Panel and widget system
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use wayle::config_runtime::ConfigRuntime;
//!
//! // Create configuration store with defaults
//! let config_runtime = ConfigRuntime::with_defaults();
//!
//! // Access configuration values
//! let config = config_runtime.get_current();
//! println!("Config loaded: {:?}", config.general);
//! ```

/// Configuration schema definitions and validation.
pub mod config;

/// Documentation generation for configuration schemas.
pub mod docs;

/// Reactive configuration runtime with change tracking.
pub mod config_runtime;

/// Command-line interface for configuration management.
pub mod cli;

/// Reactive services for system integration.
pub mod services;

/// Core runtime infrastructure.
pub mod core;
