use std::collections::HashMap;

use super::HyprMessenger;
use crate::{
    Address, BindData, ClientData, CursorPosition, DeviceInfo, Error, LayerData, LayerLevel,
    MonitorData, MonitorLayers, Result, WorkspaceData, WorkspaceId,
};

impl HyprMessenger {
    pub(crate) async fn monitor(&self, name: String) -> Result<MonitorData> {
        let monitors = self.monitors().await?;
        monitors
            .into_iter()
            .find(|monitor| monitor.name == name)
            .ok_or(Error::MonitorNotFound(name))
    }

    pub(crate) async fn monitors(&self) -> Result<Vec<MonitorData>> {
        let response = self.send("monitors -j").await?;
        serde_json::from_str(&response).map_err(Error::JsonParseError)
    }

    pub(crate) async fn workspace(&self, id: WorkspaceId) -> Result<WorkspaceData> {
        let workspaces = self.workspaces().await?;
        workspaces
            .into_iter()
            .find(|workspace| workspace.id == id)
            .ok_or(Error::WorkspaceNotFound(id))
    }

    pub(crate) async fn workspaces(&self) -> Result<Vec<WorkspaceData>> {
        let response = self.send("workspaces -j").await?;
        serde_json::from_str(&response).map_err(Error::JsonParseError)
    }

    pub(crate) async fn client(&self, address: Address) -> Result<ClientData> {
        let clients = self.clients().await?;
        clients
            .into_iter()
            .find(|client| client.address == address)
            .ok_or(Error::ClientNotFound(address))
    }

    pub(crate) async fn clients(&self) -> Result<Vec<ClientData>> {
        let response = self.send("clients -j").await?;
        serde_json::from_str(&response).map_err(Error::JsonParseError)
    }

    pub(crate) async fn layer(&self, address: Address) -> Result<LayerData> {
        let layers = self.layers().await?;
        layers
            .into_iter()
            .find(|layer| layer.address == address)
            .ok_or(Error::LayerNotFound(address))
    }

    pub(crate) async fn layers(&self) -> Result<Vec<LayerData>> {
        let response = self.send("layers -j").await?;
        let monitors: HashMap<String, MonitorLayers> =
            serde_json::from_str(&response).map_err(Error::JsonParseError)?;

        let layers = monitors
            .into_iter()
            .flat_map(|(monitor_name, monitor_layers)| {
                monitor_layers
                    .levels
                    .into_iter()
                    .flat_map(move |(layer_depth, layer_list)| {
                        let monitor_name = monitor_name.clone();

                        layer_list.into_iter().map(move |layer| LayerData {
                            address: layer.address,
                            x: layer.x,
                            y: layer.y,
                            width: layer.w,
                            height: layer.h,
                            namespace: layer.namespace,
                            monitor: monitor_name.clone(),
                            level: LayerLevel::from(layer_depth.as_str()),
                            pid: layer.pid,
                        })
                    })
            })
            .collect();

        Ok(layers)
    }

    pub(crate) async fn get_prop(&self, window: &str, property: &str) -> Result<String> {
        self.send(&format!("getprop {window} {property}")).await
    }

    pub(crate) async fn active_workspace(&self) -> Result<WorkspaceData> {
        let response = self.send("activeworkspace -j").await?;
        serde_json::from_str(&response).map_err(Error::JsonParseError)
    }

    pub(crate) async fn active_window(&self) -> Result<ClientData> {
        let response = self.send("activewindow -j").await?;
        serde_json::from_str(&response).map_err(Error::JsonParseError)
    }

    pub(crate) async fn version(&self) -> Result<String> {
        self.send("version").await
    }

    pub(crate) async fn cursor_pos(&self) -> Result<CursorPosition> {
        let response = self.send("cursorpos").await?;
        let parts: Vec<&str> = response.trim().split(", ").collect();

        if parts.len() != 2 {
            return Err(Error::EventParseError {
                event_data: response,
                reason: "Expected format 'x, y'".to_string(),
            });
        }

        let x = parts[0]
            .parse::<i32>()
            .map_err(|_| Error::EventParseError {
                event_data: parts[0].to_string(),
                reason: "Invalid x coordinate".to_string(),
            })?;

        let y = parts[1]
            .parse::<i32>()
            .map_err(|_| Error::EventParseError {
                event_data: parts[1].to_string(),
                reason: "Invalid y coordinate".to_string(),
            })?;

        Ok(CursorPosition { x, y })
    }

    pub(crate) async fn binds(&self) -> Result<Vec<BindData>> {
        let response = self.send("binds -j").await?;
        serde_json::from_str(&response).map_err(Error::JsonParseError)
    }

    pub(crate) async fn devices(&self) -> Result<DeviceInfo> {
        let response = self.send("devices -j").await?;
        serde_json::from_str(&response).map_err(Error::JsonParseError)
    }

    pub(crate) async fn layouts(&self) -> Result<Vec<String>> {
        let response = self.send("layouts -j").await?;
        serde_json::from_str(&response).map_err(Error::JsonParseError)
    }

    pub(crate) async fn submap(&self) -> Result<String> {
        let response = self.send("submap").await?;
        Ok(response.trim().to_string())
    }
}
