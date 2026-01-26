//! Pure helper functions for bar container.

use wayle_config::schemas::bar::BorderLocation;

use super::types::BarContainerClass;

pub(crate) fn compute_css_classes(
    is_vertical: bool,
    show_border: bool,
    border_location: BorderLocation,
) -> Vec<&'static str> {
    let mut classes = vec![BarContainerClass::BASE];
    if is_vertical {
        classes.push(BarContainerClass::VERTICAL);
    }
    if show_border && let Some(border_class) = border_location.css_class() {
        classes.push(border_class);
    }
    classes
}

pub(super) fn compute_orientation(is_vertical: bool) -> gtk4::Orientation {
    if is_vertical {
        gtk4::Orientation::Vertical
    } else {
        gtk4::Orientation::Horizontal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_classes_horizontal_no_border() {
        let classes = compute_css_classes(false, false, BorderLocation::None);
        assert_eq!(classes, vec![BarContainerClass::BASE]);
    }

    #[test]
    fn css_classes_vertical_no_border() {
        let classes = compute_css_classes(true, false, BorderLocation::None);
        assert_eq!(
            classes,
            vec![BarContainerClass::BASE, BarContainerClass::VERTICAL]
        );
    }

    #[test]
    fn css_classes_horizontal_with_border_top() {
        let classes = compute_css_classes(false, true, BorderLocation::Top);
        assert_eq!(classes, vec![BarContainerClass::BASE, "border-top"]);
    }

    #[test]
    fn css_classes_vertical_with_border_bottom() {
        let classes = compute_css_classes(true, true, BorderLocation::Bottom);
        assert_eq!(
            classes,
            vec![
                BarContainerClass::BASE,
                BarContainerClass::VERTICAL,
                "border-bottom"
            ]
        );
    }

    #[test]
    fn css_classes_border_show_true_but_location_none() {
        let classes = compute_css_classes(false, true, BorderLocation::None);
        assert_eq!(classes, vec![BarContainerClass::BASE]);
    }

    #[test]
    fn css_classes_border_show_false_ignores_location() {
        let classes = compute_css_classes(false, false, BorderLocation::All);
        assert_eq!(classes, vec![BarContainerClass::BASE]);
    }
}
