use gtk4::prelude::WidgetExt;
use relm4::WidgetTemplate;
use relm4::gtk;

#[relm4::widget_template(pub)]
impl WidgetTemplate for Switch {
    view! {
        gtk::Switch {
            set_css_classes: &["switch"],
            set_cursor_from_name: Some("pointer"),
        }
    }
}
