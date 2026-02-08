mod swww;

pub use swww::{
    BezierCurve, Position, TransitionAngle, TransitionConfig, TransitionDuration, TransitionFps,
    TransitionStep, TransitionType, WaveDimensions,
};
pub(crate) use swww::{SwwwBackend, spawn_daemon_if_needed};
