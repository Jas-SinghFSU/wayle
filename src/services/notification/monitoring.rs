use std::{sync::Arc, time::Duration};

use chrono::Utc;
use tracing::{info, instrument};

use super::{
    core::notification::Notification, error::Error, events::NotificationEvent,
    persistence::NotificationStore, service::NotificationService,
};
use crate::services::{common::Property, traits::ServiceMonitoring};

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
                            handle_notification_added(&notif, &notification_list, &store, &remove_expired);
                            handle_popup_added(&notif, &popup_list, &popup_dur, dnd.clone());
                        }
                        NotificationEvent::Remove(id) => {
                            handle_notification_removed(id, &notification_list, &popup_list, &store);
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
) {
    if dnd.get() {
        return;
    }

    let incoming_popup = Arc::new(incoming_popup.clone());
    let mut list = popups.get();
    list.retain(|popup| popup != &incoming_popup);
    list.insert(0, incoming_popup.clone());

    popups.set(list);

    let id = incoming_popup.id;
    let duration = popup_duration.get();
    let popups = popups.clone();

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(duration as u64)).await;
        let mut list = popups.get();
        list.retain(|popup| popup.id != id);
        popups.set(list);
    });
}

fn handle_notification_added(
    incoming_notif: &Notification,
    notifications: &Property<Vec<Arc<Notification>>>,
    store: &Option<NotificationStore>,
    remove_expired: &Property<bool>,
) {
    if incoming_notif.is_transient.get() {
        return;
    }

    let notif_arc = Arc::new(incoming_notif.clone());
    let mut list = notifications.get();
    list.retain(|notif| notif.id != notif_arc.id);
    list.insert(0, notif_arc.clone());

    notifications.set(list);

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
    let notifications = notifications.clone();
    let store = store.clone();

    tokio::spawn(async move {
        tokio::time::sleep(time_until_expiration).await;
        let mut list = notifications.get();
        list.retain(|notif| notif.id != id);
        notifications.set(list);

        if let Some(store) = store.as_ref() {
            let _ = store.remove(id);
        }
    });
}

fn handle_notification_removed(
    id: u32,
    notifications: &Property<Vec<Arc<Notification>>>,
    popups: &Property<Vec<Arc<Notification>>>,
    store: &Option<NotificationStore>,
) {
    let mut notif_list = notifications.get();
    notif_list.retain(|notif| notif.id != id);
    notifications.set(notif_list.clone());

    if let Some(store) = store.as_ref() {
        let _ = store.remove(id);
    };

    let mut popup_list = popups.get();
    popup_list.retain(|popup| popup.id != id);
    popups.set(popup_list);
}
