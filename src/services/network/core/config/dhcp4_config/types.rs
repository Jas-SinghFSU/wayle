use zbus::{Connection, zvariant::OwnedObjectPath};

pub(crate) struct Dhcp4ConfigParams<'a> {
    pub connection: &'a Connection,
    pub path: OwnedObjectPath,
}
