mod factory;
mod layout;
mod modules;
mod styling;
mod watchers;

use factory::*;
use gtk::prelude::*;
use gtk4_layer_shell::{Layer, LayerShell};
use relm4::{factory::FactoryVecDeque, gtk, gtk::gdk, prelude::*};
use wayle_common::{ConfigProperty, services};
use wayle_config::{
    ConfigService,
    schemas::bar::{BarItem, BarLayout, Location},
};
use wayle_widgets::styling::InlineStyling;

pub(crate) struct Bar {
    location: Location,
    is_vertical: ConfigProperty<bool>,
    layout: BarLayout,
    css_provider: gtk::CssProvider,

    left: FactoryVecDeque<BarItemFactory>,
    center: FactoryVecDeque<BarItemFactory>,
    right: FactoryVecDeque<BarItemFactory>,
}

#[derive(Debug)]
pub(crate) struct BarInit {
    pub(crate) monitor: gdk::Monitor,
}

#[derive(Debug)]
pub(crate) enum BarCmd {
    LayoutLoaded(BarLayout),
    StyleChanged,
    LocationChanged(Location),
}

#[relm4::component(pub(crate))]
impl Component for Bar {
    type Init = BarInit;
    type Input = ();
    type Output = ();
    type CommandOutput = BarCmd;

    view! {
        #[root]
        gtk::Window {
            set_decorated: false,
            add_css_class: "bar",
            set_size_request: (1, 1),

            #[name = "center_box"]
            gtk::CenterBox {
                #[wrap(Some)]
                #[name = "left_box"]
                set_start_widget = &gtk::Box {
                    add_css_class: "bar-section",
                    add_css_class: "bar-left",
                },

                #[wrap(Some)]
                #[name = "middle_box"]
                set_center_widget = &gtk::Box {
                    add_css_class: "bar-section",
                    add_css_class: "bar-center",
                },

                #[wrap(Some)]
                #[name = "right_box"]
                set_end_widget = &gtk::Box {
                    add_css_class: "bar-section",
                    add_css_class: "bar-right",
                },
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let config_service = services::get::<ConfigService>();
        let config = config_service.config();
        let location = config.bar.location.get();
        let inset_edge = config.bar.inset_edge.get().value();
        let inset_ends = config.bar.inset_ends.get().value();
        let is_floating = inset_edge > 0.0 || inset_ends > 0.0;

        let is_vertical = ConfigProperty::new(location.is_vertical());

        root.init_layer_shell();
        root.set_layer(Layer::Top);
        root.set_monitor(Some(&init.monitor));
        root.auto_exclusive_zone_enable();
        Self::apply_anchors(&root, location);
        Self::apply_css_classes(&root, &init.monitor, location, is_floating);

        let window = root.clone();
        init.monitor.connect_invalidate(move |_| {
            window.destroy();
        });

        let left = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .detach();

        let center = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .detach();

        let right = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .detach();

        let css_provider = gtk::CssProvider::new();

        #[allow(deprecated)]
        root.style_context()
            .add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

        watchers::layout::spawn(&sender, &init.monitor);
        watchers::location::spawn(&sender);
        watchers::style::spawn(&sender);

        let model = Self {
            location,
            is_vertical,
            layout: BarLayout {
                monitor: String::new(),
                extends: None,
                left: Vec::new(),
                center: Vec::new(),
                right: Vec::new(),
            },
            css_provider,
            left,
            center,
            right,
        };

        model.reload_css();

        let widgets = view_output!();

        let is_vert = model.is_vertical.get();
        Self::apply_orientations(
            &widgets.center_box,
            &widgets.left_box,
            &widgets.middle_box,
            &widgets.right_box,
            model.left.widget(),
            model.center.widget(),
            model.right.widget(),
            is_vert,
        );

        widgets.left_box.append(model.left.widget());
        widgets.middle_box.append(model.center.widget());
        widgets.right_box.append(model.right.widget());

        root.present();

        ComponentParts { model, widgets }
    }

    fn update_cmd(&mut self, msg: BarCmd, _sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
            BarCmd::LayoutLoaded(layout) => {
                self.apply_layout(layout);
            }
            BarCmd::StyleChanged => {
                self.reload_css();
            }
            BarCmd::LocationChanged(location) => {
                self.apply_location_change(root, location);
            }
        }
    }
}

impl Bar {
    fn apply_layout(&mut self, new_layout: BarLayout) {
        if self.layout == new_layout {
            return;
        }

        let is_vertical = self.is_vertical.clone();

        if self.layout.left != new_layout.left {
            Self::rebuild_section(&mut self.left, &new_layout.left, &is_vertical);
        }
        if self.layout.center != new_layout.center {
            Self::rebuild_section(&mut self.center, &new_layout.center, &is_vertical);
        }
        if self.layout.right != new_layout.right {
            Self::rebuild_section(&mut self.right, &new_layout.right, &is_vertical);
        }

        self.layout = new_layout;
    }

    fn rebuild_section(
        factory: &mut FactoryVecDeque<BarItemFactory>,
        items: &[BarItem],
        is_vertical: &ConfigProperty<bool>,
    ) {
        let mut guard = factory.guard();
        guard.clear();

        for item in items {
            guard.push_back(BarItemFactoryInit {
                item: item.clone(),
                is_vertical: is_vertical.clone(),
            });
        }
    }
}
