//! SVG transformation for GTK symbolic icon compatibility.
//!
//! # Why This Exists
//!
//! GTK has a "symbolic icon" system that lets icons change color via CSS.
//! For example, setting `color: red` on a widget makes its icon red.
//! This is how icons adapt to light/dark themes automatically.
//!
//! However, GTK's symbolic icon system has strict requirements that most
//! icon libraries (Tabler, Lucide, etc.) don't follow out of the box.
//! Without transformation, icons render as solid colored squares instead
//! of their actual shapes.
//!
//! # What This Module Does
//!
//! Transforms standard SVG icons into GTK-compatible symbolic icons:
//!
//! 1. **Coordinate Scaling**: GTK expects 16x16 native coordinates.
//!    Most icon libraries use 24x24. We scale all path coordinates
//!    and stroke widths proportionally.
//!
//! 2. **GTK Grappa Namespace**: Adds `xmlns:gpa` namespace and attributes
//!    like `gpa:stroke="foreground"` that tell GTK which parts should
//!    be recolored by CSS.
//!
//! 3. **Path-Level Attributes**: Moves stroke/fill attributes from the
//!    `<svg>` element onto each `<path>` element. GTK's recoloring only
//!    works when attributes are directly on paths.
//!
//! # Example Transformation
//!
//! Input (Tabler icon, 24x24):
//! ```xml
//! <svg viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
//!   <path d="M5 12 l5 5 l10 -10"/>
//! </svg>
//! ```
//!
//! Output (GTK symbolic, 16x16):
//! ```xml
//! <svg width='16' height='16' xmlns:gpa='https://www.gtk.org/grappa'>
//!   <path d='M3.33 8 l3.33 3.33 l6.67 -6.67'
//!         stroke='rgb(0,0,0)' stroke-width='1.33'
//!         gpa:stroke='foreground'/>
//! </svg>
//! ```
//!
//! Notice the path coordinates are scaled by 16/24 (≈0.667):
//! - `M5 12` becomes `M3.33 8` (move to scaled position)
//! - `l5 5` becomes `l3.33 3.33` (line by scaled amount)
//! - `stroke-width="2"` becomes `stroke-width='1.33'`
//!
//! The `gpa:stroke="foreground"` attribute is what enables CSS recoloring.

const TARGET_SIZE: f64 = 16.0;
const DEFAULT_STROKE_WIDTH: f64 = 2.0;

/// Icon coloring strategy detected from the source SVG.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconStyle {
    /// Uses `stroke="currentColor"` (e.g., Tabler icons)
    Stroke,
    /// Uses `fill="currentColor"` (e.g., Simple Icons)
    Fill,
}

/// A single path extracted from the source SVG.
#[derive(Debug)]
struct ExtractedPath {
    d: String,
    is_visible: bool,
}

/// Converts an SVG icon to GTK symbolic format.
///
/// Scales 24x24 icons to 16x16 and adds GTK Grappa attributes
/// for CSS color recoloring.
pub fn to_symbolic(svg_content: &str) -> String {
    let style = detect_icon_style(svg_content);
    let scale = calculate_scale_factor(svg_content);
    let stroke_width = extract_stroke_width(svg_content) * scale;
    let paths = extract_paths(svg_content);

    build_gtk_svg(&paths, style, scale, stroke_width)
}

fn detect_icon_style(content: &str) -> IconStyle {
    if content.contains(r#"stroke="currentColor""#) {
        IconStyle::Stroke
    } else {
        IconStyle::Fill
    }
}

fn calculate_scale_factor(content: &str) -> f64 {
    let Some(source_size) = detect_source_size(content) else {
        return 1.0;
    };

    TARGET_SIZE / source_size
}

fn detect_source_size(content: &str) -> Option<f64> {
    extract_viewbox_width(content).or_else(|| extract_width_attr(content))
}

fn extract_viewbox_width(content: &str) -> Option<f64> {
    let viewbox = extract_attribute(content, "viewBox")?;
    let parts: Vec<&str> = viewbox.split_whitespace().collect();

    if parts.len() >= 3 {
        parts[2].parse().ok()
    } else {
        None
    }
}

fn extract_width_attr(content: &str) -> Option<f64> {
    extract_attribute(content, "width")?.parse().ok()
}

fn extract_stroke_width(content: &str) -> f64 {
    extract_attribute(content, "stroke-width")
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_STROKE_WIDTH)
}

fn extract_attribute(content: &str, name: &str) -> Option<String> {
    for quote in ['"', '\''] {
        let pattern = format!("{name}={quote}");

        let Some(start) = content.find(&pattern) else {
            continue;
        };

        let value_start = start + pattern.len();

        let Some(value_end) = content[value_start..].find(quote) else {
            continue;
        };

        return Some(content[value_start..value_start + value_end].to_string());
    }
    None
}

fn extract_paths(content: &str) -> Vec<ExtractedPath> {
    let mut paths = Vec::new();

    for (start, _) in content.match_indices("<path ") {
        let Some(end) = content[start..].find("/>") else {
            continue;
        };
        let element = &content[start..start + end + 2];

        let Some(d) = extract_attribute(element, "d") else {
            continue;
        };

        let is_visible = !is_transparent_path(element);
        paths.push(ExtractedPath { d, is_visible });
    }

    paths
}

