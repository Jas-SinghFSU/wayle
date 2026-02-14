use std::fmt::{self, Display, Formatter};

/// Position for Grow/Outer transitions as percentage of screen dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Position {
    /// X position as percentage (0.0 = left edge, 1.0 = right edge).
    pub x: f32,
    /// Y position as percentage (0.0 = top edge, 1.0 = bottom edge).
    pub y: f32,
}

impl Position {
    /// CLI flag for this parameter.
    pub const FLAG: &'static str = "--transition-pos";

    /// Center of the screen (awww default).
    pub const CENTER: Self = Self { x: 0.5, y: 0.5 };

    /// Top-left corner.
    pub const TOP_LEFT: Self = Self { x: 0.0, y: 0.0 };

    /// Top-right corner.
    pub const TOP_RIGHT: Self = Self { x: 1.0, y: 0.0 };

    /// Bottom-left corner.
    pub const BOTTOM_LEFT: Self = Self { x: 0.0, y: 1.0 };

    /// Bottom-right corner.
    pub const BOTTOM_RIGHT: Self = Self { x: 1.0, y: 1.0 };
}

/// Wave dimensions for the Wave transition effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WaveDimensions {
    /// Width of each wave.
    pub width: u32,
    /// Height of each wave.
    pub height: u32,
}

impl WaveDimensions {
    /// CLI flag for this parameter.
    pub const FLAG: &'static str = "--transition-wave";

    /// Default wave dimensions (20x20).
    pub const DEFAULT: Self = Self {
        width: 20,
        height: 20,
    };
}

/// Bezier curve control points for transition easing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BezierCurve {
    /// First control point X.
    pub x1: f32,
    /// First control point Y.
    pub y1: f32,
    /// Second control point X.
    pub x2: f32,
    /// Second control point Y.
    pub y2: f32,
}

impl Default for BezierCurve {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl BezierCurve {
    /// CLI flag for this parameter.
    pub const FLAG: &'static str = "--transition-bezier";

    /// awww default bezier curve.
    pub const DEFAULT: Self = Self {
        x1: 0.54,
        y1: 0.0,
        x2: 0.34,
        y2: 0.99,
    };

    /// Linear easing (no acceleration).
    pub const LINEAR: Self = Self {
        x1: 0.0,
        y1: 0.0,
        x2: 1.0,
        y2: 1.0,
    };

    /// Ease-out (fast start, slow end).
    pub const EASE_OUT: Self = Self {
        x1: 0.0,
        y1: 0.0,
        x2: 0.2,
        y2: 1.0,
    };

