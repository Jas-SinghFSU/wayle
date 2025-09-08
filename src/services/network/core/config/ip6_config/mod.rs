mod types;

use std::{collections::HashMap, net::Ipv6Addr};

use tracing::debug;
pub(crate) use types::Ip6ConfigParams;
use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::{
    services::{
        common::Property,
        network::{error::Error, proxy::ip6_config::IP6ConfigProxy},
        traits::Static,
    },
    unwrap_i32, unwrap_string, unwrap_vec,
};

/// IPv6 Configuration Set.
///
/// Represents the current IPv6 configuration including addresses, routes,
/// DNS servers, and other network parameters.
#[derive(Debug, Clone)]
pub struct Ip6Config {
    /// D-Bus object path for this IP6Config
    pub object_path: Property<OwnedObjectPath>,

    /// Array of IP address data objects.
    pub address_data: Property<Vec<Ipv6Address>>,

    /// The gateway in use.
    pub gateway: Property<Option<Ipv6Addr>>,

    /// The nameservers in use.
    pub nameservers: Property<Vec<Ipv6Addr>>,

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
    pub route_data: Property<Vec<Ipv6Route>>,
}

/// IPv6 address with prefix length
#[derive(Debug, Clone, PartialEq)]
pub struct Ipv6Address {
    /// The IPv6 address.
    pub address: Ipv6Addr,
    /// Network prefix length in bits (0-128).
    pub prefix: u8,
}

/// IPv6 route entry
#[derive(Debug, Clone, PartialEq)]
pub struct Ipv6Route {
    /// Destination network address.
    pub destination: Ipv6Addr,
    /// Network prefix length in bits (0-128).
    pub prefix: u8,
    /// Gateway address for this route, if any.
    pub next_hop: Option<Ipv6Addr>,
    /// Route metric for priority ordering (lower is higher priority).
    pub metric: Option<u32>,
}

struct Ip6ConfigProperties {
    address_data: Vec<Ipv6Address>,
    gateway: Option<Ipv6Addr>,
    nameservers: Vec<Ipv6Addr>,
    domains: Vec<String>,
    searches: Vec<String>,
    dns_options: Vec<String>,
    dns_priority: i32,
    route_data: Vec<Ipv6Route>,
}

impl Static for Ip6Config {
    type Error = Error;
    type Context<'a> = Ip6ConfigParams<'a>;

    async fn get(params: Self::Context<'_>) -> Result<Self, Self::Error> {
        Self::from_path(params.connection, params.path).await
    }
}

impl Ip6Config {
    async fn from_path(connection: &Connection, path: OwnedObjectPath) -> Result<Self, Error> {
        let properties = Self::fetch_properties(connection, &path).await?;
        Ok(Self::from_props(path, properties))
    }

    async fn fetch_properties(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<Ip6ConfigProperties, Error> {
        let proxy = IP6ConfigProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        let (
            address_data,
            gateway,
            nameserver_data,
            domains,
            searches,
            dns_options,
            dns_priority,
            route_data,
        ) = tokio::join!(
            proxy.address_data(),
            proxy.gateway(),
            proxy.nameserver_data(),
            proxy.domains(),
            proxy.searches(),
            proxy.dns_options(),
            proxy.dns_priority(),
            proxy.route_data(),
        );

        Ok(Ip6ConfigProperties {
            address_data: Self::parse_address_data(unwrap_vec!(address_data, path)),
            gateway: Self::parse_gateway(unwrap_string!(gateway, path)),
            nameservers: Self::parse_nameserver_data(unwrap_vec!(nameserver_data, path)),
            domains: unwrap_vec!(domains, path),
            searches: unwrap_vec!(searches, path),
            dns_options: unwrap_vec!(dns_options, path),
            dns_priority: unwrap_i32!(dns_priority, path),
            route_data: Self::parse_route_data(unwrap_vec!(route_data, path)),
        })
    }

    fn from_props(path: OwnedObjectPath, props: Ip6ConfigProperties) -> Self {
        Self {
            object_path: Property::new(path),
            address_data: Property::new(props.address_data),
            gateway: Property::new(props.gateway),
            nameservers: Property::new(props.nameservers),
            domains: Property::new(props.domains),
            searches: Property::new(props.searches),
            dns_options: Property::new(props.dns_options),
            dns_priority: Property::new(props.dns_priority),
            route_data: Property::new(props.route_data),
        }
    }

    fn parse_address_data(data: Vec<HashMap<String, OwnedValue>>) -> Vec<Ipv6Address> {
        data.into_iter()
            .filter_map(|entry| {
                let address_value = entry.get("address")?;
                let address_str = address_value.downcast_ref::<String>().ok()?;
                let address = address_str.parse::<Ipv6Addr>().ok()?;

                let prefix_value = entry.get("prefix")?;
                let prefix_ref = prefix_value.downcast_ref::<&u32>().ok()?;
                let prefix = *prefix_ref as u8;

                Some(Ipv6Address { address, prefix })
            })
            .collect()
    }

    fn parse_gateway(gateway: String) -> Option<Ipv6Addr> {
        match gateway.parse::<Ipv6Addr>() {
            Ok(addr) if addr.is_unspecified() => None,
            Ok(addr) => Some(addr),
            Err(_) => None,
        }
    }

    fn parse_nameserver_data(data: Vec<HashMap<String, OwnedValue>>) -> Vec<Ipv6Addr> {
        data.into_iter()
            .filter_map(|entry| {
                let address_value = entry.get("address")?;
                let address_str = address_value.downcast_ref::<String>().ok()?;
                address_str.parse::<Ipv6Addr>().ok()
            })
            .collect()
    }

    fn parse_route_data(data: Vec<HashMap<String, OwnedValue>>) -> Vec<Ipv6Route> {
        data.into_iter()
            .filter_map(|entry| {
                let dest_value = entry.get("dest")?;
                let dest_str = dest_value.downcast_ref::<String>().ok()?;
                let destination = match dest_str.parse::<Ipv6Addr>() {
                    Ok(addr) => addr,
                    Err(_) => {
                        debug!("Skipping route with invalid destination IPv6: {}", dest_str);
                        return None;
                    }
                };

                let prefix_value = entry.get("prefix")?;
                let prefix_ref = prefix_value.downcast_ref::<&u32>().ok()?;
                let prefix = *prefix_ref as u8;

                let next_hop = entry
                    .get("next-hop")
                    .and_then(|value| value.downcast_ref::<String>().ok())
                    .and_then(|addr_str| addr_str.parse::<Ipv6Addr>().ok())
                    .filter(|addr| !addr.is_unspecified());

                let metric = entry
                    .get("metric")
                    .and_then(|value| value.downcast_ref::<&u32>().ok())
                    .copied();

                Some(Ipv6Route {
                    destination,
                    prefix,
                    next_hop,
                    metric,
                })
            })
            .collect()
    }
}
