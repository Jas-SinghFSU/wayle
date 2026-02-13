mod factory;
mod helpers;
mod messages;
mod rendering;
mod watchers;

use std::{cell::Cell, rc::Rc, sync::Arc};

use gtk::prelude::*;
use relm4::prelude::*;
use tracing::{error, info};
use wayle_cava::CavaService;
use wayle_common::{ConfigProperty, WatcherToken};
use wayle_config::{ConfigService, schemas::modules::CavaStyle};
use wayle_widgets::prelude::{
    BarContainer, BarContainerBehavior, BarContainerColors, BarContainerInit,
};

pub(crate) use self::{
    factory::Factory,
    messages::{CavaCmd, CavaInit},
};

/// Audio frequency visualizer rendered via cairo on a `DrawingArea`.
pub(crate) struct CavaModule {
    container: Controller<BarContainer>,
    drawing_area: gtk::DrawingArea,
    frame_data: Rc<Cell<Vec<f64>>>,
    frame_watcher: WatcherToken,
    is_vertical: bool,
    cava: Option<Arc<CavaService>>,
    config: Arc<ConfigService>,
}

#[relm4::component(pub(crate))]
impl Component for CavaModule {
    type Init = CavaInit;
    type Input = ();
    type Output = ();
    type CommandOutput = CavaCmd;

