mod factory;
mod helpers;
mod messages;
mod watchers;

use std::{cell::Cell, rc::Rc, sync::Arc};

use gtk4::prelude::*;
use relm4::prelude::*;
use wayle_config::{ConfigProperty, ConfigService, schemas::styling::CssToken};
use wayle_widgets::prelude::{
    BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput, BarButtonOutput,
};

pub(crate) use self::{
    factory::Factory,
    messages::{CpuCmd, CpuInit, CpuMsg},
};
use crate::shell::bar::dropdowns::{self, DropdownRegistry};

pub(crate) struct CpuModule {
    bar_button: Controller<BarButton>,
    drawing_area: Option<gtk4::DrawingArea>,
    core_values: Rc<Cell<Vec<f64>>>,
    config: Arc<ConfigService>,
    dropdowns: Rc<DropdownRegistry>,
}

#[relm4::component(pub(crate))]
impl Component for CpuModule {
    type Init = CpuInit;
    type Input = CpuMsg;
    type Output = ();
    type CommandOutput = CpuCmd;

    view! {
        gtk::Box {
            add_css_class: "cpu",

            #[local_ref]
            bar_button -> gtk::MenuButton {},
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let config = init.config.config();
        let cpu_config = &config.modules.cpu;
        let format_str = cpu_config.format.get();
        let has_barchart = helpers::has_barchart_directive(&format_str);

        let initial_label = if has_barchart {
            String::new()
        } else {
            helpers::format_label(&format_str, &init.sysinfo.cpu.get())
        };

        let cpu_data = init.sysinfo.cpu.get();
        let num_cores = cpu_data.cores.len();
        let core_values: Rc<Cell<Vec<f64>>> = Rc::new(Cell::new(vec![0.0; num_cores]));
        let is_vertical = init.settings.is_vertical.get();

        let drawing_area = if has_barchart {
            let area = gtk4::DrawingArea::new();

            helpers::setup_barchart_draw_func(&area, &core_values, &init.config);
            helpers::update_barchart_size(&area, num_cores, &init.config, is_vertical);

            Some(area)
        } else {
            None
        };

        watchers::spawn_watchers(&sender, cpu_config, &init.sysinfo, has_barchart);

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: cpu_config.icon_name.get().clone(),
                label: initial_label,
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: cpu_config.icon_color.clone(),
                    label_color: cpu_config.label_color.clone(),
                    icon_background: cpu_config.icon_bg_color.clone(),
                    button_background: cpu_config.button_bg_color.clone(),
                    border_color: cpu_config.border_color.clone(),
                    auto_icon_color: CssToken::Blue,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: cpu_config.label_max_length.clone(),
                    show_icon: cpu_config.icon_show.clone(),
                    show_label: cpu_config.label_show.clone(),
                    show_border: cpu_config.border_show.clone(),
                    visible: ConfigProperty::new(true),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => CpuMsg::LeftClick,
                BarButtonOutput::RightClick => CpuMsg::RightClick,
                BarButtonOutput::MiddleClick => CpuMsg::MiddleClick,
                BarButtonOutput::ScrollUp => CpuMsg::ScrollUp,
                BarButtonOutput::ScrollDown => CpuMsg::ScrollDown,
            });

        let model = Self {
            bar_button,
            drawing_area: drawing_area.clone(),
            core_values,
            config: init.config,
            dropdowns: init.dropdowns,
        };
        let bar_button = model.bar_button.widget();

        if let Some(ref area) = model.drawing_area {
            if let Some(button_box) = bar_button.child().and_downcast::<gtk4::Box>() {
                // Find the label_container (second child after icon_container)
                let mut child = button_box.first_child();
                while let Some(current) = child {
                    if current.css_classes().iter().any(|c| c == "label-container") {
                        if let Some(label_container) = current.downcast_ref::<gtk4::Box>() {
                            label_container.append(area);
                            break;
                        }
                    }
                    child = current.next_sibling();
                }
            }
        }

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let cpu_config = &self.config.config().modules.cpu;

        let action = match msg {
            CpuMsg::LeftClick => cpu_config.left_click.get(),
            CpuMsg::RightClick => cpu_config.right_click.get(),
            CpuMsg::MiddleClick => cpu_config.middle_click.get(),
            CpuMsg::ScrollUp => cpu_config.scroll_up.get(),
            CpuMsg::ScrollDown => cpu_config.scroll_down.get(),
        };

        dropdowns::dispatch_click(&action, &self.dropdowns, &self.bar_button);
    }

    fn update_cmd(&mut self, msg: CpuCmd, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            CpuCmd::UpdateLabel(label) => {
                self.bar_button.emit(BarButtonInput::SetLabel(label));
            }
            CpuCmd::UpdateIcon(icon) => {
                self.bar_button.emit(BarButtonInput::SetIcon(icon));
            }
            CpuCmd::UpdateBarchart(core_values) => {
                if let Some(ref area) = self.drawing_area {
                    // print all core values to stdout:
                    println!("{:?}", core_values);
                    self.core_values.set(core_values);
                    area.queue_draw();
                }
            }
        }
    }
}
