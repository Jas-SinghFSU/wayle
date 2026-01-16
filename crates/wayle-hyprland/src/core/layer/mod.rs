pub(crate) mod types;

use tracing::instrument;
use types::LayerParams;
use wayle_common::Property;
use wayle_traits::Static;

use crate::{Address, Error, LayerData, LayerLevel, ProcessId};

/// A layer shell surface (panel, overlay, wallpaper, etc.).
#[derive(Debug, Clone)]
pub struct Layer {
    /// Surface address.
    pub address: Property<Address>,
    /// X position.
    pub x: Property<i32>,
    /// Y position.
    pub y: Property<i32>,
    /// Width in pixels.
    pub width: Property<u32>,
    /// Height in pixels.
    pub height: Property<u32>,
    /// Layer namespace (app identifier).
    pub namespace: Property<String>,
    /// Monitor name.
    pub monitor: Property<String>,
    /// Stack level (background, bottom, top, overlay).
    pub level: Property<LayerLevel>,
    /// Process ID.
    pub pid: Property<ProcessId>,
}

impl PartialEq for Layer {
    fn eq(&self, other: &Self) -> bool {
        self.address.get() == other.address.get()
    }
}

impl Static for Layer {
    type Error = Error;
    type Context<'a> = LayerParams<'a>;

    #[instrument(skip(context), fields(address = %context.address), err)]
    async fn get(context: Self::Context<'_>) -> Result<Self, Self::Error> {
        let layer_data = context.hypr_messenger.layer(context.address).await?;

        Ok(Self::from_props(layer_data))
    }
}

impl Layer {
    pub(crate) fn from_props(layer_data: LayerData) -> Self {
        Self {
            address: Property::new(layer_data.address),
            x: Property::new(layer_data.x),
            y: Property::new(layer_data.y),
            width: Property::new(layer_data.width),
            height: Property::new(layer_data.height),
            namespace: Property::new(layer_data.namespace),
            monitor: Property::new(layer_data.monitor),
            level: Property::new(layer_data.level),
            pid: Property::new(layer_data.pid),
        }
    }
}
