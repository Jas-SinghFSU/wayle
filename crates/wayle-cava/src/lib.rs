//! Real-time audio frequency visualization via libcava.
//!
//! # Overview
//!
//! Captures system audio and produces frequency bar data for visual representations.
//! The service wraps libcava's FFI interface and exposes configuration and output
//! through reactive [`Property`] fields.
//!
//! # Reactive Properties
//!
//! All [`CavaService`] fields are [`Property<T>`] types:
//!
//! - **Snapshot**: Call `.get()` for the current value
//! - **Stream**: Call `.watch()` for a `Stream<Item = T>` that yields on changes
//!
//! ```rust,no_run
//! use wayle_cava::CavaService;
//! use futures::StreamExt;
//!
//! # async fn example() -> Result<(), wayle_cava::Error> {
//! let cava = CavaService::new().await?;
//!
//! // Snapshot: get current bar values
//! let bars = cava.values.get();
//! println!("Bar count: {}, first value: {:.2}", bars.len(), bars[0]);
//!
//! // Stream: react to visualization updates
//! let mut stream = cava.values.watch();
//! while let Some(values) = stream.next().await {
//!     // values updates at configured framerate (default 60fps)
//!     draw_visualization(&values);
//! }
//! # Ok(())
//! # }
//! # fn draw_visualization(_: &[f64]) {}
//! ```
//!
//! # Configuration
//!
//! Use the builder for initial configuration:
//!
//! ```rust,no_run
//! # async fn example() -> Result<(), wayle_cava::Error> {
//! use wayle_cava::{CavaService, InputMethod};
//!
//! let cava = CavaService::builder()
//!     .bars(32)
//!     .framerate(30)
//!     .input(InputMethod::PipeWire)
//!     .stereo(true)
//!     .build()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! Runtime changes trigger automatic service restarts:
//!
//! ```rust,no_run
//! # async fn example() -> Result<(), wayle_cava::Error> {
//! # let cava = wayle_cava::CavaService::new().await?;
//! cava.set_bars(64).await?;  // Restarts with new bar count
//! cava.set_noise_reduction(0.5).await?;  // Restarts with new filter
//! # Ok(())
//! # }
//! ```
//!
//! # Service Fields
//!
//! | Field | Type | Description |
//! |-------|------|-------------|
//! | [`values`] | `Vec<f64>` | Bar amplitudes (0.0-1.0, can overshoot) |
//! | [`bars`] | `usize` | Number of frequency bars (1-256) |
//! | [`framerate`] | `u32` | Update rate in fps |
//! | [`input`] | `InputMethod` | Audio capture backend |
//! | [`source`] | `String` | Audio source ("auto" for auto-detect) |
//! | [`autosens`] | `bool` | Auto-adjust sensitivity |
//! | [`stereo`] | `bool` | Split bars between L/R channels |
//! | [`noise_reduction`] | `f64` | Smoothing filter (0.0-1.0) |
//! | [`low_cutoff`] | `u32` | Low frequency filter (Hz) |
//! | [`high_cutoff`] | `u32` | High frequency filter (Hz) |
//! | [`samplerate`] | `u32` | Audio sample rate (Hz) |
//!
//! [`values`]: CavaService::values
//! [`bars`]: CavaService::bars
//! [`framerate`]: CavaService::framerate
//! [`input`]: CavaService::input
//! [`source`]: CavaService::source
//! [`autosens`]: CavaService::autosens
//! [`stereo`]: CavaService::stereo
//! [`noise_reduction`]: CavaService::noise_reduction
//! [`low_cutoff`]: CavaService::low_cutoff
//! [`high_cutoff`]: CavaService::high_cutoff
//! [`samplerate`]: CavaService::samplerate
//! [`Property`]: wayle_common::Property
//! [`Property<T>`]: wayle_common::Property

mod builder;
mod error;
mod ffi;
mod monitoring;
mod service;

/// Public types for configuring CAVA visualization.
pub mod types;

pub use builder::CavaServiceBuilder;
pub use error::{Error, Result};
pub use service::CavaService;
pub use types::InputMethod;
