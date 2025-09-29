//! NetworkManager state types.

/// NMState values indicate the current overall networking state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NMState {
    /// Networking state is unknown. Indicates a daemon error that makes it unable to
    /// reasonably assess the state. In such event the applications are expected to assume
    /// Internet connectivity might be present and not disable controls that require network
    /// access. The graphical shells may hide the network accessibility indicator altogether
    /// since no meaningful status indication can be provided.
    Unknown = 0,
    /// Networking is not enabled, the system is being suspended or resumed from suspend.
    Asleep = 10,
    /// No active network connection. The graphical shell should indicate no
    /// network connectivity and the applications should not attempt to access the network.
    Disconnected = 20,
    /// Network connections are being cleaned up. The applications should tear down their
    /// network sessions.
    Disconnecting = 30,
    /// A network connection is being started The graphical shell should indicate the
    /// network is being connected while the applications should still make no attempts to
    /// connect the network.
    Connecting = 40,
    /// Only local IPv4 and/or IPv6 connectivity, but no default route to access
    /// the Internet. The graphical shell should indicate no network connectivity.
    ConnectedLocal = 50,
    /// Site-wide IPv4 and/or IPv6 connectivity only. Default route
    /// is available, but the Internet connectivity check (see "Connectivity" property) did
    /// not succeed. The graphical shell should indicate limited network connectivity.
    ConnectedSite = 60,
    /// Global IPv4 and/or IPv6 Internet connectivity. Internet
    /// connectivity check succeeded, the graphical shell should indicate full network
    /// connectivity.
    ConnectedGlobal = 70,
}

impl NMState {
    /// Convert from D-Bus u32 representation
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            10 => Self::Asleep,
            20 => Self::Disconnected,
            30 => Self::Disconnecting,
            40 => Self::Connecting,
            50 => Self::ConnectedLocal,
            60 => Self::ConnectedSite,
            70 => Self::ConnectedGlobal,
            _ => Self::Unknown,
        }
    }
}

/// Device-specific states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NMDeviceState {
    /// the device's state is unknown
    Unknown = 0,
    /// the device is recognized, but not managed by NetworkManager
    Unmanaged = 10,
    /// the device is managed by NetworkManager, but is not available for use. Reasons may
    /// include the wireless switched off, missing firmware, no ethernet carrier, missing
    /// supplicant or modem manager, etc.
    Unavailable = 20,
    /// the device can be activated, but is currently idle and not connected to a network.
    Disconnected = 30,
    /// Device is preparing the connection to the network. May include operations
    /// like changing the MAC address, setting physical link properties, and anything else
    /// required to connect to the requested network.
    Prepare = 40,
    /// Device is connecting to the requested network. May include operations like
    /// associating with the Wi-Fi AP, dialing the modem, connecting to the remote
    /// Bluetooth device, etc.
    Config = 50,
    /// Device requires more information to continue connecting to the requested
    /// network. Includes secrets like WiFi passphrases, login passwords, PIN codes,
    /// etc.
    NeedAuth = 60,
    /// the device is requesting IPv4 and/or IPv6 addresses and routing information from
    /// the network.
    IpConfig = 70,
    /// Device is checking whether further action is required for the requested network
    /// connection. May include checking whether only local network access is
    /// available, whether a captive portal is blocking access to the Internet, etc.
    IpCheck = 80,
    /// the device is waiting for a secondary connection (like a VPN) which must activated
    /// before the device can be activated
    Secondaries = 90,
    /// the device has a network connection, either local or global.
    Activated = 100,
    /// a disconnection from the current network connection was requested, and the device
    /// is cleaning up resources used for that connection. The network connection may still
    /// be valid.
    Deactivating = 110,
    /// the device failed to connect to the requested network and is cleaning up the
    /// connection request
    Failed = 120,
}

impl NMDeviceState {
    /// Convert from D-Bus u32 representation
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            10 => Self::Unmanaged,
            20 => Self::Unavailable,
            30 => Self::Disconnected,
            40 => Self::Prepare,
            50 => Self::Config,
            60 => Self::NeedAuth,
            70 => Self::IpConfig,
            80 => Self::IpCheck,
            90 => Self::Secondaries,
            100 => Self::Activated,
            110 => Self::Deactivating,
            120 => Self::Failed,
            _ => Self::Unknown,
        }
    }
}

/// NMActiveConnectionState values indicate the state of a connection to a specific
/// network while it is starting, connected, or disconnecting from that network.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NMActiveConnectionState {
    /// the state of the connection is unknown
    Unknown = 0,
    /// a network connection is being prepared
    Activating = 1,
    /// there is a connection to the network
    Activated = 2,
    /// the network connection is being torn down and cleaned up
    Deactivating = 3,
    /// the network connection is disconnected and will be removed
    Deactivated = 4,
}

