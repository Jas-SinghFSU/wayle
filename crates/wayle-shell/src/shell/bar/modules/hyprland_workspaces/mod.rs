mod button;
mod factory;
mod filtering;
mod helpers;
mod messages;
mod styling;
mod watchers;

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use gtk::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};
use tracing::{debug, error, warn};
use wayle_config::{
    ConfigService,
    schemas::{
        bar::BorderLocation,
        modules::{HyprlandWorkspacesConfig, Numbering},
    },
};
use wayle_hyprland::{Address, HyprlandService, WorkspaceId};
use wayle_widgets::{prelude::BarSettings, utils::force_window_resize};

use self::{
    button::{
        ButtonBuildContext, WorkspaceButton, WorkspaceButtonInput, WorkspaceButtonOutput,
        build_button_init,
    },
    filtering::{
        FilterContext, FilteredWorkspace, WorkspaceData, calculate_navigation_index,
        filter_workspaces, monitor_workspaces_sorted,
    },
    helpers::{
        addresses_in_workspace, compute_display_id, prune_stale_addresses,
        should_update_for_monitor, workspace_contains_urgent_address,
    },
};
pub(crate) use self::{
    factory::Factory,
    messages::{WorkspacesCmd, WorkspacesInit, WorkspacesMsg},
};

pub(crate) struct HyprlandWorkspaces {
    hyprland: Option<Arc<HyprlandService>>,
    config: Arc<ConfigService>,
    settings: BarSettings,
    active_workspace_id: WorkspaceId,
    focused_monitor: Option<String>,
    workspace_monitor_rules: HashMap<WorkspaceId, String>,
    urgent_windows: HashSet<Address>,
    css_provider: gtk::CssProvider,
    buttons: FactoryVecDeque<WorkspaceButton>,
}

#[relm4::component(pub(crate))]
impl Component for HyprlandWorkspaces {
    type Init = WorkspacesInit;
    type Input = WorkspacesMsg;
    type Output = ();
    type CommandOutput = WorkspacesCmd;

    view! {
        gtk::Box {
            add_css_class: "workspaces",
            #[watch]
            set_orientation: model.orientation(),
            #[watch]
            set_hexpand: model.is_vertical(),
            #[watch]
            set_vexpand: !model.is_vertical(),
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let config = init.config.config();
        let workspaces_config = &config.modules.hyprland_workspaces;
        let monitor_specific = workspaces_config.monitor_specific.get();
        let theme_provider = config.styling.theme_provider.clone();

        let active_id =
            Self::initial_active_workspace(&init.hyprland, &init.settings, monitor_specific);
        let focused_monitor = Self::initial_focused_monitor(&init.hyprland);
        let bar_scale = config.bar.scale.clone();

        Self::spawn_load_workspace_rules(&sender, &init.hyprland);

        watchers::spawn_watchers(
            &sender,
            workspaces_config,
            &init.hyprland,
            theme_provider,
            bar_scale,
            &init.settings,
        );

        let css_provider = gtk::CssProvider::new();
        gtk::style_context_add_provider_for_display(
            &root.display(),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER + 1,
        );

        let buttons = FactoryVecDeque::builder().launch(root.clone()).forward(
            sender.input_sender(),
            |output| match output {
                WorkspaceButtonOutput::Clicked(id) => WorkspacesMsg::WorkspaceClicked(id),
                WorkspaceButtonOutput::ScrollUp => WorkspacesMsg::ScrollUp,
                WorkspaceButtonOutput::ScrollDown => WorkspacesMsg::ScrollDown,
            },
        );

        let mut model = Self {
            hyprland: init.hyprland,
            config: init.config,
            settings: init.settings,
            active_workspace_id: active_id,
            focused_monitor,
            workspace_monitor_rules: HashMap::new(),
            urgent_windows: HashSet::new(),
            css_provider,
            buttons,
        };

        styling::apply_styling(&model.css_provider, &model.config, &model.settings);
        model.rebuild_buttons();

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            WorkspacesMsg::WorkspaceClicked(id) => {
                self.switch_to_workspace(id);
            }
            WorkspacesMsg::ScrollUp => {
                self.navigate_workspace(-1);
            }
            WorkspacesMsg::ScrollDown => {
                self.navigate_workspace(1);
            }
        }
    }

