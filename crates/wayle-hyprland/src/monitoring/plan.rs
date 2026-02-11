//! Sync planning based on Hyprland events.
//!
//! Maps event types to the domains (clients, monitors, workspaces, layers)
//! that need reconciliation.

use crate::HyprlandEvent;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct SyncPlan {
    pub(super) clients: bool,
    pub(super) monitors: bool,
    pub(super) workspaces: bool,
    pub(super) layers: bool,
}

impl SyncPlan {
    pub(super) fn merge(self, other: Self) -> Self {
        Self {
            clients: self.clients || other.clients,
            monitors: self.monitors || other.monitors,
            workspaces: self.workspaces || other.workspaces,
            layers: self.layers || other.layers,
        }
    }

    pub(super) fn is_empty(self) -> bool {
        !self.clients && !self.monitors && !self.workspaces && !self.layers
    }
}

pub(super) fn for_event(event: &HyprlandEvent) -> SyncPlan {
    match event {
        HyprlandEvent::OpenWindow { .. }
        | HyprlandEvent::CloseWindow { .. }
        | HyprlandEvent::MoveWindow { .. }
        | HyprlandEvent::MoveWindowV2 { .. } => SyncPlan {
            clients: true,
            workspaces: true,
            ..SyncPlan::default()
        },
        HyprlandEvent::ActiveWindow { .. } | HyprlandEvent::ActiveWindowV2 { .. } => SyncPlan {
            clients: true,
            workspaces: true,
            ..SyncPlan::default()
        },
        HyprlandEvent::ChangeFloatingMode { .. }
        | HyprlandEvent::ToggleGroup { .. }
        | HyprlandEvent::MoveIntoGroup { .. }
        | HyprlandEvent::MoveOutOfGroup { .. }
        | HyprlandEvent::Pin { .. }
        | HyprlandEvent::Minimized { .. } => SyncPlan {
            clients: true,
            ..SyncPlan::default()
        },
        HyprlandEvent::WindowTitle { .. }
        | HyprlandEvent::WindowTitleV2 { .. }
        | HyprlandEvent::Fullscreen { .. } => SyncPlan {
            clients: true,
            workspaces: true,
            ..SyncPlan::default()
        },
        HyprlandEvent::Workspace { .. } | HyprlandEvent::WorkspaceV2 { .. } => SyncPlan {
            monitors: true,
            workspaces: true,
            ..SyncPlan::default()
        },
        HyprlandEvent::CreateWorkspace { .. }
        | HyprlandEvent::CreateWorkspaceV2 { .. }
        | HyprlandEvent::DestroyWorkspace { .. }
        | HyprlandEvent::DestroyWorkspaceV2 { .. }
        | HyprlandEvent::RenameWorkspace { .. }
        | HyprlandEvent::ActiveSpecial { .. }
        | HyprlandEvent::ActiveSpecialV2 { .. }
        | HyprlandEvent::MoveWorkspace { .. }
        | HyprlandEvent::MoveWorkspaceV2 { .. }
        | HyprlandEvent::MonitorAdded { .. }
        | HyprlandEvent::MonitorAddedV2 { .. }
        | HyprlandEvent::MonitorRemoved { .. }
        | HyprlandEvent::MonitorRemovedV2 { .. } => SyncPlan {
            monitors: true,
            workspaces: true,
            ..SyncPlan::default()
        },
        HyprlandEvent::FocusedMon { .. } | HyprlandEvent::FocusedMonV2 { .. } => SyncPlan {
            monitors: true,
            ..SyncPlan::default()
        },
        HyprlandEvent::OpenLayer { .. } | HyprlandEvent::CloseLayer { .. } => SyncPlan {
            layers: true,
            ..SyncPlan::default()
        },
        _ => SyncPlan::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_combines_domains() {
        let a = SyncPlan {
            clients: true,
            ..SyncPlan::default()
        };
        let b = SyncPlan {
            monitors: true,
            layers: true,
            ..SyncPlan::default()
        };

        assert_eq!(
            a.merge(b),
            SyncPlan {
                clients: true,
                monitors: true,
                layers: true,
                ..SyncPlan::default()
            }
        );
    }

    #[test]
    fn workspace_event_targets_monitors_and_workspaces() {
        let plan = for_event(&HyprlandEvent::Workspace {
            name: String::from("2"),
        });

        assert_eq!(
            plan,
            SyncPlan {
                monitors: true,
                workspaces: true,
                ..SyncPlan::default()
            }
        );
    }

    #[test]
    fn default_plan_is_empty() {
        assert!(SyncPlan::default().is_empty());
    }

    #[test]
    fn plan_with_any_domain_is_not_empty() {
        assert!(
            !SyncPlan {
                clients: true,
                ..SyncPlan::default()
            }
            .is_empty()
        );

        assert!(
            !SyncPlan {
                layers: true,
                ..SyncPlan::default()
            }
            .is_empty()
        );
    }
}
