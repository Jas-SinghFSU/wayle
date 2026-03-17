use std::{sync::Arc, time::Duration};

use chrono::Utc;
use tokio::sync::broadcast;
use tracing::{debug, info, instrument, warn};
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;
use zbus::Connection;

use crate::{
    core::notification::Notification,
    error::Error,
    events::NotificationEvent,
    persistence::NotificationStore,
    popup_timer::PopupTimerManager,
    service::NotificationService,
    types::{
        ClosedReason, Signal,
        dbus::{SERVICE_INTERFACE, SERVICE_PATH},
    },
};

impl ServiceMonitoring for NotificationService {
    type Error = Error;
    #[instrument(skip_all, err)]
    async fn start_monitoring(&self) -> Result<(), Self::Error> {
        handle_notifications(self).await?;
        Ok(())
    }
}

#[instrument(skip_all)]
async fn handle_notifications(service: &NotificationService) -> Result<(), Error> {
    let mut event_receiver = service.notif_tx.subscribe();
    let notification_list = service.notifications.clone();
    let popup_list = service.popups.clone();
    let popup_dur = service.popup_duration.clone();
    let dnd = service.dnd.clone();
    let store = service.store.clone();
    let cancellation_token = service.cancellation_token.clone();
    let remove_expired = service.remove_expired.clone();
    let connection = service.connection.clone();
    let notif_tx = service.notif_tx.clone();
    let popup_timers = service.popup_timers.clone();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    info!("Notification monitoring cancelled, stopping");
                    return;
                }
                Ok(event) = event_receiver.recv() => {
                    match event {
                        NotificationEvent::Add(notif) => {
                            handle_notification_added(
                                &notif,
                                &notification_list,
                                &store,
                                &remove_expired,
                                &notif_tx
                            );
                            handle_popup_added(
                                &notif,
                                &popup_list,
                                &popup_dur,
                                dnd.clone(),
                                &popup_timers,
                            );
                        }
                        NotificationEvent::Remove(id, reason) => {
                            handle_notification_removed(
                                id,
                                reason,
                                &notification_list,
                                &popup_list,
                                &store,
                                &connection,
                                &popup_timers,
                            ).await;
                        }
                    }
                }
            }
        }
    });

    Ok(())
}

fn handle_popup_added(
    incoming_popup: &Notification,
    popups: &Property<Vec<Arc<Notification>>>,
    popup_duration: &Property<u32>,
    dnd: Property<bool>,
    popup_timers: &Arc<PopupTimerManager>,
) {
    if dnd.get() {
        return;
    }

    let incoming_popup = Arc::new(incoming_popup.clone());
    let mut list = popups.get();
    list.retain(|popup| popup != &incoming_popup);
    list.insert(0, incoming_popup.clone());
    popups.replace(list);

    let default_duration = Duration::from_millis(popup_duration.get() as u64);

    let expire_duration = incoming_popup
        .expire_timeout
        .get()
        .map(|ttl| Duration::from_millis(ttl as u64));

    let timer_duration = match expire_duration {
        Some(expire) if !expire.is_zero() => default_duration.min(expire),
        _ => default_duration,
    };

    popup_timers.start(incoming_popup.id, timer_duration);
}

fn handle_notification_added(
    incoming_notif: &Notification,
    notifications: &Property<Vec<Arc<Notification>>>,
    store: &Option<NotificationStore>,
    remove_expired: &Property<bool>,
    notif_tx: &broadcast::Sender<NotificationEvent>,
) {
    if incoming_notif.is_transient.get() {
        return;
    }

    let notif_arc = Arc::new(incoming_notif.clone());
    let mut list = notifications.get();

    let replaced = list
        .iter()
        .find(|notif| notif.id == notif_arc.id)
        .map(|notif| (notif.id, notif.app_name.get()));
    if let Some((replaced_id, replaced_app)) = &replaced {
        debug!(
            incoming_id = notif_arc.id,
            incoming_app = ?notif_arc.app_name.get(),
            replaced_id,
            replaced_app = ?replaced_app,
            "replacing existing notification"
        );
    } else {
        debug!(
            id = notif_arc.id,
            app = ?notif_arc.app_name.get(),
            summary = %notif_arc.summary.get(),
            list_size = list.len(),
            "adding new notification"
        );
    }

    list.retain(|notif| notif.id != notif_arc.id);
    list.insert(0, notif_arc.clone());

    notifications.replace(list);

    if let Some(store) = store.as_ref() {
        let _ = store.add(incoming_notif);
    };

    if !remove_expired.get() {
        return;
    }

    let Some(ttl) = notif_arc.expire_timeout.get() else {
        return;
    };

    let expiration_time = notif_arc.timestamp.get() + Duration::from_millis(ttl as u64);
    let now = Utc::now();

    if expiration_time <= now {
        let mut list = notifications.get();
        list.retain(|notif| notif.id != notif_arc.id);
        notifications.set(list);
        return;
    }

    let time_until_expiration = (expiration_time - now).to_std().unwrap_or(Duration::ZERO);
    let id = notif_arc.id;
    let tx = notif_tx.clone();

    tokio::spawn(async move {
        tokio::time::sleep(time_until_expiration).await;
        let _ = tx.send(NotificationEvent::Remove(id, ClosedReason::Expired));
    });
}

async fn handle_notification_removed(
    id: u32,
    reason: ClosedReason,
    notifications: &Property<Vec<Arc<Notification>>>,
    popups: &Property<Vec<Arc<Notification>>>,
    store: &Option<NotificationStore>,
    connection: &Connection,
    popup_timers: &Arc<PopupTimerManager>,
) {
    if !matches!(reason, ClosedReason::Expired) {
        popup_timers.cancel(id);

        let mut popup_list = popups.get();
        popup_list.retain(|popup| popup.id != id);
        popups.set(popup_list);
    }

    let mut notif_list = notifications.get();
    let prev_len = notif_list.len();
    notif_list.retain(|notif| notif.id != id);

    if notif_list.len() == prev_len {
        return;
    }

    notifications.set(notif_list);

    if let Some(store) = store.as_ref() {
        let _ = store.remove(id);
    };

    debug!(id = id, ?reason, "emitting NotificationClosed");
    if let Err(err) = connection
        .emit_signal(
            None::<()>,
            SERVICE_PATH,
            SERVICE_INTERFACE,
            Signal::NotificationClosed.as_str(),
            &(id, reason as u32),
        )
        .await
    {
        warn!(id = id, error = %err, "cannot emit NotificationClosed signal");
    }
}
