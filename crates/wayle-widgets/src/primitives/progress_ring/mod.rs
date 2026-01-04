//! Circular progress ring with customizable center content.

use std::{cell::Cell, f64::consts::PI, rc::Rc};

use gtk::{cairo, gdk::RGBA, prelude::*};
use relm4::prelude::*;

/// Size variants for the progress ring.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Size {
    /// 2rem diameter, 2px stroke
    Sm,
    /// 3rem diameter, 3px stroke (default)
    #[default]
    Md,
    /// 4rem diameter, 4px stroke
    Lg,
    /// 5rem diameter, 5px stroke
    Xl,
    /// 6rem diameter, 6px stroke
    Xxl,
    /// 7rem diameter, 7px stroke
    Xxxl,
}

impl Size {
    fn css_class(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
            Self::Xxl => "xxl",
            Self::Xxxl => "xxxl",
        }
    }
}

/// Color variants for the progress ring.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColorVariant {
    /// Uses the accent color.
    #[default]
    Default,
    /// Green success indicator.
    Success,
    /// Yellow/orange warning indicator.
    Warning,
    /// Red error indicator.
    Error,
}

impl ColorVariant {
    fn css_class(self) -> Option<&'static str> {
        match self {
            Self::Default => None,
            Self::Success => Some("success"),
            Self::Warning => Some("warning"),
            Self::Error => Some("error"),
        }
    }
}

/// Configuration for creating a progress ring.
#[derive(Debug, Clone, Default)]
pub struct ProgressRingInit {
    /// Progress fraction from 0.0 to 1.0.
    pub fraction: f64,
    /// Size variant controlling diameter and stroke width.
    pub size: Size,
    /// Color variant for semantic meaning.
    pub color: ColorVariant,
}

/// Messages for updating the progress ring state.
#[derive(Debug)]
pub enum ProgressRingMsg {
    /// Updates the progress fraction (clamped to 0.0-1.0).
    SetFraction(f64),
    /// Updates the center label text.
    SetLabel(String),
    /// Updates the color variant.
    SetColor(ColorVariant),
}

/// Circular progress ring with Cairo-drawn arcs and optional center label.
pub struct ProgressRing {
    fraction: Rc<Cell<f64>>,
    label_text: String,
    current_color: ColorVariant,
    drawing_area: gtk::DrawingArea,
}

#[allow(missing_docs)]
#[relm4::component(pub)]
impl SimpleComponent for ProgressRing {
    type Init = ProgressRingInit;
    type Input = ProgressRingMsg;
    type Output = ();

    view! {
        #[root]
        gtk::Overlay {
            set_css_classes: &["progress-ring"],

            #[local_ref]
            drawing_area -> gtk::DrawingArea {
                set_hexpand: true,
                set_vexpand: true,
            },

            add_overlay = &gtk::Label {
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::Center,
                add_css_class: "ring-text",
                #[watch]
                set_label: &model.label_text,
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let fraction = Rc::new(Cell::new(init.fraction.clamp(0.0, 1.0)));
        let drawing_area = gtk::DrawingArea::new();

        drawing_area.add_css_class("progress-ring-canvas");

        let fraction_for_draw = fraction.clone();
        drawing_area.set_draw_func(move |area, cr, width, height| {
            draw_ring(area, cr, width, height, fraction_for_draw.get());
        });

        let model = ProgressRing {
            fraction,
            label_text: String::new(),
            current_color: init.color,
            drawing_area: drawing_area.clone(),
        };

        let widgets = view_output!();

        let size_class = init.size.css_class();
        root.add_css_class(size_class);
        drawing_area.add_css_class(size_class);

        if let Some(color_class) = init.color.css_class() {
            drawing_area.add_css_class(color_class);
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ProgressRingMsg::SetFraction(f) => {
                self.fraction.set(f.clamp(0.0, 1.0));
                self.drawing_area.queue_draw();
            }
            ProgressRingMsg::SetLabel(text) => {
                self.label_text = text;
            }
            ProgressRingMsg::SetColor(new_color) => {
                if new_color == self.current_color {
                    return;
                }
                if let Some(old_class) = self.current_color.css_class() {
                    self.drawing_area.remove_css_class(old_class);
                }
                if let Some(new_class) = new_color.css_class() {
                    self.drawing_area.add_css_class(new_class);
                }
                self.current_color = new_color;
                self.drawing_area.queue_draw();
            }
        }
    }
}

fn draw_ring(area: &gtk::DrawingArea, cr: &cairo::Context, width: i32, height: i32, fraction: f64) {
    let size = width.min(height) as f64;
    let center_x = width as f64 / 2.0;
    let center_y = height as f64 / 2.0;

    let stroke_width = stroke_width_from_css(area);
    let radius = (size / 2.0) - (stroke_width / 2.0);

    if radius <= 0.0 {
        return;
    }

    let fill_color = area.color();
    let track_color = RGBA::new(
        fill_color.red(),
        fill_color.green(),
        fill_color.blue(),
        fill_color.alpha() * 0.25,
    );

    cr.set_line_width(stroke_width);
    cr.set_line_cap(cairo::LineCap::Round);

    set_source_color(cr, &track_color);
    cr.arc(center_x, center_y, radius, 0.0, 2.0 * PI);
    let _ = cr.stroke();

    if fraction > 0.0 {
        set_source_color(cr, &fill_color);
        let start_angle = -PI / 2.0;
        let end_angle = start_angle + (2.0 * PI * fraction);
        cr.arc(center_x, center_y, radius, start_angle, end_angle);
        let _ = cr.stroke();
    }
}

fn set_source_color(cr: &cairo::Context, color: &RGBA) {
    cr.set_source_rgba(
        color.red().into(),
        color.green().into(),
        color.blue().into(),
        color.alpha().into(),
    );
}

#[allow(deprecated)]
fn stroke_width_from_css(widget: &gtk::DrawingArea) -> f64 {
    let style_context = widget.style_context();
    let border_width = style_context.border().top() as f64;
    if border_width > 0.0 {
        border_width
    } else {
        3.0
    }
}
