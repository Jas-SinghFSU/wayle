use crate::{Address, LayerLevel, ProcessId};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LayerData {
    pub address: Address,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub namespace: String,
    pub monitor: String,
    pub level: LayerLevel,
    pub pid: ProcessId,
}
