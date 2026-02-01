use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

/// Percentage (0-100) for humidity, cloud cover, and precipitation chance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Percentage(u8);

impl Percentage {
    /// Lower bound.
    pub const ZERO: Self = Self(0);
    /// Upper bound.
    pub const MAX: Self = Self(100);

    /// Returns `None` if `value > 100`.
    pub fn new(value: u8) -> Option<Self> {
        (value <= 100).then_some(Self(value))
    }

    /// Clamps values exceeding 100 to handle quirky provider data.
    #[must_use]
    pub fn saturating(value: u8) -> Self {
        Self(value.min(100))
    }

    /// The underlying `u8`.
    #[must_use]
    pub fn get(self) -> u8 {
        self.0
    }
}

impl Display for Percentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

/// Wind bearing (0-359°) with cardinal direction conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct WindDirection(u16);

impl WindDirection {
    /// 0°
    pub const NORTH: Self = Self(0);
    /// 90°
    pub const EAST: Self = Self(90);
    /// 180°
    pub const SOUTH: Self = Self(180);
    /// 270°
    pub const WEST: Self = Self(270);

    /// Returns `None` if `degrees >= 360`.
    pub fn new(degrees: u16) -> Option<Self> {
        (degrees < 360).then_some(Self(degrees))
    }

    /// Wraps using modulo 360 to normalize out-of-range values.
    #[must_use]
    pub fn saturating(degrees: u16) -> Self {
        Self(degrees % 360)
    }

    /// The bearing as degrees.
    #[must_use]
    pub fn degrees(self) -> u16 {
        self.0
    }

    /// Converts to 8-point compass (N, NE, E, SE, S, SW, W, NW).
    #[must_use]
    pub fn cardinal(self) -> &'static str {
        match self.0 {
            0..=22 | 338..=359 => "N",
            23..=67 => "NE",
            68..=112 => "E",
            113..=157 => "SE",
            158..=202 => "S",
            203..=247 => "SW",
            248..=292 => "W",
            _ => "NW",
        }
    }
}

impl Display for WindDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}°", self.0)
    }
}

/// UV radiation index (0-15) with WHO risk classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UvIndex(u8);

impl UvIndex {
    /// No UV exposure.
    pub const ZERO: Self = Self(0);

    /// Returns `None` if `value > 15`.
    pub fn new(value: u8) -> Option<Self> {
        (value <= 15).then_some(Self(value))
    }

    /// Caps extreme values at 15.
    #[must_use]
    pub fn saturating(value: u8) -> Self {
        Self(value.min(15))
    }

    /// The underlying index.
    #[must_use]
    pub fn get(self) -> u8 {
        self.0
    }

    /// WHO classification: Low, Moderate, High, Very High, or Extreme.
    #[must_use]
    pub fn risk_level(self) -> &'static str {
        match self.0 {
            0..=2 => "Low",
            3..=5 => "Moderate",
            6..=7 => "High",
            8..=10 => "Very High",
            _ => "Extreme",
        }
    }
}

impl Display for UvIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Temperature stored in Celsius, convertible to Fahrenheit.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Temperature(f32);

impl Temperature {
    /// Returns `None` for NaN or infinity.
    pub fn new(celsius: f32) -> Option<Self> {
        celsius.is_finite().then_some(Self(celsius))
    }

    /// Value in degrees Celsius.
    #[must_use]
    pub fn celsius(self) -> f32 {
        self.0
    }

    /// Converted to degrees Fahrenheit.
    #[must_use]
    pub fn fahrenheit(self) -> f32 {
        self.0 * 9.0 / 5.0 + 32.0
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.0)
    }
}

/// Wind or gust speed stored in km/h, convertible to mph.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Speed(f32);

impl Speed {
    /// Calm conditions.
    pub const ZERO: Self = Self(0.0);

    /// Returns `None` for negative or non-finite values.
    pub fn new(kmh: f32) -> Option<Self> {
        (kmh >= 0.0 && kmh.is_finite()).then_some(Self(kmh))
    }

    /// Value in kilometers per hour.
    #[must_use]
    pub fn kmh(self) -> f32 {
        self.0
    }

    /// Converted to miles per hour.
    #[must_use]
    pub fn mph(self) -> f32 {
        self.0 * 0.621371
    }
}

impl Display for Speed {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} km/h", self.0)
    }
}

/// Visibility distance stored in km, convertible to miles.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Distance(f32);

impl Distance {
    /// Zero visibility (dense fog, etc).
    pub const ZERO: Self = Self(0.0);

