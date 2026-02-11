//! Event projector that coalesces bursts and syncs state before forwarding.
//!
//! Implements the Projector pattern: events are collected, model state is
//! reconciled via IPC, then events are forwarded to consumers.

use tokio::sync::broadcast::{
    Receiver,
    error::{RecvError, TryRecvError},
};
use tracing::{Span, instrument, warn};

use super::{
    SyncRuntime,
    plan::{self, SyncPlan},
    reconcile,
};
use crate::HyprlandEvent;

const MAX_COALESCED_EVENTS: usize = 64;

pub(super) fn spawn(runtime: SyncRuntime) {
    let mut event_rx = runtime.event_tx.subscribe();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = runtime.cancellation_token.cancelled() => {
                    return;
                }
                event = event_rx.recv() => {
                    match event {
                        Ok(event) => {
                            project_event_burst(&runtime, &mut event_rx, event).await;
                        }
                        Err(RecvError::Lagged(skipped)) => {
                            warn!(skipped, "Hyprland event projector lagged behind ingress stream");
                        }
                        Err(RecvError::Closed) => {
                            return;
                        }
                    }
                }
            }
        }
    });
}

#[instrument(skip_all, fields(event_count))]
async fn project_event_burst(
    runtime: &SyncRuntime,
    event_rx: &mut Receiver<HyprlandEvent>,
    first_event: HyprlandEvent,
) {
    let mut events = Vec::with_capacity(MAX_COALESCED_EVENTS);
    let mut merged_plan = SyncPlan::default();
    record_event(&mut events, &mut merged_plan, first_event);

    while events.len() < MAX_COALESCED_EVENTS {
        match event_rx.try_recv() {
            Ok(event) => {
                record_event(&mut events, &mut merged_plan, event);
            }
            Err(TryRecvError::Empty) | Err(TryRecvError::Closed) => {
                break;
            }
            Err(TryRecvError::Lagged(skipped)) => {
                warn!(
                    skipped,
                    "Hyprland event projector lagged behind ingress stream"
                );
            }
        }
    }

    Span::current().record("event_count", events.len());

    if !merged_plan.is_empty() {
        reconcile::sync_model_state(runtime, merged_plan).await;
    }

    for event in events {
        let _ = runtime.hyprland_tx.send(event);
    }
}

fn record_event(events: &mut Vec<HyprlandEvent>, merged_plan: &mut SyncPlan, event: HyprlandEvent) {
    let event_plan = plan::for_event(&event);
    *merged_plan = merged_plan.merge(event_plan);
    events.push(event);
}
