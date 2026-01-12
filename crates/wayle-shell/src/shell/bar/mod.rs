mod modules;

use gdk4::prelude::MonitorExt;
use gtk4::prelude::GtkWindowExt;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use relm4::{gtk::gdk, prelude::*};

/// Initialization data for a Bar component.
pub struct BarInit {
    /// The monitor this bar should appear on.
    pub monitor: gdk::Monitor,
}

/// A single status bar bound to a specific monitor.
pub struct Bar {
    #[allow(dead_code)]
    monitor: gdk::Monitor,
}

/// Input messages for Bar.
#[derive(Debug)]
pub enum BarInput {}

#[relm4::component(pub)]
impl Component for Bar {
    type Init = BarInit;
    type Input = BarInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Window {
            set_decorated: false,

            gtk::CenterBox {
                #[wrap(Some)]
                set_center_widget = &gtk::Label {
                    set_label: "Wayle",
                },
            }
        }
    }

    fn init(
        params: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let connector = params
            .monitor
            .connector()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        tracing::info!("Creating bar for monitor: {}", connector);

        root.init_layer_shell();
        tracing::info!("Layer shell initialized: {}", root.is_layer_window());

        root.set_layer(Layer::Top);
        root.set_monitor(Some(&params.monitor));
        root.set_anchor(Edge::Top, true);
        root.set_anchor(Edge::Left, true);
        root.set_anchor(Edge::Right, true);
        root.auto_exclusive_zone_enable();
        root.set_default_size(-1, 32);

        let model = Self {
            monitor: params.monitor,
        };
        let widgets = view_output!();

        root.present();
        tracing::info!("Bar created and presented for {}", connector);

        ComponentParts { model, widgets }
    }
}
