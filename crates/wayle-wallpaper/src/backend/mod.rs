mod swww;

pub(crate) use swww::SwwwBackend;
pub use swww::{
    BezierCurve, Position, TransitionAngle, TransitionConfig, TransitionDuration, TransitionFps,
    TransitionStep, TransitionType, WaveDimensions,
};
