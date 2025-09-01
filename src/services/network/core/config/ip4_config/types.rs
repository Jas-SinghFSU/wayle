use zbus::{Connection, zvariant::OwnedObjectPath};

pub(crate) struct Ip4ConfigParams<'a> {
    pub connection: &'a Connection,
    pub path: OwnedObjectPath,
}
