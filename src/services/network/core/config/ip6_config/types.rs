use zbus::{Connection, zvariant::OwnedObjectPath};

pub(crate) struct Ip6ConfigParams<'a> {
    pub connection: &'a Connection,
    pub path: OwnedObjectPath,
}
