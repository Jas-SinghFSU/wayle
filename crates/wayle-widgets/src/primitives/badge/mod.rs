//! Badge widget templates for status indicators and labels.
#![allow(missing_docs)]

use gtk4::prelude::WidgetExt;
use relm4::WidgetTemplate;
use relm4::gtk;

/// Filled badge with accent background.
#[relm4::widget_template(pub)]
impl WidgetTemplate for Badge {
    view! {
        gtk::Label {
            set_css_classes: &["badge"],
        }
    }
}

/// Filled badge with success status background.
#[relm4::widget_template(pub)]
impl WidgetTemplate for SuccessBadge {
    view! {
        gtk::Label {
            set_css_classes: &["badge", "success"],
        }
    }
}

/// Filled badge with warning status background.
#[relm4::widget_template(pub)]
impl WidgetTemplate for WarningBadge {
    view! {
        gtk::Label {
            set_css_classes: &["badge", "warning"],
        }
    }
}

/// Filled badge with error status background.
#[relm4::widget_template(pub)]
impl WidgetTemplate for ErrorBadge {
    view! {
        gtk::Label {
            set_css_classes: &["badge", "error"],
        }
    }
}

/// Filled badge with info status background.
#[relm4::widget_template(pub)]
impl WidgetTemplate for InfoBadge {
    view! {
        gtk::Label {
            set_css_classes: &["badge", "info"],
        }
    }
}

/// Subtle badge with tinted accent background.
#[relm4::widget_template(pub)]
impl WidgetTemplate for SubtleBadge {
    view! {
        gtk::Label {
            set_css_classes: &["badge-subtle"],
        }
    }
}

/// Subtle badge with tinted success background.
#[relm4::widget_template(pub)]
impl WidgetTemplate for SubtleSuccessBadge {
    view! {
        gtk::Label {
            set_css_classes: &["badge-subtle", "success"],
        }
    }
}

/// Subtle badge with tinted warning background.
#[relm4::widget_template(pub)]
impl WidgetTemplate for SubtleWarningBadge {
    view! {
        gtk::Label {
            set_css_classes: &["badge-subtle", "warning"],
        }
    }
}

/// Subtle badge with tinted error background.
#[relm4::widget_template(pub)]
impl WidgetTemplate for SubtleErrorBadge {
    view! {
        gtk::Label {
            set_css_classes: &["badge-subtle", "error"],
        }
    }
}

/// Subtle badge with tinted info background.
#[relm4::widget_template(pub)]
impl WidgetTemplate for SubtleInfoBadge {
    view! {
        gtk::Label {
            set_css_classes: &["badge-subtle", "info"],
        }
    }
}
