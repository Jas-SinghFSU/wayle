use std::fmt::{Display, Formatter, Result};

pub(crate) type RawPixmap = (i32, i32, Vec<u8>);
pub(crate) type RawPixmaps = Vec<RawPixmap>;
pub(crate) type RawTooltip = (String, RawPixmaps, String, String);

/// Describes the category of a StatusNotifierItem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Category {
    /// The item describes the status of a generic application.
    #[default]
    ApplicationStatus,
    /// The item describes the status of communication oriented applications.
    Communications,
    /// The item describes services of the system not seen as a stand alone application.
    SystemServices,
    /// The item describes the state and control of a particular hardware.
    Hardware,
}

impl From<&str> for Category {
    fn from(s: &str) -> Self {
        match s {
            "ApplicationStatus" => Self::ApplicationStatus,
            "Communications" => Self::Communications,
            "SystemServices" => Self::SystemServices,
            "Hardware" => Self::Hardware,
            _ => Self::ApplicationStatus,
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::ApplicationStatus => write!(f, "ApplicationStatus"),
            Self::Communications => write!(f, "Communications"),
            Self::SystemServices => write!(f, "SystemServices"),
            Self::Hardware => write!(f, "Hardware"),
        }
    }
}

/// Describes the status of a StatusNotifierItem or its associated application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Status {
    /// The item doesn't convey important information to the user.
    #[default]
    Passive,
    /// The item is active and should be shown to the user.
    Active,
    /// The item carries really important information for the user.
    NeedsAttention,
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        match s {
            "Passive" => Self::Passive,
            "Active" => Self::Active,
            "NeedsAttention" => Self::NeedsAttention,
            _ => Self::Passive,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Passive => write!(f, "Passive"),
            Self::Active => write!(f, "Active"),
            Self::NeedsAttention => write!(f, "NeedsAttention"),
        }
    }
}

/// Icon pixmap data for a StatusNotifierItem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconPixmap {
    /// Width of the icon in pixels.
    pub width: i32,
    /// Height of the icon in pixels.
    pub height: i32,
    /// ARGB32 binary data in network byte order.
    pub data: Vec<u8>,
}

impl From<(i32, i32, Vec<u8>)> for IconPixmap {
    fn from(tuple: (i32, i32, Vec<u8>)) -> Self {
        Self {
            width: tuple.0,
            height: tuple.1,
            data: tuple.2,
        }
    }
}

impl From<IconPixmap> for (i32, i32, Vec<u8>) {
    fn from(pixmap: IconPixmap) -> Self {
        (pixmap.width, pixmap.height, pixmap.data)
    }
}

/// Tooltip information for a StatusNotifierItem.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Tooltip {
    /// Freedesktop-compliant name for an icon.
    pub icon_name: String,
    /// Icon data as pixmaps.
    pub icon_pixmap: Vec<IconPixmap>,
    /// Title for this tooltip.
    pub title: String,
    /// Descriptive text for this tooltip (may contain HTML markup).
    pub description: String,
}

impl From<(String, Vec<(i32, i32, Vec<u8>)>, String, String)> for Tooltip {
    fn from(tuple: (String, Vec<(i32, i32, Vec<u8>)>, String, String)) -> Self {
        Self {
            icon_name: tuple.0,
            icon_pixmap: tuple.1.into_iter().map(IconPixmap::from).collect(),
            title: tuple.2,
            description: tuple.3,
        }
    }
}

/// Scroll orientation for StatusNotifierItem scroll events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollOrientation {
    /// Horizontal scroll.
    Horizontal,
    /// Vertical scroll.
    Vertical,
}

impl From<&str> for ScrollOrientation {
    fn from(s: &str) -> Self {
        match s {
            "horizontal" => Self::Horizontal,
            "vertical" => Self::Vertical,
            _ => Self::Vertical,
        }
    }
}

