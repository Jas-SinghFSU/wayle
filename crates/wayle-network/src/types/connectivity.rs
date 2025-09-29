//! NetworkManager connectivity types.

/// Internet connectivity state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NMConnectivityState {
    /// Network connectivity is unknown.
    ///
    /// This means the connectivity checks are disabled (e.g. on server installations)
    /// or has not run yet.
    ///
    /// The graphical shell should assume the Internet connection might be
    /// available and not present a captive portal window.
    Unknown = 0,
    /// The host is not connected to any network.
    ///
    /// There's no active connection that contains a default route to the
    /// internet and thus it makes no sense to even attempt a connectivity check.
    ///
    /// The graphical shell should use this state to indicate the network
    /// connection is unavailable.
    None = 1,
    /// The Internet connection is hijacked by a captive portal gateway.
    ///
    /// The graphical shell may open a sandboxed web browser window (because the captive
    /// portals typically attempt a man-in-the-middle attacks against the https connections)
    /// for the purpose of authenticating to a gateway and retrigger the connectivity
    /// check with CheckConnectivity() when the browser window is dismissed.
    Portal = 2,
    /// The host is connected to a network, does not appear to be able to reach
    /// the full Internet, but a captive portal has not been detected.
    Limited = 3,
    /// The host is connected to a network, and appears to be able to reach the
    /// full Internet.
    Full = 4,
}

impl NMConnectivityState {
    /// Convert from D-Bus u32 representation
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::None,
            2 => Self::Portal,
            3 => Self::Limited,
            4 => Self::Full,
            _ => Self::Unknown,
        }
    }
}

/// The NMMetered enum has two different purposes: one is to configure
/// "connection.metered" setting of a connection profile in NMSettingConnection,
/// and the other is to express the actual metered state of the NMDevice at a
/// given moment.
///
/// For the connection profile only NM_METERED_UNKNOWN, NM_METERED_NO and
/// NM_METERED_YES are allowed.
///
/// The device's metered state at runtime is determined by the profile which is
/// currently active. If the profile explicitly specifies NM_METERED_NO or
/// NM_METERED_YES, then the device's metered state is as such. If the connection
/// profile leaves it undecided at NM_METERED_UNKNOWN (the default), then
/// NetworkManager tries to guess the metered state, for example based on the
/// device type or on DHCP options (like Android devices exposing a
/// "ANDROID_METERED" DHCP vendor option). This then leads to either
/// NM_METERED_GUESS_NO or NM_METERED_GUESS_YES.
///
/// Most applications probably should treat the runtime state NM_METERED_GUESS_YES
/// like NM_METERED_YES, and all other states as not metered.
///
/// Note that the per-device metered states are then combined to a global metered
/// state. Basically the metered state of the device with the best default
/// route. However, that generalization of a global metered state may not be
/// correct if the default routes for IPv4 and IPv6 are on different devices, or
/// if policy routing is configured. In general, the global metered state tries to
/// express whether the traffic is likely metered, but since that depends on the
/// traffic itself, there is not one answer in all cases. Hence, an application
/// may want to consider the per-device's metered states.
///
/// Since: 1.2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NMMetered {
    /// The metered status is unknown
    Unknown = 0,
    /// Metered, the value was explicitly configured
    Yes = 1,
    /// Not metered, the value was explicitly configured
    No = 2,
    /// Metered, the value was guessed
    GuessYes = 3,
    /// Not metered, the value was guessed
    GuessNo = 4,
}

impl NMMetered {
    /// Convert from D-Bus u32 representation
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Yes,
            2 => Self::No,
            3 => Self::GuessYes,
            4 => Self::GuessNo,
            _ => Self::Unknown,
        }
    }
}

/// Type of network connection currently active.
///
/// Tracks which interface type is providing the primary
/// network connectivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    /// Connection type cannot be determined or no connection is active.
    Unknown,
    /// Primary connectivity is through an ethernet/wired interface.
    Wired,
    /// Primary connectivity is through a WiFi interface.
    Wifi,
}
