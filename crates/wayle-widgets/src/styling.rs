//! Inline CSS styling traits and helpers for Wayle components.
//!
//! Components with runtime-configurable styling implement [`InlineStyling`]
//! to standardize the CSS custom property injection pattern.

use std::borrow::Cow;

use gtk4 as gtk;
use wayle_common::ConfigProperty;
use wayle_config::schemas::styling::ColorValue;

/// Trait for components that inject CSS custom properties at runtime.
///
/// Implementors provide a `CssProvider` and a method to build CSS strings
/// containing custom property definitions. The trait provides a default
/// `reload_css` implementation that recompiles and reloads styles.
///
/// # Pattern
///
/// 1. Rust reads config values and injects them as CSS custom properties
/// 2. SCSS defines styling rules that consume those properties via `var()`
/// 3. Watchers trigger `reload_css()` when config changes
///
/// # Example
///
/// ```rust,ignore
/// impl InlineStyling for MyComponent {
///     fn css_provider(&self) -> &gtk::CssProvider {
///         &self.css_provider
///     }
///
///     fn build_css(&self) -> String {
///         let config = services::get::<ConfigService>().config();
///         format!(".my-component {{ --my-bg: {}; }}", config.my.bg.get())
///     }
/// }
/// ```
pub trait InlineStyling {
    /// Returns a reference to the component's CSS provider.
    fn css_provider(&self) -> &gtk::CssProvider;

    /// Builds CSS string containing custom property definitions.
    ///
    /// The returned CSS should define custom properties scoped to the
    /// component's selector (e.g., `.bar { --bar-bg: #fff; }`).
    fn build_css(&self) -> String;

    /// Recompiles CSS and loads it into the provider.
    ///
    /// Call this when config properties change to update the component's
    /// visual appearance.
    fn reload_css(&self) {
        self.css_provider().load_from_string(&self.build_css());
    }
}

/// Resolves a color config property based on theme provider context.
///
/// When using Wayle's built-in theme, returns the user-configured color.
/// When using an external GTK theme, returns the default color to avoid
/// clashing with the theme's color scheme.
pub fn resolve_color(prop: &ConfigProperty<ColorValue>, is_wayle_theme: bool) -> Cow<'static, str> {
    if is_wayle_theme {
        prop.get().to_css()
    } else {
        prop.default().to_css()
    }
}