impl NMActiveConnectionState {
    /// Convert from D-Bus u32 representation
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Activating,
            2 => Self::Activated,
            3 => Self::Deactivating,
            4 => Self::Deactivated,
            _ => Self::Unknown,
        }
    }
}

/// VPN connection states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NMVpnConnectionState {
    /// The state of the VPN connection is unknown.
    Unknown = 0,
    /// The VPN connection is preparing to connect.
    Prepare = 1,
    /// The VPN connection needs authorization credentials.
    NeedAuth = 2,
    /// The VPN connection is being established.
    Connect = 3,
    /// The VPN connection is getting an IP address.
    IpConfigGet = 4,
    /// The VPN connection is active.
    Activated = 5,
    /// The VPN connection failed.
    Failed = 6,
    /// The VPN connection is disconnected.
    Disconnected = 7,
}

impl NMVpnConnectionState {
    /// Convert from D-Bus u32 representation
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Prepare,
            2 => Self::NeedAuth,
            3 => Self::Connect,
            4 => Self::IpConfigGet,
            5 => Self::Activated,
            6 => Self::Failed,
            7 => Self::Disconnected,
            _ => Self::Unknown,
        }
    }
}

/// Device state change reason codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NMDeviceStateReason {
    /// No reason given
    None = 0,
    /// Unknown error
    Unknown = 1,
    /// Device is now managed
    NowManaged = 2,
    /// Device is now unmanaged
    NowUnmanaged = 3,
    /// The device could not be readied for configuration
    ConfigFailed = 4,
    /// IP configuration could not be reserved (no available address, timeout, etc)
    IpConfigUnavailable = 5,
    /// The IP config is no longer valid
    IpConfigExpired = 6,
    /// Secrets were required, but not provided
    NoSecrets = 7,
    /// 802.1x supplicant disconnected
    SupplicantDisconnect = 8,
    /// 802.1x supplicant configuration failed
    SupplicantConfigFailed = 9,
    /// 802.1x supplicant failed
    SupplicantFailed = 10,
    /// 802.1x supplicant took too long to authenticate
    SupplicantTimeout = 11,
    /// PPP service failed to start
    PppStartFailed = 12,
    /// PPP service disconnected
    PppDisconnect = 13,
    /// PPP failed
    PppFailed = 14,
    /// DHCP client failed to start
    DhcpStartFailed = 15,
    /// DHCP client error
    DhcpError = 16,
    /// DHCP client failed
    DhcpFailed = 17,
    /// Shared connection service failed to start
    SharedStartFailed = 18,
    /// Shared connection service failed
    SharedFailed = 19,
    /// AutoIP service failed to start
    AutoIpStartFailed = 20,
    /// AutoIP service error
    AutoIpError = 21,
    /// AutoIP service failed
    AutoIpFailed = 22,
    /// The line is busy
    ModemBusy = 23,
    /// No dial tone
    ModemNoDialTone = 24,
    /// No carrier could be established
    ModemNoCarrier = 25,
    /// The dialing request timed out
    ModemDialTimeout = 26,
    /// The dialing attempt failed
    ModemDialFailed = 27,
    /// Modem initialization failed
    ModemInitFailed = 28,
    /// Failed to select the specified APN
    GsmApnFailed = 29,
    /// Not searching for networks
    GsmRegistrationNotSearching = 30,
    /// Network registration denied
    GsmRegistrationDenied = 31,
    /// Network registration timed out
    GsmRegistrationTimeout = 32,
    /// Failed to register with the requested network
    GsmRegistrationFailed = 33,
    /// PIN check failed
    GsmPinCheckFailed = 34,
    /// Necessary firmware for the device may be missing
    FirmwareMissing = 35,
    /// The device was removed
    Removed = 36,
    /// NetworkManager went to sleep
    Sleeping = 37,
    /// The device's active connection disappeared
    ConnectionRemoved = 38,
    /// Device disconnected by user or client
    UserRequested = 39,
    /// Carrier/link changed
    Carrier = 40,
    /// The device's existing connection was assumed
    ConnectionAssumed = 41,
    /// The supplicant is now available
    SupplicantAvailable = 42,
    /// The modem could not be found
    ModemNotFound = 43,
    /// The Bluetooth connection failed or timed out
    BtFailed = 44,
    /// GSM Modem's SIM Card not inserted
    GsmSimNotInserted = 45,
    /// GSM Modem's SIM Pin required
    GsmSimPinRequired = 46,
    /// GSM Modem's SIM Puk required
    GsmSimPukRequired = 47,
    /// GSM Modem's SIM wrong
    GsmSimWrong = 48,
    /// InfiniBand device does not support connected mode
    InfinibandMode = 49,
    /// A dependency of the connection failed
    DependencyFailed = 50,
    /// Problem with the RFC 2684 Ethernet over ADSL bridge
    Br2684Failed = 51,
    /// ModemManager not running
    ModemManagerUnavailable = 52,
    /// The Wi-Fi network could not be found
    SsidNotFound = 53,
    /// A secondary connection of the base connection failed
    SecondaryConnectionFailed = 54,
    /// DCB or FCoE setup failed
    DcbFcoeFailed = 55,
    /// teamd control failed
    TeamdControlFailed = 56,
    /// Modem failed or no longer available
    ModemFailed = 57,
    /// Modem now ready and available
    ModemAvailable = 58,
    /// SIM PIN was incorrect
    SimPinIncorrect = 59,
    /// New connection activation was enqueued
    NewActivation = 60,
    /// the device's parent changed
    ParentChanged = 61,
    /// the device parent's management changed
    ParentManagedChanged = 62,
    /// problem communicating with Open vSwitch database
    OvsdbFailed = 63,
    /// a duplicate IP address was detected
    IpAddressDuplicate = 64,
    /// The selected IP method is not supported
    IpMethodUnsupported = 65,
    /// configuration of SR-IOV parameters failed
    SriovConfigurationFailed = 66,
    /// The Wi-Fi P2P peer could not be found
    PeerNotFound = 67,
}

