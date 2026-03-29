mod compositing;

use std::{
    cell::RefCell,
    fmt,
    rc::Rc,
    sync::{mpsc, Arc},
    time::Duration,
};

use gtk::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use relm4::{gtk, prelude::*};
use wayle_capture::{CaptureClient, CaptureCommand, CaptureRequest, CaptureResult};
use wayle_hyprland::{HyprlandService, WorkspaceId};
use wayle_widgets::prelude::BarSettings;

use self::compositing::ClickRegion;

// ---------------------------------------------------------------------------
// Public messages
// ---------------------------------------------------------------------------

/// Input messages for the preview popup.
pub(crate) enum WorkspacePreviewMsg {
    /// Show preview for a workspace.
    Show {
        ws_id: WorkspaceId,
        hyprland: Option<Arc<HyprlandService>>,
        settings: Box<BarSettings>,
        /// Margin from the top edge of the monitor (px).
        margin_top: i32,
        /// Margin from the left edge of the monitor (px).
        margin_left: i32,
        /// Current preview width from config (may change via hot-reload).
        preview_width: u32,
        /// Current close delay from config (may change via hot-reload).
        close_delay_ms: u32,
    },
    /// Start close timer (mouse left trigger area).
    Dismiss,
    /// A capture result is ready to composite.
    CaptureReady(CaptureResult),
    /// User clicked a window in the composite thumbnail.
    ThumbnailClicked(String),
    /// User clicked a window label in the list.
    LabelClicked(String),
    /// Mouse entered the popup window itself.
    PopupHoverEnter,
    /// Mouse left the popup window.
    PopupHoverLeave,
    /// The close timer has fired.
    CloseTimerFired,
}

impl fmt::Debug for WorkspacePreviewMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Show { ws_id, .. } => f.debug_struct("Show").field("ws_id", ws_id).finish(),
            Self::Dismiss => write!(f, "Dismiss"),
            Self::CaptureReady(_) => write!(f, "CaptureReady(..)"),
            Self::ThumbnailClicked(a) => write!(f, "ThumbnailClicked({a})"),
            Self::LabelClicked(a) => write!(f, "LabelClicked({a})"),
            Self::PopupHoverEnter => write!(f, "PopupHoverEnter"),
            Self::PopupHoverLeave => write!(f, "PopupHoverLeave"),
            Self::CloseTimerFired => write!(f, "CloseTimerFired"),
        }
    }
}

/// Output messages sent to the parent `HyprlandWorkspaces`.
#[derive(Debug)]
pub(crate) enum WorkspacePreviewOutput {
    /// Request the parent to focus a window by address.
    FocusWindow(String),
}

// ---------------------------------------------------------------------------
// Init
// ---------------------------------------------------------------------------

