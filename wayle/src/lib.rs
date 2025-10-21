//! Wayle - Compositor-agnostic desktop environment framework.
//!
//! Wayle provides a unified framework for building desktop environment components
//! that work across different Wayland compositors. The main features include:
//!
//! - Reactive configuration system with TOML imports
//! - CLI interface for configuration management
//! - Compositor abstraction layer
//! - Panel and widget system

/// Configuration schema definitions and validation.
pub mod config;

/// Documentation generation for configuration schemas.
pub mod docs;

/// Command-line interface for configuration management.
pub mod cli;

/// Core runtime infrastructure.
pub mod core;
