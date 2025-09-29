use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};

/// Type of power source as defined by UPower
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    /// Unknown device type
    Unknown,
    /// Line Power (AC adapter)
    LinePower,
    /// Battery
    Battery,
    /// Uninterruptible Power Supply
    Ups,
    /// Monitor
    Monitor,
    /// Mouse
    Mouse,
    /// Keyboard
    Keyboard,
    /// Personal Digital Assistant
    Pda,
    /// Phone
    Phone,
    /// Media Player
    MediaPlayer,
    /// Tablet
    Tablet,
    /// Computer
    Computer,
    /// Gaming Input
    GamingInput,
    /// Pen
    Pen,
    /// Touchpad
    Touchpad,
    /// Modem
    Modem,
    /// Network
    Network,
    /// Headset
    Headset,
    /// Speakers
    Speakers,
    /// Headphones
    Headphones,
    /// Video
    Video,
    /// Other Audio
    OtherAudio,
    /// Remote Control
    RemoteControl,
    /// Printer
    Printer,
    /// Scanner
    Scanner,
    /// Camera
    Camera,
    /// Wearable
    Wearable,
    /// Toy
    Toy,
    /// Bluetooth Generic
    BluetoothGeneric,
}

impl From<u32> for DeviceType {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::LinePower,
            2 => Self::Battery,
            3 => Self::Ups,
            4 => Self::Monitor,
            5 => Self::Mouse,
            6 => Self::Keyboard,
            7 => Self::Pda,
            8 => Self::Phone,
            9 => Self::MediaPlayer,
            10 => Self::Tablet,
            11 => Self::Computer,
            12 => Self::GamingInput,
            13 => Self::Pen,
            14 => Self::Touchpad,
            15 => Self::Modem,
            16 => Self::Network,
            17 => Self::Headset,
            18 => Self::Speakers,
            19 => Self::Headphones,
            20 => Self::Video,
            21 => Self::OtherAudio,
            22 => Self::RemoteControl,
            23 => Self::Printer,
            24 => Self::Scanner,
            25 => Self::Camera,
            26 => Self::Wearable,
            27 => Self::Toy,
            28 => Self::BluetoothGeneric,
            _ => Self::Unknown,
        }
    }
}

impl Display for DeviceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::LinePower => write!(f, "Line Power"),
            Self::Battery => write!(f, "Battery"),
            Self::Ups => write!(f, "UPS"),
            Self::Monitor => write!(f, "Monitor"),
            Self::Mouse => write!(f, "Mouse"),
            Self::Keyboard => write!(f, "Keyboard"),
            Self::Pda => write!(f, "PDA"),
            Self::Phone => write!(f, "Phone"),
            Self::MediaPlayer => write!(f, "Media Player"),
            Self::Tablet => write!(f, "Tablet"),
            Self::Computer => write!(f, "Computer"),
            Self::GamingInput => write!(f, "Gaming Input"),
            Self::Pen => write!(f, "Pen"),
            Self::Touchpad => write!(f, "Touchpad"),
            Self::Modem => write!(f, "Modem"),
            Self::Network => write!(f, "Network"),
            Self::Headset => write!(f, "Headset"),
            Self::Speakers => write!(f, "Speakers"),
            Self::Headphones => write!(f, "Headphones"),
            Self::Video => write!(f, "Video"),
            Self::OtherAudio => write!(f, "Other Audio"),
            Self::RemoteControl => write!(f, "Remote Control"),
            Self::Printer => write!(f, "Printer"),
            Self::Scanner => write!(f, "Scanner"),
            Self::Camera => write!(f, "Camera"),
            Self::Wearable => write!(f, "Wearable"),
            Self::Toy => write!(f, "Toy"),
            Self::BluetoothGeneric => write!(f, "Bluetooth Generic"),
        }
    }
}