    view! {
        gtk::Box {
            add_css_class: "cava",

            #[local_ref]
            container -> gtk::Box {
                #[local_ref]
                drawing_area -> gtk::DrawingArea {},
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let is_vertical = init.settings.is_vertical.get();
        let config = &init.config;
        let full_config = config.config();
        let cava_config = &full_config.modules.cava;
        let styling_config = &full_config.styling;
        let bar_config = &full_config.bar;

        let container = BarContainer::builder()
            .launch(BarContainerInit {
                colors: BarContainerColors {
                    background: cava_config.button_bg_color.clone(),
                    border_color: cava_config.border_color.clone(),
                },
                behavior: BarContainerBehavior {
                    show_border: cava_config.border_show.clone(),
                    visible: ConfigProperty::new(true),
                },
                is_vertical: init.settings.is_vertical.clone(),
                theme_provider: styling_config.theme_provider.clone(),
                border_width: bar_config.button_border_width.clone(),
                border_location: bar_config.button_border_location.clone(),
            })
            .detach();

        let bars = cava_config.bars.get().value();
        let bar_width = cava_config.bar_width.get();
        let bar_gap = cava_config.bar_gap.get();
        let bar_scale = bar_config.scale.get().value();
        let internal_padding = cava_config.internal_padding.get().value();
        let padding_px = helpers::rem_to_px(internal_padding, bar_scale);

        let drawing_area = gtk::DrawingArea::new();
        let length = helpers::calculate_widget_length(bars, bar_width, bar_gap, padding_px);

        if is_vertical {
            drawing_area.set_size_request(-1, length);
            drawing_area.set_hexpand(true);
        } else {
            drawing_area.set_size_request(length, -1);
            drawing_area.set_vexpand(true);
        }

        let frame_data: Rc<Cell<Vec<f64>>> = Rc::new(Cell::new(vec![0.0; bars as usize]));

        Self::setup_draw_func(&drawing_area, &frame_data, is_vertical, config);

        let config_clone = init.config.clone();
        sender.oneshot_command(async move {
            match helpers::build_cava_service(&config_clone).await {
                Ok(service) => CavaCmd::ServiceReady(service),
                Err(err) => {
                    error!(error = %err, "cava service failed to start");
                    CavaCmd::ServiceFailed
                }
            }
        });

        watchers::spawn_config_watchers(&sender, init.settings.is_vertical, &init.config);

        let model = Self {
            container,
            drawing_area: drawing_area.clone(),
            frame_data,
            frame_watcher: WatcherToken::new(),
            is_vertical,
            cava: None,
            config: init.config.clone(),
        };
        let container = model.container.widget();
        let drawing_area = &model.drawing_area;
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_cmd(&mut self, msg: CavaCmd, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            CavaCmd::ServiceReady(service) => {
                info!("cava service started");
                let token = self.frame_watcher.reset();
                watchers::spawn_frame_watcher(&sender, &service, token);
                self.cava = Some(service);
            }
            CavaCmd::ServiceFailed => {}
            CavaCmd::ServiceConfigChanged => {
                self.cava = None;
                let bars = self.config.config().modules.cava.bars.get().value();
                self.frame_data.set(vec![0.0; bars as usize]);
                self.update_size();
                Self::setup_draw_func(
                    &self.drawing_area,
                    &self.frame_data,
                    self.is_vertical,
                    &self.config,
                );

                let config_clone = self.config.clone();
                sender.oneshot_command(async move {
                    match helpers::build_cava_service(&config_clone).await {
                        Ok(service) => CavaCmd::ServiceReady(service),
                        Err(err) => {
                            error!(error = %err, "cava service restart failed");
                            CavaCmd::ServiceFailed
                        }
                    }
                });
            }
            CavaCmd::Frame(values) => {
                self.frame_data.set(values);
                self.drawing_area.queue_draw();
            }
            CavaCmd::StylingChanged => {
                self.update_size();
                Self::setup_draw_func(
                    &self.drawing_area,
                    &self.frame_data,
                    self.is_vertical,
                    &self.config,
                );
                self.drawing_area.queue_draw();
            }
            CavaCmd::OrientationChanged(is_vertical) => {
                self.is_vertical = is_vertical;
                self.update_size();
                Self::setup_draw_func(
                    &self.drawing_area,
                    &self.frame_data,
                    self.is_vertical,
                    &self.config,
                );
                self.drawing_area.queue_draw();
            }
        }
    }
}

impl CavaModule {
    fn setup_draw_func(
        drawing_area: &gtk::DrawingArea,
        frame_data: &Rc<Cell<Vec<f64>>>,
        is_vertical: bool,
        config: &Arc<ConfigService>,
    ) {
        let frame_ref = frame_data.clone();
        let full_config = config.config();
        let cava_config = &full_config.modules.cava;

        let style = cava_config.style.get();
        let direction = cava_config.direction.get();
        let bar_width = cava_config.bar_width.get() as f64;
        let bar_gap = cava_config.bar_gap.get() as f64;
        let bar_scale = full_config.bar.scale.get().value();
        let color = helpers::resolve_rgba(&cava_config.color.get(), config);
        let padding_rem = cava_config.internal_padding.get().value();
        let internal_padding = helpers::rem_to_px(padding_rem, bar_scale);

        let draw_config = helpers::DrawConfig {
            bar_width,
            bar_gap,
            color,
            internal_padding,
        };

        let peak_state = Cell::new(Vec::<f64>::new());

        drawing_area.set_draw_func(move |_area, cr, width, height| {
            let w = width as f64;
            let h = height as f64;

            let values = frame_ref.take();
            if values.is_empty() {
                frame_ref.set(values);
                return;
            }

            let (draw_w, draw_h) = if is_vertical { (h, w) } else { (w, h) };

            if is_vertical {
                cr.translate(0.0, h);
                cr.rotate(-std::f64::consts::FRAC_PI_2);
            }

            cr.translate(draw_config.internal_padding, 0.0);

            match style {
                CavaStyle::Bars => {
                    rendering::draw_bars(cr, &values, draw_h, direction, &draw_config);
                }
                CavaStyle::Wave => {
                    rendering::draw_wave(cr, &values, draw_w, draw_h, direction, &draw_config);
                }
                CavaStyle::Peaks => {
                    let mut peaks = peak_state.take();
                    rendering::draw_peak_bars(
                        cr,
                        &values,
                        &mut peaks,
                        draw_h,
                        direction,
                        &draw_config,
                    );
                    peak_state.set(peaks);
                }
            }

            frame_ref.set(values);
        });
    }

    fn update_size(&self) {
        let full_config = self.config.config();
        let cava_config = &full_config.modules.cava;
        let bars = cava_config.bars.get().value();
        let bar_width = cava_config.bar_width.get();
        let bar_gap = cava_config.bar_gap.get();
        let bar_scale = full_config.bar.scale.get().value();
        let padding_rem = cava_config.internal_padding.get().value();
        let padding_px = helpers::rem_to_px(padding_rem, bar_scale);
        let length = helpers::calculate_widget_length(bars, bar_width, bar_gap, padding_px);

        if self.is_vertical {
            self.drawing_area.set_size_request(-1, length);
            self.drawing_area.set_hexpand(true);
            self.drawing_area.set_vexpand(false);
        } else {
            self.drawing_area.set_size_request(length, -1);
            self.drawing_area.set_vexpand(true);
            self.drawing_area.set_hexpand(false);
        }
    }
}
