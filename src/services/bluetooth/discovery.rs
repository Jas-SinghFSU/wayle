use super::core::{Adapter, Device};

pub(crate) struct BluetoothDiscovery;

impl BluetoothDiscovery {
    pub(crate) async fn adapters() -> Vec<Adapter> {
        todo!()
    }

    pub(crate) async fn primary_adapter() -> Adapter {
        todo!()
    }

    pub(crate) async fn devices() -> Vec<Device> {
        todo!()
    }

    pub(crate) async fn available() -> bool {
        todo!()
    }

    pub(crate) async fn enabled() -> bool {
        todo!()
    }

    pub(crate) async fn connected() -> bool {
        todo!()
    }
}