    fn update_cmd(&mut self, msg: WorkspacesCmd, sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
            WorkspacesCmd::WorkspacesChanged => {
                self.rebuild_buttons();
                force_window_resize(root);
            }
            WorkspacesCmd::ConfigChanged => {
                styling::apply_styling(&self.css_provider, &self.config, &self.settings);
                self.rebuild_buttons();
                force_window_resize(root);
            }
            WorkspacesCmd::ClientsChanged => {
                self.prune_stale_urgent_windows();
                self.rebuild_buttons();
                force_window_resize(root);
            }
            WorkspacesCmd::TitleChanged => {
                self.update_app_icons_on_title_change();
            }
            WorkspacesCmd::ActiveWorkspaceChanged(id) => {
                let config = self.config.config();
                let monitor_specific = config.modules.hyprland_workspaces.monitor_specific.get();
                let has_min_workspace_count =
                    config.modules.hyprland_workspaces.min_workspace_count.get() > 0;

                if !self.should_apply_active_workspace_change(id, monitor_specific) {
                    return;
                }

                self.clear_urgent_windows_for_workspace(id);
                self.active_workspace_id = id;
                self.sync_after_active_workspace_change(has_min_workspace_count);
            }
            WorkspacesCmd::MonitorFocused {
                monitor,
                workspace_id,
            } => {
                self.focused_monitor = Some(monitor.clone());

                let config = self.config.config();
                let monitor_specific = config.modules.hyprland_workspaces.monitor_specific.get();
                let has_min_workspace_count =
                    config.modules.hyprland_workspaces.min_workspace_count.get() > 0;

                if should_update_for_monitor(
                    monitor_specific,
                    self.settings.monitor_name.as_deref(),
                    &monitor,
                ) {
                    self.clear_urgent_windows_for_workspace(workspace_id);
                    self.active_workspace_id = workspace_id;
                    self.sync_after_active_workspace_change(has_min_workspace_count);
                }
            }
            WorkspacesCmd::HyprlandConfigReloaded => {
                debug!("Hyprland config reloaded, refreshing workspace rules");
                Self::spawn_load_workspace_rules(&sender, &self.hyprland);
            }
            WorkspacesCmd::UrgentWindow(address) => {
                self.urgent_windows.insert(address);
                self.update_active_state();
            }
            WorkspacesCmd::WindowFocused(address) => {
                if self.urgent_windows.remove(&address) {
                    self.update_active_state();
                }
            }
            WorkspacesCmd::WorkspaceRulesLoaded(rules) => {
                self.workspace_monitor_rules = rules;
                self.rebuild_buttons();
                force_window_resize(root);
            }
        }
    }
}

impl HyprlandWorkspaces {
    fn is_vertical(&self) -> bool {
        self.settings.is_vertical.get()
    }

    fn orientation(&self) -> gtk::Orientation {
        if self.is_vertical() {
            gtk::Orientation::Vertical
        } else {
            gtk::Orientation::Horizontal
        }
    }

    fn spawn_load_workspace_rules(
        sender: &ComponentSender<Self>,
        hyprland: &Option<Arc<HyprlandService>>,
    ) {
        let Some(hyprland) = hyprland.clone() else {
            return;
        };

        sender.oneshot_command(async move {
            match hyprland.workspace_rules().await {
                Ok(rules) => {
                    let map = rules
                        .into_iter()
                        .filter_map(|rule| {
                            let id = rule.workspace_string.parse::<WorkspaceId>().ok()?;
                            if id > 0 {
                                Some((id, rule.monitor))
                            } else {
                                None
                            }
                        })
                        .collect();
                    WorkspacesCmd::WorkspaceRulesLoaded(map)
                }
                Err(e) => {
                    warn!(error = %e, "cannot load workspace rules");
                    WorkspacesCmd::WorkspaceRulesLoaded(HashMap::new())
                }
            }
        });
    }

    fn workspace_monitor_name(&self, id: WorkspaceId) -> Option<String> {
        if let Some(hyprland) = &self.hyprland
            && let Some(monitor_name) = hyprland
                .workspaces
                .get()
                .into_iter()
                .find(|ws| ws.id.get() == id)
                .map(|ws| ws.monitor.get())
        {
            return Some(monitor_name);
        }

        self.workspace_monitor_rules.get(&id).cloned()
    }

    fn display_id(&self, id: WorkspaceId, numbering: Numbering) -> WorkspaceId {
        let monitor_workspaces = self
            .settings
            .monitor_name
            .as_ref()
            .map(|m| monitor_workspaces_sorted(m, &self.workspace_monitor_rules))
            .unwrap_or_default();
        compute_display_id(
            id,
            numbering,
            self.settings.monitor_name.as_deref(),
            &monitor_workspaces,
        )
    }

    fn initial_focused_monitor(hyprland: &Option<Arc<HyprlandService>>) -> Option<String> {
        let hyprland = hyprland.as_ref()?;
        hyprland
            .monitors
            .get()
            .into_iter()
            .find(|monitor| monitor.focused.get())
            .map(|monitor| monitor.name.get())
    }

