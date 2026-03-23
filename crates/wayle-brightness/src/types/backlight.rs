use std::fmt::{self, Display, Formatter};

/// Kernel backlight device type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BacklightType {
    /// Controlled using hardware registers.
    Raw,
    /// Controlled using a platform-specific interface.
    Platform,
    /// Controlled using a standard firmware interface.
    Firmware,
}

impl BacklightType {
    /// Parses the sysfs `type` file content.
    pub fn from_sysfs(value: &str) -> Option<Self> {
        match value.trim() {
            "firmware" => Some(Self::Firmware),
            "platform" => Some(Self::Platform),
            "raw" => Some(Self::Raw),
            _ => None,
        }
    }
}

/// Brightness as a percentage, clamped to 0.0-100.0.
///
/// ```
/// use wayle_brightness::types::Percentage;
///
/// let p = Percentage::new(75.0);
/// assert_eq!(p.value(), 75.0);
/// assert!((p.fraction() - 0.75).abs() < f64::EPSILON);
///
/// let clamped = Percentage::new(150.0);
/// assert_eq!(clamped.value(), 100.0);
///
/// let from_frac = Percentage::from_fraction(0.5);
/// assert_eq!(from_frac.value(), 50.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage(f64);

impl Percentage {
    /// Creates a percentage, clamping to 0.0-100.0.
    ///
    /// NaN is treated as 0.0.
    pub fn new(value: f64) -> Self {
        if value.is_nan() {
            return Self(0.0);
        }

        Self(value.clamp(0.0, 100.0))
    }

    /// Creates a percentage from a 0.0-1.0 fraction.
    pub fn from_fraction(fraction: f64) -> Self {
        Self::new(fraction * 100.0)
    }

    /// Clamped value between 0.0 and 100.0.
    pub fn value(self) -> f64 {
        self.0
    }

    /// The value as a 0.0-1.0 fraction.
    pub fn fraction(self) -> f64 {
        self.0 / 100.0
    }
}

impl Display for Percentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0.round() as u32)
    }
}

/// Backlight device name from `/sys/class/backlight/`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeviceName(String);

impl DeviceName {
    /// Creates from a sysfs backlight directory entry name.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Borrowed view of the device name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for DeviceName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for DeviceName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Snapshot of a backlight device's current state.
#[derive(Debug, Clone)]
pub struct BacklightInfo {
    /// Sysfs directory entry name.
    pub name: DeviceName,
    /// Controls primary device selection priority.
    pub backlight_type: BacklightType,
    /// Raw value from `actual_brightness` sysfs attribute.
    pub brightness: u32,
    /// Hardware ceiling from `max_brightness` sysfs attribute.
    pub max_brightness: u32,
}
