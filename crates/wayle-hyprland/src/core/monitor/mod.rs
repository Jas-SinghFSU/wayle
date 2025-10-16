mod monitoring;
pub(crate) mod types;

use std::sync::Arc;

use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;
use tracing::instrument;
use types::{LiveMonitorParams, MonitorParams};
use wayle_common::Property;
use wayle_traits::{ModelMonitoring, Reactive};

use crate::{
    Address, DirectScanoutBlocker, Error, MonitorData, MonitorId, Reserved, SolitaryBlocker,
    TearingBlocker, Transform, WorkspaceInfo,
    ipc::{HyprMessenger, events::types::ServiceNotification},
};

#[derive(Debug, Clone)]
pub struct Monitor {
    pub(crate) hypr_messenger: HyprMessenger,
    pub(crate) internal_tx: Option<Sender<ServiceNotification>>,
    pub(crate) cancellation_token: Option<CancellationToken>,

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

    #[instrument(skip(context), fields(name = %context.name), err)]
    async fn get(context: Self::Context<'_>) -> Result<Self, Self::Error> {
        let monitor_data = context.hypr_messenger.monitor(&context.name).await?;

        Ok(Self::from_props(
            monitor_data,
            context.hypr_messenger,
            None,
            None,
        ))
    }

    #[instrument(skip(context), fields(name = %context.name), err)]
    async fn get_live(context: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let monitor_data = context.hypr_messenger.monitor(&context.name).await?;
        let arc_monitor = Arc::new(Self::from_props(
            monitor_data,
            context.hypr_messenger,
            Some(context.internal_tx.clone()),
            Some(context.cancellation_token.child_token()),
        ));

        arc_monitor.clone().start_monitoring().await?;

        Ok(arc_monitor)
    }
}

impl Monitor {
    pub(crate) fn from_props(
        monitor_data: MonitorData,
        hypr_messenger: &HyprMessenger,
        internal_tx: Option<Sender<ServiceNotification>>,
        cancellation_token: Option<CancellationToken>,
    ) -> Self {
        Self {
            hypr_messenger: hypr_messenger.clone(),
            internal_tx,
            cancellation_token,
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

    pub(crate) fn update(&self, monitor_data: MonitorData) {
        self.id.set(monitor_data.id);
        self.name.set(monitor_data.name);
        self.description.set(monitor_data.description);
        self.make.set(monitor_data.make);
        self.model.set(monitor_data.model);
        self.serial.set(monitor_data.serial);
        self.width.set(monitor_data.width);
        self.height.set(monitor_data.height);
        self.physical_width.set(monitor_data.physical_width);
        self.physical_height.set(monitor_data.physical_height);
        self.refresh_rate.set(monitor_data.refresh_rate);
        self.x.set(monitor_data.x);
        self.y.set(monitor_data.y);
        self.active_workspace.set(monitor_data.active_workspace);
        self.special_workspace.set(monitor_data.special_workspace);
        self.reserved.set(monitor_data.reserved);
        self.scale.set(monitor_data.scale);
        self.transform.set(monitor_data.transform);
        self.focused.set(monitor_data.focused);
        self.dpms_status.set(monitor_data.dpms_status);
        self.vrr.set(monitor_data.vrr);
        self.solitary.set(monitor_data.solitary);
        self.solitary_blocked_by
            .set(monitor_data.solitary_blocked_by);
        self.actively_tearing.set(monitor_data.actively_tearing);
        self.tearing_blocked_by.set(monitor_data.tearing_blocked_by);
        self.direct_scanout_to.set(monitor_data.direct_scanout_to);
        self.direct_scanout_blocked_by
            .set(monitor_data.direct_scanout_blocked_by);
        self.disabled.set(monitor_data.disabled);
        self.current_format.set(monitor_data.current_format);
        self.mirror_of.set(monitor_data.mirror_of);
        self.available_modes.set(monitor_data.available_modes);
    }
}