/// The battery power state as defined by UPower
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceState {
    /// Unknown state
    Unknown,
    /// Battery is charging
    Charging,
    /// Battery is discharging
    Discharging,
    /// Battery is empty
    Empty,
    /// Battery is fully charged
    FullyCharged,
    /// Battery is pending charge
    PendingCharge,
    /// Battery is pending discharge
    PendingDischarge,
}

impl From<u32> for DeviceState {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Charging,
            2 => Self::Discharging,
            3 => Self::Empty,
            4 => Self::FullyCharged,
            5 => Self::PendingCharge,
            6 => Self::PendingDischarge,
            _ => Self::Unknown,
        }
    }
}

impl Display for DeviceState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Charging => write!(f, "Charging"),
            Self::Discharging => write!(f, "Discharging"),
            Self::Empty => write!(f, "Empty"),
            Self::FullyCharged => write!(f, "Fully Charged"),
            Self::PendingCharge => write!(f, "Pending Charge"),
            Self::PendingDischarge => write!(f, "Pending Discharge"),
        }
    }
}

/// Technology used in the battery
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatteryTechnology {
    /// Unknown technology
    Unknown,
    /// Lithium ion
    LithiumIon,
    /// Lithium polymer
    LithiumPolymer,
    /// Lithium iron phosphate
    LithiumIronPhosphate,
    /// Lead acid
    LeadAcid,
    /// Nickel cadmium
    NickelCadmium,
    /// Nickel metal hydride
    NickelMetalHydride,
}

impl From<u32> for BatteryTechnology {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::LithiumIon,
            2 => Self::LithiumPolymer,
            3 => Self::LithiumIronPhosphate,
            4 => Self::LeadAcid,
            5 => Self::NickelCadmium,
            6 => Self::NickelMetalHydride,
            _ => Self::Unknown,
        }
    }
}

impl Display for BatteryTechnology {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::LithiumIon => write!(f, "Lithium Ion"),
            Self::LithiumPolymer => write!(f, "Lithium Polymer"),
            Self::LithiumIronPhosphate => write!(f, "Lithium Iron Phosphate"),
            Self::LeadAcid => write!(f, "Lead Acid"),
            Self::NickelCadmium => write!(f, "Nickel Cadmium"),
            Self::NickelMetalHydride => write!(f, "Nickel Metal Hydride"),
        }
    }
}

/// Warning level of the battery
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarningLevel {
    /// Unknown warning level
    Unknown,
    /// No warning
    None,
    /// Discharging (only for UPSes)
    Discharging,
    /// Low battery
    Low,
    /// Critical battery
    Critical,
    /// Action required
    Action,
}

impl From<u32> for WarningLevel {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::None,
            2 => Self::Discharging,
            3 => Self::Low,
            4 => Self::Critical,
            5 => Self::Action,
            _ => Self::Unknown,
        }
    }
}

impl Display for WarningLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::None => write!(f, "None"),
            Self::Discharging => write!(f, "Discharging"),
            Self::Low => write!(f, "Low"),
            Self::Critical => write!(f, "Critical"),
            Self::Action => write!(f, "Action"),
        }
    }
}

/// Battery level for devices with coarse reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatteryLevel {
    /// Unknown battery level
    Unknown,
    /// Device doesn't use coarse level reporting
    None,
    /// Low battery
    Low,
    /// Critical battery
    Critical,
    /// Normal battery
    Normal,
    /// High battery
    High,
    /// Full battery
    Full,
}

impl From<u32> for BatteryLevel {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::None,
            3 => Self::Low,
            4 => Self::Critical,
            6 => Self::Normal,
            7 => Self::High,
            8 => Self::Full,
            _ => Self::Unknown,
        }
    }
}

impl Display for BatteryLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::None => write!(f, "None"),
            Self::Low => write!(f, "Low"),
            Self::Critical => write!(f, "Critical"),
            Self::Normal => write!(f, "Normal"),
            Self::High => write!(f, "High"),
            Self::Full => write!(f, "Full"),
        }
    }
}
