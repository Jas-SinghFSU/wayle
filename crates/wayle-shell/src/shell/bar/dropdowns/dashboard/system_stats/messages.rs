use std::sync::Arc;

use wayle_sysinfo::SysinfoService;

pub(crate) struct SystemStatsInit {
    pub sysinfo: Arc<SysinfoService>,
}

#[derive(Debug)]
pub(crate) enum SystemStatsInput {
    SetActive(bool),
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum SystemStatsCmd {
    CpuChanged { usage: f32, temp: Option<f32> },
    MemoryChanged { usage: f32 },
    DiskChanged { usage: f32 },
}