impl NMDeviceStateReason {
    /// Convert from D-Bus u32 representation
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Unknown,
            2 => Self::NowManaged,
            3 => Self::NowUnmanaged,
            4 => Self::ConfigFailed,
            5 => Self::IpConfigUnavailable,
            6 => Self::IpConfigExpired,
            7 => Self::NoSecrets,
            8 => Self::SupplicantDisconnect,
            9 => Self::SupplicantConfigFailed,
            10 => Self::SupplicantFailed,
            11 => Self::SupplicantTimeout,
            12 => Self::PppStartFailed,
            13 => Self::PppDisconnect,
            14 => Self::PppFailed,
            15 => Self::DhcpStartFailed,
            16 => Self::DhcpError,
            17 => Self::DhcpFailed,
            18 => Self::SharedStartFailed,
            19 => Self::SharedFailed,
            20 => Self::AutoIpStartFailed,
            21 => Self::AutoIpError,
            22 => Self::AutoIpFailed,
            23 => Self::ModemBusy,
            24 => Self::ModemNoDialTone,
            25 => Self::ModemNoCarrier,
            26 => Self::ModemDialTimeout,
            27 => Self::ModemDialFailed,
            28 => Self::ModemInitFailed,
            29 => Self::GsmApnFailed,
            30 => Self::GsmRegistrationNotSearching,
            31 => Self::GsmRegistrationDenied,
            32 => Self::GsmRegistrationTimeout,
            33 => Self::GsmRegistrationFailed,
            34 => Self::GsmPinCheckFailed,
            35 => Self::FirmwareMissing,
            36 => Self::Removed,
            37 => Self::Sleeping,
            38 => Self::ConnectionRemoved,
            39 => Self::UserRequested,
            40 => Self::Carrier,
            41 => Self::ConnectionAssumed,
            42 => Self::SupplicantAvailable,
            43 => Self::ModemAvailable,
            44 => Self::ModemFailed,
            45 => Self::ModemAvailable,
            46 => Self::SimPinIncorrect,
            47 => Self::NewActivation,
            48 => Self::ParentChanged,
            49 => Self::ParentManagedChanged,
            50 => Self::OvsdbFailed,
            51 => Self::IpAddressDuplicate,
            52 => Self::IpMethodUnsupported,
            53 => Self::SriovConfigurationFailed,
            54 => Self::PeerNotFound,
            _ => Self::Unknown,
        }
    }
}

/// Active connection state reasons.
///
/// Since: 1.8
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NMActiveConnectionStateReason {
    /// The reason for the active connection state change is unknown.
    Unknown = 0,
    /// No reason was given for the active connection state change.
    None = 1,
    /// The active connection changed state because the user disconnected it.
    UserDisconnected = 2,
    /// The active connection changed state because the device it was using was
    /// disconnected.
    DeviceDisconnected = 3,
    /// The service providing the VPN connection was stopped.
    ServiceStopped = 4,
    /// The IP config of the active connection was invalid.
    IpConfigInvalid = 5,
    /// The connection attempt to the VPN service timed out.
    ConnectTimeout = 6,
    /// A timeout occurred while starting the service providing the VPN connection.
    ServiceStartTimeout = 7,
    /// Starting the service providing the VPN connection failed.
    ServiceStartFailed = 8,
    /// Necessary secrets for the connection were not provided.
    NoSecrets = 9,
    /// Authentication to the server failed.
    LoginFailed = 10,
    /// The connection was deleted from settings.
    ConnectionRemoved = 11,
    /// Master connection of this connection failed to activate.
    DependencyFailed = 12,
    /// Could not create the software device link.
    DeviceRealizeFailed = 13,
    /// The device this connection depended on disappeared.
    DeviceRemoved = 14,
}

