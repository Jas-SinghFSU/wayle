mod dropdowns;
mod factory;
pub(crate) mod icons;
mod layout;
mod modules;
mod styling;
mod watchers;

use std::{cell::Cell, rc::Rc};

use factory::*;
use gtk::prelude::*;
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};
use relm4::{
    factory::FactoryVecDeque,
    gtk,
    gtk::{gdk, glib},
    prelude::*,
};
use wayle_common::ConfigProperty;
use wayle_config::schemas::bar::{BarItem, BarLayout};
use wayle_widgets::{prelude::BarSettings, styling::InlineStyling};

use self::dropdowns::DropdownRegistry;
use crate::shell::services::ShellServices;

pub(crate) struct Bar {
    settings: BarSettings,
    services: ShellServices,
    dropdowns: Rc<DropdownRegistry>,
    layout: BarLayout,
    css_provider: gtk::CssProvider,
    last_css: String,

    left: FactoryVecDeque<BarItemFactory>,
    center: FactoryVecDeque<BarItemFactory>,
    right: FactoryVecDeque<BarItemFactory>,
}

pub(crate) struct BarInit {
    pub(crate) monitor: gdk::Monitor,
    pub(crate) services: ShellServices,
}

#[derive(Debug)]
pub(crate) enum BarCmd {
    LayoutLoaded(BarLayout),
    StyleChanged,
    DropdownAutohideChanged(bool),
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
        let config = init.services.config.config();
        let location = config.bar.location.get();
        let inset_edge = config.bar.inset_edge.get().value();
        let inset_ends = config.bar.inset_ends.get().value();
        let is_floating = inset_edge > 0.0 || inset_ends > 0.0;

        let monitor_name = init.monitor.connector().map(|s| s.to_string());

        let settings = BarSettings {
            variant: config.bar.button_variant.clone(),
            theme_provider: config.styling.theme_provider.clone(),
            border_location: config.bar.button_border_location.clone(),
            border_width: config.bar.button_border_width.clone(),
            icon_position: config.bar.button_icon_position.clone(),
            is_vertical: ConfigProperty::new(location.is_vertical()),
            scroll_sensitivity: 1.0,
            monitor_name,
        };

        root.init_layer_shell();
        root.set_layer(Layer::Top);
        root.set_keyboard_mode(KeyboardMode::OnDemand);
        root.set_monitor(Some(&init.monitor));
        Self::apply_anchors(&root, location);
        Self::apply_css_classes(&root, &init.monitor, location, is_floating);
        Self::start_exclusive_zone_tracker(&root);
        Self::suppress_alt_focus(&root);

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

        watchers::layout::spawn(&sender, &init.monitor, &init.services.config);
        watchers::dropdowns::spawn(&sender, &init.services.config);

        let dropdowns = Rc::new(DropdownRegistry::new(&init.services));

        let mut model = Self {
            settings,
            services: init.services,
            dropdowns,
            layout: BarLayout {
                monitor: String::new(),
                extends: None,
                left: Vec::new(),
                center: Vec::new(),
                right: Vec::new(),
            },
            css_provider,
            last_css: String::new(),
            left,
            center,
            right,
        };

        model.spawn_style_watcher(&sender);
        model.last_css = model.build_css();
        model.css_provider.load_from_string(&model.last_css);

        let widgets = view_output!();

        let is_vert = model.settings.is_vertical.get();
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

    fn update_cmd(&mut self, msg: BarCmd, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            BarCmd::LayoutLoaded(layout) => {
                self.apply_layout(layout);
            }
            BarCmd::StyleChanged => {
                let new_css = self.build_css();
                if new_css != self.last_css {
                    self.css_provider.load_from_string(&new_css);
                    self.last_css = new_css;
                }
            }
            BarCmd::DropdownAutohideChanged(autohide) => {
                self.dropdowns.set_all_autohide(autohide);
            }
        }
    }
}

impl Bar {
    /// Tracks the bar's thickness each frame and sets the exclusive zone
    /// only when it actually changes. This decouples the exclusive zone from
    /// transient `set_default_size` pokes, preventing compositor flicker.
    fn start_exclusive_zone_tracker(window: &gtk::Window) {
        let last = Rc::new(Cell::new(0i32));
        window.add_tick_callback(move |window, _| {
            let is_vert = window.has_css_class("left") || window.has_css_class("right");
            let thickness = if is_vert {
                window.width()
            } else {
                window.height()
            };
            if thickness > 1 && thickness != last.get() {
                last.set(thickness);
                window.set_exclusive_zone(thickness);
            }
            glib::ControlFlow::Continue
        });
    }

    fn suppress_alt_focus(window: &gtk::Window) {
        use gtk::prelude::GtkWindowExt;
        window.connect_focus_visible_notify(|window| {
            if window.gets_focus_visible() {
                window.set_focus_visible(false);
            }
        });
        window.connect_mnemonics_visible_notify(|window| {
            if window.is_mnemonics_visible() {
                window.set_mnemonics_visible(false);
            }
        });
    }

    fn apply_layout(&mut self, new_layout: BarLayout) {
        if self.layout == new_layout {
            return;
        }

        let settings = &self.settings;
        let services = &self.services;
        let dropdowns = &self.dropdowns;

        if self.layout.left != new_layout.left {
            Self::rebuild_section(
                &mut self.left,
                &new_layout.left,
                settings,
                services,
                dropdowns,
            );
        }
        if self.layout.center != new_layout.center {
            Self::rebuild_section(
                &mut self.center,
                &new_layout.center,
                settings,
                services,
                dropdowns,
            );
        }
        if self.layout.right != new_layout.right {
            Self::rebuild_section(
                &mut self.right,
                &new_layout.right,
                settings,
                services,
                dropdowns,
            );
        }

        self.layout = new_layout;
    }

    fn rebuild_section(
        factory: &mut FactoryVecDeque<BarItemFactory>,
        items: &[BarItem],
        settings: &BarSettings,
        services: &ShellServices,
        dropdowns: &Rc<DropdownRegistry>,
    ) {
        let mut guard = factory.guard();
        guard.clear();

        for item in items {
            guard.push_back(BarItemFactoryInit {
                item: item.clone(),
                settings: settings.clone(),
                services: services.clone(),
                dropdowns: dropdowns.clone(),
            });
        }
    }
}
