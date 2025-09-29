use zbus::{Connection, zvariant::OwnedObjectPath};

/// Parameters for creating a Dhcp4Config instance.
///
/// **Note**: This type is exposed for trait implementation requirements
/// but should not be constructed directly by external consumers.
#[doc(hidden)]
pub struct Dhcp4ConfigParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) path: OwnedObjectPath,
}
