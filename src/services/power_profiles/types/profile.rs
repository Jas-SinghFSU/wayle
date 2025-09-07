use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult},
};

use serde::{Deserialize, Serialize};
use zbus::zvariant::OwnedValue;

use crate::services::power_profiles::error::PowerProfilesError;

/// Power profile types available in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerProfile {
    /// Battery saving profile
    PowerSaver,
    /// The default balanced profile
    Balanced,
    /// High performance profile
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    /// Driver name providing this profile
    pub driver: String,
    /// The power profile type
    pub profile: PowerProfile,
}

impl TryFrom<HashMap<String, OwnedValue>> for Profile {
    type Error = PowerProfilesError;

    fn try_from(dict: HashMap<String, OwnedValue>) -> Result<Self, Self::Error> {
        let driver = dict
            .get("Driver")
            .and_then(|v| v.downcast_ref::<String>().ok())
            .ok_or_else(|| PowerProfilesError::InvalidFieldType {
                field: "Driver".to_string(),
                expected: "String".to_string(),
            })?
            .clone();

        let profile_str = dict
            .get("Profile")
            .and_then(|v| v.downcast_ref::<String>().ok())
            .ok_or_else(|| PowerProfilesError::InvalidFieldType {
                field: "Profile".to_string(),
                expected: "String".to_string(),
            })?;

        let profile = PowerProfile::from(profile_str.as_str());

        Ok(Profile { driver, profile })
    }
}

/// Profile hold information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileHold {
    /// Application ID that requested the hold
    pub application_id: String,
    /// The power profile type
    pub profile: PowerProfile,
    /// Reason for requesting the profile hold
    pub reason: String,
}

impl TryFrom<HashMap<String, OwnedValue>> for ProfileHold {
    type Error = PowerProfilesError;

    fn try_from(dict: HashMap<String, OwnedValue>) -> Result<Self, Self::Error> {
        let application_id = dict
            .get("ApplicationId")
            .and_then(|v| v.downcast_ref::<String>().ok())
            .ok_or_else(|| PowerProfilesError::InvalidFieldType {
                field: "ApplicationId".to_string(),
                expected: "String".to_string(),
            })?
            .clone();

        let profile_str = dict
            .get("Profile")
            .and_then(|v| v.downcast_ref::<String>().ok())
            .ok_or_else(|| PowerProfilesError::InvalidFieldType {
                field: "Profile".to_string(),
                expected: "String".to_string(),
            })?;

        let profile = PowerProfile::from(profile_str.as_str());

        let reason = dict
            .get("Reason")
            .and_then(|v| v.downcast_ref::<String>().ok())
            .ok_or_else(|| PowerProfilesError::InvalidFieldType {
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
