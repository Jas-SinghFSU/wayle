use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::sync::Arc;
use tracing::debug;
use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::services::{
    common::Property,
    network_manager::{NetworkError, proxy::ip4_config::IP4ConfigProxy},
};

/// IPv4 configuration for a network device.
///
/// Represents the current IPv4 configuration including addresses, routes,
/// DNS servers, and other network parameters.
#[derive(Debug, Clone)]
pub struct Ip4Config {
    /// D-Bus object path for this IP4Config
    pub path: Property<OwnedObjectPath>,

    /// Array of IP address data objects.
    pub address_data: Property<Vec<Ipv4Address>>,

    /// The gateway in use.
    pub gateway: Property<Option<Ipv4Addr>>,

    /// Array of nameserver data objects.
    pub nameserver_data: Property<Vec<Ipv4Addr>>,

    /// A list of domains this address belongs to.
    pub domains: Property<Vec<String>>,

    /// A list of dns searches.
    pub searches: Property<Vec<String>>,

    /// A list of DNS options that modify the behavior of the DNS resolver.
    /// See resolv.conf(5) manual page for the list of supported options.
    pub dns_options: Property<Vec<String>>,

    /// The relative priority of DNS servers.
    pub dns_priority: Property<i32>,

    /// Array of IP route data objects.
    pub route_data: Property<Vec<Ipv4Route>>,

    /// The Windows Internet Name Service servers associated with the connection.
    pub wins_server_data: Property<Vec<Ipv4Addr>>,
}

/// IPv4 address with prefix length
#[derive(Debug, Clone, PartialEq)]
pub struct Ipv4Address {
    /// The IPv4 address.
    pub address: Ipv4Addr,
    /// Network prefix length in bits (0-32).
    pub prefix: u8,
}

/// IPv4 route entry
#[derive(Debug, Clone, PartialEq)]
pub struct Ipv4Route {
    /// Destination network address.
    pub destination: Ipv4Addr,
    /// Network prefix length in bits (0-32).
    pub prefix: u8,
    /// Gateway address for this route, if any.
    pub next_hop: Option<Ipv4Addr>,
    /// Route metric for priority ordering (lower is higher priority).
    pub metric: Option<u32>,
}

struct Ip4ConfigProperties {
    address_data: Vec<Ipv4Address>,
    gateway: Option<Ipv4Addr>,
    nameserver_data: Vec<Ipv4Addr>,
    domains: Vec<String>,
    searches: Vec<String>,
    dns_options: Vec<String>,
    dns_priority: i32,
    route_data: Vec<Ipv4Route>,
    wins_server_data: Vec<Ipv4Addr>,
}

impl Ip4Config {
    /// Get a snapshot of the current IPv4 configuration state (no monitoring).
    pub async fn get(connection: &Connection, path: OwnedObjectPath) -> Option<Arc<Self>> {
        let config = Self::from_path(connection, path).await?;
        Some(Arc::new(config))
    }

    async fn from_path(connection: &Connection, path: OwnedObjectPath) -> Option<Self> {
        let properties = Self::fetch_properties(connection, &path).await.ok()?;
        Some(Self::from_props(path, properties))
    }

    async fn fetch_properties(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<Ip4ConfigProperties, NetworkError> {
        let proxy = IP4ConfigProxy::new(connection, path.clone())
            .await
            .map_err(NetworkError::DbusError)?;

        let (
            address_data,
            gateway,
            nameserver_data,
            domains,
            searches,
            dns_options,
            dns_priority,
            route_data,
            wins_server_data,
        ) = tokio::join!(
            proxy.address_data(),
            proxy.gateway(),
            proxy.nameserver_data(),
            proxy.domains(),
            proxy.searches(),
            proxy.dns_options(),
            proxy.dns_priority(),
            proxy.route_data(),
            proxy.wins_server_data(),
        );

        Ok(Ip4ConfigProperties {
            address_data: Self::parse_address_data(address_data?),
            gateway: Self::parse_gateway(gateway?),
            nameserver_data: Self::parse_nameserver_data(nameserver_data?),
            domains: domains?,
            searches: searches?,
            dns_options: dns_options?,
            dns_priority: dns_priority?,
            route_data: Self::parse_route_data(route_data?),
            wins_server_data: wins_server_data?
                .into_iter()
                .filter_map(|s| s.parse().ok())
                .collect(),
        })
    }

    fn from_props(path: OwnedObjectPath, props: Ip4ConfigProperties) -> Self {
        Self {
            path: Property::new(path),
            address_data: Property::new(props.address_data),
            gateway: Property::new(props.gateway),
            nameserver_data: Property::new(props.nameserver_data),
            domains: Property::new(props.domains),
            searches: Property::new(props.searches),
            dns_options: Property::new(props.dns_options),
            dns_priority: Property::new(props.dns_priority),
            route_data: Property::new(props.route_data),
            wins_server_data: Property::new(props.wins_server_data),
        }
    }

    fn parse_address_data(data: Vec<HashMap<String, OwnedValue>>) -> Vec<Ipv4Address> {
        data.into_iter()
            .filter_map(|entry| {
                let address_value = entry.get("address")?;
                let address_str = address_value.downcast_ref::<String>().ok()?;
                let address = address_str.parse::<Ipv4Addr>().ok()?;

                let prefix_value = entry.get("prefix")?;
                let prefix_ref = prefix_value.downcast_ref::<&u8>().ok()?;

                Some(Ipv4Address {
                    address,
                    prefix: *prefix_ref,
                })
            })
            .collect()
    }

    fn parse_gateway(gateway: String) -> Option<Ipv4Addr> {
        match gateway.parse::<Ipv4Addr>() {
            Ok(addr) if addr.is_unspecified() => None,
            Ok(addr) => Some(addr),
            Err(_) => None,
        }
    }

    fn parse_nameserver_data(data: Vec<HashMap<String, OwnedValue>>) -> Vec<Ipv4Addr> {
        data.into_iter()
            .filter_map(|entry| {
                let address_value = entry.get("address")?;
                let address_str = address_value.downcast_ref::<String>().ok()?;
                address_str.parse::<Ipv4Addr>().ok()
            })
            .collect()
    }

    fn parse_route_data(data: Vec<HashMap<String, OwnedValue>>) -> Vec<Ipv4Route> {
        data.into_iter()
            .filter_map(|entry| {
                let dest_value = entry.get("dest")?;
                let dest_str = dest_value.downcast_ref::<String>().ok()?;
                let destination = match dest_str.parse::<Ipv4Addr>() {
                    Ok(addr) => addr,
                    Err(_) => {
                        debug!("Skipping route with invalid destination IP: {}", dest_str);
                        return None;
                    }
                };

                let prefix_value = entry.get("prefix")?;
                let prefix_ref = prefix_value.downcast_ref::<&u8>().ok()?;

                let next_hop = entry
                    .get("next-hop")
                    .and_then(|value| value.downcast_ref::<String>().ok())
                    .and_then(|addr_str| addr_str.parse::<Ipv4Addr>().ok())
                    .filter(|addr| !addr.is_unspecified());

                let metric = entry
                    .get("metric")
                    .and_then(|value| value.downcast_ref::<&u32>().ok())
                    .copied();

                Some(Ipv4Route {
                    destination,
                    prefix: *prefix_ref,
                    next_hop,
                    metric,
                })
            })
            .collect()
    }
}
