use std::sync::{Arc, Weak};

use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;

use super::TrayItem;
use crate::services::{
    systray::{
        error::Error,
        proxy::status_notifier_item::StatusNotifierItemProxy,
        types::item::{Category, IconPixmap, Status, Tooltip},
    },
    traits::ModelMonitoring,
};

impl ModelMonitoring for TrayItem {
    type Error = Error;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(Error::ServiceInitializationFailed(String::from(
                "A cancellation_token was not found.",
            )));
        };

        let bus_name = self.bus_name.get();
        let proxy = StatusNotifierItemProxy::new(&self.zbus_connection, bus_name.clone()).await?;
        let cancel_token = cancellation_token.clone();
        let weak_self = Arc::downgrade(&self);

        debug!("Starting property monitoring for tray item: {bus_name}");

        tokio::spawn(async move {
            monitor_properties(&bus_name, weak_self, proxy, cancel_token).await;
        });

        Ok(())
    }
}

#[allow(clippy::cognitive_complexity)]
#[allow(clippy::too_many_lines)]
async fn monitor_properties(
    bus_name: &str,
    weak_item: Weak<TrayItem>,
    proxy: StatusNotifierItemProxy<'static>,
    cancellation_token: CancellationToken,
) {
    let mut category_changed = proxy.receive_category_changed().await;
    let mut id_changed = proxy.receive_id_changed().await;
    let mut title_changed = proxy.receive_title_changed().await;
    let mut status_changed = proxy.receive_status_changed().await;
    let mut window_id_changed = proxy.receive_window_id_changed().await;
    let mut icon_name_changed = proxy.receive_icon_name_changed().await;
    let mut icon_pixmap_changed = proxy.receive_icon_pixmap_changed().await;
    let mut overlay_icon_name_changed = proxy.receive_overlay_icon_name_changed().await;
    let mut overlay_icon_pixmap_changed = proxy.receive_overlay_icon_pixmap_changed().await;
    let mut attention_icon_name_changed = proxy.receive_attention_icon_name_changed().await;
    let mut attention_icon_pixmap_changed = proxy.receive_attention_icon_pixmap_changed().await;
    let mut attention_movie_name_changed = proxy.receive_attention_movie_name_changed().await;
    let mut tooltip_changed = proxy.receive_tool_tip_changed().await;
    let mut item_is_menu_changed = proxy.receive_item_is_menu_changed().await;
    let mut menu_changed = proxy.receive_menu_changed().await;
    let mut icon_theme_path_changed = proxy.receive_icon_theme_path_changed().await;

    loop {
        let Some(tray_item) = weak_item.upgrade() else {
            return;
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => {
                debug!("Tray item '{bus_name}' monitor received cancellation, stopping");
                return;
            }

            Some(change) = category_changed.next() => {
                if let Ok(new_category) = change.get().await {
                    let category = Category::from(new_category.as_str());
                    tray_item.category.set(category);
                }
            }

            Some(change) = id_changed.next() => {
                if let Ok(new_id) = change.get().await {
                    tray_item.id.set(new_id);
                }
            }

            Some(change) = title_changed.next() => {
                if let Ok(new_title) = change.get().await {
                    tray_item.title.set(new_title);
                }
            }

            Some(change) = status_changed.next() => {
                if let Ok(new_status) = change.get().await {
                    let status = Status::from(new_status.as_str());
                    tray_item.status.set(status);
                }
            }

            Some(change) = window_id_changed.next() => {
                if let Ok(new_window_id) = change.get().await {
                    tray_item.window_id.set(new_window_id);
                }
            }

            Some(change) = icon_name_changed.next() => {
                if let Ok(new_icon_name) = change.get().await {
                    let icon_name = if new_icon_name.is_empty() {
                        None
                    } else {
                        Some(new_icon_name)
                    };
                    tray_item.icon_name.set(icon_name);
                }
            }

            Some(change) = icon_pixmap_changed.next() => {
                if let Ok(new_pixmaps) = change.get().await {
                    let pixmaps: Vec<IconPixmap> = new_pixmaps.into_iter().map(Into::into).collect();
                    tray_item.icon_pixmap.set(pixmaps);
                }
            }

            Some(change) = overlay_icon_name_changed.next() => {
                if let Ok(new_overlay_icon_name) = change.get().await {
                    let overlay_icon_name = if new_overlay_icon_name.is_empty() {
                        None
                    } else {
                        Some(new_overlay_icon_name)
                    };
                    tray_item.overlay_icon_name.set(overlay_icon_name);
                }
            }

            Some(change) = overlay_icon_pixmap_changed.next() => {
                if let Ok(new_pixmaps) = change.get().await {
                    let pixmaps: Vec<IconPixmap> = new_pixmaps.into_iter().map(Into::into).collect();
                    tray_item.overlay_icon_pixmap.set(pixmaps);
                }
            }

            Some(change) = attention_icon_name_changed.next() => {
                if let Ok(new_attention_icon_name) = change.get().await {
                    let attention_icon_name = if new_attention_icon_name.is_empty() {
                        None
                    } else {
                        Some(new_attention_icon_name)
                    };
                    tray_item.attention_icon_name.set(attention_icon_name);
                }
            }

            Some(change) = attention_icon_pixmap_changed.next() => {
                if let Ok(new_pixmaps) = change.get().await {
                    let pixmaps: Vec<IconPixmap> = new_pixmaps.into_iter().map(Into::into).collect();
                    tray_item.attention_icon_pixmap.set(pixmaps);
                }
            }

            Some(change) = attention_movie_name_changed.next() => {
                if let Ok(new_movie_name) = change.get().await {
                    let attention_movie_name = if new_movie_name.is_empty() {
                        None
                    } else {
                        Some(new_movie_name)
                    };
                    tray_item.attention_movie_name.set(attention_movie_name);
                }
            }

            Some(change) = tooltip_changed.next() => {
                if let Ok(raw_tooltip) = change.get().await {
                    let tooltip = Tooltip::from(raw_tooltip);
                    tray_item.tooltip.set(tooltip);
                }
            }

            Some(change) = item_is_menu_changed.next() => {
                if let Ok(new_item_is_menu) = change.get().await {
                    tray_item.item_is_menu.set(new_item_is_menu);
                }
            }

            Some(change) = menu_changed.next() => {
                if let Ok(new_menu_path) = change.get().await {
                    tray_item.menu_path.set(new_menu_path);
                }
            }

            Some(change) = icon_theme_path_changed.next() => {
                if let Ok(new_icon_theme_path) = change.get().await {
                    let icon_theme_path = if new_icon_theme_path.is_empty() {
                        None
                    } else {
                        Some(new_icon_theme_path)
                    };
                    tray_item.icon_theme_path.set(icon_theme_path);
                }
            }

            else => {
                debug!("All property streams ended for tray item {bus_name}, exiting monitor");
                break;
            }
        }
    }
}