    fn initial_active_workspace(
        hyprland: &Option<Arc<HyprlandService>>,
        settings: &BarSettings,
        monitor_specific: bool,
    ) -> WorkspaceId {
        let Some(hyprland) = hyprland else {
            return 1;
        };

        if monitor_specific && let Some(bar_monitor) = &settings.monitor_name {
            let monitors = hyprland.monitors.get();
            if let Some(monitor) = monitors.iter().find(|m| &m.name.get() == bar_monitor) {
                return monitor.active_workspace.get().id;
            }
        }

        let runtime = tokio::runtime::Handle::current();
        match runtime.block_on(hyprland.active_workspace()) {
            Some(ws) => ws.id.get(),
            None => 1,
        }
    }

    fn should_apply_workspace_event(&self) -> bool {
        let Some(bar_monitor) = self.settings.monitor_name.as_ref() else {
            return true;
        };

        if let Some(focused_monitor) = self.focused_monitor.as_ref() {
            return focused_monitor == bar_monitor;
        }

        let Some(hyprland) = &self.hyprland else {
            return true;
        };

        hyprland
            .monitors
            .get()
            .into_iter()
            .find(|monitor| monitor.focused.get())
            .map(|monitor| monitor.name.get() == bar_monitor.as_str())
            .unwrap_or(true)
    }

    fn should_apply_active_workspace_change(
        &self,
        workspace_id: WorkspaceId,
        monitor_specific: bool,
    ) -> bool {
        if !monitor_specific {
            return true;
        }

        let Some(bar_monitor) = self.settings.monitor_name.as_ref() else {
            return self.should_apply_workspace_event();
        };

        match self.workspace_monitor_name(workspace_id) {
            Some(ws_monitor) => ws_monitor == *bar_monitor,
            None => self.should_apply_workspace_event(),
        }
    }

    fn rebuild_buttons(&mut self) {
        debug!(
            bar_monitor = ?self.settings.monitor_name,
            active_workspace = self.active_workspace_id,
            "rebuild_buttons called"
        );

        let Some(hyprland) = &self.hyprland else {
            warn!(
                module = "hyprland-workspaces",
                "HyprlandService unavailable"
            );
            return;
        };

        let config = self.config.config();
        let ws_config = &config.modules.hyprland_workspaces;
        let is_vertical = self.is_vertical();

        self.update_border_classes(ws_config.border_show.get());

        let workspaces = self.filtered_workspaces(hyprland, ws_config);
        let clients = hyprland.clients.get();

        let numbering = ws_config.numbering.get();
        let button_inits: Vec<_> = workspaces
            .iter()
            .map(|ws| {
                let ctx = ButtonBuildContext {
                    id: ws.id,
                    display_id: self.display_id(ws.id, numbering),
                    name: &ws.name,
                    windows: ws.windows,
                    is_active: ws.id == self.active_workspace_id,
                    is_urgent: ws_config.urgent_show.get()
                        && self.workspace_has_urgent_window(ws.id, hyprland),
                    is_vertical,
                };
                build_button_init(&ctx, ws_config, &clients)
            })
            .collect();

        let mut guard = self.buttons.guard();
        guard.clear();
        for init in button_inits {
            guard.push_back(init);
        }
    }

    fn filtered_workspaces(
        &self,
        hyprland: &Arc<HyprlandService>,
        config: &HyprlandWorkspacesConfig,
    ) -> Vec<FilteredWorkspace> {
        let all_workspaces = hyprland.workspaces.get();
        let ignore_patterns = config.workspace_ignore.get();

        let workspace_data: Vec<WorkspaceData> = all_workspaces
            .iter()
            .map(|ws| WorkspaceData {
                id: ws.id.get(),
                name: ws.name.get(),
                windows: ws.windows.get(),
                monitor: ws.monitor.get(),
            })
            .collect();

        let ctx = FilterContext {
            show_special: config.show_special.get(),
            monitor_specific: config.monitor_specific.get(),
            min_workspace_count: usize::from(config.min_workspace_count.get()),
            active_workspace_id: self.active_workspace_id,
            bar_monitor: self.settings.monitor_name.as_deref(),
            ignore_patterns: &ignore_patterns,
            workspace_monitor_rules: &self.workspace_monitor_rules,
        };

        filter_workspaces(&workspace_data, &ctx)
    }