pub(crate) struct WorkspacePreviewInit {
    pub preview_width: u32,
    pub close_delay_ms: u32,
    pub monitor_connector: Option<String>,
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

pub(crate) struct WorkspacePreview {
    preview_width: f64,
    close_delay_ms: u32,
    hovered_ws: Option<WorkspaceId>,
    /// Monotonically increasing session counter. Incremented on each `Show`,
    /// carried through capture requests/results so stale frames from a
    /// previous session can be discarded.
    session: u64,
    close_timer: Option<glib::SourceId>,
    click_regions: Rc<RefCell<Vec<ClickRegion>>>,
    popup_items: Vec<(String, gtk::Button)>,
    /// Shared with the thumbnail hover highlight closure.
    highlight_items: Rc<RefCell<Vec<(String, gtk::Button)>>>,
    capture_tx: mpsc::Sender<CaptureCommand>,
    labels_box: gtk::Box,
    preview_picture: gtk::Picture,
}

#[relm4::component(pub(crate))]
impl Component for WorkspacePreview {
    type Init = WorkspacePreviewInit;
    type Input = WorkspacePreviewMsg;
    type Output = WorkspacePreviewOutput;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Window {
            set_decorated: false,
            add_css_class: "ws-preview-popup",
            set_default_size: (1, 1),
            set_visible: false,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 2,

                #[local_ref]
                preview_picture -> gtk::Picture {
                    add_css_class: "ws-preview-canvas",
                    set_can_shrink: true,
                    set_visible: false,
                },

                #[local_ref]
                labels_box -> gtk::Box {
                    add_css_class: "ws-preview-labels",
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 2,
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Layer shell setup.
        root.init_layer_shell();
        root.set_layer(Layer::Overlay);
        root.set_exclusive_zone(-1);
        root.set_keyboard_mode(KeyboardMode::None);
        root.set_namespace(Some("wayle-workspace-preview"));

        // Bind to the same monitor as the bar.
        if let Some(connector) = &init.monitor_connector {
            crate::shell::helpers::layer_shell::apply_monitor_by_connector(&root, connector);
        } else {
            crate::shell::helpers::layer_shell::apply_primary_monitor(&root);
        }

        // Spawn capture thread.
        let (capture_tx, capture_rx) = wayle_capture::spawn_capture_thread();

        let click_regions: Rc<RefCell<Vec<ClickRegion>>> = Rc::new(RefCell::new(Vec::new()));

        let preview_picture = gtk::Picture::new();
        let labels_box = gtk::Box::default();

        // Bridge capture results into the glib main loop on demand.
        // A helper thread blocks on the std mpsc receiver and forwards each
        // result through a futures channel. A glib-spawned future awaits
        // results and emits them as component messages — no fixed-rate polling.
        let (bridge_tx, mut bridge_rx) = futures::channel::mpsc::unbounded();
        std::thread::Builder::new()
            .name("wayle-capture-bridge".into())
            .spawn(move || {
                while let Ok(result) = capture_rx.recv() {
                    if bridge_tx.unbounded_send(result).is_err() {
                        return; // receiver dropped
                    }
                }
            })
            .ok();
        let bridge_sender = sender.input_sender().clone();
        glib::spawn_future_local(async move {
            use futures::StreamExt;
            while let Some(result) = bridge_rx.next().await {
                bridge_sender.emit(WorkspacePreviewMsg::CaptureReady(result));
            }
        });

        // Set up click/hover controllers on the preview picture and popup.
        let highlight_items: Rc<RefCell<Vec<(String, gtk::Button)>>> =
            Rc::new(RefCell::new(Vec::new()));
        Self::attach_thumbnail_controllers(
            &preview_picture,
            &click_regions,
            &highlight_items,
            &sender,
        );
        Self::attach_popup_hover_controller(&root, &sender);

        let model = Self {
            preview_width: f64::from(init.preview_width),
            close_delay_ms: init.close_delay_ms,
            hovered_ws: None,
            session: 0,
            close_timer: None,
            click_regions,
            popup_items: Vec::new(),
            highlight_items,
            capture_tx,
            labels_box: labels_box.clone(),
            preview_picture: preview_picture.clone(),
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
            WorkspacePreviewMsg::Show {
                ws_id,
                hyprland,
                settings,
                margin_top,
                margin_left,
                preview_width,
                close_delay_ms,
            } => {
                self.cancel_close_timer();
                self.preview_width = f64::from(preview_width);
                self.close_delay_ms = close_delay_ms;
                self.show_for_workspace(
                    ws_id,
                    hyprland.as_deref(),
                    &settings,
                    root,
                    &sender,
                    margin_top,
                    margin_left,
                );
            }
            WorkspacePreviewMsg::Dismiss => {
                self.start_close_timer(&sender, root);
            }
            WorkspacePreviewMsg::CaptureReady(result) => {
                if result.session == self.session && self.hovered_ws == Some(result.ws_id) {
                    compositing::apply_capture_result(
                        &self.preview_picture,
                        &result,
                        &self.click_regions,
                        self.preview_width,
                    );
                }
            }
            WorkspacePreviewMsg::ThumbnailClicked(address)
            | WorkspacePreviewMsg::LabelClicked(address) => {
                self.cancel_close_timer();
                self.stop_streaming();
                root.set_visible(false);
                self.hovered_ws = None;
                sender
                    .output(WorkspacePreviewOutput::FocusWindow(address))
                    .ok();
            }
            WorkspacePreviewMsg::PopupHoverEnter => {
                self.cancel_close_timer();
            }
            WorkspacePreviewMsg::PopupHoverLeave => {
                self.start_close_timer(&sender, root);
            }
            WorkspacePreviewMsg::CloseTimerFired => {
                self.close_timer = None;
                self.stop_streaming();
                root.set_visible(false);
                self.hovered_ws = None;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Private methods
// ---------------------------------------------------------------------------

impl WorkspacePreview {
    fn attach_thumbnail_controllers(
        picture: &gtk::Picture,
        click_regions: &Rc<RefCell<Vec<ClickRegion>>>,
        highlight_items: &Rc<RefCell<Vec<(String, gtk::Button)>>>,
        sender: &ComponentSender<Self>,
    ) {
        // Click on thumbnail → focus window.
        let click = gtk::GestureClick::new();
        let regions_ref = click_regions.clone();
        let click_sender = sender.input_sender().clone();
        click.connect_released(move |_, _, x, y| {
            let regions = regions_ref.borrow();
            for region in regions.iter() {
                if x >= region.x
                    && x < region.x + region.w
                    && y >= region.y
                    && y < region.y + region.h
                {
                    click_sender.emit(WorkspacePreviewMsg::ThumbnailClicked(
                        region.address.clone(),
                    ));
                    break;
                }
            }
        });
        picture.add_controller(click);

        // Hover on thumbnail → highlight corresponding label.
        let motion = gtk::EventControllerMotion::new();
        let regions_ref = click_regions.clone();
        let items_ref = highlight_items.clone();
        motion.connect_motion(move |_, x, y| {
            let regions = regions_ref.borrow();
            let mut matched_addr: Option<&str> = None;
            for region in regions.iter() {
                if x >= region.x
                    && x < region.x + region.w
                    && y >= region.y
                    && y < region.y + region.h
                {
                    matched_addr = Some(&region.address);
                    break;
                }
            }
            let items = items_ref.borrow();
            for (addr, btn) in items.iter() {
                if matched_addr == Some(addr.as_str()) {
                    btn.add_css_class("preview-highlight");
                } else {
                    btn.remove_css_class("preview-highlight");
                }
            }
        });
        let items_ref = highlight_items.clone();
        motion.connect_leave(move |_| {
            let items = items_ref.borrow();
            for (_, btn) in items.iter() {
                btn.remove_css_class("preview-highlight");
            }
        });
        picture.add_controller(motion);
    }

    fn attach_popup_hover_controller(root: &gtk::Window, sender: &ComponentSender<Self>) {
        let motion = gtk::EventControllerMotion::new();
        let enter_sender = sender.input_sender().clone();
        motion.connect_enter(move |_, _, _| {
            enter_sender.emit(WorkspacePreviewMsg::PopupHoverEnter);
        });
        let leave_sender = sender.input_sender().clone();
        motion.connect_leave(move |_| {
            leave_sender.emit(WorkspacePreviewMsg::PopupHoverLeave);
        });
        root.add_controller(motion);
    }

    fn stop_streaming(&self) {
        let _ = self.capture_tx.send(CaptureCommand::StopStreaming);
    }

    fn cancel_close_timer(&mut self) {
        if let Some(id) = self.close_timer.take() {
            id.remove();
        }
    }

    fn start_close_timer(&mut self, sender: &ComponentSender<Self>, _root: &gtk::Window) {
        self.cancel_close_timer();
        let timer_sender = sender.input_sender().clone();
        let delay = self.close_delay_ms;
        let id = glib::timeout_add_local_once(Duration::from_millis(u64::from(delay)), move || {
            timer_sender.emit(WorkspacePreviewMsg::CloseTimerFired);
        });
        self.close_timer = Some(id);
    }

    #[allow(clippy::too_many_arguments)]
    fn show_for_workspace(
        &mut self,
        ws_id: WorkspaceId,
        hyprland: Option<&HyprlandService>,
        _settings: &BarSettings,
        root: &gtk::Window,
        sender: &ComponentSender<Self>,
        margin_top: i32,
        margin_left: i32,
    ) {
        self.session = self.session.wrapping_add(1);
        self.hovered_ws = Some(ws_id);

        // Clear previous labels.
        while let Some(child) = self.labels_box.first_child() {
            self.labels_box.remove(&child);
        }
        self.popup_items.clear();
        self.highlight_items.borrow_mut().clear();

        // Hide preview until capture arrives.
        self.preview_picture
            .set_paintable(None::<&gdk4::MemoryTexture>);
        self.preview_picture.set_visible(false);

        let Some(hyprland) = hyprland else {
            self.show_empty_label();
            root.set_visible(true);
            return;
        };

        // Gather clients in this workspace.
        let clients = hyprland.clients.get();
        let ws_clients: Vec<_> = clients
            .iter()
            .filter(|c| {
                let ws = c.workspace.get();
                let sz = c.size.get();
                ws.id == ws_id && c.mapped.get() && sz.width > 0 && sz.height > 0
            })
            .collect();

        if ws_clients.is_empty() {
            self.show_empty_label();
            root.set_visible(true);
            return;
        }

        // Build label buttons for each client.
        for client in &ws_clients {
            let class = client.class.get();
            let title = client.title.get();
            let text = if title.is_empty() {
                class.clone()
            } else {
                format!("{class}: {}", truncate_title(&title, 40))
            };

            let btn = gtk::Button::new();
            btn.add_css_class("ws-popup-item");
            let addr_str = client.address.get().to_string();
            btn.set_widget_name(&format!("ws-popup-addr-{addr_str}"));

            let label = gtk::Label::new(Some(&text));
            label.set_halign(gtk::Align::Start);
            btn.set_child(Some(&label));

            let btn_sender = sender.input_sender().clone();
            let addr_clone = addr_str.clone();
            btn.connect_clicked(move |_| {
                btn_sender.emit(WorkspacePreviewMsg::LabelClicked(addr_clone.clone()));
            });

            self.popup_items.push((addr_str, btn.clone()));
            self.labels_box.append(&btn);
        }

        // Sync highlight items for the thumbnail hover closure.
        *self.highlight_items.borrow_mut() = self.popup_items.clone();

        // Gather monitor info for the capture request.
        let monitors = hyprland.monitors.get();
        let monitor = monitors.iter().find(|m| {
            _settings
                .monitor_name
                .as_deref()
                .is_some_and(|name| m.name.get() == name)
        });

        if let Some(monitor) = monitor {
            let mon_x = monitor.x.get();
            let mon_y = monitor.y.get();
            let scale = monitor.scale.get() as f64;
            let mon_w = (f64::from(monitor.width.get()) / scale) as u32;
            let mon_h = (f64::from(monitor.height.get()) / scale) as u32;

            let capture_clients: Vec<CaptureClient> = ws_clients
                .iter()
                .map(|c| {
                    let addr = c.address.get().to_string();
                    let loc = c.at.get();
                    let sz = c.size.get();
                    CaptureClient {
                        address: addr,
                        x: loc.x - mon_x,
                        y: loc.y - mon_y,
                        width: sz.width,
                        height: sz.height,
                    }
                })
                .collect();

            let _ = self.capture_tx.send(CaptureCommand::StartStreaming(
                CaptureRequest {
                    session: self.session,
                    ws_id,
                    monitor_width: mon_w,
                    monitor_height: mon_h,
                    clients: capture_clients,
                },
            ));
        }

        // Position popup below the hovered button.
        Self::position_popup(root, margin_top, margin_left);
        root.set_visible(true);
    }

    fn show_empty_label(&self) {
        let label = gtk::Label::new(Some("(empty)"));
        label.add_css_class("ws-popup-item");
        label.add_css_class("dim");
        label.set_halign(gtk::Align::Start);
        self.labels_box.append(&label);
    }

    fn position_popup(popup: &gtk::Window, margin_top: i32, margin_left: i32) {
        popup.set_anchor(Edge::Top, true);
        popup.set_anchor(Edge::Left, true);
        popup.set_anchor(Edge::Bottom, false);
        popup.set_anchor(Edge::Right, false);
        popup.set_margin(Edge::Top, margin_top);
        popup.set_margin(Edge::Left, margin_left);
    }
}

impl Drop for WorkspacePreview {
    fn drop(&mut self) {
        self.cancel_close_timer();
        // The bridge thread and future shut down when capture_tx is dropped
        // (capture thread exits → bridge thread exits → future completes).
    }
}

fn truncate_title(title: &str, max_len: usize) -> String {
    let char_count = title.chars().count();
    if char_count <= max_len {
        return title.to_string();
    }
    let end: usize = title
        .char_indices()
        .nth(max_len)
        .map(|(i, _)| i)
        .unwrap_or(title.len());
    format!("{}...", &title[..end])
}
