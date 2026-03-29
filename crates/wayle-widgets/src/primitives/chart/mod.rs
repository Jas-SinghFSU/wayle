//! Generic chart rendering parameters shared across charting primitives.

/// RGBA color with components normalized to 0.0-1.0.
#[derive(Clone)]
pub struct Rgba {
    /// Red component (0.0-1.0).
    pub red: f64,
    /// Green component (0.0-1.0).
    pub green: f64,
    /// Blue component (0.0-1.0).
    pub blue: f64,
    /// Alpha/opacity component (0.0-1.0).
    pub alpha: f64,
}

/// Generic rendering parameters shared across visualizations.
#[derive(Clone)]
pub struct Params {
    /// Fill color for the visualization.
    pub fill_color: Rgba,
}
