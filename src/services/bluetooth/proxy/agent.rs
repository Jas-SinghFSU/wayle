use zbus::proxy;

/// Proxy for the org.bluez.Agent1 interface.
///
/// BlueZ D-Bus Agent API that must be implemented by applications that want to handle
/// pairing and authorization requests.
///
/// Service: `unique name`
/// Interface: `org.bluez.Agent1`
/// Object path: `freely definable`
#[proxy(interface = "org.bluez.Agent1")]
pub trait Agent1 {
    /// This method gets called when bluetoothd unregisters the agent.
    ///
    /// An agent can use it to do cleanup tasks. There is no need to unregister the
    /// agent, because when this method gets called it has already been unregistered.
    async fn release(&self) -> zbus::Result<()>;

    /// This method gets called when bluetoothd needs to get the passkey for an
    /// authentication.
    ///
    /// The return value should be a string of 1-16 characters length. The string can be
    /// alphanumeric.
    ///
    /// # Errors
    ///
    /// - `Rejected` - Request rejected by user
    /// - `Canceled` - Request canceled
    async fn request_pin_code(
        &self,
        device: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<String>;

    /// This method gets called when bluetoothd needs to display a pincode for
    /// an authentication.
    ///
    /// An empty reply should be returned. When the pincode needs no longer to be
    /// displayed, the Cancel method of the agent will be called.
    ///
    /// This is used during the pairing process of keyboards that don't support
    /// Bluetooth 2.1 Secure Simple Pairing, in contrast to DisplayPasskey which is used
    /// for those that do.
    ///
    /// This method will only ever be called once since older keyboards do not support
    /// typing notification.
    ///
    /// Note that the PIN will always be a 6-digit number, zero-padded to 6 digits. This
    /// is for harmony with the later specification.
    ///
    /// # Errors
    ///
    /// - `Rejected` - Request rejected
    /// - `Canceled` - Request canceled
    async fn display_pin_code(
        &self,
        device: &zbus::zvariant::ObjectPath<'_>,
        pincode: &str,
    ) -> zbus::Result<()>;

    /// This method gets called when bluetoothd needs to get the passkey for an
    /// authentication.
    ///
    /// The return value should be a numeric value between 0-999999.
    ///
    /// # Errors
    ///
    /// - `Rejected` - Request rejected by user
    /// - `Canceled` - Request canceled
    async fn request_passkey(&self, device: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<u32>;

    /// This method gets called when bluetoothd needs to display a passkey for
    /// an authentication.
    ///
    /// The entered parameter indicates the number of already typed keys on the remote
    /// side.
    ///
    /// An empty reply should be returned. When the passkey needs no longer to be
    /// displayed, the Cancel method of the agent will be called.
    ///
    /// During the pairing process this method might be called multiple times to update
    /// the entered value.
    ///
    /// Note that the passkey will always be a 6-digit number, so the display should be
    /// zero-padded at the start if the value contains less than 6 digits.
    async fn display_passkey(
        &self,
        device: &zbus::zvariant::ObjectPath<'_>,
        passkey: u32,
        entered: u16,
    ) -> zbus::Result<()>;

    /// This method gets called when bluetoothd needs to confirm a passkey for
    /// an authentication.
    ///
    /// To confirm the value it should return an empty reply or an error in case the
    /// passkey is invalid.
    ///
    /// Note that the passkey will always be a 6-digit number, so the display should be
    /// zero-padded at the start if the value contains less than 6 digits.
    ///
    /// # Errors
    ///
    /// - `Rejected` - Passkey rejected
    /// - `Canceled` - Request canceled
    async fn request_confirmation(
        &self,
        device: &zbus::zvariant::ObjectPath<'_>,
        passkey: u32,
    ) -> zbus::Result<()>;

    /// This method gets called to request the user to authorize an incoming pairing
    /// attempt which would in other circumstances trigger the just-works model, or when
    /// the user plugged in a device that implements cable pairing. In the latter case,
    /// the device would not be connected to the adapter via Bluetooth yet.
    ///
    /// # Errors
    ///
    /// - `Rejected` - Authorization rejected
    /// - `Canceled` - Request canceled
    async fn request_authorization(
        &self,
        device: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// This method gets called when bluetoothd needs to authorize a
    /// connection/service request.
    ///
    /// # Errors
    ///
    /// - `Rejected` - Service authorization rejected
    /// - `Canceled` - Request canceled
    async fn authorize_service(
        &self,
        device: &zbus::zvariant::ObjectPath<'_>,
        uuid: &str,
    ) -> zbus::Result<()>;

    /// This method gets called to indicate that the agent request failed before a reply
    /// was returned.
    async fn cancel(&self) -> zbus::Result<()>;
}
