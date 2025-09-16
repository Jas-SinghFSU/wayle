use std::{sync::Arc, time::Duration};

use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
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
        handle_notifications(
            &self.notif_tx,
            &self.notifications,
            &self.popups,
            &self.popup_duration,
            &self.dnd,
            &self.store,
            &self.cancellation_token,
        )
        .await?;

        Ok(())
    }
}

#[instrument(skip_all)]
async fn handle_notifications(
    notif_tx: &broadcast::Sender<NotificationEvent>,
    notifications: &Property<Vec<Arc<Notification>>>,
    popups: &Property<Vec<Arc<Notification>>>,
    popup_duration: &Property<u32>,
    dnd: &Property<bool>,
    store: &Option<NotificationStore>,
    cancellation_token: &CancellationToken,
) -> Result<(), Error> {
    let mut event_receiver = notif_tx.subscribe();
    let notification_list = notifications.clone();
    let popup_list = popups.clone();
    let popup_dur = popup_duration.clone();
    let dnd = dnd.clone();
    let store = store.clone();
    let cancellation_token = cancellation_token.clone();

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
                            handle_notification_added(&notif, &notification_list, &store);
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
) {
    let inc_notif_arc = Arc::new(incoming_notif.clone());
    let mut list = notifications.get();
    list.retain(|notif| notif != &inc_notif_arc);
    list.insert(0, inc_notif_arc);

    notifications.set(list);

    if let Some(store) = store.as_ref() {
        let _ = store.add(incoming_notif);
    };
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
