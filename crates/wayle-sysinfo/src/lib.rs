//! System information monitoring service.
//!
//! Provides reactive access to CPU, memory, disk, and network metrics
//! via polling-based background tasks.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use wayle_sysinfo::SysinfoService;
//!
//! let service = SysinfoService::builder().build();
//!
//! // Access current values
//! let cpu = service.cpu.get();
//! println!("CPU: {:.1}%", cpu.usage_percent);
//!
//! let memory = service.memory.get();
//! println!("Memory: {:.1}%", memory.usage_percent);
//! ```
//!
//! # Reactive Streams
//!
//! All properties support `.watch()` for reactive updates:
//!
//! ```rust,no_run
//! use wayle_sysinfo::SysinfoService;
//! use futures::StreamExt;
//!
//! # async fn example() {
//! let service = SysinfoService::builder().build();
//!
//! let mut cpu_stream = service.cpu.watch();
//! while let Some(cpu) = cpu_stream.next().await {
//!     println!("CPU changed: {:.1}%", cpu.usage_percent);
//! }
//! # }
//! ```

mod builder;
mod error;
mod polling;
mod service;
/// Data types for system metrics.
pub mod types;

pub use builder::SysinfoServiceBuilder;
pub use error::Error;
pub use service::SysinfoService;
