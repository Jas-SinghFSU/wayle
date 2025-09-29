//! NetworkManager flag types.

use bitflags::bitflags;

bitflags! {
    /// General device capability flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NMDeviceCapabilities: u32 {
        /// device has no special capabilities
        const NONE = 0x00000000;
        /// NetworkManager supports this device
        const NM_SUPPORTED = 0x00000001;
        /// this device can indicate carrier status
        const CARRIER_DETECT = 0x00000002;
        /// this device is a software device
        const IS_SOFTWARE = 0x00000004;
        /// this device supports single-root I/O virtualization
        const SRIOV = 0x00000008;
    }

    /// 802.11 access point flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NM80211ApFlags: u32 {
        /// access point has no special capabilities
        const NONE = 0x00000000;
        /// access point requires authentication and encryption (usually means WEP)
        const PRIVACY = 0x00000001;
        /// access point supports some WPS method
        const WPS = 0x00000002;
        /// access point supports push-button WPS
        const WPS_PBC = 0x00000004;
        /// access point supports PIN-based WPS
        const WPS_PIN = 0x00000008;
    }

    /// 802.11 access point security and authentication flags. These flags describe the
    /// current security requirements of an access point as determined from the access
    /// point's beacon.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NM80211ApSecurityFlags: u32 {
        /// the access point has no special security requirements
        const NONE = 0x00000000;
        /// 40/64-bit WEP is supported for pairwise/unicast encryption
        const PAIR_WEP40 = 0x00000001;
        /// 104/128-bit WEP is supported for pairwise/unicast encryption
        const PAIR_WEP104 = 0x00000002;
        /// TKIP is supported for pairwise/unicast encryption
        const PAIR_TKIP = 0x00000004;
        /// AES/CCMP is supported for pairwise/unicast encryption
        const PAIR_CCMP = 0x00000008;
        /// 40/64-bit WEP is supported for group/broadcast encryption
        const GROUP_WEP40 = 0x00000010;
        /// 104/128-bit WEP is supported for group/broadcast encryption
        const GROUP_WEP104 = 0x00000020;
        /// TKIP is supported for group/broadcast encryption
        const GROUP_TKIP = 0x00000040;
        /// AES/CCMP is supported for group/broadcast encryption
        const GROUP_CCMP = 0x00000080;
        /// WPA/RSN Pre-Shared Key encryption is supported
        const KEY_MGMT_PSK = 0x00000100;
        /// 802.1x authentication and key management is supported
        const KEY_MGMT_802_1X = 0x00000200;
        /// WPA/RSN Simultaneous Authentication of Equals is supported
        const KEY_MGMT_SAE = 0x00000400;
        /// WPA/RSN Opportunistic Wireless Encryption is supported
        const KEY_MGMT_OWE = 0x00000800;
        /// WPA/RSN Opportunistic Wireless Encryption transition mode is supported.
        /// Since: 1.26.
        const KEY_MGMT_OWE_TM = 0x00001000;
        /// WPA3 Enterprise Suite-B 192 bit mode is supported. Since: 1.30.
        const KEY_MGMT_EAP_SUITE_B_192 = 0x00002000;
    }

    /// 802.11 specific device encryption and authentication capabilities.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NMDeviceWifiCapabilities: u32 {
        /// device has no encryption/authentication capabilities
        const NONE = 0x00000000;
        /// device supports 40/64-bit WEP encryption
        const CIPHER_WEP40 = 0x00000001;
        /// device supports 104/128-bit WEP encryption
        const CIPHER_WEP104 = 0x00000002;
        /// device supports TKIP encryption
        const CIPHER_TKIP = 0x00000004;
        /// device supports AES/CCMP encryption
        const CIPHER_CCMP = 0x00000008;
        /// device supports WPA1 authentication
        const WPA = 0x00000010;
        /// device supports WPA2/RSN authentication
        const RSN = 0x00000020;
        /// device supports Access Point mode
        const AP = 0x00000040;
        /// device supports Ad-Hoc mode
        const ADHOC = 0x00000080;
        /// device reports frequency capabilities
        const FREQ_VALID = 0x00000100;
        /// device supports 2.4GHz frequencies
        const FREQ_2GHZ = 0x00000200;
        /// device supports 5GHz frequencies
        const FREQ_5GHZ = 0x00000400;
        /// device supports acting as a mesh point. Since: 1.20.
        const MESH = 0x00001000;
        /// device supports WPA2/RSN in an IBSS network. Since: 1.22.
        const IBSS_RSN = 0x00002000;
    }

    /// NMBluetoothCapabilities values indicate the usable capabilities of a Bluetooth device.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NMBluetoothCapabilities: u32 {
        /// device has no usable capabilities
        const NONE = 0x00000000;
        /// device provides Dial-Up Networking capability
        const DUN = 0x00000001;
        /// device provides Network Access Point capability
        const NAP = 0x00000002;
    }

    /// NMDeviceModemCapabilities values indicate the generic radio access technology
    /// families a modem device supports. For more information on the specific access
    /// technologies the device supports use the ModemManager D-Bus API.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NMDeviceModemCapabilities: u32 {
        /// modem has no usable capabilities
        const NONE = 0x00000000;
        /// modem uses the analog wired telephone network and is not a wireless/cellular
        /// device
        const POTS = 0x00000001;
        /// modem supports at least one of CDMA 1xRTT, EVDO revision 0, EVDO revision A,
        /// or EVDO revision B
        const CDMA_EVDO = 0x00000002;
        /// modem supports at least one of GSM, GPRS, EDGE, UMTS, HSDPA, HSUPA, or HSPA+
        /// packet switched data capability
        const GSM_UMTS = 0x00000004;
        /// modem has LTE data capability
        const LTE = 0x00000008;
    }

    /// NMSecretAgentCapabilities indicate various capabilities of the agent.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NMSecretAgentCapabilities: u32 {
        /// the agent supports no special capabilities
        const NONE = 0x00000000;
        /// the agent supports passing hints to VPN plugin authentication dialogs.
        const VPN_HINTS = 0x00000001;
    }

    /// NMSecretAgentGetSecretsFlags values modify the behavior of a GetSecrets request.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NMSecretAgentGetSecretsFlags: u32 {
        /// no special behavior; by default no user interaction is allowed and requests
        /// for secrets are fulfilled from persistent storage, or if no secrets are
        /// available an error is returned.
        const NONE = 0x00000000;
        /// allows the request to interact with the user, possibly prompting via UI for
        /// secrets if any are required, or if none are found in persistent storage.
        const ALLOW_INTERACTION = 0x00000001;
        /// explicitly prompt for new secrets from the user. This flag signals that
        /// NetworkManager thinks any existing secrets are invalid or wrong. This flag
        /// implies that interaction is allowed.
        const REQUEST_NEW = 0x00000002;
        /// set if the request was initiated by user-requested action via the D-Bus
        /// interface, as opposed to automatically initiated by NetworkManager in response
        /// to (for example) scan results or carrier changes.
        const USER_REQUESTED = 0x00000004;
        /// indicates that WPS enrollment is active with PBC method. The agent may suggest
        /// that the user pushes a button on the router instead of supplying a PSK.
        const WPS_PBC_ACTIVE = 0x00000008;
        /// Internal flag, not part of the D-Bus API.
        const ONLY_SYSTEM = 0x80000000;
        /// Internal flag, not part of the D-Bus API.
        const NO_ERRORS = 0x40000000;
    }

    /// Numeric flags for the "flags" argument of AddConnection2() D-Bus API.
    ///
    /// Since: 1.20
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NMSettingsAddConnection2Flags: u32 {
        /// an alias for numeric zero, no flags set.
        const NONE = 0x00000000;
        /// to persist the connection to disk.
        const TO_DISK = 0x00000001;
        /// to make the connection in-memory only.
        const IN_MEMORY = 0x00000002;
        /// usually, when the connection has autoconnect enabled and gets added, it
        /// becomes eligible to autoconnect right away. Setting this flag, disables
        /// autoconnect until the connection is manually activated.
        const BLOCK_AUTOCONNECT = 0x00000020;
    }

    /// Since: 1.12
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NMSettingsUpdate2Flags: u32 {
        /// an alias for numeric zero, no flags set.
        const NONE = 0x00000000;
        /// to persist the connection to disk.
        const TO_DISK = 0x00000001;
        /// makes the profile in-memory. Note that such profiles are stored in keyfile
        /// format under /run. If the file is already in-memory, the file in /run is
        /// updated in-place. Otherwise, the previous storage for the profile is left
        /// unchanged on disk, and the in-memory copy shadows it. Note that the original
        /// filename of the previous persistent storage (if any) is remembered. That means,
        /// when later persisting the profile again to disk, the file on disk will be
        /// overwritten again. Likewise, when finally deleting the profile, both the
        /// storage from /run and persistent storage are deleted (or if the persistent
        /// storage does not allow deletion, and nmmeta file is written to mark the UUID
        /// as deleted).
        const IN_MEMORY = 0x00000002;
        /// this is almost the same as %NM_SETTINGS_UPDATE2_FLAG_IN_MEMORY, with one
        /// difference: when later deleting the profile, the original profile will not be
        /// deleted. Instead a nmmeta file is written to /run to indicate that the profile
        /// is gone. Note that if such a nmmeta tombstone file exists and hides a file in
        /// persistent storage, then when re-adding the profile with the same UUID, then
        /// the original storage is taken over again.
        const IN_MEMORY_DETACHED = 0x00000004;
        /// this is like %NM_SETTINGS_UPDATE2_FLAG_IN_MEMORY, but if the connection has a
        /// corresponding file on persistent storage, the file will be deleted right away.
        /// If the profile is later again persisted to disk, a new, unused filename will
        /// be chosen.
        const IN_MEMORY_ONLY = 0x00000008;
        /// This can be specified with either %NM_SETTINGS_UPDATE2_FLAG_IN_MEMORY,
        /// %NM_SETTINGS_UPDATE2_FLAG_IN_MEMORY_DETACHED or
        /// %NM_SETTINGS_UPDATE2_FLAG_IN_MEMORY_ONLY. After making the connection
        /// in-memory only, the connection is marked as volatile. That means, if the
        /// connection is currently not active it will be deleted right away. Otherwise,
        /// it is marked to for deletion once the connection deactivates. A volatile
        /// connection cannot autoactivate again (because it's about to be deleted), but
        /// a manual activation will clear the volatile flag.
        const VOLATILE = 0x00000010;
        /// usually, when the connection has autoconnect enabled and is modified, it
        /// becomes eligible to autoconnect right away. Setting this flag, disables
        /// autoconnect until the connection is manually activated.
        const BLOCK_AUTOCONNECT = 0x00000020;
        /// when a profile gets modified that is currently active, then these changes
        /// don't take effect for the active device unless the profile gets reactivated
        /// or the configuration reapplied. There are two exceptions: by default
        /// "connection.zone" and "connection.metered" properties take effect immediately.
        /// Specify this flag to prevent these properties to take effect, so that the
        /// change is restricted to modify the profile. Since: 1.20.
        const NO_REAPPLY = 0x00000040;
    }

    /// The flags for CheckpointCreate call
    ///
    /// Since: 1.4 (gi flags generated since 1.12)
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NMCheckpointCreateFlags: u32 {
        /// no flags
        const NONE = 0x00000000;
        /// when creating a new checkpoint, destroy all existing ones.
        const DESTROY_ALL = 0x00000001;
        /// upon rollback, delete any new connection added after the checkpoint.
        /// Since: 1.6.
        const DELETE_NEW_CONNECTIONS = 0x00000002;
        /// upon rollback, disconnect any new device appeared after the checkpoint.
        /// Since: 1.6.
        const DISCONNECT_NEW_DEVICES = 0x00000004;
        /// by default, creating a checkpoint fails if there are already existing
        /// checkoints that reference the same devices. With this flag, creation of such
        /// checkpoints is allowed, however, if an older checkpoint that references
        /// overlapping devices gets rolled back, it will automatically destroy this
        /// checkpoint during rollback. This allows to create several overlapping
        /// checkpoints in parallel, and rollback to them at will. With the special case
        /// that rolling back to an older checkpoint will invalidate all overlapping
        /// younger checkpoints. This opts-in that the checkpoint can be automatically
        /// destroyed by the rollback of an older checkpoint. Since: 1.12.
        const ALLOW_OVERLAPPING = 0x00000008;
    }

    /// Flags describing the current activation state.
    ///
    /// Since: 1.12
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NMConnectionSettingsFlags: u32 {
        /// an alias for numeric zero, no flags set.
        const NONE = 0x00000000;
        /// the connection is not saved to disk. That either means, that the connection
        /// is in-memory only and currently is not backed by a file. Or, that the
        /// connection is backed by a file, but has modifications in-memory that were not
        /// persisted to disk.
        const UNSAVED = 0x00000001;
        /// A connection is "nm-generated" if it was generated by NetworkManger. If the
        /// connection gets modified or saved by the user, the flag gets cleared. A
        /// nm-generated is also unsaved and has no backing file as it is in-memory only.
        const NM_GENERATED = 0x00000002;
        /// The connection will be deleted when it disconnects. That is for in-memory
        /// connections (unsaved), which are currently active but deleted on disconnect.
        /// Volatile connections are always unsaved, but they are also no backing file on
        /// disk and are entirely in-memory only.
        const VOLATILE = 0x00000004;
        /// the profile was generated to represent an external configuration of a
        /// networking device. Since: 1.26.
        const EXTERNAL = 0x00000008;
    }

    /// Flags for a network interface.
    ///
    /// Since: 1.22
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NMDeviceInterfaceFlags: u32 {
        /// an alias for numeric zero, no flags set.
        const NONE = 0x00000000;
        /// the interface is enabled from the administrative point of view. Corresponds
        /// to kernel IFF_UP.
        const UP = 0x00000001;
        /// the physical link is up. Corresponds to kernel IFF_LOWER_UP.
        const LOWER_UP = 0x00000002;
        /// receive all packets. Corresponds to kernel IFF_PROMISC. Since: 1.32.
        const PROMISC = 0x00000004;
        /// the interface has carrier. In most cases this is equal to the value of
        /// @NM_DEVICE_INTERFACE_FLAG_LOWER_UP. However some devices have a non-standard
        /// carrier detection mechanism.
        const CARRIER = 0x00010000;
        /// the flag to indicate device LLDP status. Since: 1.32.
        const LLDP_CLIENT_ENABLED = 0x00020000;
    }

    /// Flags describing the current activation state.
    ///
    /// Since: 1.10
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NMActivationStateFlags: u32 {
        /// an alias for numeric zero, no flags set.
        const NONE = 0x00000000;
        /// the device is a master.
        const IS_MASTER = 0x00000001;
        /// the device is a slave.
        const IS_SLAVE = 0x00000002;
        /// layer2 is activated and ready.
        const LAYER2_READY = 0x00000004;
        /// IPv4 setting is completed.
        const IP4_READY = 0x00000008;
        /// IPv6 setting is completed.
        const IP6_READY = 0x00000010;
        /// The master has any slave devices attached. This only makes sense if the device
        /// is a master.
        const MASTER_HAS_SLAVES = 0x00000020;
        /// the lifetime of the activation is bound to the visibility of the connection
        /// profile, which in turn depends on "connection.permissions" and whether a
        /// session for the user exists. Since: 1.16.
        const LIFETIME_BOUND_TO_PROFILE_VISIBILITY = 0x00000040;
        /// the active connection was generated to represent an external configuration of
        /// a networking device. Since: 1.26.
        const EXTERNAL = 0x00000080;
    }

    /// Flags for the manager Reload() call.
    ///
    /// Since: 1.22
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NMManagerReloadFlags: u32 {
        /// an alias for numeric zero, no flags set. This reloads everything that is
        /// supported and is identical to a SIGHUP.
        const NONE = 0x00000000;
        /// reload the NetworkManager.conf configuration from disk. Note that this does
        /// not include connections, which can be reloaded via Setting's
        /// ReloadConnections().
        const CONF = 0x00000001;
        /// update DNS configuration, which usually involves writing /etc/resolv.conf anew.
        const DNS_RC = 0x00000002;
        /// means to restart the DNS plugin. This is for example useful when using dnsmasq
        /// plugin, which uses additional configuration in /etc/NetworkManager/dnsmasq.d.
        /// If you edit those files, you can restart the DNS plugin. This action shortly
        /// interrupts name resolution.
        const DNS_FULL = 0x00000004;
        /// all flags.
        const ALL = 0x00000007;
    }
}
