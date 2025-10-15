mod types;

use std::sync::Arc;

use types::{LiveMonitorParams, MonitorParams};
use wayle_common::Property;
use wayle_traits::Reactive;

use crate::{
    Address, DirectScanoutBlocker, Error, MonitorData, MonitorId, Reserved, SolitaryBlocker,
    TearingBlocker, Transform, WorkspaceInfo,
};

#[derive(Debug, Clone)]
pub struct Monitor {
    pub id: Property<MonitorId>,
    pub name: Property<String>,
    pub description: Property<String>,
    pub make: Property<String>,
    pub model: Property<String>,
    pub serial: Property<String>,
    pub width: Property<u32>,
    pub height: Property<u32>,
    pub physical_width: Property<u32>,
    pub physical_height: Property<u32>,
    pub refresh_rate: Property<f32>,
    pub x: Property<i32>,
    pub y: Property<i32>,
    pub active_workspace: Property<WorkspaceInfo>,
    pub special_workspace: Property<WorkspaceInfo>,
    pub reserved: Property<Reserved>,
    pub scale: Property<f32>,
    pub transform: Property<Transform>,
    pub focused: Property<bool>,
    pub dpms_status: Property<bool>,
    pub vrr: Property<bool>,
    pub solitary: Property<Option<Address>>,
    pub solitary_blocked_by: Property<Vec<SolitaryBlocker>>,
    pub actively_tearing: Property<bool>,
    pub tearing_blocked_by: Property<Vec<TearingBlocker>>,
    pub direct_scanout_to: Property<Option<Address>>,
    pub direct_scanout_blocked_by: Property<Vec<DirectScanoutBlocker>>,
    pub disabled: Property<bool>,
    pub current_format: Property<String>,
    pub mirror_of: Property<Option<String>>,
    pub available_modes: Property<Vec<String>>,
}

impl PartialEq for Monitor {
    fn eq(&self, other: &Self) -> bool {
        self.name.get() == other.name.get()
    }
}

impl Reactive for Monitor {
    type Error = Error;
    type Context<'a> = MonitorParams<'a>;
    type LiveContext<'a> = LiveMonitorParams<'a>;

    async fn get(context: Self::Context<'_>) -> Result<Self, Self::Error> {
        let monitor_data = context.hypr_messenger.monitor(context.name).await?;

        Ok(Self::from_props(monitor_data))
    }

    async fn get_live(context: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let monitor_data = context.hypr_messenger.monitor(context.name).await?;
        let arc_monitor_data = Arc::new(Self::from_props(monitor_data));

        // TODO: Add monitoring
        Ok(arc_monitor_data)
    }
}

impl Monitor {
    pub(crate) fn from_props(monitor_data: MonitorData) -> Self {
        Self {
            id: Property::new(monitor_data.id),
            name: Property::new(monitor_data.name),
            description: Property::new(monitor_data.description),
            make: Property::new(monitor_data.make),
            model: Property::new(monitor_data.model),
            serial: Property::new(monitor_data.serial),
            width: Property::new(monitor_data.width),
            height: Property::new(monitor_data.height),
            physical_width: Property::new(monitor_data.physical_width),
            physical_height: Property::new(monitor_data.physical_height),
            refresh_rate: Property::new(monitor_data.refresh_rate),
            x: Property::new(monitor_data.x),
            y: Property::new(monitor_data.y),
            active_workspace: Property::new(monitor_data.active_workspace),
            special_workspace: Property::new(monitor_data.special_workspace),
            reserved: Property::new(monitor_data.reserved),
            scale: Property::new(monitor_data.scale),
            transform: Property::new(monitor_data.transform),
            focused: Property::new(monitor_data.focused),
            dpms_status: Property::new(monitor_data.dpms_status),
            vrr: Property::new(monitor_data.vrr),
            solitary: Property::new(monitor_data.solitary),
            solitary_blocked_by: Property::new(monitor_data.solitary_blocked_by),
            actively_tearing: Property::new(monitor_data.actively_tearing),
            tearing_blocked_by: Property::new(monitor_data.tearing_blocked_by),
            direct_scanout_to: Property::new(monitor_data.direct_scanout_to),
            direct_scanout_blocked_by: Property::new(monitor_data.direct_scanout_blocked_by),
            disabled: Property::new(monitor_data.disabled),
            current_format: Property::new(monitor_data.current_format),
            mirror_of: Property::new(monitor_data.mirror_of),
            available_modes: Property::new(monitor_data.available_modes),
        }
    }
}