impl NMActiveConnectionStateReason {
    /// Convert from D-Bus u32 representation
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::None,
            2 => Self::UserDisconnected,
            3 => Self::DeviceDisconnected,
            4 => Self::ServiceStopped,
            5 => Self::IpConfigInvalid,
            6 => Self::ConnectTimeout,
            7 => Self::ServiceStartTimeout,
            8 => Self::ServiceStartFailed,
            9 => Self::NoSecrets,
            10 => Self::LoginFailed,
            11 => Self::ConnectionRemoved,
            12 => Self::DependencyFailed,
            13 => Self::DeviceRealizeFailed,
            14 => Self::DeviceRemoved,
            _ => Self::Unknown,
        }
    }
}

/// VPN state change reasons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NMVpnConnectionStateReason {
    /// The reason for the VPN connection state change is unknown.
    Unknown = 0,
    /// No reason was given for the VPN connection state change.
    None = 1,
    /// The VPN connection changed state because the user disconnected it.
    UserDisconnected = 2,
    /// The VPN connection changed state because the device it was using was disconnected.
    DeviceDisconnected = 3,
    /// The service providing the VPN connection was stopped.
    ServiceStopped = 4,
    /// The IP config of the VPN connection was invalid.
    IpConfigInvalid = 5,
    /// The connection attempt to the VPN service timed out.
    ConnectTimeout = 6,
    /// A timeout occurred while starting the service providing the VPN connection.
    ServiceStartTimeout = 7,
    /// Starting the service providing the VPN connection failed.
    ServiceStartFailed = 8,
    /// Necessary secrets for the VPN connection were not provided.
    NoSecrets = 9,
    /// Authentication to the VPN server failed.
    LoginFailed = 10,
    /// The VPN connection was deleted from settings.
    ConnectionRemoved = 11,
}

impl NMVpnConnectionStateReason {
    /// Convert from D-Bus u32 representation
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::None,
            2 => Self::UserDisconnected,
            3 => Self::DeviceDisconnected,
            4 => Self::ServiceStopped,
            5 => Self::IpConfigInvalid,
            6 => Self::ConnectTimeout,
            7 => Self::ServiceStartTimeout,
            8 => Self::ServiceStartFailed,
            9 => Self::NoSecrets,
            10 => Self::LoginFailed,
            11 => Self::ConnectionRemoved,
            _ => Self::Unknown,
        }
    }
}

/// The result of a checkpoint Rollback() operation for a specific device.
///
/// Since: 1.4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NMRollbackResult {
    /// the rollback succeeded.
    Ok = 0,
    /// the device no longer exists.
    ErrNoDevice = 1,
    /// the device is now unmanaged.
    ErrDeviceUnmanaged = 2,
    /// other errors during rollback.
    ErrFailed = 3,
}

impl NMRollbackResult {
    /// Convert from D-Bus u32 representation
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => Self::Ok,
            1 => Self::ErrNoDevice,
            2 => Self::ErrDeviceUnmanaged,
            3 => Self::ErrFailed,
            _ => Self::ErrFailed,
        }
    }
}

/// Current network connectivity status.
///
/// Simplified view of network state that combines multiple NetworkManager
/// states into broader categories.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkStatus {
    /// Full network connectivity with internet access
    Connected,

    /// Establishing network connection
    /// (obtaining IP, authenticating, etc.)
    Connecting,

    /// No network connection
    Disconnected,
}

impl NetworkStatus {
    /// Derive network status from device state.
    ///
    /// Maps the detailed NetworkManager device states to simplified
    /// connectivity status for UI display.
    pub fn from_device_state(state: NMDeviceState) -> Self {
        match state {
            NMDeviceState::Activated => Self::Connected,
            NMDeviceState::Prepare
            | NMDeviceState::Config
            | NMDeviceState::NeedAuth
            | NMDeviceState::IpConfig
            | NMDeviceState::IpCheck
            | NMDeviceState::Secondaries => Self::Connecting,
            _ => Self::Disconnected,
        }
    }
}
