use std::sync::Arc;

use derive_more::Debug;
use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

use self::types::{AccessPointParams, Bssid, LiveAccessPointParams, SecurityType, Ssid};
use crate::{
    services::{
        common::Property,
        network::{
            error::Error,
            proxy::access_point::AccessPointProxy,
            types::{
                flags::{NM80211ApFlags, NM80211ApSecurityFlags},
                wifi::NM80211Mode,
            },
        },
        traits::{ModelMonitoring, Reactive},
    },
    unwrap_i32_or, unwrap_string, unwrap_u8, unwrap_u32, unwrap_vec,
};

/// Access point monitoring implementation
pub(crate) mod monitoring;
/// Access point type definitions
pub(crate) mod types;

/// WiFi access point representation.
///
/// Provides information about a detected WiFi access point including its
/// security configuration, signal strength, frequency, and identification.
/// Access points are discovered and monitored through the WiFi device interface.
#[derive(Debug, Clone)]
pub struct AccessPoint {
    #[debug(skip)]
    pub(crate) connection: Connection,
    #[debug(skip)]
    pub(crate) object_path: OwnedObjectPath,
    #[debug(skip)]
    pub(crate) cancellation_token: Option<CancellationToken>,
    /// Flags describing the capabilities of the access point. See NM80211ApFlags.
    pub flags: Property<NM80211ApFlags>,

    /// Flags describing the access point's capabilities according to WPA (Wifi Protected Access).
    /// See NM80211ApSecurityFlags.
    pub wpa_flags: Property<NM80211ApSecurityFlags>,

    /// Flags describing the access point's capabilities according to the
    /// RSN (Robust Secure Network) protocol. See NM80211ApSecurityFlags.
    pub rsn_flags: Property<NM80211ApSecurityFlags>,

    /// The Service Set Identifier identifying the access point.
    /// The Ssid is a binary array to support non-UTF-8 Ssids.
    pub ssid: Property<Ssid>,

    /// The radio channel frequency in use by the access point, in MHz.
    pub frequency: Property<u32>,

    /// The hardware address (Bssid) of the access point.
    pub bssid: Property<Bssid>,

    /// Describes the operating mode of the access point.
    pub mode: Property<NM80211Mode>,

    /// The maximum bitrate this access point is capable of, in kilobits/second (Kb/s).
    pub max_bitrate: Property<u32>,

    /// The current signal quality of the access point, in percent.
    pub strength: Property<u8>,

    /// The timestamp (in CLOCK_BOOTTIME seconds) for the last time the access point
    /// was found in scan results. A value of -1 means the access point has never
    /// been found in scan results.
    pub last_seen: Property<i32>,

    /// Simplified security type derived from flags.
    ///
    /// Provides a user-friendly classification of the AP's security.
    pub security: Property<SecurityType>,

    /// Whether this is a hidden network (non-broadcasting Ssid).
    pub is_hidden: Property<bool>,
}

impl Reactive for AccessPoint {
    type Context<'a> = AccessPointParams<'a>;
    type LiveContext<'a> = LiveAccessPointParams<'a>;
    type Error = Error;

    async fn get(params: Self::Context<'_>) -> Result<Self, Self::Error> {
        let ap = Self::from_path(params.connection, params.path.clone(), None)
            .await
            .map_err(|e| match e {
                Error::ObjectNotFound(_) => e,
                _ => Error::ObjectCreationFailed {
                    object_type: String::from("AccessPoint"),
                    object_path: params.path.clone(),
                    reason: e.to_string(),
                },
            })?;

        Ok(ap)
    }

    async fn get_live(params: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let access_point = Self::from_path(
            params.connection,
            params.path.clone(),
            Some(params.cancellation_token.child_token()),
        )
        .await
        .map_err(|e| match e {
            Error::ObjectNotFound(_) => e,
            _ => Error::ObjectCreationFailed {
                object_type: String::from("AccessPoint"),
                object_path: params.path.clone(),
                reason: e.to_string(),
            },
        })?;
        let access_point = Arc::new(access_point);
        access_point.clone().start_monitoring().await?;

        Ok(access_point)
    }
}

impl PartialEq for AccessPoint {
    fn eq(&self, other: &Self) -> bool {
        self.bssid.get() == other.bssid.get()
    }
}

impl AccessPoint {
    async fn from_path(
        connection: &Connection,
        path: OwnedObjectPath,
        cancellation_token: Option<CancellationToken>,
    ) -> Result<Self, Error> {
        let ap_proxy = AccessPointProxy::new(connection, &path)
            .await
            .map_err(Error::DbusError)?;

        if ap_proxy.strength().await.is_err() {
            return Err(Error::ObjectNotFound(path.clone()));
        }

        let (
            flags,
            wpa_flags,
            rsn_flags,
            ssid,
            frequency,
            hw_address,
            mode,
            max_bitrate,
            strength,
            last_seen,
        ) = tokio::join!(
            ap_proxy.flags(),
            ap_proxy.wpa_flags(),
            ap_proxy.rsn_flags(),
            ap_proxy.ssid(),
            ap_proxy.frequency(),
            ap_proxy.hw_address(),
            ap_proxy.mode(),
            ap_proxy.max_bitrate(),
            ap_proxy.strength(),
            ap_proxy.last_seen(),
        );

        let flags = NM80211ApFlags::from_bits_truncate(unwrap_u32!(flags, path));
        let wpa_flags = NM80211ApSecurityFlags::from_bits_truncate(unwrap_u32!(wpa_flags, path));
        let rsn_flags = NM80211ApSecurityFlags::from_bits_truncate(unwrap_u32!(rsn_flags, path));
        let ssid = Ssid::new(unwrap_vec!(ssid, path));
        let frequency = unwrap_u32!(frequency, path);
        let hw_address = Bssid::new(unwrap_string!(hw_address, path).into_bytes());
        let mode = NM80211Mode::from_u32(unwrap_u32!(mode, path));
        let max_bitrate = unwrap_u32!(max_bitrate, path);
        let strength = unwrap_u8!(strength, path);
        let last_seen = unwrap_i32_or!(last_seen, path, -1);

        let security = SecurityType::from_flags(flags, wpa_flags, rsn_flags);
        let is_hidden = ssid.is_empty();

        Ok(Self {
            connection: connection.clone(),
            object_path: path.clone(),
            cancellation_token,
            flags: Property::new(flags),
            wpa_flags: Property::new(wpa_flags),
            rsn_flags: Property::new(rsn_flags),
            ssid: Property::new(ssid),
            frequency: Property::new(frequency),
            bssid: Property::new(hw_address),
            mode: Property::new(mode),
            max_bitrate: Property::new(max_bitrate),
            strength: Property::new(strength),
            last_seen: Property::new(last_seen),
            security: Property::new(security),
            is_hidden: Property::new(is_hidden),
        })
    }
}
