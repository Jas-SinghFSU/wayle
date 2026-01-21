use tracing::instrument;
use zbus::{Connection, zvariant::OwnedValue};

use crate::{
    error::Error,
    proxy::{dbusmenu::DBusMenuProxy, status_notifier_item::StatusNotifierItemProxy},
    types::menu::RawMenuItemsPropsList,
};

pub(super) struct TrayItemController;

impl TrayItemController {
    #[instrument(skip(connection), fields(service = %service, path = %path, x, y), err)]
    pub async fn context_menu(
        connection: &Connection,
        service: &str,
        path: &str,
        x: i32,
        y: i32,
    ) -> Result<(), Error> {
        let proxy = StatusNotifierItemProxy::builder(connection)
            .destination(service)?
            .path(path)?
            .build()
            .await?;

        proxy
            .context_menu(x, y)
            .await
            .map_err(|source| Error::Operation {
                operation: "context_menu",
                source,
            })
    }

    #[instrument(skip(connection), fields(service = %service, path = %path, x, y), err)]
    pub async fn activate(
        connection: &Connection,
        service: &str,
        path: &str,
        x: i32,
        y: i32,
    ) -> Result<(), Error> {
        let proxy = StatusNotifierItemProxy::builder(connection)
            .destination(service)?
            .path(path)?
            .build()
            .await?;

        proxy.activate(x, y).await.map_err(|source| {
            const UNKNOWN_METHOD: &str = "org.freedesktop.DBus.Error.UnknownMethod";
            if let zbus::Error::MethodError(name, _, _) = &source
                && name.as_str() == UNKNOWN_METHOD {
                    return Error::OperationNotSupported {
                        operation: "activate",
                    };
                }
            Error::Operation {
                operation: "activate",
                source,
            }
        })
    }

    #[instrument(skip(connection), fields(service = %service, path = %path, x, y), err)]
    pub async fn secondary_activate(
        connection: &Connection,
        service: &str,
        path: &str,
        x: i32,
        y: i32,
    ) -> Result<(), Error> {
        let proxy = StatusNotifierItemProxy::builder(connection)
            .destination(service)?
            .path(path)?
            .build()
            .await?;

        proxy
            .secondary_activate(x, y)
            .await
            .map_err(|source| Error::Operation {
                operation: "secondary_activate",
                source,
            })
    }

    #[instrument(
        skip(connection),
        fields(service = %service, path = %path, delta, orientation = %orientation),
        err
    )]
    pub async fn scroll(
        connection: &Connection,
        service: &str,
        path: &str,
        delta: i32,
        orientation: &str,
    ) -> Result<(), Error> {
        let proxy = StatusNotifierItemProxy::builder(connection)
            .destination(service)?
            .path(path)?
            .build()
            .await?;

        proxy
            .scroll(delta, orientation)
            .await
            .map_err(|source| Error::Operation {
                operation: "scroll",
                source,
            })
    }

    #[instrument(
        skip(connection),
        fields(bus_name = %bus_name, menu_path = %menu_path, id),
        err
    )]
    pub async fn menu_about_to_show(
        connection: &Connection,
        bus_name: &str,
        menu_path: &str,
        id: i32,
    ) -> Result<bool, Error> {
        let proxy = DBusMenuProxy::new(connection, bus_name, menu_path).await?;

        proxy
            .about_to_show(id)
            .await
            .map_err(|source| Error::Operation {
                operation: "menu_about_to_show",
                source,
            })
    }

    #[instrument(
        skip(connection, data),
        fields(bus_name = %bus_name, menu_path = %menu_path, id, event_id = %event_id, timestamp),
        err
    )]
    pub async fn menu_event(
        connection: &Connection,
        bus_name: &str,
        menu_path: &str,
        id: i32,
        event_id: &str,
        data: OwnedValue,
        timestamp: u32,
    ) -> Result<(), Error> {
        let proxy = DBusMenuProxy::new(connection, bus_name, menu_path).await?;

        proxy
            .event(id, event_id, data, timestamp)
            .await
            .map_err(|source| Error::Operation {
                operation: "menu_event",
                source,
            })
    }

    #[instrument(
        skip(connection),
        fields(bus_name = %bus_name, menu_path = %menu_path, ids = ?ids),
        err
    )]
    pub async fn menu_about_to_show_group(
        connection: &Connection,
        bus_name: &str,
        menu_path: &str,
        ids: Vec<i32>,
    ) -> Result<(Vec<i32>, Vec<i32>), Error> {
        let proxy = DBusMenuProxy::new(connection, bus_name, menu_path).await?;

        proxy
            .about_to_show_group(ids)
            .await
            .map_err(|source| Error::Operation {
                operation: "menu_about_to_show_group",
                source,
            })
    }

    #[instrument(
        skip(connection, events),
        fields(bus_name = %bus_name, menu_path = %menu_path, events_count = events.len()),
        err
    )]
    pub async fn menu_event_group(
        connection: &Connection,
        bus_name: &str,
        menu_path: &str,
        events: Vec<(i32, String, OwnedValue, u32)>,
    ) -> Result<Vec<i32>, Error> {
        let proxy = DBusMenuProxy::new(connection, bus_name, menu_path).await?;

        proxy
            .event_group(events)
            .await
            .map_err(|source| Error::Operation {
                operation: "menu_event_group",
                source,
            })
    }

    #[instrument(
        skip(connection),
        fields(bus_name = %bus_name, menu_path = %menu_path, id, property = %property),
        err
    )]
    pub async fn menu_get_property(
        connection: &Connection,
        bus_name: &str,
        menu_path: &str,
        id: i32,
        property: &str,
    ) -> Result<OwnedValue, Error> {
        let proxy = DBusMenuProxy::new(connection, bus_name, menu_path).await?;

        proxy
            .get_property(id, property)
            .await
            .map_err(|source| Error::Operation {
                operation: "menu_get_property",
                source,
            })
    }

    #[instrument(
        skip(connection),
        fields(
            bus_name = %bus_name,
            menu_path = %menu_path,
            ids = ?ids,
            props_count = property_names.len()
        ),
        err
    )]
    pub async fn menu_get_group_properties(
        connection: &Connection,
        bus_name: &str,
        menu_path: &str,
        ids: Vec<i32>,
        property_names: Vec<String>,
    ) -> Result<RawMenuItemsPropsList, Error> {
        let proxy = DBusMenuProxy::new(connection, bus_name, menu_path).await?;

        proxy
            .get_group_properties(ids, property_names)
            .await
            .map_err(|source| Error::Operation {
                operation: "menu_get_group_properties",
                source,
            })
    }
}
