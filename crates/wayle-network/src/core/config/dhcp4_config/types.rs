use zbus::{Connection, zvariant::OwnedObjectPath};

#[doc(hidden)]
pub struct Dhcp4ConfigParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) path: OwnedObjectPath,
}
