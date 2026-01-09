//! Wallpaper rendering backends.

mod swww;

pub use swww::{
    BezierCurve, Position, SwwwBackend, TransitionAngle, TransitionConfig, TransitionDuration,
    TransitionFps, TransitionStep, TransitionType, WaveDimensions,
};
