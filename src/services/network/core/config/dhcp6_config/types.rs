use zbus::{Connection, zvariant::OwnedObjectPath};

pub(crate) struct Dhcp6ConfigParams<'a> {
    pub connection: &'a Connection,
    pub path: OwnedObjectPath,
}
