use std::sync::Arc;

use wayle_network::NetworkService;
use wayle_sysinfo::SysinfoService;

pub(crate) struct NetworkSectionInit {
    pub network: Option<Arc<NetworkService>>,
    pub sysinfo: Arc<SysinfoService>,
}

#[derive(Debug)]
pub(crate) enum NetworkSectionInput {
    SetActive(bool),
}

#[derive(Debug)]
pub(crate) enum NetworkSectionCmd {
    ConnectionChanged {
        connected: bool,
    },
    SpeedChanged {
        upload: String,
        upload_is_megabytes: bool,
        download: String,
        download_is_megabytes: bool,
    },
}