    /// Ease-in-out (slow start and end).
    pub const EASE_IN_OUT: Self = Self {
        x1: 0.42,
        y1: 0.0,
        x2: 0.58,
        y2: 1.0,
    };
}

/// Transition animation duration in seconds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransitionDuration(pub f32);

impl TransitionDuration {
    /// CLI flag for this parameter.
    pub const FLAG: &'static str = "--transition-duration";
}

impl Default for TransitionDuration {
    fn default() -> Self {
        Self(0.7)
    }
}

impl Display for TransitionDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

/// Transition animation frame rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransitionFps(pub u32);

impl TransitionFps {
    /// CLI flag for this parameter.
    pub const FLAG: &'static str = "--transition-fps";
}

impl Default for TransitionFps {
    fn default() -> Self {
        Self(60)
    }
}

impl Display for TransitionFps {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

/// How much RGB values change per frame (1-255).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransitionStep(pub u8);

impl TransitionStep {
    /// CLI flag for this parameter.
    pub const FLAG: &'static str = "--transition-step";
}

impl Display for TransitionStep {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

/// Transition angle in degrees (0=right-to-left, 90=top-to-bottom).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransitionAngle(pub u16);

impl TransitionAngle {
    /// CLI flag for this parameter.
    pub const FLAG: &'static str = "--transition-angle";
}

impl Default for TransitionAngle {
    fn default() -> Self {
        Self(45)
    }
}

impl Display for TransitionAngle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

/// Transition animation type with its type-specific parameters.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TransitionType {
    /// Instant change with no animation.
    None,
    /// Basic fade into the new image (awww default).
    #[default]
    Simple,
    /// Fade with bezier-controlled timing.
    Fade {
        /// Easing curve for the fade.
        bezier: BezierCurve,
    },
    /// Wipe from left edge to right.
    Left,
    /// Wipe from right edge to left.
    Right,
    /// Wipe from top edge to bottom.
    Top,
    /// Wipe from bottom edge to top.
    Bottom,
    /// Wipe at configurable angle.
    Wipe {
        /// Wipe angle.
        angle: TransitionAngle,
    },
    /// Wavy wipe effect.
    Wave {
        /// Wave angle.
        angle: TransitionAngle,
        /// Wave dimensions.
        dimensions: WaveDimensions,
    },
    /// Growing circle from a position.
    Grow {
        /// Circle center position.
        position: Position,
    },
    /// Growing circle from center.
    Center,
    /// Shrinking circle from edges inward.
    Outer {
        /// Circle center position.
        position: Position,
    },
    /// Growing circle from random screen position.
    Any,
    /// Randomly selects from all transition types.
    Random,
}

impl TransitionType {
    /// CLI flag for the transition type parameter.
    pub const FLAG: &'static str = "--transition-type";

    pub(super) fn type_name(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Simple => "simple",
            Self::Fade { .. } => "fade",
            Self::Left => "left",
            Self::Right => "right",
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Wipe { .. } => "wipe",
            Self::Wave { .. } => "wave",
            Self::Grow { .. } => "grow",
            Self::Center => "center",
            Self::Outer { .. } => "outer",
            Self::Any => "any",
            Self::Random => "random",
        }
    }

    pub(super) fn cli_args(&self) -> Vec<(&'static str, String)> {
        match self {
            Self::None => vec![(TransitionStep::FLAG, "255".to_string())],
            Self::Fade { bezier } => vec![(
                BezierCurve::FLAG,
                format!("{},{},{},{}", bezier.x1, bezier.y1, bezier.x2, bezier.y2),
            )],
            Self::Wipe { angle } => vec![(TransitionAngle::FLAG, angle.to_string())],
            Self::Wave { angle, dimensions } => vec![
                (TransitionAngle::FLAG, angle.to_string()),
                (
                    WaveDimensions::FLAG,
                    format!("{},{}", dimensions.width, dimensions.height),
                ),
            ],
            Self::Grow { position } | Self::Outer { position } => {
                vec![(Position::FLAG, format!("{},{}", position.x, position.y))]
            }
            Self::Simple
            | Self::Left
            | Self::Right
            | Self::Top
            | Self::Bottom
            | Self::Center
            | Self::Any
            | Self::Random => vec![],
        }
    }

    /// Returns the display name as a static string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Simple => "Simple",
            Self::Fade { .. } => "Fade",
            Self::Left => "Left",
            Self::Right => "Right",
            Self::Top => "Top",
            Self::Bottom => "Bottom",
            Self::Wipe { .. } => "Wipe",
            Self::Wave { .. } => "Wave",
            Self::Grow { .. } => "Grow",
            Self::Center => "Center",
            Self::Outer { .. } => "Outer",
            Self::Any => "Any",
            Self::Random => "Random",
        }
    }
}

impl Display for TransitionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Configuration for wallpaper transitions.
///
/// Default values match awww defaults.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TransitionConfig {
    /// Transition type with its parameters.
    pub transition_type: TransitionType,
    /// Duration in seconds. Does not apply to Simple.
    pub duration: TransitionDuration,
    /// Animation frame rate.
    pub fps: TransitionFps,
    /// How much RGB values change per frame (1-255).
    pub step: Option<TransitionStep>,
}
