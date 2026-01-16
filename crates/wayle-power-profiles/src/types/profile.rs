use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult},
};

use zbus::zvariant::OwnedValue;

use crate::error::Error;

/// Cookie returned by hold operations.
pub type HoldCookie = u32;

/// System power profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    /// Battery saving mode.
    PowerSaver,
    /// Default balanced mode.
    Balanced,
    /// High performance mode.
    Performance,
}

impl From<&str> for PowerProfile {
    fn from(s: &str) -> Self {
        match s {
            "power-saver" => Self::PowerSaver,
            "balanced" => Self::Balanced,
            "performance" => Self::Performance,
            _ => Self::Balanced,
        }
    }
}

impl Display for PowerProfile {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::PowerSaver => write!(f, "power-saver"),
            Self::Balanced => write!(f, "balanced"),
            Self::Performance => write!(f, "performance"),
        }
    }
}

/// Performance degradation reasons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceDegradationReason {
    /// No degradation
    None,
    /// Computer is sitting on user's lap
    LapDetected,
    /// Computer is close to overheating
    HighOperatingTemperature,
    /// Unknown degradation reason
    Unknown,
}

impl From<&str> for PerformanceDegradationReason {
    fn from(s: &str) -> Self {
        match s {
            "" => Self::None,
            "lap-detected" => Self::LapDetected,
            "high-operating-temperature" => Self::HighOperatingTemperature,
            _ => Self::Unknown,
        }
    }
}

impl Display for PerformanceDegradationReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::None => write!(f, ""),
            Self::LapDetected => write!(f, "lap-detected"),
            Self::HighOperatingTemperature => write!(f, "high-operating-temperature"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// Profile information with driver details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    /// Driver name providing this profile
    pub driver: String,
    /// The power profile type
    pub profile: PowerProfile,
}

impl TryFrom<HashMap<String, OwnedValue>> for Profile {
    type Error = Error;

    fn try_from(dict: HashMap<String, OwnedValue>) -> Result<Self, Self::Error> {
        let driver = dict
            .get("Driver")
            .and_then(|v| v.downcast_ref::<String>().ok())
            .ok_or_else(|| Error::InvalidFieldType {
                field: "Driver".to_string(),
                expected: "String".to_string(),
            })?
            .clone();

        let profile_str = dict
            .get("Profile")
            .and_then(|v| v.downcast_ref::<String>().ok())
            .ok_or_else(|| Error::InvalidFieldType {
                field: "Profile".to_string(),
                expected: "String".to_string(),
            })?;

        let profile = PowerProfile::from(profile_str.as_str());

        Ok(Profile { driver, profile })
    }
}

/// Profile hold information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileHold {
    /// Application ID that requested the hold
    pub application_id: String,
    /// The power profile type
    pub profile: PowerProfile,
    /// Reason for requesting the profile hold
    pub reason: String,
}

impl TryFrom<HashMap<String, OwnedValue>> for ProfileHold {
    type Error = Error;

    fn try_from(dict: HashMap<String, OwnedValue>) -> Result<Self, Self::Error> {
        let application_id = dict
            .get("ApplicationId")
            .and_then(|v| v.downcast_ref::<String>().ok())
            .ok_or_else(|| Error::InvalidFieldType {
                field: "ApplicationId".to_string(),
                expected: "String".to_string(),
            })?
            .clone();

        let profile_str = dict
            .get("Profile")
            .and_then(|v| v.downcast_ref::<String>().ok())
            .ok_or_else(|| Error::InvalidFieldType {
                field: "Profile".to_string(),
                expected: "String".to_string(),
            })?;

        let profile = PowerProfile::from(profile_str.as_str());

        let reason = dict
            .get("Reason")
            .and_then(|v| v.downcast_ref::<String>().ok())
            .ok_or_else(|| Error::InvalidFieldType {
                field: "Reason".to_string(),
                expected: "String".to_string(),
            })?
            .clone();

        Ok(ProfileHold {
            application_id,
            profile,
            reason,
        })
    }
}

#[cfg(test)]
#[allow(clippy::panic)]
mod tests {
    use std::collections::HashMap;

    use zbus::zvariant::{OwnedValue, Str};

    use super::{PerformanceDegradationReason, PowerProfile, Profile, ProfileHold};
    use crate::error::Error;

    #[test]
    fn from_str_power_saver_returns_power_saver_variant() {
        let profile = PowerProfile::from("power-saver");

        assert_eq!(profile, PowerProfile::PowerSaver);
    }

    #[test]
    fn from_str_balanced_returns_balanced_variant() {
        let profile = PowerProfile::from("balanced");

        assert_eq!(profile, PowerProfile::Balanced);
    }

    #[test]
    fn from_str_performance_returns_performance_variant() {
        let profile = PowerProfile::from("performance");

        assert_eq!(profile, PowerProfile::Performance);
    }

    #[test]
    fn from_str_unknown_value_returns_balanced_default() {
        let profile = PowerProfile::from("unknown-profile");

        assert_eq!(profile, PowerProfile::Balanced);
    }

    #[test]
    fn from_str_empty_string_returns_balanced_default() {
        let profile = PowerProfile::from("");

        assert_eq!(profile, PowerProfile::Balanced);
    }

    #[test]
    fn from_str_empty_returns_none_variant() {
        let reason = PerformanceDegradationReason::from("");

        assert_eq!(reason, PerformanceDegradationReason::None);
    }

    #[test]
    fn from_str_lap_detected_returns_lap_detected_variant() {
        let reason = PerformanceDegradationReason::from("lap-detected");

        assert_eq!(reason, PerformanceDegradationReason::LapDetected);
    }