impl Display for ScrollOrientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Horizontal => write!(f, "horizontal"),
            Self::Vertical => write!(f, "vertical"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_from_str_with_application_status_returns_correct_variant() {
        let category = Category::from("ApplicationStatus");
        assert_eq!(category, Category::ApplicationStatus);
    }

    #[test]
    fn category_from_str_with_communications_returns_correct_variant() {
        let category = Category::from("Communications");
        assert_eq!(category, Category::Communications);
    }

    #[test]
    fn category_from_str_with_system_services_returns_correct_variant() {
        let category = Category::from("SystemServices");
        assert_eq!(category, Category::SystemServices);
    }

    #[test]
    fn category_from_str_with_hardware_returns_correct_variant() {
        let category = Category::from("Hardware");
        assert_eq!(category, Category::Hardware);
    }

    #[test]
    fn category_from_str_with_unknown_value_returns_default() {
        let category = Category::from("UnknownCategory");
        assert_eq!(category, Category::ApplicationStatus);
    }

    #[test]
    fn status_from_str_with_passive_returns_correct_variant() {
        let status = Status::from("Passive");
        assert_eq!(status, Status::Passive);
    }

    #[test]
    fn status_from_str_with_active_returns_correct_variant() {
        let status = Status::from("Active");
        assert_eq!(status, Status::Active);
    }

    #[test]
    fn status_from_str_with_needs_attention_returns_correct_variant() {
        let status = Status::from("NeedsAttention");
        assert_eq!(status, Status::NeedsAttention);
    }

    #[test]
    fn status_from_str_with_unknown_value_returns_default() {
        let status = Status::from("UnknownStatus");
        assert_eq!(status, Status::Passive);
    }

    #[test]
    fn scroll_orientation_from_str_with_horizontal_returns_correct_variant() {
        let orientation = ScrollOrientation::from("horizontal");
        assert_eq!(orientation, ScrollOrientation::Horizontal);
    }

    #[test]
    fn scroll_orientation_from_str_with_vertical_returns_correct_variant() {
        let orientation = ScrollOrientation::from("vertical");
        assert_eq!(orientation, ScrollOrientation::Vertical);
    }

    #[test]
    fn scroll_orientation_from_str_with_unknown_value_returns_default() {
        let orientation = ScrollOrientation::from("diagonal");
        assert_eq!(orientation, ScrollOrientation::Vertical);
    }

    #[test]
    fn icon_pixmap_from_tuple_creates_correct_structure() {
        let tuple = (32, 32, vec![1, 2, 3, 4]);
        let pixmap = IconPixmap::from(tuple);

        assert_eq!(pixmap.width, 32);
        assert_eq!(pixmap.height, 32);
        assert_eq!(pixmap.data, vec![1, 2, 3, 4]);
    }

    #[test]
    fn icon_pixmap_to_tuple_returns_correct_values() {
        let pixmap = IconPixmap {
            width: 64,
            height: 48,
            data: vec![5, 6, 7, 8],
        };
        let tuple: (i32, i32, Vec<u8>) = pixmap.into();

        assert_eq!(tuple.0, 64);
        assert_eq!(tuple.1, 48);
        assert_eq!(tuple.2, vec![5, 6, 7, 8]);
    }

    #[test]
    fn tooltip_from_tuple_with_empty_pixmaps_creates_correct_structure() {
        let tuple = (
            String::from("icon-name"),
            vec![],
            String::from("Title"),
            String::from("Description"),
        );
        let tooltip = Tooltip::from(tuple);

        assert_eq!(tooltip.icon_name, "icon-name");
        assert!(tooltip.icon_pixmap.is_empty());
        assert_eq!(tooltip.title, "Title");
        assert_eq!(tooltip.description, "Description");
    }

    #[test]
    fn tooltip_from_tuple_with_multiple_pixmaps_converts_all() {
        let tuple = (
            String::from("icon"),
            vec![(16, 16, vec![1, 2]), (32, 32, vec![3, 4])],
            String::from("Title"),
            String::from("Desc"),
        );
        let tooltip = Tooltip::from(tuple);

        assert_eq!(tooltip.icon_pixmap.len(), 2);
        assert_eq!(tooltip.icon_pixmap[0].width, 16);
        assert_eq!(tooltip.icon_pixmap[0].height, 16);
        assert_eq!(tooltip.icon_pixmap[1].width, 32);
        assert_eq!(tooltip.icon_pixmap[1].height, 32);
    }
}
