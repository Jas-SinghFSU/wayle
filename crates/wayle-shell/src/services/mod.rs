//! Shell-specific services that run alongside the UI.

pub mod idle_inhibit;

pub use idle_inhibit::IdleInhibitService;