    fn update_active_state(&mut self) {
        let Some(hyprland) = &self.hyprland else {
            return;
        };

        let config = self.config.config();
        let ws_config = &config.modules.hyprland_workspaces;

        for idx in 0..self.buttons.len() {
            let Some(button) = self.buttons.get(idx) else {
                continue;
            };
            let button_id = button.id();
            let is_urgent = ws_config.urgent_show.get()
                && self.workspace_has_urgent_window(button_id, hyprland);

            self.buttons.send(
                idx,
                WorkspaceButtonInput::UpdateState {
                    windows: self.window_count_for_workspace(button_id, hyprland),
                    is_active: button_id == self.active_workspace_id,
                    is_urgent,
                },
            );
        }
    }

    fn sync_after_active_workspace_change(&mut self, has_min_workspace_count: bool) {
        if has_min_workspace_count {
            self.rebuild_buttons();
            return;
        }

        self.update_active_state();
    }

    fn update_app_icons_on_title_change(&mut self) {
        let config = self.config.config();
        let ws_config = &config.modules.hyprland_workspaces;

        if !ws_config.app_icons_show.get() {
            return;
        }

        if !helpers::has_title_patterns(&ws_config.app_icon_map.get()) {
            return;
        }

        self.rebuild_buttons();
    }

    fn switch_to_workspace(&self, id: WorkspaceId) {
        let Some(hyprland) = &self.hyprland else {
            return;
        };

        let hyprland = hyprland.clone();
        tokio::spawn(async move {
            let command = format!("workspace {}", id);
            if let Err(e) = hyprland.dispatch(&command).await {
                error!(error = %e, workspace = id, "Failed to switch workspace");
            }
        });
    }

    fn navigate_workspace(&self, direction: i64) {
        let Some(hyprland) = &self.hyprland else {
            return;
        };

        let config = self.config.config();
        let ws_config = &config.modules.hyprland_workspaces;
        let workspaces = self.filtered_workspaces(hyprland, ws_config);

        if workspaces.is_empty() {
            return;
        }

        let current_idx = workspaces
            .iter()
            .position(|ws| ws.id == self.active_workspace_id)
            .unwrap_or(0);

        let new_idx = calculate_navigation_index(current_idx, direction, workspaces.len());

        if let Some(ws) = workspaces.get(new_idx) {
            self.switch_to_workspace(ws.id);
        }
    }

    fn workspace_has_urgent_window(
        &self,
        workspace_id: WorkspaceId,
        hyprland: &Arc<HyprlandService>,
    ) -> bool {
        let clients = hyprland.clients.get();
        let client_workspaces: Vec<_> = clients
            .iter()
            .map(|c| (c.address.get(), c.workspace.get().id))
            .collect();
        workspace_contains_urgent_address(workspace_id, &self.urgent_windows, &client_workspaces)
    }

    fn window_count_for_workspace(
        &self,
        workspace_id: WorkspaceId,
        hyprland: &Arc<HyprlandService>,
    ) -> u16 {
        hyprland
            .workspaces
            .get()
            .iter()
            .find(|ws| ws.id.get() == workspace_id)
            .map(|ws| ws.windows.get())
            .unwrap_or(0)
    }

    fn clear_urgent_windows_for_workspace(&mut self, workspace_id: WorkspaceId) {
        let Some(hyprland) = &self.hyprland else {
            return;
        };

        let clients = hyprland.clients.get();
        let client_workspaces: Vec<_> = clients
            .iter()
            .map(|c| (c.address.get(), c.workspace.get().id))
            .collect();
        let to_clear = addresses_in_workspace(workspace_id, &client_workspaces);
        for address in to_clear {
            self.urgent_windows.remove(&address);
        }
    }

    fn prune_stale_urgent_windows(&mut self) {
        if self.urgent_windows.is_empty() {
            return;
        }

        let Some(hyprland) = &self.hyprland else {
            return;
        };

        let clients = hyprland.clients.get();
        let current_addresses: HashSet<Address> =
            clients.iter().map(|client| client.address.get()).collect();

        self.urgent_windows = prune_stale_addresses(&self.urgent_windows, &current_addresses);
    }

    fn update_border_classes(&self, show_border: bool) {
        let container = self.buttons.widget();
        for location in [
            BorderLocation::Top,
            BorderLocation::Bottom,
            BorderLocation::Left,
            BorderLocation::Right,
            BorderLocation::All,
        ] {
            if let Some(class) = location.css_class() {
                container.remove_css_class(class);
            }
        }

        if show_border && let Some(class) = self.settings.border_location.get().css_class() {
            container.add_css_class(class);
        }
    }
}

impl Drop for HyprlandWorkspaces {
    fn drop(&mut self) {
        gtk::style_context_remove_provider_for_display(
            &self.buttons.widget().display(),
            &self.css_provider,
        );
    }
}
