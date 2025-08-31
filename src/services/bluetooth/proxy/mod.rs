/// BlueZ Adapter1 D-Bus proxy.
pub mod adapter;
/// BlueZ Agent1 D-Bus proxy.
pub mod agent;
/// BlueZ AgentManager1 D-Bus proxy.
pub mod agent_manager;
/// BlueZ Battery1 D-Bus proxy.
pub mod battery;
/// BlueZ Device1 D-Bus proxy.
pub mod device;

pub use adapter::Adapter1Proxy;
pub use agent_manager::AgentManager1Proxy;
pub use battery::Battery1Proxy;
pub use device::Device1Proxy;