    /// Returns `None` for negative or non-finite values.
    pub fn new(km: f32) -> Option<Self> {
        (km >= 0.0 && km.is_finite()).then_some(Self(km))
    }

    /// Creates from meters since Open-Meteo reports visibility in meters.
    pub fn from_meters(m: f32) -> Option<Self> {
        Self::new(m / 1000.0)
    }

    /// Value in kilometers.
    #[must_use]
    pub fn km(self) -> f32 {
        self.0
    }

    /// Converted to miles.
    #[must_use]
    pub fn miles(self) -> f32 {
        self.0 * 0.621371
    }
}

impl Display for Distance {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} km", self.0)
    }
}

/// Atmospheric pressure stored in hPa, convertible to inHg.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Pressure(f32);

impl Pressure {
    /// Returns `None` for negative or non-finite values.
    pub fn new(hpa: f32) -> Option<Self> {
        (hpa >= 0.0 && hpa.is_finite()).then_some(Self(hpa))
    }

    /// Value in hectopascals.
    #[must_use]
    pub fn hpa(self) -> f32 {
        self.0
    }

    /// Converted to inches of mercury.
    #[must_use]
    pub fn inhg(self) -> f32 {
        self.0 * 0.02953
    }
}

impl Display for Pressure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0} hPa", self.0)
    }
}

/// Precipitation amount stored in mm, convertible to inches.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Precipitation(f32);

impl Precipitation {
    /// No precipitation.
    pub const ZERO: Self = Self(0.0);

    /// Returns `None` for negative or non-finite values.
    pub fn new(mm: f32) -> Option<Self> {
        (mm >= 0.0 && mm.is_finite()).then_some(Self(mm))
    }

    /// Value in millimeters.
    #[must_use]
    pub fn mm(self) -> f32 {
        self.0
    }

    /// Converted to inches.
    #[must_use]
    pub fn inches(self) -> f32 {
        self.0 * 0.0393701
    }
}

impl Display for Precipitation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} mm", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentage_validates() {
        assert!(Percentage::new(0).is_some());
        assert!(Percentage::new(100).is_some());
        assert!(Percentage::new(101).is_none());
    }

    #[test]
    fn percentage_saturates() {
        assert_eq!(Percentage::saturating(150).get(), 100);
        assert_eq!(Percentage::saturating(50).get(), 50);
    }

    #[test]
    fn wind_direction_validates() {
        assert!(WindDirection::new(0).is_some());
        assert!(WindDirection::new(359).is_some());
        assert!(WindDirection::new(360).is_none());
    }

    #[test]
    fn wind_direction_wraps() {
        assert_eq!(WindDirection::saturating(360).degrees(), 0);
        assert_eq!(WindDirection::saturating(450).degrees(), 90);
    }

    #[test]
    fn wind_cardinal_directions() {
        assert_eq!(WindDirection::NORTH.cardinal(), "N");
        assert_eq!(WindDirection::EAST.cardinal(), "E");
        assert_eq!(WindDirection::SOUTH.cardinal(), "S");
        assert_eq!(WindDirection::WEST.cardinal(), "W");
    }

    #[test]
    fn uv_risk_levels() {
        assert_eq!(UvIndex::saturating(1).risk_level(), "Low");
        assert_eq!(UvIndex::saturating(5).risk_level(), "Moderate");
        assert_eq!(UvIndex::saturating(7).risk_level(), "High");
        assert_eq!(UvIndex::saturating(10).risk_level(), "Very High");
        assert_eq!(UvIndex::saturating(11).risk_level(), "Extreme");
    }

    #[test]
    fn temperature_conversion() {
        let freezing = Temperature::new(0.0).unwrap();
        assert!((freezing.fahrenheit() - 32.0).abs() < 0.01);

        let boiling = Temperature::new(100.0).unwrap();
        assert!((boiling.fahrenheit() - 212.0).abs() < 0.01);
    }

    #[test]
    fn speed_conversion() {
        let speed = Speed::new(100.0).unwrap();
        assert!((speed.mph() - 62.1371).abs() < 0.01);
    }

    #[test]
    fn distance_from_meters() {
        let dist = Distance::from_meters(10000.0).unwrap();
        assert!((dist.km() - 10.0).abs() < 0.01);
    }

    #[test]
    fn rejects_nan() {
        assert!(Temperature::new(f32::NAN).is_none());
        assert!(Speed::new(f32::NAN).is_none());
        assert!(Distance::new(f32::INFINITY).is_none());
    }
}