    #[test]
    fn from_str_high_operating_temperature_returns_high_operating_temperature_variant() {
        let reason = PerformanceDegradationReason::from("high-operating-temperature");

        assert_eq!(
            reason,
            PerformanceDegradationReason::HighOperatingTemperature
        );
    }

    #[test]
    fn from_str_unknown_value_returns_unknown_variant() {
        let reason = PerformanceDegradationReason::from("some-unknown-reason");

        assert_eq!(reason, PerformanceDegradationReason::Unknown);
    }

    #[test]
    fn try_from_valid_dict_returns_profile() {
        let mut dict = HashMap::new();
        dict.insert(
            "Driver".to_string(),
            OwnedValue::from(Str::from("platform_profile")),
        );
        dict.insert(
            "Profile".to_string(),
            OwnedValue::from(Str::from("performance")),
        );

        let result = Profile::try_from(dict);

        assert!(result.is_ok());
        let profile = result.unwrap();
        assert_eq!(profile.driver, "platform_profile");
        assert_eq!(profile.profile, PowerProfile::Performance);
    }

    #[test]
    fn try_from_missing_driver_returns_error_with_field_name() {
        let mut dict = HashMap::new();
        dict.insert(
            "Profile".to_string(),
            OwnedValue::from(Str::from("balanced")),
        );

        let result = Profile::try_from(dict);

        match result {
            Err(Error::InvalidFieldType { field, expected }) => {
                assert_eq!(field, "Driver");
                assert_eq!(expected, "String");
            }
            _ => panic!("Expected InvalidFieldType error for missing Driver field"),
        }
    }

    #[test]
    fn try_from_missing_profile_returns_error_with_field_name() {
        let mut dict = HashMap::new();
        dict.insert(
            "Driver".to_string(),
            OwnedValue::from(Str::from("platform_profile")),
        );

        let result = Profile::try_from(dict);

        match result {
            Err(Error::InvalidFieldType { field, expected }) => {
                assert_eq!(field, "Profile");
                assert_eq!(expected, "String");
            }
            _ => panic!("Expected InvalidFieldType error for missing Profile field"),
        }
    }

    #[test]
    fn profile_hold_try_from_valid_dict_returns_profile_hold() {
        let mut dict = HashMap::new();
        dict.insert(
            "ApplicationId".to_string(),
            OwnedValue::from(Str::from("my-app")),
        );
        dict.insert(
            "Profile".to_string(),
            OwnedValue::from(Str::from("performance")),
        );
        dict.insert(
            "Reason".to_string(),
            OwnedValue::from(Str::from("Running intensive workload")),
        );

        let result = ProfileHold::try_from(dict);

        assert!(result.is_ok());
        let hold = result.unwrap();
        assert_eq!(hold.application_id, "my-app");
        assert_eq!(hold.profile, PowerProfile::Performance);
        assert_eq!(hold.reason, "Running intensive workload");
    }

    #[test]
    fn profile_hold_try_from_missing_application_id_returns_error_with_field_name() {
        let mut dict = HashMap::new();
        dict.insert(
            "Profile".to_string(),
            OwnedValue::from(Str::from("performance")),
        );
        dict.insert("Reason".to_string(), OwnedValue::from(Str::from("test")));

        let result = ProfileHold::try_from(dict);

        match result {
            Err(Error::InvalidFieldType { field, expected }) => {
                assert_eq!(field, "ApplicationId");
                assert_eq!(expected, "String");
            }
            _ => panic!("Expected InvalidFieldType error for missing ApplicationId field"),
        }
    }

    #[test]
    fn profile_hold_try_from_missing_profile_returns_error_with_field_name() {
        let mut dict = HashMap::new();
        dict.insert(
            "ApplicationId".to_string(),
            OwnedValue::from(Str::from("my-app")),
        );
        dict.insert("Reason".to_string(), OwnedValue::from(Str::from("test")));

        let result = ProfileHold::try_from(dict);

        match result {
            Err(Error::InvalidFieldType { field, expected }) => {
                assert_eq!(field, "Profile");
                assert_eq!(expected, "String");
            }
            _ => panic!("Expected InvalidFieldType error for missing Profile field"),
        }
    }

    #[test]
    fn profile_hold_try_from_missing_reason_returns_error_with_field_name() {
        let mut dict = HashMap::new();
        dict.insert(
            "ApplicationId".to_string(),
            OwnedValue::from(Str::from("my-app")),
        );
        dict.insert(
            "Profile".to_string(),
            OwnedValue::from(Str::from("performance")),
        );

        let result = ProfileHold::try_from(dict);

        match result {
            Err(Error::InvalidFieldType { field, expected }) => {
                assert_eq!(field, "Reason");
                assert_eq!(expected, "String");
            }
            _ => panic!("Expected InvalidFieldType error for missing Reason field"),
        }
    }

    #[test]
    fn power_profile_string_conversions_are_consistent() {
        let profiles = [
            PowerProfile::PowerSaver,
            PowerProfile::Balanced,
            PowerProfile::Performance,
        ];

        for original in profiles {
            let string = original.to_string();
            let parsed = PowerProfile::from(string.as_str());
            assert_eq!(original, parsed, "Round-trip failed for {:?}", original);
        }
    }

    #[test]
    fn performance_degradation_reason_string_conversions_are_consistent() {
        let reasons = [
            PerformanceDegradationReason::LapDetected,
            PerformanceDegradationReason::HighOperatingTemperature,
            PerformanceDegradationReason::Unknown,
        ];

        for original in reasons {
            let string = original.to_string();
            let parsed = PerformanceDegradationReason::from(string.as_str());
            assert_eq!(original, parsed, "Round-trip failed for {:?}", original);
        }
    }
}
