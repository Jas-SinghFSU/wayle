mod cpu;
mod disk;
mod memory;
mod network;

pub use cpu::{CoreData, CpuData};
pub use disk::DiskData;
pub use memory::MemoryData;
pub use network::NetworkData;
