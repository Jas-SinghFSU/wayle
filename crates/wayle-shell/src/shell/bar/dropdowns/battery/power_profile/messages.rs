use std::sync::Arc;

use wayle_power_profiles::{PowerProfilesService, types::profile::PowerProfile};

pub(crate) struct PowerProfileInit {
    pub power_profiles: Option<Arc<PowerProfilesService>>,
}

#[derive(Debug)]
pub(crate) enum PowerProfileInput {
    ProfileSelected(PowerProfile),
}

#[derive(Debug)]
pub(crate) enum PowerProfileCmd {
    ProfileChanged(PowerProfile),
    AvailableProfilesChanged(Vec<PowerProfile>),
}
