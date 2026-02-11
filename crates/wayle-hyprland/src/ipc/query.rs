use std::collections::HashMap;

use serde::de::DeserializeOwned;
use tracing::{instrument, warn};

use super::HyprMessenger;
use crate::{
    Address, BindData, ClientData, CursorPosition, DeviceInfo, Error, LayerData, LayerLevel,
    MonitorData, MonitorLayers, Result, WorkspaceData, WorkspaceRule,
};

impl HyprMessenger {
    #[instrument(skip(self), err)]
    pub(crate) async fn monitors(&self) -> Result<Vec<MonitorData>> {
        let response = self.send("j/monitors").await?;
        parse_json_response("j/monitors", &response)
    }

    #[instrument(skip(self), err)]
    pub(crate) async fn workspaces(&self) -> Result<Vec<WorkspaceData>> {
        let response = self.send("j/workspaces").await?;
        parse_json_response("j/workspaces", &response)
    }

    #[instrument(skip(self), err)]
    pub(crate) async fn workspace_rules(&self) -> Result<Vec<WorkspaceRule>> {
        let response = self.send("j/workspacerules").await?;
        parse_json_response("j/workspacerules", &response)
    }

    #[instrument(skip(self), err)]
    pub(crate) async fn clients(&self) -> Result<Vec<ClientData>> {
        let response = self.send("j/clients").await?;
        parse_json_response("j/clients", &response)
    }

    #[instrument(skip(self), fields(address = %address), err)]
    pub(crate) async fn layer(&self, address: Address) -> Result<LayerData> {
        let layers = self.layers().await?;
        layers
            .into_iter()
            .find(|layer| layer.address == address)
            .ok_or(Error::LayerNotFound(address))
    }

    #[instrument(skip(self), err)]
    pub(crate) async fn layers(&self) -> Result<Vec<LayerData>> {
        let response = self.send("j/layers").await?;
        let monitors: HashMap<String, MonitorLayers> = parse_json_response("j/layers", &response)?;

        let layers = monitors
            .into_iter()
            .flat_map(|(monitor_name, monitor_layers)| {
                monitor_layers
                    .levels
                    .into_iter()
                    .flat_map(move |(layer_level, layer_list)| {
                        let monitor_name = monitor_name.clone();

                        layer_list.into_iter().map(move |layer| LayerData {
                            address: layer.address,
                            x: layer.x,
                            y: layer.y,
                            width: layer.w,
                            height: layer.h,
                            namespace: layer.namespace,
                            monitor: monitor_name.clone(),
                            level: LayerLevel::from(layer_level.as_str()),
                            pid: layer.pid,
                        })
                    })
            })
            .collect();

        Ok(layers)
    }

    #[instrument(skip(self), fields(window = %window, property = %property), err)]
    pub(crate) async fn get_prop(&self, window: &str, property: &str) -> Result<String> {
        self.send(&format!("getprop {window} {property}")).await
    }

    #[instrument(skip(self), err)]
    pub(crate) async fn active_workspace(&self) -> Result<WorkspaceData> {
        let response = self.send("j/activeworkspace").await?;
        parse_json_response("j/activeworkspace", &response)
    }

    #[instrument(skip(self), err)]
    pub(crate) async fn active_window(&self) -> Result<Option<ClientData>> {
        let response = self.send("j/activewindow").await?;
        if response.trim() == "{}" {
            return Ok(None);
        }
        parse_json_response("j/activewindow", &response).map(Some)
    }

    #[instrument(skip(self), err)]
    pub(crate) async fn version(&self) -> Result<String> {
        self.send("version").await
    }

    #[instrument(skip(self), err)]
    pub(crate) async fn cursor_pos(&self) -> Result<CursorPosition> {
        let response = self.send("cursorpos").await?;
        let trimmed = response.trim().to_string();
        let parts: Vec<&str> = trimmed.split(", ").collect();

        if parts.len() != 2 {
            return Err(Error::EventParseError {
                event_data: response.clone(),
                field: "cursor_position",
                expected: "format 'x, y'",
                value: response,
            });
        }

        let x = parts[0]
            .parse::<i32>()
            .map_err(|_| Error::EventParseError {
                event_data: response.clone(),
                field: "x_coordinate",
                expected: "integer",
                value: parts[0].to_string(),
            })?;

        let y = parts[1]
            .parse::<i32>()
            .map_err(|_| Error::EventParseError {
                event_data: response,
                field: "y_coordinate",
                expected: "integer",
                value: parts[1].to_string(),
            })?;

        Ok(CursorPosition { x, y })
    }

    #[instrument(skip(self), err)]
    pub(crate) async fn binds(&self) -> Result<Vec<BindData>> {
        let response = self.send("j/binds").await?;
        parse_json_response("j/binds", &response)
    }

    #[instrument(skip(self), err)]
    pub(crate) async fn devices(&self) -> Result<DeviceInfo> {
        let response = self.send("j/devices").await?;
        parse_json_response("j/devices", &response)
    }

    #[instrument(skip(self), err)]
    pub(crate) async fn layouts(&self) -> Result<Vec<String>> {
        let response = self.send("j/layouts").await?;
        parse_json_response("j/layouts", &response)
    }

    #[instrument(skip(self), err)]
    pub(crate) async fn submap(&self) -> Result<String> {
        let response = self.send("submap").await?;
        Ok(response.trim().to_string())
    }
}

const RESPONSE_PREVIEW_CHARS: usize = 256;

fn parse_json_response<T>(command: &'static str, response: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    serde_json::from_str(response).map_err(|error| {
        let (preview, truncated) = response_preview(response);
        warn!(
            %command,
            response_len = response.len(),
            response_preview_truncated = truncated,
            response_preview = %preview,
            parse_error = %error,
            "cannot parse hyprland JSON response"
        );
        Error::JsonParseError(error)
    })
}

fn response_preview(response: &str) -> (String, bool) {
    let mut chars = response.chars();
    let preview: String = chars.by_ref().take(RESPONSE_PREVIEW_CHARS).collect();
    let truncated = chars.next().is_some();
    let escaped_preview = preview.chars().flat_map(|ch| ch.escape_default()).collect();
    (escaped_preview, truncated)
}