fn is_transparent_path(element: &str) -> bool {
    let has_no_stroke = element.contains(r#"stroke="none""#);
    let has_no_fill = element.contains(r#"fill="none""#);

    has_no_stroke && has_no_fill
}

fn build_gtk_svg(
    paths: &[ExtractedPath],
    style: IconStyle,
    scale: f64,
    stroke_width: f64,
) -> String {
    let mut output = String::new();

    output.push_str("<svg width='16' height='16'\n");
    output.push_str("     xmlns:gpa='https://www.gtk.org/grappa'\n");
    output.push_str("     gpa:version='1'>\n");

    for path in paths.iter().filter(|p| p.is_visible) {
        let scaled_d = scale_path_data(&path.d, scale);
        let path_element = build_path_element(&scaled_d, style, stroke_width);
        output.push_str(&path_element);
    }

    output.push_str("</svg>\n");
    output
}

fn build_path_element(d: &str, style: IconStyle, stroke_width: f64) -> String {
    match style {
        IconStyle::Stroke => format!(
            "  <path d='{d}'\n\
             \x20       stroke-width='{stroke_width:.2}'\n\
             \x20       stroke-linecap='round'\n\
             \x20       stroke-linejoin='round'\n\
             \x20       stroke='rgb(0,0,0)'\n\
             \x20       fill='none'\n\
             \x20       class='foreground-stroke transparent-fill'\n\
             \x20       gpa:stroke='foreground'/>\n"
        ),
        IconStyle::Fill => format!(
            "  <path d='{d}'\n\
             \x20       stroke='none'\n\
             \x20       fill='rgb(0,0,0)'\n\
             \x20       class='foreground-fill'\n\
             \x20       gpa:fill='foreground'/>\n"
        ),
    }
}

fn scale_path_data(d: &str, scale: f64) -> String {
    if is_scale_unnecessary(scale) {
        return d.to_string();
    }

    let mut result = String::with_capacity(d.len());
    let mut parser = PathDataParser::new(d, scale);

    while let Some(token) = parser.next_token() {
        result.push_str(&token);
    }

    result
}

fn is_scale_unnecessary(scale: f64) -> bool {
    (scale - 1.0).abs() < 0.001
}

struct PathDataParser<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    scale: f64,
    current_command: char,
    param_index: usize,
}

impl<'a> PathDataParser<'a> {
    fn new(data: &'a str, scale: f64) -> Self {
        Self {
            chars: data.chars().peekable(),
            scale,
            current_command: ' ',
            param_index: 0,
        }
    }

    fn next_token(&mut self) -> Option<String> {
        let ch = *self.chars.peek()?;

        if ch.is_ascii_alphabetic() {
            Some(self.parse_command())
        } else if ch == '-' || ch == '.' || ch.is_ascii_digit() {
            Some(self.parse_number())
        } else {
            self.chars.next();
            Some(ch.to_string())
        }
    }

    fn parse_command(&mut self) -> String {
        let cmd = self.chars.next().unwrap_or(' ');
        self.current_command = cmd.to_ascii_lowercase();
        self.param_index = 0;
        cmd.to_string()
    }

    fn parse_number(&mut self) -> String {
        let raw = self.collect_number_chars();

        let Ok(value) = raw.parse::<f64>() else {
            return raw;
        };

        let scaled = if self.should_scale_current_param() {
            value * self.scale
        } else {
            value
        };

        self.param_index += 1;
        format_scaled_number(scaled)
    }

    fn collect_number_chars(&mut self) -> String {
        let mut chars = String::new();

        if let Some(&ch) = self.chars.peek() {
            if ch == '-' {
                chars.push(self.chars.next().unwrap_or('-'));
            }
        }

        while let Some(&ch) = self.chars.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                chars.push(self.chars.next().unwrap_or('0'));
            } else {
                break;
            }
        }

        chars
    }

    fn should_scale_current_param(&self) -> bool {
        match self.current_command {
            'a' => !is_arc_flag_param(self.param_index),
            _ => true,
        }
    }
}

fn is_arc_flag_param(index: usize) -> bool {
    matches!(index % 7, 2..=4)
}

fn format_scaled_number(value: f64) -> String {
    let rounded = value.round();

    if (value - rounded).abs() < 0.01 {
        format!("{}", rounded as i32)
    } else {
        format!("{value:.2}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_stroke_style() {
        let svg = r#"<svg stroke="currentColor"><path d="M0 0"/></svg>"#;
        assert_eq!(detect_icon_style(svg), IconStyle::Stroke);
    }

    #[test]
    fn detects_fill_style() {
        let svg = r#"<svg fill="currentColor"><path d="M0 0"/></svg>"#;
        assert_eq!(detect_icon_style(svg), IconStyle::Fill);
    }

    #[test]
    fn calculates_scale_from_viewbox() {
        let svg = r#"<svg viewBox="0 0 24 24"><path d="M0 0"/></svg>"#;
        let scale = calculate_scale_factor(svg);
        assert!((scale - (16.0 / 24.0)).abs() < 0.001);
    }

    #[test]
    fn scales_path_coordinates() {
        let scaled = scale_path_data("M12 12", 0.5);
        assert_eq!(scaled, "M6 6");
    }

    #[test]
    fn preserves_arc_flags() {
        let scaled = scale_path_data("a6 6 0 1 0 12 0", 16.0 / 24.0);
        assert!(scaled.contains(" 0 1 0 "));
    }
}
