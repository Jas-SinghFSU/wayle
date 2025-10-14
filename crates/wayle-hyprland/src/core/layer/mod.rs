use wayle_common::Property;

use crate::{Address, LayerLevel, ProcessId};

pub struct Layer {
    pub address: Property<Address>,
    pub x: Property<i32>,
    pub y: Property<i32>,
    pub width: Property<u32>,
    pub height: Property<u32>,
    pub namespace: Property<String>,
    pub monitor: Property<String>,
    pub level: Property<LayerLevel>,
    pub pid: Property<ProcessId>,
}
